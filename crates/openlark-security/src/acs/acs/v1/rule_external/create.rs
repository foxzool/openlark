//! 创建或更新权限组
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/rule_external/create>
//!
//! 文档核对：`POST /open-apis/acs/v1/rule_external?rule_id={rule_id}&user_id_type={...}`，
//! body 为 `{"rule": {...}}` 包装（`12cc9fe09` 的包装正确，但补回了漏掉的查询参数）。
//! body 内容（devices 等）用 `serde_json::Value` 透传，字段细化见 spec §9。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 创建或更新权限组请求
///
/// `rule` 的内容（devices 等）通过 [`Self::rule`] 以 `serde_json::Value` 传入，
/// 内部自动包装为 `{"rule": ...}`。
#[derive(Debug)]
pub struct CreateRuleExternalRequest {
    /// 配置信息。
    config: Config,
    /// 权限组 ID（查询参数，必填）。
    rule_id: String,
    /// 用户 ID 类型（查询参数，可选）。
    user_id_type: Option<String>,
    /// 权限组内容（必填，作为 body 的 `rule` 字段内容）。
    rule: Option<serde_json::Value>,
}

impl CreateRuleExternalRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            rule_id: rule_id.into(),
            user_id_type: None,
            rule: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置权限组内容（`rule` 字段）。
    pub fn rule(mut self, rule: serde_json::Value) -> Self {
        self.rule = Some(rule);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.rule_id, "rule_id 不能为空");
        let rule = self
            .rule
            .ok_or_else(|| validation_error("创建或更新权限组", "rule 不能为空"))?;

        // 文档要求 body 为 {"rule": <内容>}
        let wrapped = serde_json::json!({ "rule": rule });
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/acs/v1/rule_external")
                .query("rule_id", &self.rule_id)
                .query_opt("user_id_type", self.user_id_type.as_ref())
                .body(wrapped)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("创建或更新权限组", "响应数据为空"))
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
    async fn test_create_rule_external_rejects_empty_rule_id() {
        let req = CreateRuleExternalRequest::new(test_config(), "").rule(serde_json::json!({}));
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule_id"));
    }

    #[tokio::test]
    async fn test_create_rule_external_rejects_empty_rule() {
        let req = CreateRuleExternalRequest::new(test_config(), "rule_123");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule"));
    }
}
