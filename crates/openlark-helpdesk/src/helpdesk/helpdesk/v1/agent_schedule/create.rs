//! 创建客服工作日程
//!
//! 为指定客服创建工作日程。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/create

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建客服工作日程请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentScheduleBody {
    /// 客服ID
    pub agent_id: String,
    /// 工作日期 (格式: YYYY-MM-DD)
    pub work_date: String,
    /// 开始时间 (格式: HH:mm:ss)
    pub start_time: String,
    /// 结束时间 (格式: HH:mm:ss)
    pub end_time: String,
    /// 星期几 (1-7, 1表示周一)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}

impl CreateAgentScheduleBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.agent_id, "agent_id is required");
        validate_required!(self.work_date, "work_date is required");
        validate_required!(self.start_time, "start_time is required");
        validate_required!(self.end_time, "end_time is required");
        if self.start_time >= self.end_time {
            return Err(openlark_core::CoreError::validation_msg(
                "start_time must be less than end_time",
            ));
        }
        Ok(())
    }
}

/// 创建客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentScheduleResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateAgentScheduleResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateAgentScheduleResponse {}

/// 创建客服工作日程结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentScheduleResult {
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
    /// 星期几
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}

/// 创建客服工作日程请求
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleRequest {
    config: Arc<Config>,
}

impl CreateAgentScheduleRequest {
    /// 创建新的创建客服工作日程请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建客服工作日程请求
    pub async fn execute(
        self,
        body: CreateAgentScheduleBody,
    ) -> SDKResult<CreateAgentScheduleResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建客服工作日程请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateAgentScheduleBody,
        option: RequestOption,
    ) -> SDKResult<CreateAgentScheduleResponse> {
        body.validate()?;

        let req: ApiRequest<CreateAgentScheduleResponse> =
            ApiRequest::post(HelpdeskApiV1::AgentScheduleCreate.to_url())
                .body(serialize_params(&body, "创建客服工作日程")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建客服工作日程")
    }
}

/// 创建客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleRequestBuilder {
    config: Arc<Config>,
    agent_id: Option<String>,
    work_date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    day_of_week: Option<i32>,
}

impl CreateAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: None,
            work_date: None,
            start_time: None,
            end_time: None,
            day_of_week: None,
        }
    }

    /// 设置客服ID
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    /// 设置工作日期 (格式: YYYY-MM-DD)
    pub fn work_date(mut self, work_date: impl Into<String>) -> Self {
        self.work_date = Some(work_date.into());
        self
    }

    /// 设置开始时间 (格式: HH:mm:ss)
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间 (格式: HH:mm:ss)
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 设置星期几 (1-7)
    pub fn day_of_week(mut self, day_of_week: i32) -> Self {
        self.day_of_week = Some(day_of_week);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateAgentScheduleBody, String> {
        let agent_id = self.agent_id.clone().ok_or("agent_id is required")?;
        let work_date = self.work_date.clone().ok_or("work_date is required")?;
        let start_time = self.start_time.clone().ok_or("start_time is required")?;
        let end_time = self.end_time.clone().ok_or("end_time is required")?;

        Ok(CreateAgentScheduleBody {
            agent_id,
            work_date,
            start_time,
            end_time,
            day_of_week: self.day_of_week,
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateAgentScheduleResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateAgentScheduleRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建客服工作日程
pub async fn create_agent_schedule(
    config: &Config,
    body: CreateAgentScheduleBody,
) -> SDKResult<CreateAgentScheduleResponse> {
    create_agent_schedule_with_options(config, body, RequestOption::default()).await
}

/// 执行创建客服工作日程（支持自定义选项）
pub async fn create_agent_schedule_with_options(
    config: &Config,
    body: CreateAgentScheduleBody,
    option: RequestOption,
) -> SDKResult<CreateAgentScheduleResponse> {
    body.validate()?;

    let req: ApiRequest<CreateAgentScheduleResponse> =
        ApiRequest::post(HelpdeskApiV1::AgentScheduleCreate.to_url())
            .body(serialize_params(&body, "创建客服工作日程")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建客服工作日程")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateAgentScheduleBody {
            agent_id: "agent_123".to_string(),
            work_date: "2024-01-15".to_string(),
            start_time: "09:00:00".to_string(),
            end_time: "18:00:00".to_string(),
            day_of_week: Some(1),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_agent_id() {
        let body = CreateAgentScheduleBody {
            agent_id: "".to_string(),
            work_date: "2024-01-15".to_string(),
            start_time: "09:00:00".to_string(),
            end_time: "18:00:00".to_string(),
            day_of_week: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_invalid_time() {
        let body = CreateAgentScheduleBody {
            agent_id: "agent_123".to_string(),
            work_date: "2024-01-15".to_string(),
            start_time: "18:00:00".to_string(),
            end_time: "09:00:00".to_string(),
            day_of_week: None,
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
        let builder = CreateAgentScheduleRequestBuilder::new(Arc::new(config));

        assert!(builder.agent_id.is_none());
    }
}
