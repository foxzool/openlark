//! 上传附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/aily-v1/agent-attachment/create>

use crate::endpoints::AILY_V1_AGENT_ATTACHMENTS;
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

/// 上传附件响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentAttachmentResponse {
    /// 附件 id。
    pub agent_attachment_id: String,
}

impl ApiResponseTrait for CreateAgentAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传附件请求。
#[derive(Debug, Clone)]
pub struct CreateAgentAttachmentRequest {
    config: Arc<Config>,
    agent_id: String,
    attachment_type: Option<String>,
    file_name: Option<String>,
    doc_url: Option<String>,
}

impl CreateAgentAttachmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
            attachment_type: None,
            file_name: None,
            doc_url: None,
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 设置附件类型（必填，表单字段 `type`）：
    /// `image`/`file`（file 必传，doc_url 不生效）、
    /// `feishu_doc`/`bitable`（doc_url 必传，file 不生效）。
    pub fn attachment_type(mut self, attachment_type: impl Into<String>) -> Self {
        self.attachment_type = Some(attachment_type.into());
        self
    }

    /// 可选：设置上传文件名（仅用于 multipart 的文件名，不作为字段发送）。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 设置云文档/多维表格 URL（仅在 type=`bitable`/`feishu_doc` 时生效）。
    pub fn doc_url(mut self, doc_url: impl Into<String>) -> Self {
        self.doc_url = Some(doc_url.into());
        self
    }

    /// 执行请求。
    ///
    /// 说明：该接口为 multipart/form-data 上传，请直接传入文件二进制内容；
    /// type=`feishu_doc`/`bitable` 时 file 不生效，但仍需传入非空字节以构成 multipart 表单。
    pub async fn execute(self, file_bytes: Vec<u8>) -> SDKResult<CreateAgentAttachmentResponse> {
        self.execute_with_options(file_bytes, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        file_bytes: Vec<u8>,
        option: RequestOption,
    ) -> SDKResult<CreateAgentAttachmentResponse> {
        validate_required!(self.agent_id, "agent_id 不能为空");
        let attachment_type = self.attachment_type.ok_or_else(|| {
            openlark_core::error::validation_error(
                "type".to_string(),
                "附件类型（type）不能为空，可选 image、file、feishu_doc、bitable".to_string(),
            )
        })?;
        if file_bytes.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file",
                "上传文件内容不能为空",
            ));
        }

        // multipart 表单：type 必填；doc_url 可选；__file_name 仅用于 multipart 文件名
        let mut body = serde_json::json!({ "type": attachment_type });
        if let Some(doc_url) = self.doc_url
            && let Some(obj) = body.as_object_mut()
        {
            obj.insert("doc_url".to_string(), serde_json::Value::String(doc_url));
        }
        if let Some(file_name) = self.file_name
            && let Some(obj) = body.as_object_mut()
        {
            obj.insert(
                "__file_name".to_string(),
                serde_json::Value::String(file_name),
            );
        }

        let url = AILY_V1_AGENT_ATTACHMENTS.replace("{agent_id}", &self.agent_id);
        let req: ApiRequest<CreateAgentAttachmentResponse> =
            ApiRequest::post(&url).body(body).file_content(file_bytes);

        Transport::request_typed(req, &self.config, Some(option), "上传附件").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CreateAgentAttachmentRequest::new(config);
    }

    #[test]
    fn builder_sets_fields() {
        let config = Arc::new(Config::default());
        let request = CreateAgentAttachmentRequest::new(config)
            .agent_id("agent_123")
            .attachment_type("image")
            .file_name("photo.png")
            .doc_url("https://bytedance.larkoffice.com/wiki/PRSdJR8NowW5gxguFGcm9Hpn");
        assert_eq!(request.agent_id, "agent_123");
        assert_eq!(request.attachment_type, Some("image".to_string()));
        assert_eq!(request.file_name, Some("photo.png".to_string()));
        assert!(request.doc_url.is_some());
    }

    #[test]
    fn test_create_agent_attachment_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_ATTACHMENTS;
        let url = AILY_V1_AGENT_ATTACHMENTS.replace("{agent_id}", "agent_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/attachments");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }

    /// 端到端：POST /open-apis/aily/v1/agents/agent_1/attachments（multipart）
    #[tokio::test]
    async fn test_create_agent_attachment_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/aily/v1/agents/agent_1/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "agent_attachment_id": "3d058789-6952-4697-bf9c-1add1ebc206e" }
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

        let resp = CreateAgentAttachmentRequest::new(config)
            .agent_id("agent_1")
            .attachment_type("image")
            .execute(vec![1, 2, 3, 4])
            .await
            .expect("请求应成功");
        assert_eq!(
            resp.agent_attachment_id,
            "3d058789-6952-4697-bf9c-1add1ebc206e"
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    #[tokio::test]
    async fn test_create_agent_attachment_rejects_empty_file() {
        let config = Arc::new(Config::default());
        let result = CreateAgentAttachmentRequest::new(config)
            .agent_id("agent_1")
            .attachment_type("image")
            .execute(vec![])
            .await;
        assert!(result.is_err(), "空文件内容应被必填校验拒绝");
    }
}
