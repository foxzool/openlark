//! 获取权限组信息
//!
//! docPath: https://open.feishu.cn/document/acs-v1/rule_external/get
//!
//! 文档核对：`GET /open-apis/acs/v1/rule_external?device_id={device_id}&user_id_type={...}`。
//! 查询参数为 `device_id`（必填）+ `user_id_type`（可选），无 body。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 获取权限组信息请求
#[derive(Debug)]
pub struct GetRuleExternalRequest {
    /// 配置信息。
    config: Config,
    /// 设备 ID（查询参数，必填）。
    device_id: String,
    /// 用户 ID 类型（查询参数，可选，如 `open_id`/`user_id`/`union_id`）。
    user_id_type: Option<String>,
}

impl GetRuleExternalRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.device_id, "device_id 不能为空");

        let req: ApiRequest<serde_json::Value> = ApiRequest::get("/open-apis/acs/v1/rule_external")
            .query("device_id", &self.device_id)
            .query_opt("user_id_type", self.user_id_type.as_ref())
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("获取权限组信息", "响应数据为空"))
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
    async fn test_get_rule_external_rejects_empty_device_id() {
        let req = GetRuleExternalRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_id"));
    }
}
