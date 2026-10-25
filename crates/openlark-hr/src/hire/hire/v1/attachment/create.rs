//! 创建附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/attachment/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 官方文档限制：附件文件最大 300 MB
const MAX_FILE_SIZE_BYTES: usize = 300 * 1024 * 1024;

/// 创建附件请求体（multipart/form-data）
///
/// 官方请求体仅一个文件部件 `content`（file，必填，最大 300 MB）；
/// 文件名作为 multipart 文件部件的 filename 元数据随 `execute` 参数传入，不是官方表单字段。
#[derive(Debug, Clone)]
pub struct CreateBody {
    /// 文件二进制内容（官方请求体字段 `content`，必填，最大 300 MB）
    pub content: Vec<u8>,
}

/// 创建附件请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// `file_name` 作为 multipart 文件部件的 filename 元数据发送。
    pub async fn execute(
        self,
        file_name: impl Into<String>,
        body: CreateBody,
    ) -> SDKResult<CreateResponse> {
        self.execute_with_options(
            file_name,
            body,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        file_name: impl Into<String>,
        body: CreateBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let file_name = file_name.into();
        validate_required!(file_name, "file_name 不能为空");
        if body.content.is_empty() {
            return Err(openlark_core::error::validation_error(
                "content",
                "文件内容不能为空",
            ));
        }
        if body.content.len() > MAX_FILE_SIZE_BYTES {
            return Err(openlark_core::error::validation_error(
                "content",
                "文件大小不能超过 300 MB",
            ));
        }

        let api_endpoint = HireApiV1::AttachmentCreate;
        // multipart 表单：__file_name 仅用于设置 multipart 文件名，不作为表单字段发送；
        // 官方文档的文件部件名为 "content"（Content-Disposition: form-data; name="content"）
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .body(serde_json::json!({ "__file_name": file_name }))
            .file_content(body.content)
            .file_part_name("content");

        Transport::request_typed(request, &self.config, Some(option), "创建附件响应数据为空").await
    }
}

/// 创建附件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/attachments（multipart，文件部件名 content）
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {
                    "id": "attach_001", "name": "resume.pdf", "url": "https://example.feishu.cn/attach_001"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateRequest::new(config)
            .execute(
                "resume.pdf",
                CreateBody {
                    content: b"%PDF-1.4 fake resume".to_vec(),
                },
            )
            .await
            .expect("请求应成功");
        assert_eq!(resp.id, Some("attach_001".to_string()));
        assert_eq!(resp.name, Some("resume.pdf".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // multipart 形状：文件部件名必须是官方的 "content"，且带 filename
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
            body.contains("name=\"content\""),
            "文件部件名应为 content: {body}"
        );
        assert!(
            body.contains("filename=\"resume.pdf\""),
            "缺少文件名 resume.pdf: {body}"
        );
    }

    /// 必填校验：缺 file_name / 空文件均快速失败
    #[tokio::test]
    async fn test_create_missing_required_fields_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CreateRequest::new(config.clone())
            .execute(
                "  ",
                CreateBody {
                    content: b"data".to_vec(),
                },
            )
            .await
            .expect_err("空白 file_name 应被拒绝");
        assert!(err.to_string().contains("file_name"), "实际: {err}");

        let err = CreateRequest::new(config)
            .execute(
                "resume.pdf",
                CreateBody {
                    content: Vec::new(),
                },
            )
            .await
            .expect_err("空文件应被拒绝");
        assert!(err.to_string().contains("不能为空"), "实际: {err}");
    }
}
