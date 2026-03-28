//! 获取客服邮箱
//!
//! 该接口用于获取客服邮箱地址。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent/agent_email>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取客服邮箱请求
#[derive(Debug, Clone)]
pub struct GetAgentEmailRequest {
    config: Arc<Config>,
}

impl GetAgentEmailRequest {
    /// 创建新的获取客服邮箱请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取客服邮箱请求
    pub async fn execute(self) -> SDKResult<GetAgentEmailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行获取客服邮箱请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAgentEmailResponse> {
        let req: ApiRequest<GetAgentEmailResponse> =
            ApiRequest::get(HelpdeskApiV1::AgentEmail.to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取客服邮箱")
    }
}

/// 获取客服邮箱响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentEmailResponse {
    /// 客服邮箱列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAgentEmailResult>,
}

impl openlark_core::api::ApiResponseTrait for GetAgentEmailResponse {}

/// 获取客服邮箱结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentEmailResult {
    /// 客服邮箱列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_emails: Option<Vec<AgentEmail>>,
}

/// 客服邮箱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEmail {
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 是否为主邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main: Option<bool>,
}

/// 执行获取客服邮箱
pub async fn get_agent_email(config: &Config) -> SDKResult<GetAgentEmailResponse> {
    get_agent_email_with_options(config, RequestOption::default()).await
}

/// 执行获取客服邮箱（支持自定义选项）
pub async fn get_agent_email_with_options(
    config: &Config,
    option: RequestOption,
) -> SDKResult<GetAgentEmailResponse> {
    let req: ApiRequest<GetAgentEmailResponse> =
        ApiRequest::get(HelpdeskApiV1::AgentEmail.to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "获取客服邮箱")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let request = GetAgentEmailRequest::new(Arc::new(config));
        assert_eq!(request.config.app_id(), "test_app_id");
    }
}
