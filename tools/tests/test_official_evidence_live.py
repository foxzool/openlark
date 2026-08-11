import json
import os
import tempfile
import threading
import time
import unittest
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official_evidence import (
    AdapterContractError,
    EvidenceDimension,
    EvidenceSource,
    EvidenceStatus,
    FreshOfficialPolicy,
    PreferSnapshotPolicy,
    SnapshotStoreError,
    compose,
)


class _ResponseServer(ThreadingHTTPServer):
    response_status = 200
    response_body = b"{}"
    response_delay = 0.0
    declared_length = None
    requests = 0


class _Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.server.requests += 1
        if self.server.response_delay:
            time.sleep(self.server.response_delay)
        self.send_response(self.server.response_status)
        self.send_header("Content-Type", "application/json")
        if self.server.declared_length is not None:
            self.send_header(
                "Content-Length",
                str(self.server.declared_length),
            )
        self.end_headers()
        try:
            self.wfile.write(self.server.response_body)
        except BrokenPipeError:
            pass

    def log_message(self, format, *args):
        pass


@contextmanager
def detail_server(
    payload=None,
    *,
    status=200,
    body=None,
    delay=0.0,
    declared_length=None,
):
    server = _ResponseServer(("127.0.0.1", 0), _Handler)
    server.response_status = status
    server.response_body = (
        body
        if body is not None
        else json.dumps(payload, ensure_ascii=False).encode("utf-8")
    )
    server.response_delay = delay
    server.declared_length = declared_length
    server.requests = 0
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        yield server, f"http://127.0.0.1:{server.server_port}/detail"
    finally:
        server.shutdown()
        server.server_close()
        thread.join()


