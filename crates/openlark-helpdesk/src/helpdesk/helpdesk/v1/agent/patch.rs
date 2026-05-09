//! 更新客服信息
//!
//! 更新客服状态等信息。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent/patch

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新客服信息请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAgentBody {
    /// 客服状态: offline:离线, online:在线, busy:忙碌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PatchAgentBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(status) = &self.status
            && !["offline", "online", "busy"].contains(&status.as_str())
        {
            return Err(openlark_core::CoreError::validation_msg(
                "status must be offline, online, or busy",
            ));
        }
        Ok(())
    }
}

/// 更新客服信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchAgentResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchAgentResponse {}

/// 更新客服信息结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentResult {
    /// 客服ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// 客服状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 更新客服信息请求
#[derive(Debug, Clone)]
pub struct PatchAgentRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl PatchAgentRequest {
    /// 创建新的更新客服信息请求
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self { config, agent_id }
    }

    /// 执行更新客服信息请求
    pub async fn execute(self, body: PatchAgentBody) -> SDKResult<PatchAgentResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新客服信息请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchAgentBody,
        option: RequestOption,
    ) -> SDKResult<PatchAgentResponse> {
        body.validate()?;

        let req: ApiRequest<PatchAgentResponse> =
            ApiRequest::patch(HelpdeskApiV1::AgentPatch(self.agent_id.clone()).to_url())
                .body(serialize_params(&body, "更新客服信息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新客服信息")
    }
}

/// 更新客服信息请求构建器
#[derive(Debug, Clone)]
pub struct PatchAgentRequestBuilder {
    config: Arc<Config>,
    agent_id: String,
    status: Option<String>,
}

impl PatchAgentRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self {
            config,
            agent_id,
            status: None,
        }
    }

    /// 设置客服状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchAgentBody {
        PatchAgentBody {
            status: self.status.clone(),
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchAgentResponse> {
        let body = self.body();
        let request = PatchAgentRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchAgentResponse> {
        let body = self.body();
        let request = PatchAgentRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新客服信息
pub async fn patch_agent(
    config: &Config,
    agent_id: String,
    body: PatchAgentBody,
) -> SDKResult<PatchAgentResponse> {
    patch_agent_with_options(config, agent_id, body, RequestOption::default()).await
}

/// 执行更新客服信息（支持自定义选项）
pub async fn patch_agent_with_options(
    config: &Config,
    agent_id: String,
    body: PatchAgentBody,
    option: RequestOption,
) -> SDKResult<PatchAgentResponse> {
    body.validate()?;

    let req: ApiRequest<PatchAgentResponse> =
        ApiRequest::patch(HelpdeskApiV1::AgentPatch(agent_id).to_url())
            .body(serialize_params(&body, "更新客服信息")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新客服信息")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid_status() {
        let body = PatchAgentBody {
            status: Some("online".to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_invalid_status() {
        let body = PatchAgentBody {
            status: Some("invalid_status".to_string()),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchAgentRequestBuilder::new(Arc::new(config), "agent_123".to_string());

        assert!(builder.status.is_none());
    }
}
