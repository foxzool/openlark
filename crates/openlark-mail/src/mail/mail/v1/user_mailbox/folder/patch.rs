//! 修改邮箱文件夹

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mailbox Folder Request。
#[derive(Debug, Clone)]
pub struct PatchMailboxFolderRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    folder_id: String,
}

/// Patch Mailbox Folder Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailboxFolderResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchMailboxFolderRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        folder_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            folder_id: folder_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailboxFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailboxFolderResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/folders/{}",
            self.user_mailbox_id, self.folder_id
        );
        let req: ApiRequest<PatchMailboxFolderResponse> = ApiRequest::patch(&path);

        Transport::request_typed(req, &self.config, Some(option), "修改邮箱文件夹").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../user_mailboxes/{id}/folders/{folder_id} → PatchMailboxFolderResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_mailbox_folder_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/mb_001/folders/folder_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "updated": true } }
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

        let resp = PatchMailboxFolderRequest::new(config, "mb_001", "folder_001")
            .execute()
            .await
            .expect("修改邮箱文件夹应成功");
        assert_eq!(resp.data.unwrap()["updated"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/mb_001/folders/folder_001"
        );
    }
}
