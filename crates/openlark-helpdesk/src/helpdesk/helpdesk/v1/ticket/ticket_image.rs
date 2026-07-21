//! 获取工单内图像
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/ticket_image>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取工单图片请求。
#[derive(Debug, Clone)]
pub struct GetTicketImageRequest {
    config: Arc<Config>,
    ticket_id: String,
    image_key: String,
}

/// 获取工单图片响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketImageResponse {
    /// 响应数据。
    pub data: Option<GetTicketImageData>,
}

impl ApiResponseTrait for GetTicketImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工单图片数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketImageData {
    /// 图片地址。
    pub image_url: String,
}

impl GetTicketImageRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        ticket_id: impl Into<String>,
        image_key: impl Into<String>,
    ) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            image_key: image_key.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetTicketImageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetTicketImageResponse> {
        let req: ApiRequest<GetTicketImageResponse> =
            ApiRequest::get(HelpdeskApiV1::TicketImage.to_url())
                .query("ticket_id", self.ticket_id)
                .query("image_key", self.image_key);

        Transport::request_typed(req, &self.config, Some(option), "获取工单内图像").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../ticket_images（带 ticket_id/image_key 查询参数）→ 强类型 GetTicketImageResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_ticket_image_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/ticket_images"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "image_url": "https://example.com/img_001.png" } }
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

        let resp = GetTicketImageRequest::new(config, "tk_001", "img_key_001")
            .execute()
            .await
            .expect("获取工单内图像应成功");
        assert!(resp.data.is_some());
        assert_eq!(
            resp.data.unwrap().image_url,
            "https://example.com/img_001.png"
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_images"
        );
        // 验证查询参数正确传递
        assert_eq!(received[0].url.query_pairs().count(), 2);
    }
}
