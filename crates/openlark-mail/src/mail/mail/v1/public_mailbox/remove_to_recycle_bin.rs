//! 将公共邮箱移至回收站
//! docPath: <https://open.feishu.cn/document/mail-v1/public-mailbox/public_mailbox/remove_to_recycle_bin>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 将公共邮箱移至回收站的请求。
#[derive(Debug, Clone)]
pub struct RemovePublicMailboxToRecycleBinRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

/// 将公共邮箱移至回收站的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePublicMailboxToRecycleBinResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RemovePublicMailboxToRecycleBinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RemovePublicMailboxToRecycleBinRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    /// 执行将公共邮箱移至回收站请求。
    pub async fn execute(self) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/remove_to_recycle_bin",
            self.public_mailbox_id
        );
        let req: ApiRequest<RemovePublicMailboxToRecycleBinResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("将公共邮箱移至回收站", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../public_mailboxes/{id}/remove_to_recycle_bin → RemovePublicMailboxToRecycleBinResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_remove_public_mailbox_to_recycle_bin_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/public_mailboxes/mb_001/remove_to_recycle_bin",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "success": true } }
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

        let resp = RemovePublicMailboxToRecycleBinRequest::new(config, "mb_001")
            .execute()
            .await
            .expect("将公共邮箱移至回收站应成功");
        assert_eq!(resp.data.unwrap()["success"].as_bool(), Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/public_mailboxes/mb_001/remove_to_recycle_bin"
        );
    }
}
