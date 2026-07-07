//! 查询设备信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/device/query>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 查询设备信息请求
#[derive(Debug)]
pub struct QueryDeviceRequest {
    /// 配置信息。
    config: Config,
    /// 设备 ID（路径参数，必填）。
    device_id: String,
}

impl QueryDeviceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.device_id, "device_id 不能为空");

        let path = format!("/open-apis/acs/v1/devices/{}/query", self.device_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("查询设备信息", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_query_device_rejects_empty_id() {
        let req = QueryDeviceRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_id"));
    }

    /// 端到端：GET .../devices/{id}/query + 响应解析。
    #[tokio::test]
    async fn test_query_device_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/devices/dev_001/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "device_id": "dev_001", "status": "online" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryDeviceRequest::new(config, "dev_001")
            .execute()
            .await
            .expect("查询设备信息应成功");
        assert_eq!(data["device_id"], "dev_001");
        assert_eq!(data["status"], "online");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/acs/v1/devices/dev_001/query"
        );
    }
}
