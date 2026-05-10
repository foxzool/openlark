//! 获取指定客服的工作日程
//!
//! 获取指定客服的工作日程详情。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取指定客服工作日程查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAgentScheduleQuery {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetAgentScheduleQuery {
    /// 验证查询参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(page_size) = self.page_size
            && page_size <= 0
        {
            return Err(openlark_core::CoreError::validation_msg(
                "page_size must be greater than 0",
            ));
        }
        Ok(())
    }
}

/// 获取指定客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentScheduleResponse {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 客服工作日程列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AgentScheduleItem>>,
}

impl ApiResponseTrait for GetAgentScheduleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 客服工作日程项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentScheduleItem {
    /// 客服ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// 工作日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 星期几 (1-7)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}

/// 获取指定客服工作日程请求
#[derive(Debug, Clone)]
pub struct GetAgentScheduleRequest {
    config: Arc<Config>,
    agent_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetAgentScheduleRequest {
    /// 创建新的获取指定客服工作日程请求
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self {
            config,
            agent_id,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行获取指定客服工作日程请求
    pub async fn execute(self) -> SDKResult<GetAgentScheduleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAgentScheduleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentScheduleGet(self.agent_id.clone());
        let mut request = ApiRequest::<GetAgentScheduleResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取指定客服工作日程")
    }
}

/// 获取指定客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct GetAgentScheduleRequestBuilder {
    config: Arc<Config>,
    agent_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self {
            config,
            agent_id,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetAgentScheduleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentScheduleGet(self.agent_id.clone());
        let mut request = ApiRequest::<GetAgentScheduleResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取指定客服工作日程")
    }
}

/// 执行获取指定客服工作日程
pub async fn get_agent_schedule(
    config: &Config,
    agent_id: String,
) -> SDKResult<GetAgentScheduleResponse> {
    let api_endpoint = HelpdeskApiV1::AgentScheduleGet(agent_id);
    let request = ApiRequest::<GetAgentScheduleResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取指定客服工作日程")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_query_default() {
        let query = GetAgentScheduleQuery::default();
        let result = query.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            GetAgentScheduleRequestBuilder::new(Arc::new(config), "agent_123".to_string());

        assert_eq!(builder.agent_id, "agent_123");
        assert!(builder.page_size.is_none());
    }
}
