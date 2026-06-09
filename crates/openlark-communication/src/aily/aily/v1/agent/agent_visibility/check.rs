//! 获取当前用户的可见性
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/aily-v1/agent-visibility/check

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::AILY_V1_AGENT_VISIBILITY_CHECK;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取当前用户的可见性请求。
#[derive(Debug, Clone)]
pub struct CheckAgentVisibilityRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl CheckAgentVisibilityRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");

        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let url = AILY_V1_AGENT_VISIBILITY_CHECK.replace("{agent_id}", &self.agent_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(&url).body(serialize_params(&body, "获取当前用户的可见性")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取当前用户的可见性")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CheckAgentVisibilityRequest::new(config);
    }

    #[test]
    fn builder_sets_agent_id() {
        let config = Arc::new(Config::default());
        let request = CheckAgentVisibilityRequest::new(config).agent_id("agent_123");
        assert_eq!(request.agent_id, "agent_123");
    }
}
