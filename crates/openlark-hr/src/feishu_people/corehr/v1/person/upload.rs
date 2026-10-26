//! 上传文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/upload>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 官方文档限制：文件大小不超过 50 MB
const MAX_FILE_SIZE_BYTES: usize = 50 * 1024 * 1024;

/// 上传文件请求体（multipart/form-data）
///
/// 官方请求体两个必填部件：
/// - `file_content`（file）：文件二进制内容，经 `.file_content(...)` 以官方部件名发送
/// - `file_name`（string）：文件名称（需带有文件后缀），作为表单字段发送
#[derive(Debug, Clone)]
pub struct UploadBody {
    /// 文件二进制内容（官方请求体字段 `file_content`，必填，大小不超过 50 MB）
    pub file_content: Vec<u8>,
    /// 文件名称，需带有文件后缀（官方请求体字段 `file_name`，必填）
    pub file_name: String,
}

/// 上传文件请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    config: Config,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self, body: UploadBody) -> SDKResult<UploadResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UploadBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(body.file_name, "file_name 不能为空");
        if body.file_content.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_content",
                "文件内容不能为空",
            ));
        }
        if body.file_content.len() > MAX_FILE_SIZE_BYTES {
            return Err(openlark_core::error::validation_error(
                "file_content",
                "文件大小不能超过 50 MB",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::PersonUpload;
        // multipart 表单：file_name 作为表单字段发送；文件二进制经
        // `.file_content(...)` 传入，部件名为官方的 "file_content"
        let request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url())
            .body(serde_json::json!({ "file_name": body.file_name }))
            .file_content(body.file_content)
            .file_part_name("file_content");

        Transport::request_typed(request, &self.config, Some(option), "上传文件响应数据为空").await
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 上传文件 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ApiResponseTrait for UploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/persons/upload（multipart，文件部件名 file_content）
    #[tokio::test]
    async fn test_upload_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/persons/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "100" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UploadRequest::new(config)
            .execute(UploadBody {
                file_content: b"fake xlsx bytes".to_vec(),
                file_name: "个人信息.xlsx".to_string(),
            })
            .await
            .expect("请求应成功");

        assert_eq!(resp.id, Some("100".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 官方 multipart 形状：文件部件名 file_content + 表单字段 file_name
        let content_type = received[0]
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        assert!(
            content_type.starts_with("multipart/form-data"),
            "应为 multipart/form-data 请求，实际 {content_type}"
        );
        let body = String::from_utf8_lossy(&received[0].body);
        assert!(
            body.contains("name=\"file_content\""),
            "文件部件名应为 file_content: {body}"
        );
        assert!(
            body.contains("name=\"file_name\""),
            "缺少表单字段 file_name: {body}"
        );
        assert!(
            body.contains("个人信息.xlsx"),
            "file_name 字段值应为文件名: {body}"
        );
    }

    /// 必填校验：缺 file_name / 空文件均快速失败
    #[tokio::test]
    async fn test_upload_missing_required_fields_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = UploadRequest::new(config.clone())
            .execute(UploadBody {
                file_content: b"data".to_vec(),
                file_name: "  ".to_string(),
            })
            .await
            .expect_err("空白 file_name 应被拒绝");
        assert!(err.to_string().contains("file_name"), "实际: {err}");

        let err = UploadRequest::new(config)
            .execute(UploadBody {
                file_content: Vec::new(),
                file_name: "个人信息.xlsx".to_string(),
            })
            .await
            .expect_err("空文件应被拒绝");
        assert!(err.to_string().contains("不能为空"), "实际: {err}");
    }
}
