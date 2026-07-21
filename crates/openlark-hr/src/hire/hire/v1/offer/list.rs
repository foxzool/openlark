//! 获取 Offer 列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 获取 Offer 列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::OfferList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 Offer 列表响应数据为空",
        )
        .await
    }
}

/// 获取 Offer 列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferJobInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_name` 字段。
    pub job_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferCatalogRef`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCatalogRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferListItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferListItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<OfferJobInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_status` 字段。
    pub offer_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_type` 字段。
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_type` 字段。
    pub employee_type: Option<OfferCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ListResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<OfferListItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/hire/v1/offers
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/offers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
