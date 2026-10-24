//! 获取纪要详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/note/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取纪要详情请求。
#[derive(Debug, Clone)]
pub struct NoteGetRequest {
    config: Arc<Config>,
    note_id: String,
    /// 用户 ID 类型（默认 open_id，可选 open_id/union_id/user_id）。
    user_id_type: Option<String>,
}

/// 获取纪要详情响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteGetResponse {
    /// 纪要信息。
    pub note: NoteInfo,
}

/// 纪要信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteInfo {
    /// 纪要创建者 User ID（格式由 user_id_type 决定）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 纪要创建时间（unix 时间，单位 sec）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 纪要产物列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<NoteArtifactInfo>>,
    /// 关联引用列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<NoteReferenceInfo>>,
    /// 纪要来源。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_source: Option<GeneratedSource>,
    /// 纪要展示类型：1-普通纪要，2-集成展示纪要。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_display_type: Option<i32>,
}

/// 纪要产物。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteArtifactInfo {
    /// 纪要产物类型：1-纪要文档，2-逐字稿文档。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<i32>,
    /// 产物创建时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 产物的 doc token。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
}

/// 关联引用。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteReferenceInfo {
    /// 引用类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<i32>,
    /// 引用的 doc token。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
}

/// 纪要来源。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratedSource {
    /// 来源类型（可选值 meeting）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// 来源实体 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_id: Option<String>,
}

impl ApiResponseTrait for NoteGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl NoteGetRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            note_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置路径参数 `note_id`。
    pub fn note_id(mut self, note_id: impl Into<String>) -> Self {
        self.note_id = note_id.into();
        self
    }

    /// 设置查询参数 `user_id_type`（默认 open_id，可选 open_id/union_id/user_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<NoteGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<NoteGetResponse> {
        validate_required!(self.note_id, "note_id 不能为空");
        let path = format!("/open-apis/vc/v1/notes/{}", self.note_id);
        let mut req: ApiRequest<NoteGetResponse> = ApiRequest::get(path);
        if let Some(ref user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type);
        }
        Transport::request_typed(req, &self.config, Some(option), "获取纪要详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = NoteGetRequest::new(config);
    }

    /// 端到端：GET .../notes/{note_id}?user_id_type=... → 类型化 NoteGetResponse 解析。
    #[tokio::test]
    async fn test_note_get_returns_typed_note_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/notes/note_001"))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "note": {
                        "creator_id": "ou_123",
                        "create_time": "1760500000",
                        "artifacts": [
                            { "artifact_type": 1, "create_time": "1760500000", "doc_token": "doccnabc" }
                        ],
                        "references": [
                            { "reference_type": 1, "doc_token": "doccnxyz" }
                        ],
                        "note_source": { "source_type": "meeting", "source_entity_id": "meet_001" },
                        "note_display_type": 1
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = NoteGetRequest::new(config)
            .note_id("note_001")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("获取纪要详情应成功");
        assert_eq!(resp.note.creator_id.as_deref(), Some("ou_123"));
        assert_eq!(resp.note.note_display_type, Some(1));
        assert_eq!(resp.note.artifacts.as_ref().unwrap().len(), 1);
        assert_eq!(
            resp.note.artifacts.as_ref().unwrap()[0].artifact_type,
            Some(1)
        );
        assert_eq!(
            resp.note
                .note_source
                .as_ref()
                .unwrap()
                .source_type
                .as_deref(),
            Some("meeting")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/notes/note_001");
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("user_id_type=open_id")
        );
    }
}
