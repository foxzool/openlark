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

/// 获取客服工作日程列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListAgentScheduleQuery {
    /// 筛选条件（必填）：1-online 客服；2-offline(手动)客服；3-off duty(下班)客服；4-移除客服。
    /// GET 请求中以 `status=1&status=2` 同名多次传递。
    pub status: Vec<i32>,
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
        openlark_core::validate_required!(self.status, "status 不能为空");
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
    status: Vec<i32>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListAgentScheduleRequest {
    /// 创建新的获取客服工作日程列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            status: Vec::new(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置筛选条件（1-online；2-offline 手动；3-off duty；4-移除），同名多次传递
    pub fn status(mut self, status: Vec<i32>) -> Self {
        self.status = status;
        self
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
        openlark_core::validate_required!(self.status, "status 不能为空");

        let api_endpoint = HelpdeskApiV1::AgentScheduleList;
        let mut request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url());

        request = request.query_multi("status", self.status.iter().map(|s| s.to_string()));

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(request, &self.config, Some(option), "获取客服工作日程列表").await
    }
}

/// 获取客服工作日程列表请求构建器
#[derive(Debug, Clone)]
pub struct ListAgentScheduleRequestBuilder {
    config: Arc<Config>,
    status: Vec<i32>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            status: Vec::new(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置筛选条件（1-online；2-offline 手动；3-off duty；4-移除），同名多次传递
    pub fn status(mut self, status: Vec<i32>) -> Self {
        self.status = status;
        self
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
        openlark_core::validate_required!(self.status, "status 不能为空");

        let api_endpoint = HelpdeskApiV1::AgentScheduleList;
        let mut request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url());

        request = request.query_multi("status", self.status.iter().map(|s| s.to_string()));

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(request, &self.config, None, "获取客服工作日程列表").await
    }
}

/// 执行获取客服工作日程列表
pub async fn list_agent_schedules(
    config: &Config,
    status: Vec<i32>,
) -> SDKResult<ListAgentScheduleResponse> {
    openlark_core::validate_required!(status, "status 不能为空");

    let api_endpoint = HelpdeskApiV1::AgentScheduleList;
    let request = ApiRequest::<ListAgentScheduleResponse>::get(api_endpoint.to_url())
        .query_multi("status", status.iter().map(|s| s.to_string()));

    Transport::request_typed(request, config, None, "获取客服工作日程列表").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_query_default_missing_status() {
        // status 为官方必填查询参数，缺省时校验应失败
        let query = ListAgentScheduleQuery::default();
        let result = query.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_query_with_status() {
        let query = ListAgentScheduleQuery {
            status: vec![1, 2],
            page_size: Some(10),
            page_token: None,
        };
        let result = query.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_with_invalid_page_size() {
        let query = ListAgentScheduleQuery {
            status: vec![1],
            page_size: Some(-1),
            page_token: None,
        };
        let result = query.validate();
        assert!(result.is_err());
    }

    /// 端到端：GET .../agent_schedules（status 同名多次传递）→ 强类型 ListAgentScheduleResponse 解析（单层 data 信封，items 直挂）。
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
            .status(vec![1, 2])
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
        // 官方要求同名多次传递：status=1&status=2
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("status=1"), "query 应包含 status=1: {query}");
        assert!(query.contains("status=2"), "query 应包含 status=2: {query}");
    }
}
