//! 删除指定客服的工作日程
//!
//! 删除指定客服的工作日程。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 删除客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentScheduleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteAgentScheduleResult>,
}

impl openlark_core::api::ApiResponseTrait for DeleteAgentScheduleResponse {}

/// 删除客服工作日程结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentScheduleResult {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 删除客服工作日程请求
#[derive(Debug, Clone)]
pub struct DeleteAgentScheduleRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl DeleteAgentScheduleRequest {
    /// 创建新的删除客服工作日程请求
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self { config, agent_id }
    }

    /// 执行删除客服工作日程请求
    pub async fn execute(self) -> SDKResult<DeleteAgentScheduleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行删除客服工作日程请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteAgentScheduleResponse> {
        let req: ApiRequest<DeleteAgentScheduleResponse> =
            ApiRequest::delete(HelpdeskApiV1::AgentScheduleDelete(self.agent_id.clone()).to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除客服工作日程")
    }
}

/// 删除客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct DeleteAgentScheduleRequestBuilder {
    config: Arc<Config>,
    agent_id: String,
}

impl DeleteAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self { config, agent_id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<DeleteAgentScheduleResponse> {
        let request = DeleteAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<DeleteAgentScheduleResponse> {
        let request = DeleteAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行删除客服工作日程
pub async fn delete_agent_schedule(
    config: &Config,
    agent_id: String,
) -> SDKResult<DeleteAgentScheduleResponse> {
    delete_agent_schedule_with_options(config, agent_id, RequestOption::default()).await
}

/// 执行删除客服工作日程（支持自定义选项）
pub async fn delete_agent_schedule_with_options(
    config: &Config,
    agent_id: String,
    option: RequestOption,
) -> SDKResult<DeleteAgentScheduleResponse> {
    let req: ApiRequest<DeleteAgentScheduleResponse> =
        ApiRequest::delete(HelpdeskApiV1::AgentScheduleDelete(agent_id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "删除客服工作日程")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            DeleteAgentScheduleRequestBuilder::new(Arc::new(config), "agent_123".to_string());

        assert_eq!(builder.agent_id, "agent_123");
    }
}
