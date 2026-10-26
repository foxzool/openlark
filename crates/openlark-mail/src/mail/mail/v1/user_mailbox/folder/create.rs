//! 创建邮箱文件夹

use crate::common::api_utils::serialize_params;
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

/// Create Mailbox Folder Request。
#[derive(Debug, Clone)]
pub struct CreateMailboxFolderRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    body: CreateMailboxFolderBody,
}

/// 创建邮箱文件夹的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailboxFolderBody {
    /// 文件夹名称（1～250 字符）。
    pub name: String,
    /// 父文件夹 id，该值为 0 表示根文件夹。
    pub parent_folder_id: String,
}

/// Create Mailbox Folder Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailboxFolderResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateMailboxFolderRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            body: CreateMailboxFolderBody::default(),
        }
    }

    /// 设置文件夹名称（必填，1～250 字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置父文件夹 id（必填，0 表示根文件夹）。
    pub fn parent_folder_id(mut self, parent_folder_id: impl Into<String>) -> Self {
        self.body.parent_folder_id = parent_folder_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateMailboxFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailboxFolderResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.body.name, "name 不能为空");
        validate_required!(self.body.parent_folder_id, "parent_folder_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/folders",
            self.user_mailbox_id
        );
        let req: ApiRequest<CreateMailboxFolderResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮箱文件夹")?);

        Transport::request_typed(req, &self.config, Some(option), "创建邮箱文件夹").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_mailboxes/{umb}/folders → CreateMailboxFolderResponse 解析，请求体对齐官方示例。
    #[tokio::test]
    async fn test_create_mailbox_folder_sends_official_body() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mail/v1/user_mailboxes/umb_001/folders"))
            .and(body_json(json!({
                "name": "newsletter 相关",
                "parent_folder_id": "725627422334644"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "folder_id": "f_001" } }
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

        let resp = CreateMailboxFolderRequest::new(config, "umb_001")
            .name("newsletter 相关")
            .parent_folder_id("725627422334644")
            .execute()
            .await
            .expect("创建邮箱文件夹应成功");
        assert_eq!(resp.data.unwrap()["folder_id"], "f_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/folders"
        );
    }

    /// 失败路径：未设置 parent_folder_id 时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_create_mailbox_folder_rejects_missing_parent() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = CreateMailboxFolderRequest::new(config, "umb_001")
            .name("newsletter")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
