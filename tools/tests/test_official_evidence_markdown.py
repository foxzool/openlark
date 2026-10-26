import json
import sys
import tempfile
import unittest
from contextlib import contextmanager
from pathlib import Path
from unittest.mock import patch

from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official_evidence import (
    EvidenceDimension as Dimension,
    EvidenceSource as Source,
    EvidenceStatus as Status,
    FreshOfficialPolicy,
    PreferSnapshotPolicy,
    RecordedOnlyPolicy,
    RecordedSnapshot,
    collect,
    compose,
    compose_full,
)
from tools.tests.test_official_evidence_live import detail_server, structured_payload


FIXTURES = Path(__file__).parent / "fixtures"
AUTH = (FIXTURES / "official_markdown_auth.md").read_text(encoding="utf-8")
SEARCH = (FIXTURES / "official_markdown_search.md").read_text(encoding="utf-8")
FLAT = AUTH.replace(
    'Content-Type | string | 是 | **固定值**："application/json; charset=utf-8"',
    "Authorization | string | 是 | Bearer tenant_access_token 或 user_access_token",
)
MODULE = "tools.api_contracts.official_evidence"


class MarkdownEvidenceTests(unittest.TestCase):
    def setUp(self):
        self.api = ApiIdentity(
            api_id="42", name="测试", biz_tag="auth", meta_project="auth",
            meta_version="v3", meta_resource="app_access_token", meta_name="internal",
            url="POST:/open-apis/auth/v3/app_access_token/internal",
            doc_path="https://open.feishu.cn/document/wrong-redirect",
            expected_file="auth/auth/v3/app_access_token/internal.rs",
            full_path="/document/canonical/auth/internal",
        )

    def snapshot(self, content=FLAT):
        return RecordedSnapshot.markdown(
            version=1, catalog_entry=self.api, acquired_at="2026-10-26T00:00:00Z",
            source_uri="https://open.feishu.cn" + self.api.full_path + ".md",
            content=content,
        )

    def evidence(self, content=FLAT, dimension=Dimension.REQUEST_FIELDS):
        return collect(self.api, (dimension,), RecordedOnlyPolicy((self.snapshot(content),))).for_dimension(dimension)

    @contextmanager
    def live(self, payload=None, *, content=FLAT, markdown_options=None, full=False):
        with (
            tempfile.TemporaryDirectory() as directory,
            detail_server(payload if payload is not None else {"code": 41404}) as (detail, url),
            detail_server(body=content.encode("utf-8"), **(markdown_options or {})) as (markdown, md_url),
            patch(MODULE + "._MARKDOWN_DOCUMENT_ORIGIN", md_url.removesuffix("/detail")),
            (compose_full if full else compose)(
                snapshot_directory=Path(directory), structured_detail_url=url,
                timeout_seconds=0.1, retries=0,
            ) as collector,
        ):
            yield collector, detail, markdown, Path(directory)

    def test_flat_official_format_has_separate_source_and_provenance(self):
        snapshot = self.snapshot()
        evidence = collect(self.api, tuple(Dimension), RecordedOnlyPolicy((snapshot,)))
        for dimension in Dimension:
            item = evidence.for_dimension(dimension)
            self.assertEqual(item.status, Status.TRUSTED)
            self.assertEqual(item.selected_source, Source.MARKDOWN_DOCUMENT)
            self.assertEqual(item.snapshot_provenance.content_digest, snapshot.content_digest)
            self.assertEqual(item.interpretation_provenance.interpreter_revision, "official-evidence/2")
            self.assertEqual([attempt.source for attempt in item.acquisition_trail], [Source.STRUCTURED_DETAIL, Source.MARKDOWN_DOCUMENT])
            self.assertTrue(all(observation.source is Source.MARKDOWN_DOCUMENT for observation in item.observations))
        fields = evidence.for_dimension(Dimension.REQUEST_FIELDS).observations
        self.assertEqual([(field.canonical_path, field.required, field.location) for field in fields], [
            ("app_id", True, "request_body"), ("app_secret", True, "request_body"),
        ])
        self.assertEqual(evidence.for_dimension(Dimension.ENDPOINT).observations[0].path, "/open-apis/auth/v3/app_access_token/internal")
        self.assertEqual([token.token for token in evidence.for_dimension(Dimension.TOKENS).observations], ["tenant_access_token", "user_access_token"])

    def test_actual_auth_document_does_not_infer_auth_from_response_tokens(self):
        self.assertEqual(self.evidence(AUTH).status, Status.TRUSTED)
        self.assertEqual(self.evidence(AUTH, Dimension.RESPONSE_FIELDS).status, Status.TRUSTED)
        self.assertEqual(self.evidence(AUTH, Dimension.TOKENS).status, Status.INCOMPLETE)

    def test_actual_search_document_does_not_promote_flattened_response_or_missing_body(self):
        for dimension in (Dimension.REQUEST_FIELDS, Dimension.RESPONSE_FIELDS):
            with self.subTest(dimension=dimension):
                item = self.evidence(SEARCH, dimension)
                self.assertEqual(item.status, Status.INCOMPLETE)
                if dimension is Dimension.RESPONSE_FIELDS:
                    self.assertEqual(item.observations, ())
        self.assertEqual(self.evidence(SEARCH, Dimension.ENDPOINT).status, Status.TRUSTED)
        self.assertEqual(self.evidence(SEARCH, Dimension.TOKENS).status, Status.TRUSTED)

    def test_missing_section_and_example_are_not_empty_or_complete_contracts(self):
        before, after = FLAT.split("### 请求体\n", 1)
        _, after = after.split("### 请求体示例", 1)
        missing = before + "### 请求体示例" + after
        example_table = "```markdown\n### 请求体\n名称 | 类型 | 必填\n---|---|---\nexample | string | 是\n```\n"
        self.assertEqual(self.evidence(missing).status, Status.INCOMPLETE)
        self.assertEqual(self.evidence(missing + example_table).status, Status.INCOMPLETE)
        explicit = missing.replace("### 请求体示例", "### 请求体\n无\n\n### 请求体示例")
        item = self.evidence(explicit)
        self.assertEqual(item.status, Status.TRUSTED)
        self.assertEqual(item.observations, ())
        response_example = FLAT.split("### 响应体\n")[0] + '### 响应体示例\n```json\n{"field":1}\n```'
        self.assertEqual(self.evidence(response_example, Dimension.RESPONSE_FIELDS).status, Status.INCOMPLETE)

    def test_ambiguous_tables_cannot_prove_field_paths(self):
        for content in (
            FLAT.replace("app_id | string", "app_id | object"),
            FLAT.replace("app_id | string", r"app_id | user\[\]"),
            FLAT.replace("app_secret | string", "app_id | string"),
            FLAT.replace("app_id | string", "app_id.child | string"),
            FLAT.replace("app_id | string | 是 |", "app_id | string | 是 | extra |"),
            FLAT.replace("### 请求体示例", "### 请求体\n无\n\n### 请求体示例"),
            FLAT.replace("### 请求体示例", "another | string | 是 | text\n\n### 请求体示例"),
        ):
            with self.subTest(content=content[:80]):
                item = self.evidence(content)
                self.assertEqual(item.status, Status.INCOMPLETE)
                self.assertEqual(item.observations, ())

    def test_unknown_requiredness_is_preserved_and_primitive_arrays_are_supported(self):
        item = self.evidence(FLAT.replace("app_id | string | 是", r"app_id | string\[\] | 按条件必填"))
        self.assertEqual(item.status, Status.INCOMPLETE)
        self.assertIsNone(item.observations[0].required)
        self.assertEqual(item.observations[0].field_type, "string[]")
        self.assertEqual(self.evidence(FLAT.replace("app_id | string | 是", r"app_id | string\[\] | 否")).status, Status.TRUSTED)

    def test_damaged_last_field_row_is_not_silently_discarded(self):
        item = self.evidence(FLAT.replace("app_secret | string | 是 |", "app_secret string 是 "))
        self.assertEqual(item.status, Status.INCOMPLETE)
        self.assertEqual(item.observations, ())

    def test_query_and_path_observations_retain_their_location(self):
        content = FLAT.replace("### 请求体示例", "### 查询参数\n名称 | 类型 | 必填\n---|---|---\npage | int | 否\n\n### 路径参数\n名称 | 类型 | 必填\n---|---|---\nid | string | 是\n\n### 请求体示例")
        item = self.evidence(content)
        self.assertEqual(item.status, Status.TRUSTED)
        self.assertEqual([(field.canonical_path, field.location) for field in item.observations[-2:]], [("page", "query"), ("id", "path")])
        closed_headings = content.replace("### 查询参数\n", "### 查询参数 ###\n").replace("### 路径参数\n", "### 路径参数 ###\n")
        self.assertEqual(self.evidence(closed_headings).observations, item.observations)
        duplicate_query = closed_headings.replace("### 请求体示例", "### 查询参数 ###\n无\n\n### 请求体示例")
        self.assertEqual(self.evidence(duplicate_query).status, Status.INCOMPLETE)

    def test_unhealthy_content_is_rejected_even_after_http_200(self):
        for content in ("", "404 Not Found", "<html># 错误\n## 请求</html>", "# 文档不存在\n## 请求", '{"code":0}', "# unrelated page"):
            with self.subTest(content=content):
                item = self.evidence(content, Dimension.ENDPOINT)
                self.assertEqual(item.status, Status.REJECTED)
                self.assertEqual(item.observations, ())

    def test_endpoint_and_auth_require_unambiguous_explicit_rows(self):
        for content in (
            FLAT.replace("HTTP Method | POST", "HTTP Method | POST\nHTTP Method | GET"),
            FLAT.replace("https://open.feishu.cn/open-apis/", "https://other.example/open-apis/"),
            FLAT.replace("HTTP URL |", "Other URL |"),
        ):
            self.assertEqual(self.evidence(content, Dimension.ENDPOINT).status, Status.INCOMPLETE)
        for content in (
            FLAT.replace("Authorization | string | 是", "Authorization | string | 否"),
            FLAT.replace("tenant_access_token", "unknown_access_token"),
            FLAT.replace("Authorization | string | 是 |", "Other | string | 是 |"),
        ):
            self.assertEqual(self.evidence(content, Dimension.TOKENS).status, Status.INCOMPLETE)

    def test_structured_priority_does_not_merge_sources(self):
        structured = RecordedSnapshot.structured(
            version=1, catalog_entry=self.api, acquired_at="2026-10-26T00:00:00Z",
            source_uri="https://open.feishu.cn/detail", payload=structured_payload(),
        )
        item = collect(self.api, (Dimension.REQUEST_FIELDS,), RecordedOnlyPolicy((structured, self.snapshot()))).for_dimension(Dimension.REQUEST_FIELDS)
        self.assertEqual(item.selected_source, Source.STRUCTURED_DETAIL)
        self.assertEqual(len(item.acquisition_trail), 1)
        self.assertNotIn("app_id", [field.canonical_path for field in item.observations])

    def test_live_41404_falls_back_to_canonical_url_and_all_policies(self):
        with self.live() as (collector, detail, markdown, directory):
            first = collector.collect(self.api, tuple(Dimension), FreshOfficialPolicy())
            self.assertEqual((detail.requests, markdown.requests), (1, 1))
            item = first.for_dimension(Dimension.ENDPOINT)
            self.assertEqual(item.status, Status.TRUSTED)
            self.assertTrue(item.snapshot_provenance.source_uri.endswith(self.api.full_path + ".md"))
            self.assertEqual([attempt.status for attempt in item.acquisition_trail], [Status.REJECTED, Status.TRUSTED])
            # Rejected structured detail 不进入持久缓存；Markdown 原文可独立回放。
            files = list(directory.rglob("*.json"))
            self.assertEqual(len(files), 1)
            saved = json.loads(files[0].read_text())
            self.assertEqual(saved["source_kind"], "markdown_document")
            self.assertEqual(saved["raw_representation"], FLAT)
            replay = collect(self.api, tuple(Dimension), RecordedOnlyPolicy((self.snapshot(),)))
            self.assertEqual(replay.for_dimension(Dimension.REQUEST_FIELDS).observations, first.for_dimension(Dimension.REQUEST_FIELDS).observations)
            collector.collect(self.api, tuple(Dimension), PreferSnapshotPolicy())
            self.assertEqual((detail.requests, markdown.requests), (2, 1))
            collector.collect(self.api, tuple(Dimension), FreshOfficialPolicy())
            self.assertEqual((detail.requests, markdown.requests), (3, 2))
            collector.collect(self.api, tuple(Dimension), PreferSnapshotPolicy(max_age_days=0))
            self.assertEqual((detail.requests, markdown.requests), (4, 3))

    def test_live_success_without_schema_uses_markdown(self):
        with self.live({"code": 0, "data": {"content": "<md-text>API 文档</md-text>"}}) as (collector, _, markdown, _):
            item = collector.collect(self.api, (Dimension.ENDPOINT,), FreshOfficialPolicy()).for_dimension(Dimension.ENDPOINT)
            self.assertEqual(item.status, Status.TRUSTED)
            self.assertEqual(markdown.requests, 1)

    def test_other_errors_and_existing_schemas_do_not_trigger_markdown(self):
        cases = [
            {"code": 500}, {"code": 0, "data": {}}, {"code": False, "data": {"content": "text"}},
            {"code": 0, "data": {"content": "text", "schema": {"apiSchema": {}}}},
            structured_payload(), [41404],
        ]
        for payload in cases:
            with self.subTest(payload=payload), self.live(payload) as (collector, _, markdown, _):
                collector.collect(self.api, tuple(Dimension), FreshOfficialPolicy())
                self.assertEqual(markdown.requests, 0)

    def test_transport_failure_or_bad_json_does_not_trigger_markdown(self):
        for options in ({"status": 404}, {"body": b"not json"}, {"delay": 0.2}):
            with (
                self.subTest(options=options), tempfile.TemporaryDirectory() as directory,
                detail_server({}, **options) as (_, url),
                compose(snapshot_directory=Path(directory), structured_detail_url=url, timeout_seconds=0.05, retries=0) as collector,
                patch.object(collector._markdown_document, "acquire") as acquire,
            ):
                collector.collect(self.api, tuple(Dimension), FreshOfficialPolicy())
                acquire.assert_not_called()

    def test_only_remaining_dimensions_continue_to_rendered_without_merging(self):
        worker = FIXTURES / "rendered_document_worker.py"
        with (
            patch(MODULE + "._RENDERED_WORKER_COMMAND", (sys.executable, str(worker))),
            self.live(content=FLAT.replace("app_id | string", "app_id | object"), full=True) as (collector, _, _, _),
        ):
            evidence = collector.collect(self.api, tuple(Dimension), FreshOfficialPolicy())
        endpoint = evidence.for_dimension(Dimension.ENDPOINT)
        request = evidence.for_dimension(Dimension.REQUEST_FIELDS)
        self.assertEqual(endpoint.selected_source, Source.MARKDOWN_DOCUMENT)
        self.assertEqual(len(endpoint.acquisition_trail), 2)
        self.assertEqual(request.selected_source, Source.RENDERED_DOCUMENT)
        self.assertEqual([attempt.source for attempt in request.acquisition_trail], [Source.STRUCTURED_DETAIL, Source.MARKDOWN_DOCUMENT, Source.RENDERED_DOCUMENT])
        self.assertEqual([field.canonical_path for field in request.observations], ["rendered_only"])

    def test_rejected_markdown_is_evicted_and_refetched(self):
        with self.live(content="<html>error</html>") as (collector, _, markdown, directory):
            item = collector.collect(self.api, (Dimension.ENDPOINT,), FreshOfficialPolicy()).for_dimension(Dimension.ENDPOINT)
            self.assertEqual(item.status, Status.REJECTED)
            self.assertEqual(list(directory.rglob("*.json")), [])
            markdown.response_body = FLAT.encode("utf-8")
            item = collector.collect(self.api, (Dimension.ENDPOINT,), PreferSnapshotPolicy()).for_dimension(Dimension.ENDPOINT)
            self.assertEqual(item.status, Status.TRUSTED)
            self.assertEqual(markdown.requests, 2)

    def test_markdown_http_failures_remain_visible_and_are_not_cached(self):
        for options, code in (({"status": 404}, "document_not_found"), ({"delay": 0.2}, "acquisition_timeout"), ({"declared_length": 100_000_000}, "acquisition_failed")):
            with self.subTest(options=options), self.live(markdown_options=options) as (collector, _, _, directory):
                item = collector.collect(self.api, (Dimension.ENDPOINT,), FreshOfficialPolicy()).for_dimension(Dimension.ENDPOINT)
                attempt = item.acquisition_trail[1]
                self.assertEqual(attempt.source, Source.MARKDOWN_DOCUMENT)
                self.assertEqual(attempt.status, Status.UNAVAILABLE)
                self.assertEqual(attempt.diagnostics[0].code, code)
                self.assertEqual(list(directory.rglob("*.json")), [])


if __name__ == "__main__":
    unittest.main()
