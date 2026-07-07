//! 获取单个设备信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 获取单个设备信息请求
#[derive(Debug)]
pub struct GetDeviceRecordRequest {
    /// 配置信息。
    config: Config,
    /// 设备记录 ID（路径参数，必填）。
    device_record_id: String,
}

impl GetDeviceRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_record_id: device_record_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.device_record_id, "device_record_id 不能为空");

        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            self.device_record_id
        );
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("获取设备信息", "响应数据为空"))
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
    async fn test_get_device_record_rejects_empty_id() {
        let req = GetDeviceRecordRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_record_id"));
    }

    /// 端到端：GET .../device_records/{id} + 响应解析。
    #[tokio::test]
    async fn test_get_device_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/security_and_compliance/v2/device_records/dr_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "device_record_id": "dr_001", "status": "approved" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetDeviceRecordRequest::new(config, "dr_001")
            .execute()
            .await
            .expect("获取设备记录应成功");
        assert_eq!(data["device_record_id"], "dr_001");
        assert_eq!(data["status"], "approved");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/security_and_compliance/v2/device_records/dr_001"
        );
    }
}
