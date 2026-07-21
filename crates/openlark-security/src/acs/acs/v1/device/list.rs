//! 获取门禁设备列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/device/list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};

/// 获取门禁设备列表请求
///
/// 支持分页及设备类型过滤。
#[derive(Debug)]
pub struct ListDevicesRequest {
    /// 配置信息。
    config: Config,
    /// 页面大小（可选）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
    /// 设备类型过滤（可选）。
    device_type: Option<String>,
}

impl ListDevicesRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            device_type: None,
        }
    }

    /// 设置页面大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置设备类型过滤。
    pub fn device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = Some(device_type.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = ApiRequest::get("/open-apis/acs/v1/devices")
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token.as_ref())
            .query_opt("device_type", self.device_type.as_ref())
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        Transport::request_typed(req, &self.config, Some(option), "获取门禁设备列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET + query 参数拼装 + 响应解析。
    #[tokio::test]
    async fn test_list_devices_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/devices"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "device_id": "dev_001" }, { "device_id": "dev_002" }],
                    "page_token": "next_page",
                    "has_more": true
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ListDevicesRequest::new(config)
            .page_size(10)
            .device_type("gate")
            .execute()
            .await
            .expect("获取设备列表应成功");

        assert_eq!(data["items"].as_array().unwrap().len(), 2);
        assert_eq!(data["page_token"], "next_page");
        assert_eq!(data["has_more"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=10"));
        assert!(query.contains("device_type=gate"));
    }
}
