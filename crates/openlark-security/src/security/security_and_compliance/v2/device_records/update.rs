//! 更新设备信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-update
//!
//! 文档核对：`PUT /open-apis/security_and_compliance/v2/device_records/{device_record_id}`。
//! body 用 `serde_json::Value` 透传（字段细化不在本次范围）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 更新设备信息请求
#[derive(Debug)]
pub struct UpdateDeviceRecordRequest {
    /// 配置信息。
    config: Config,
    /// 设备记录 ID（路径参数，必填）。
    device_record_id: String,
    /// 请求体（可选，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl UpdateDeviceRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_record_id: device_record_id.into(),
            body: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
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
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::put(&path).with_supported_access_token_types(vec![AccessTokenType::App]);
        if let Some(body) = self.body {
            req = req.body(body);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("更新设备信息", "响应数据为空"))
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
    async fn test_update_device_record_rejects_empty_id() {
        let req = UpdateDeviceRecordRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_record_id"));
    }
}
