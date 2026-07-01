//! 下载智能体产物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/aily-v1/agent-artifact/get>

use crate::common::api_utils::extract_response_data;
use crate::endpoints::AILY_V1_AGENT_ARTIFACT;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 下载智能体产物请求。
#[derive(Debug, Clone)]
pub struct GetAgentArtifactRequest {
    config: Arc<Config>,
    agent_id: String,
    agent_artifact_id: String,
}

impl GetAgentArtifactRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
            agent_artifact_id: String::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 设置路径参数 `agent_artifact_id`。
    pub fn agent_artifact_id(mut self, agent_artifact_id: impl Into<String>) -> Self {
        self.agent_artifact_id = agent_artifact_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");
        validate_required!(self.agent_artifact_id, "agent_artifact_id 不能为空");

        let url = AILY_V1_AGENT_ARTIFACT
            .replace("{agent_id}", &self.agent_id)
            .replace("{agent_artifact_id}", &self.agent_artifact_id);

        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "下载智能体产物")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = GetAgentArtifactRequest::new(config);
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = GetAgentArtifactRequest::new(config)
            .agent_id("agent_123")
            .agent_artifact_id("artifact_456");
        assert_eq!(request.agent_id, "agent_123");
        assert_eq!(request.agent_artifact_id, "artifact_456");
    }

    #[test]
    fn test_get_agent_artifact_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_ARTIFACT;
        let url = AILY_V1_AGENT_ARTIFACT
            .replace("{agent_id}", "agent_1")
            .replace("{agent_artifact_id}", "art_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/artifacts/art_1");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
