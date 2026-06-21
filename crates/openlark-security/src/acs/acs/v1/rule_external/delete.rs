//! 删除权限组
//!
//! docPath: https://open.feishu.cn/document/acs-v1/rule_external/delete
//!
//! 文档核对：`DELETE /open-apis/acs/v1/rule_external?rule_id={rule_id}`，**无 body**。
//! `rule_id` 是查询参数。旧实现错误地发了 JSON body。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};

/// 删除权限组请求
#[derive(Debug)]
pub struct DeleteRuleExternalRequest {
    /// 配置信息。
    config: Config,
    /// 权限组 ID（查询参数，必填）。
    rule_id: String,
}

impl DeleteRuleExternalRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            rule_id: rule_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.rule_id, "rule_id 不能为空");

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete("/open-apis/acs/v1/rule_external")
                .query("rule_id", &self.rule_id)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("删除权限组", "响应数据为空"))
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
    async fn test_delete_rule_external_rejects_empty_id() {
        let req = DeleteRuleExternalRequest::new(test_config(), "  ");
        let result = req.execute_with_options(RequestOption::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule_id"));
    }
}
