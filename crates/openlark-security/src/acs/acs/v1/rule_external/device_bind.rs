//! 设备绑定权限组
//!
//! docPath: https://open.feishu.cn/document/acs-v1/rule_external/device_bind
//!
//! 文档核对：`POST /open-apis/acs/v1/rule_external/device_bind`，flat body
//! `{"device_id": "<单个>", "rule_ids": [...]}`。
//! 旧实现字段名错误（发的是 `{rule_id, device_ids[], overwrite}`），已修正。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 设备绑定权限组请求 body（按文档字段名）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBindBody {
    /// 设备 ID（单个，非数组）。
    pub device_id: String,
    /// 权限组 ID 列表。
    pub rule_ids: Vec<String>,
}

/// 设备绑定权限组请求
#[derive(Debug)]
pub struct BindDeviceToRuleRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: DeviceBindBody,
}

impl BindDeviceToRuleRequest {
    /// 创建新的请求构建器（`device_id` 单个，`rule_ids` 数组）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: DeviceBindBody {
                device_id: String::new(),
                rule_ids: Vec::new(),
            },
        }
    }

    /// 设置设备 ID（单个）。
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.body.device_id = device_id.into();
        self
    }

    /// 设置权限组 ID 列表。
    pub fn rule_ids(mut self, rule_ids: Vec<String>) -> Self {
        self.body.rule_ids = rule_ids;
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.body.device_id, "device_id 不能为空");
        validate_required_list!(
            self.body.rule_ids,
            10000,
            "rule_ids 不能为空且不能超过 10000 个"
        );

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/acs/v1/rule_external/device_bind")
                .body(
                    serde_json::to_value(&self.body).map_err(|e| {
                        validation_error("设备绑定权限组", format!("序列化失败: {e}"))
                    })?,
                )
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("设备绑定权限组", "响应数据为空"))
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
    async fn test_bind_device_rejects_empty_device_id() {
        let req = BindDeviceToRuleRequest::new(test_config()).rule_ids(vec!["r1".into()]);
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_id"));
    }

    #[tokio::test]
    async fn test_bind_device_rejects_empty_rule_ids() {
        let req = BindDeviceToRuleRequest::new(test_config()).device_id("dev_1");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule_ids"));
    }
}
