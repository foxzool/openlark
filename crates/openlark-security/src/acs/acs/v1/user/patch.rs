//! 修改用户部分信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 修改用户信息请求
///
/// 通过 body 传入需要更新的字段。本计划范围只做 Transport 迁移，body 用
/// `serde_json::Value` 透传（字段细化见 spec §9，不在本次范围）。
#[derive(Debug)]
pub struct PatchUserRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
    /// 请求体（可选，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl PatchUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
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
        validate_required!(self.user_id, "user_id 不能为空");

        let path = format!("/open-apis/acs/v1/users/{}", self.user_id);
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(&path).with_supported_access_token_types(vec![AccessTokenType::App]);
        if let Some(body) = self.body {
            req = req.body(body);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("修改用户信息", "响应数据为空"))
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
    async fn test_patch_user_rejects_empty_id() {
        let req = PatchUserRequest::new(test_config(), "  ");
        let result = req.execute_with_options(RequestOption::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }
}
