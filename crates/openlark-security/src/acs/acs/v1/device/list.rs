//! 获取门禁设备列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/device/list>
//!
//! 文档核对：官方请求仅要求 Authorization 头，无查询参数；响应 `data` 内
//! 仅 `items`（device_id/device_name/device_sn），无分页字段。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};

/// 获取门禁设备列表请求
#[derive(Debug)]
pub struct ListDevicesRequest {
    /// 配置信息。
    config: Config,
}

impl ListDevicesRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = ApiRequest::get("/open-apis/acs/v1/devices")
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

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

    /// 端到端：GET .../devices（官方无查询参数）+ 响应解析。
    #[tokio::test]
    async fn test_list_devices_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/devices"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "device_id": "dev_001" }, { "device_id": "dev_002" }]
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
            .execute()
            .await
            .expect("获取设备列表应成功");

        assert_eq!(data["items"].as_array().unwrap().len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/devices");
        assert!(
            received[0].url.query().is_none(),
            "官方文档无查询参数，请求不应携带 query"
        );
    }
}
