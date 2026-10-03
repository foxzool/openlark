import unittest
from dataclasses import replace


from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official_evidence import (
    EvidenceDimension,
    EvidenceSource,
    EvidenceStatus,
    InterpreterError,
    InvalidEvidenceRequest,
    RecordedOnlyPolicy,
    RecordedSnapshot,
    SnapshotInvariantError,
    collect,
)


class OfficialEvidenceCollectTests(unittest.TestCase):
    def setUp(self):
        self.api = ApiIdentity(
            api_id="42",
            name="创建记录",
            biz_tag="base",
            meta_project="bitable",
            meta_version="v1",
            meta_resource="app.table.record",
            meta_name="create",
            url="POST:/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records",
            doc_path="https://open.feishu.cn/document/mock",
            expected_file="base/bitable/v1/app/table/record/create.rs",
            full_path="/document/uAjLw4CM/mock",
        )

    def test_collect_returns_trusted_endpoint_with_separate_provenance(self):
        snapshot = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document_portal/v1/document/get_detail",
            payload={
                "data": {
                    "schema": {
                        "apiSchema": {
                            "httpMethod": "post",
                            "path": "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records",
                        }
                    }
                }
            },
        )

        result = collect(
            self.api,
            (EvidenceDimension.ENDPOINT,),
            RecordedOnlyPolicy((snapshot,)),
        )

        evidence = result.for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(evidence.status, EvidenceStatus.TRUSTED)
        self.assertEqual(evidence.selected_source, EvidenceSource.STRUCTURED_DETAIL)
        self.assertEqual(len(evidence.observations), 1)
        self.assertEqual(evidence.observations[0].method, "POST")
        self.assertEqual(
            evidence.observations[0].path,
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records",
        )
        self.assertEqual(evidence.snapshot_provenance.catalog_entry, self.api)
        self.assertEqual(
            evidence.interpretation_provenance.snapshot_digest,
            evidence.snapshot_provenance.content_digest,
        )
        self.assertEqual(
            evidence.interpretation_provenance.dimension,
            EvidenceDimension.ENDPOINT,
        )
        self.assertEqual(len(evidence.acquisition_trail), 1)
        self.assertEqual(evidence.acquisition_trail[0].status, EvidenceStatus.TRUSTED)

    def test_collect_interprets_hierarchical_fields_and_trusted_empty_tokens(self):
        snapshot = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document_portal/v1/document/get_detail",
            payload={
                "data": {
                    "schema": {
                        "apiSchema": {
                            "requestBody": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "required": ["record"],
                                            "properties": {
                                                "record": {
                                                    "type": "object",
                                                    "properties": {
                                                        "fields": {
                                                            "properties": {
                                                                "name": {"type": "string"}
                                                            }
                                                        }
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
                            "security": {"supportedAccessToken": []},
                        }
                    }
                }
            },
        )

        result = collect(
            self.api,
            (
                EvidenceDimension.REQUEST_FIELDS,
                EvidenceDimension.RESPONSE_FIELDS,
                EvidenceDimension.TOKENS,
            ),
            RecordedOnlyPolicy((snapshot,)),
        )

        request = result.for_dimension(EvidenceDimension.REQUEST_FIELDS)
        self.assertEqual(request.status, EvidenceStatus.TRUSTED)
        by_path = {observation.canonical_path: observation for observation in request.observations}
        self.assertIn("record.fields.name", by_path)
        self.assertIsNone(by_path["record.fields.name"].required)
        self.assertEqual(by_path["record.fields.name"].field_type, "string")
        self.assertEqual(by_path["record.fields.name"].location, "request_body")
        self.assertEqual(
            by_path["record.fields.name"].source,
            EvidenceSource.STRUCTURED_DETAIL,
        )

        response = result.for_dimension(EvidenceDimension.RESPONSE_FIELDS)
        self.assertEqual(response.status, EvidenceStatus.TRUSTED)
        self.assertIn(
            "data.record_id",
            {observation.canonical_path for observation in response.observations},
        )

        tokens = result.for_dimension(EvidenceDimension.TOKENS)
        self.assertEqual(tokens.status, EvidenceStatus.TRUSTED)
        self.assertEqual(tokens.observations, ())

    def test_collect_falls_back_per_dimension_without_merging_observations(self):
        structured = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document_portal/v1/document/get_detail",
            payload={
                "data": {
                    "schema": {
                        "apiSchema": {
                            "httpMethod": "POST",
                            "path": "/open-apis/bitable/v1/records",
                            "requestBody": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "properties": {
                                                "structured_only": {"type": "string"},
                                                "broken": "not-a-schema",
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            },
        )
        rendered = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:01:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content=(
                "# Official API Documentation\n"
                "## Request Fields\n"
                "| Path | Location | Required | Type |\n"
                "| fallback_only | request_body | unknown | string |\n"
            ),
        )

        result = collect(
            self.api,
            (EvidenceDimension.ENDPOINT, EvidenceDimension.REQUEST_FIELDS),
            RecordedOnlyPolicy((structured, rendered)),
        )

        endpoint = result.for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(endpoint.selected_source, EvidenceSource.STRUCTURED_DETAIL)
        self.assertEqual(len(endpoint.acquisition_trail), 1)

        request = result.for_dimension(EvidenceDimension.REQUEST_FIELDS)
        self.assertEqual(request.status, EvidenceStatus.TRUSTED)
        self.assertEqual(request.selected_source, EvidenceSource.RENDERED_DOCUMENT)
        self.assertEqual(
            [observation.canonical_path for observation in request.observations],
            ["fallback_only"],
        )
        self.assertEqual(
            [attempt.status for attempt in request.acquisition_trail],
            [EvidenceStatus.INCOMPLETE, EvidenceStatus.TRUSTED],
        )
        self.assertEqual(
            request.acquisition_trail[0].diagnostics[0].code,
            "structure_incomplete",
        )

    def test_collect_exposes_incomplete_rejected_and_unavailable_statuses(self):
        partial = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/detail",
            payload={
                "data": {
                    "schema": {
                        "apiSchema": {
                            "requestBody": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "properties": {
                                                "usable": {"type": "string"},
                                                "broken": [],
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        )
        incomplete = collect(
            self.api,
            (EvidenceDimension.REQUEST_FIELDS,),
            RecordedOnlyPolicy((partial,)),
        ).for_dimension(EvidenceDimension.REQUEST_FIELDS)
        self.assertEqual(incomplete.status, EvidenceStatus.INCOMPLETE)
        self.assertEqual(
            [observation.canonical_path for observation in incomplete.observations],
            ["usable"],
        )

        unhealthy_structured = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/detail",
            payload={"code": 404, "message": "not found"},
        )
        unhealthy_rendered = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:01:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content="404 Not Found",
        )
        rejected = collect(
            self.api,
            (EvidenceDimension.ENDPOINT,),
            RecordedOnlyPolicy((unhealthy_structured, unhealthy_rendered)),
        ).for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(rejected.status, EvidenceStatus.REJECTED)
        self.assertEqual(rejected.selected_source, EvidenceSource.STRUCTURED_DETAIL)
        self.assertEqual(len(rejected.acquisition_trail), 2)

        unavailable = collect(
            self.api,
            (EvidenceDimension.TOKENS,),
            RecordedOnlyPolicy(()),
        ).for_dimension(EvidenceDimension.TOKENS)
        self.assertEqual(unavailable.status, EvidenceStatus.UNAVAILABLE)
        self.assertEqual(unavailable.selected_source, EvidenceSource.STRUCTURED_DETAIL)
        self.assertEqual(
            [attempt.source for attempt in unavailable.acquisition_trail],
            [
                EvidenceSource.STRUCTURED_DETAIL,
                EvidenceSource.RENDERED_DOCUMENT,
            ],
        )

    def test_collect_selects_most_informative_status_then_structured_source(self):
        rejected_structured = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/detail",
            payload={"code": 404},
        )
        incomplete_rendered = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:01:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content="# Official API Documentation\n## Endpoint\nunsupported layout\n",
        )
        incomplete = collect(
            self.api,
            (EvidenceDimension.ENDPOINT,),
            RecordedOnlyPolicy((rejected_structured, incomplete_rendered)),
        ).for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(incomplete.status, EvidenceStatus.INCOMPLETE)
        self.assertEqual(incomplete.selected_source, EvidenceSource.RENDERED_DOCUMENT)

        incomplete_structured = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/detail",
            payload={"data": {"schema": {"apiSchema": {}}}},
        )
        rejected_rendered = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:01:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content="404 Not Found",
        )
        selected = collect(
            self.api,
            (EvidenceDimension.ENDPOINT,),
            RecordedOnlyPolicy((incomplete_structured, rejected_rendered)),
        ).for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(selected.status, EvidenceStatus.INCOMPLETE)
        self.assertEqual(selected.selected_source, EvidenceSource.STRUCTURED_DETAIL)

    def test_collect_requires_explicit_field_schema_to_trust_zero_fields(self):
        def snapshot_with_content(content):
            return RecordedSnapshot.structured(
                version=1,
                catalog_entry=self.api,
                acquired_at="2026-08-11T00:00:00Z",
                source_uri="https://open.feishu.cn/detail",
                payload={
                    "data": {
                        "schema": {
                            "apiSchema": {
                                "requestBody": {"content": content},
                            }
                        }
                    }
                },
            )

        explicit_empty = collect(
            self.api,
            (EvidenceDimension.REQUEST_FIELDS,),
            RecordedOnlyPolicy(
                (
                    snapshot_with_content(
                        {"application/json": {"schema": {"properties": {}}}}
                    ),
                )
            ),
        ).for_dimension(EvidenceDimension.REQUEST_FIELDS)
        self.assertEqual(explicit_empty.status, EvidenceStatus.TRUSTED)
        self.assertEqual(explicit_empty.observations, ())

        unproven_empty = collect(
            self.api,
            (EvidenceDimension.REQUEST_FIELDS,),
            RecordedOnlyPolicy((snapshot_with_content({}),)),
        ).for_dimension(EvidenceDimension.REQUEST_FIELDS)
        self.assertEqual(unproven_empty.status, EvidenceStatus.INCOMPLETE)

    def test_collect_interprets_rendered_endpoint_and_tokens(self):
        rendered = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content=(
                "# Official API Documentation\n"
                "## Endpoint\n"
                "POST /open-apis/bitable/v1/apps/:app_token/records\n"
                "## Tokens\n"
                "- tenant_access_token\n"
                "- user_access_token\n"
            ),
        )

        result = collect(
            self.api,
            (EvidenceDimension.ENDPOINT, EvidenceDimension.TOKENS),
            RecordedOnlyPolicy((rendered,)),
        )

        endpoint = result.for_dimension(EvidenceDimension.ENDPOINT)
        self.assertEqual(endpoint.status, EvidenceStatus.TRUSTED)
        self.assertEqual(
            endpoint.observations[0].path,
            "/open-apis/bitable/v1/apps/{app_token}/records",
        )
        tokens = result.for_dimension(EvidenceDimension.TOKENS)
        self.assertEqual(tokens.status, EvidenceStatus.TRUSTED)
        self.assertEqual(
            [observation.token for observation in tokens.observations],
            ["tenant_access_token", "user_access_token"],
        )

    def test_collect_aborts_on_invalid_requests_and_snapshot_invariants(self):
        with self.assertRaises(InvalidEvidenceRequest):
            collect(self.api, (), RecordedOnlyPolicy(()))

        snapshot = RecordedSnapshot.structured(
            version=1,
            catalog_entry=self.api,
            acquired_at="not-a-time",
            source_uri="https://open.feishu.cn/detail",
            payload={"data": {"schema": {"apiSchema": {}}}},
        )
        with self.assertRaises(SnapshotInvariantError):
            collect(
                self.api,
                (EvidenceDimension.ENDPOINT,),
                RecordedOnlyPolicy((snapshot,)),
            )

        valid = replace(snapshot, acquired_at="2026-08-11T00:00:00Z")
        with self.assertRaises(SnapshotInvariantError):
            collect(
                self.api,
                (EvidenceDimension.ENDPOINT,),
                RecordedOnlyPolicy((replace(valid, content_digest="0" * 64),)),
            )

    def test_collect_wraps_interpreter_defects(self):
        class DefectiveRenderedText(str):
            def strip(self, *args, **kwargs):
                raise RuntimeError("interpreter exploded")

        snapshot = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=self.api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content=DefectiveRenderedText("raw rendered document"),
        )

        with self.assertRaises(InterpreterError):
            collect(
                self.api,
                (EvidenceDimension.ENDPOINT,),
                RecordedOnlyPolicy((snapshot,)),
            )


if __name__ == "__main__":
    unittest.main()
