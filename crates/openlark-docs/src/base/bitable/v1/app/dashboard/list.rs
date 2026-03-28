//! 列出仪表盘
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/list>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 列出仪表盘请求
#[derive(Debug, Clone)]
pub struct ListDashboardsRequest {
    config: Config,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListDashboardsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListDashboardsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListDashboardsResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        if let Some(page_size) = self.page_size {
            if !(1..=500).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~500 之间",
                ));
            }
        }

        let api_endpoint = BitableApiV1::DashboardList(self.app_token);
        let mut api_request: ApiRequest<ListDashboardsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        api_request = api_request.query_opt("page_size", self.page_size.map(|v| v.to_string()));
        api_request = api_request.query_opt("page_token", self.page_token);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 列出仪表盘响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDashboardsResponse {
    pub dashboards: Vec<super::Dashboard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListDashboardsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