def structured_payload(*, path="/open-apis/bitable/v1/apps/:app_token/records"):
    return {
        "data": {
            "schema": {
                "apiSchema": {
                    "httpMethod": "POST",
                    "path": path,
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "required": ["record"],
                                    "properties": {
                                        "record": {
                                            "type": "object",
                                            "properties": {
                                                "name": {"type": "string"}
                                            },
                                        }
                                    },
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "properties": {
                                            "data": {
                                                "properties": {
                                                    "record_id": {"type": "string"}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "security": {
                        "supportedAccessToken": ["tenant_access_token"]
                    },
                }
            }
        }
    }


class LiveOfficialEvidenceCollectTests(unittest.TestCase):
    def setUp(self):
        self.api = ApiIdentity(
            api_id="42",
            name="创建记录",
            biz_tag="base",
            meta_project="bitable",
            meta_version="v1",
            meta_resource="app.table.record",
            meta_name="create",
            url="POST:/open-apis/bitable/v1/apps/{app_token}/records",
            doc_path="https://open.feishu.cn/document/mock",
            expected_file="base/bitable/v1/app/table/record/create.rs",
            full_path="/document/uAjLw4CM/mock",
        )
        self.dimensions = tuple(EvidenceDimension)

    def test_live_structured_detail_collects_every_dimension_and_reuses_snapshot(self):
        with tempfile.TemporaryDirectory() as directory:
            with detail_server(structured_payload()) as (server, url):
                collector = compose(
                    snapshot_directory=Path(directory),
                    structured_detail_url=url,
                    timeout_seconds=1,
                    retries=0,
                )
                fresh = collector.collect(
                    self.api, self.dimensions, FreshOfficialPolicy()
                )

                self.assertEqual(server.requests, 1)
                for dimension in self.dimensions:
                    evidence = fresh.for_dimension(dimension)
                    self.assertEqual(evidence.status, EvidenceStatus.TRUSTED)
                    self.assertEqual(
                        evidence.selected_source, EvidenceSource.STRUCTURED_DETAIL
                    )
                    self.assertEqual(
                        evidence.snapshot_provenance.source_uri,
                        f"{url}?fullPath=%2FuAjLw4CM%2Fmock",
                    )
                    self.assertEqual(
                        evidence.interpretation_provenance.dimension, dimension
                    )

                cached = collector.collect(
                    self.api, self.dimensions, PreferSnapshotPolicy()
                )
                self.assertEqual(server.requests, 1)
                for dimension in self.dimensions:
                    current = cached.for_dimension(dimension)
                    previous = fresh.for_dimension(dimension)
                    self.assertEqual(current.status, previous.status)
                    self.assertEqual(
                        current.snapshot_provenance.content_digest,
                        previous.snapshot_provenance.content_digest,
                    )
                    self.assertIsNot(
                        current.interpretation_provenance,
                        previous.interpretation_provenance,
                    )

    def test_fresh_policy_never_succeeds_from_an_existing_snapshot(self):
        with tempfile.TemporaryDirectory() as directory:
            with detail_server(structured_payload()) as (server, url):
                collector = compose(
                    snapshot_directory=Path(directory),
                    structured_detail_url=url,
                    timeout_seconds=0.05,
                    retries=0,
                )
                trusted = collector.collect(
                    self.api,
                    (EvidenceDimension.ENDPOINT,),
                    FreshOfficialPolicy(),
                ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(trusted.status, EvidenceStatus.TRUSTED)

            fresh = collector.collect(
                self.api,
                (EvidenceDimension.ENDPOINT,),
                FreshOfficialPolicy(),
            ).for_dimension(EvidenceDimension.ENDPOINT)
            self.assertEqual(fresh.status, EvidenceStatus.UNAVAILABLE)
            self.assertEqual(fresh.diagnostics[0].code, "acquisition_failed")

            cached = collector.collect(
                self.api,
                (EvidenceDimension.ENDPOINT,),
                PreferSnapshotPolicy(),
            ).for_dimension(EvidenceDimension.ENDPOINT)
            self.assertEqual(cached.status, EvidenceStatus.TRUSTED)

    def test_timeout_not_found_and_unhealthy_content_are_observable(self):
        cases = (
            ({"delay": 0.15}, "acquisition_timeout", EvidenceStatus.UNAVAILABLE),
            ({"status": 404, "payload": {}}, "document_not_found", EvidenceStatus.UNAVAILABLE),
            ({"body": b"not-json"}, "document_unhealthy", EvidenceStatus.REJECTED),
            (
                {"body": b"{}", "declared_length": 20},
                "acquisition_failed",
                EvidenceStatus.UNAVAILABLE,
            ),
        )
        for server_options, diagnostic, status in cases:
            with self.subTest(diagnostic=diagnostic), tempfile.TemporaryDirectory() as directory:
                payload = server_options.pop("payload", structured_payload())
                with detail_server(payload, **server_options) as (_, url):
                    evidence = compose(
                        snapshot_directory=Path(directory),
                        structured_detail_url=url,
                        timeout_seconds=0.03,
                        retries=0,
                    ).collect(
                        self.api,
                        (EvidenceDimension.ENDPOINT,),
                        FreshOfficialPolicy(),
                    ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(evidence.status, status)
                self.assertEqual(evidence.diagnostics[0].code, diagnostic)
                self.assertEqual(evidence.acquisition_trail[0].status, status)

    def test_store_is_immutable_reinterprets_and_evicts_rejected_snapshots(self):
        with tempfile.TemporaryDirectory() as directory:
            snapshot_directory = Path(directory)
            first = structured_payload(path="/open-apis/first")
            second = structured_payload(path="/open-apis/second")
            with detail_server(first) as (server, url):
                collector = compose(
                    snapshot_directory=snapshot_directory,
                    structured_detail_url=url,
                    timeout_seconds=1,
                    retries=0,
                )
                collector.collect(
                    self.api, (EvidenceDimension.ENDPOINT,), FreshOfficialPolicy()
                )
                original_files = tuple(snapshot_directory.rglob("*.json"))
                self.assertEqual(len(original_files), 1)
                original_bytes = original_files[0].read_bytes()
                original_record = json.loads(
                    original_bytes.decode("utf-8")
                )
                self.assertEqual(
                    original_record["raw_representation"],
                    json.dumps(first, ensure_ascii=False),
                )

                server.response_body = json.dumps(second).encode("utf-8")
                collector.collect(
                    self.api, (EvidenceDimension.ENDPOINT,), FreshOfficialPolicy()
                )
                self.assertEqual(original_files[0].read_bytes(), original_bytes)
                self.assertEqual(len(tuple(snapshot_directory.rglob("*.json"))), 2)

                latest = collector.collect(
                    self.api,
                    (EvidenceDimension.ENDPOINT,),
                    PreferSnapshotPolicy(),
                ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(latest.observations[0].path, "/open-apis/second")

                server.response_body = b"not-json"
                rejected = collector.collect(
                    self.api, (EvidenceDimension.ENDPOINT,), FreshOfficialPolicy()
                ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(rejected.status, EvidenceStatus.REJECTED)
                self.assertEqual(len(tuple(snapshot_directory.rglob("*.json"))), 2)

                server.response_status = 503
                unavailable = collector.collect(
                    self.api, (EvidenceDimension.ENDPOINT,), FreshOfficialPolicy()
                ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(unavailable.status, EvidenceStatus.UNAVAILABLE)
                self.assertEqual(len(tuple(snapshot_directory.rglob("*.json"))), 2)

    def test_store_io_and_http_adapter_contract_failures_abort(self):
        with tempfile.TemporaryDirectory() as directory:
            store_file = Path(directory) / "not-a-directory"
            store_file.write_text("occupied", encoding="utf-8")
            with detail_server(structured_payload()) as (_, url):
                collector = compose(
                    snapshot_directory=store_file,
                    structured_detail_url=url,
                    timeout_seconds=1,
                    retries=0,
                )
                with self.assertRaises(SnapshotStoreError):
                    collector.collect(
                        self.api,
                        (EvidenceDimension.ENDPOINT,),
                        FreshOfficialPolicy(),
                    )

            with detail_server(body=b"[]") as (_, url):
                collector = compose(
                    snapshot_directory=Path(directory) / "snapshots",
                    structured_detail_url=url,
                    timeout_seconds=1,
                    retries=0,
                )
                rejected = collector.collect(
                    self.api,
                    (EvidenceDimension.ENDPOINT,),
                    FreshOfficialPolicy(),
                ).for_dimension(EvidenceDimension.ENDPOINT)
                self.assertEqual(rejected.status, EvidenceStatus.REJECTED)
                self.assertEqual(
                    rejected.diagnostics[0].code,
                    "document_unhealthy",
                )

            class BrokenAdapter:
                def acquire(self, catalog_entry):
                    return object()

            collector._structured_detail = BrokenAdapter()
            with self.assertRaises(AdapterContractError):
                collector.collect(
                    self.api,
                    (EvidenceDimension.ENDPOINT,),
                    FreshOfficialPolicy(),
                )


@unittest.skipUnless(
    os.environ.get("OPENLARK_LIVE_STRUCTURED_DETAIL") == "1",
    "需要显式启用 live Structured Detail smoke",
)
class LiveStructuredDetailSmokeTests(unittest.TestCase):
    def test_live_collect_returns_trusted_endpoint(self):
        api = ApiIdentity(
            api_id="6960166873968574467",
            name="获取多维表格元数据",
            biz_tag="base",
            meta_project="bitable",
            meta_version="v1",
            meta_resource="app",
            meta_name="get",
            url="GET:/open-apis/bitable/v1/apps/{app_token}",
            doc_path="https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get",
            expected_file="base/bitable/v1/app/get.rs",
            full_path="/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/get",
        )
        with tempfile.TemporaryDirectory() as directory:
            evidence = compose(
                snapshot_directory=Path(directory),
                timeout_seconds=10,
                retries=1,
            ).collect(
                api,
                (EvidenceDimension.ENDPOINT,),
                FreshOfficialPolicy(),
            ).for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(evidence.status, EvidenceStatus.TRUSTED, evidence)


if __name__ == "__main__":
    unittest.main()
