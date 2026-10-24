//! 获取工单内图像
//!
//! 官方查询参数为 `ticket_id`（必填）、`msg_id`（必填）、`index`（选填，
//! 消息类型为 post 时需指定图片 index，从 0 开始；img 消息无需传）。
//! 响应为「返回文件二进制流」，按二进制下载处理，返回原始文件字节。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/ticket_image>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取工单图片请求。
#[derive(Debug, Clone)]
pub struct GetTicketImageRequest {
    config: Arc<Config>,
    /// 工单 ID（必填）。
    ticket_id: String,
    /// 消息 ID（必填）。
    msg_id: String,
    /// 图片 index（选填；消息类型为 post 时需指定，从 0 开始；img 消息无需传）。
    index: Option<i32>,
}

impl GetTicketImageRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        ticket_id: impl Into<String>,
        msg_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            msg_id: msg_id.into(),
            index: None,
        }
    }

    /// 设置图片 index（消息类型为 post 时需指定，从 0 开始）。
    pub fn index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }

    /// 执行请求（返回图片二进制内容）。
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求（返回图片二进制内容）。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Vec<u8>> {
        validate_required!(self.ticket_id, "ticket_id 不能为空");
        validate_required!(self.msg_id, "msg_id 不能为空");

        let mut req: ApiRequest<Vec<u8>> = ApiRequest::get(HelpdeskApiV1::TicketImage.to_url())
            .query("ticket_id", self.ticket_id)
            .query("msg_id", self.msg_id);
        if let Some(index) = self.index {
            req = req.query("index", index.to_string());
        }

        Transport::request_typed(req, &self.config, Some(option), "获取工单内图像").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../ticket_images（带 ticket_id/msg_id/index 查询参数）→ 返回原始文件二进制流。
    #[tokio::test]
    async fn test_get_ticket_image_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let image_bytes = b"FAKE_IMAGE_BYTES".to_vec();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/ticket_images"))
            .and(query_param("ticket_id", "tk_001"))
            .and(query_param("msg_id", "om_001"))
            .and(query_param("index", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(image_bytes.clone()))
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

        let bytes = GetTicketImageRequest::new(config, "tk_001", "om_001")
            .index(0)
            .execute()
            .await
            .expect("获取工单内图像应成功");
        assert_eq!(bytes, image_bytes);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_images"
        );
        // 验证查询参数正确传递（ticket_id/msg_id/index）
        assert_eq!(received[0].url.query_pairs().count(), 3);
    }
}
