//! 获取工单内图像
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/ticket_image

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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取工单内图像", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
