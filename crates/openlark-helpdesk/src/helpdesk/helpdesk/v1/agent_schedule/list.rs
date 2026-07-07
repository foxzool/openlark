//! 获取客服工作日程列表
//!
////! 获取所有客服的工作日程列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/list>

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

/// 获取客服工作日程列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListAgentScheduleQuery {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListAgentScheduleQuery {
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

/// 获取客服工作日程列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentScheduleResponse {
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

impl ApiResponseTrait for ListAgentScheduleResponse {
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

/// 获取客服工作日程列表请求
#[derive(Debug, Clone)]
pub struct ListAgentScheduleRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListAgentScheduleRequest {
    /// 创建新的获取客服工作日程列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
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

    /// 执行获取客服工作日程列表请求
    pub async fn execute(self) -> SDKResult<ListAgentScheduleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListAgentScheduleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentScheduleList;
        let mut request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取客服工作日程列表")
    }
}

/// 获取客服工作日程列表请求构建器
#[derive(Debug, Clone)]
pub struct ListAgentScheduleRequestBuilder {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
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
    pub async fn execute(&self) -> SDKResult<ListAgentScheduleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentScheduleList;
        let mut request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取客服工作日程列表")
    }
}

/// 执行获取客服工作日程列表
pub async fn list_agent_schedules(config: &Config) -> SDKResult<ListAgentScheduleResponse> {
    let api_endpoint = HelpdeskApiV1::AgentScheduleList;
    let request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取客服工作日程列表")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_query_default() {
        let query = ListAgentScheduleQuery::default();
        let result = query.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_with_page_size() {
        let query = ListAgentScheduleQuery {
            page_size: Some(10),
            page_token: None,
        };
        let result = query.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_with_invalid_page_size() {
        let query = ListAgentScheduleQuery {
            page_size: Some(-1),
            page_token: None,
        };
        let result = query.validate();
        assert!(result.is_err());
    }

    /// 端到端：GET .../agent_schedules → 强类型 ListAgentScheduleResponse 解析（单层 data 信封，items 直挂）。
    #[tokio::test]
    async fn test_list_agent_schedule_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/agent_schedules"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "agent_id": "ag_001", "work_date": "2024-01-15" }
                    ],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = ListAgentScheduleRequest::new(config)
            .execute()
            .await
            .expect("获取客服工作日程列表应成功");
        assert!(resp.items.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_schedules"
        );
    }
}
