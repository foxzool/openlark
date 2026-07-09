//! 批量查询国家/地区信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/country_region/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询国家/地区信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
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
        use crate::common::api_endpoints::CorehrApiV1;

        let api_endpoint = CorehrApiV1::CountryRegionList;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<ListResponse>::get(endpoint_url);
        let request = request.body(serde_json::json!({
            "page_size": self.page_size,
            "page_token": self.page_token,
        }));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询国家/地区列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量查询国家/地区信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CountryRegionItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 国家/地区条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CountryRegionItem {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
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

    /// 端到端：GET /open-apis/corehr/v1/country_regions
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/country_regions"))
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
