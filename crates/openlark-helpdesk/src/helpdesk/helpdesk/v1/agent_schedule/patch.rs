//! 更新指定客服的工作日程
//!
//! 更新指定客服的工作日程信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新客服工作日程请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAgentScheduleBody {
    /// 工作日期 (格式: YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<String>,
    /// 开始时间 (格式: HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间 (格式: HH:mm:ss)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 星期几 (1-7)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}

impl PatchAgentScheduleBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let (Some(start_time), Some(end_time)) = (&self.start_time, &self.end_time) {
            if start_time >= end_time {
                return Err("start_time must be less than end_time".to_string());
            }
        }
        Ok(())
    }
}

/// 更新客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentScheduleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchAgentScheduleResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchAgentScheduleResponse {}

/// 更新客服工作日程结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentScheduleResult {
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

/// 更新客服工作日程请求
#[derive(Debug, Clone)]
pub struct PatchAgentScheduleRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl PatchAgentScheduleRequest {
    /// 创建新的更新客服工作日程请求
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self { config, agent_id }
    }

    /// 执行更新客服工作日程请求
    pub async fn execute(
        self,
        body: PatchAgentScheduleBody,
    ) -> SDKResult<PatchAgentScheduleResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新客服工作日程请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchAgentScheduleBody,
        option: RequestOption,
    ) -> SDKResult<PatchAgentScheduleResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<PatchAgentScheduleResponse> =
            ApiRequest::patch(HelpdeskApiV1::AgentSchedulePatch(self.agent_id.clone()).to_url())
                .body(serialize_params(&body, "更新客服工作日程")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新客服工作日程")
    }
}

/// 更新客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct PatchAgentScheduleRequestBuilder {
    config: Arc<Config>,
    agent_id: String,
    work_date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    day_of_week: Option<i32>,
}

impl PatchAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_id: String) -> Self {
        Self {
            config,
            agent_id,
            work_date: None,
            start_time: None,
            end_time: None,
            day_of_week: None,
        }
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
    pub fn body(&self) -> PatchAgentScheduleBody {
        PatchAgentScheduleBody {
            work_date: self.work_date.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
            day_of_week: self.day_of_week,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchAgentScheduleResponse> {
        let body = self.body();
        let request = PatchAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchAgentScheduleResponse> {
        let body = self.body();
        let request = PatchAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新客服工作日程
pub async fn patch_agent_schedule(
    config: &Config,
    agent_id: String,
    body: PatchAgentScheduleBody,
) -> SDKResult<PatchAgentScheduleResponse> {
    patch_agent_schedule_with_options(config, agent_id, body, RequestOption::default()).await
}

/// 执行更新客服工作日程（支持自定义选项）
pub async fn patch_agent_schedule_with_options(
    config: &Config,
    agent_id: String,
    body: PatchAgentScheduleBody,
    option: RequestOption,
) -> SDKResult<PatchAgentScheduleResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<PatchAgentScheduleResponse> =
        ApiRequest::patch(HelpdeskApiV1::AgentSchedulePatch(agent_id).to_url())
            .body(serialize_params(&body, "更新客服工作日程")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新客服工作日程")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchAgentScheduleBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid_times() {
        let body = PatchAgentScheduleBody {
            work_date: Some("2024-01-15".to_string()),
            start_time: Some("09:00:00".to_string()),
            end_time: Some("18:00:00".to_string()),
            day_of_week: Some(1),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_invalid_times() {
        let body = PatchAgentScheduleBody {
            work_date: None,
            start_time: Some("18:00:00".to_string()),
            end_time: Some("09:00:00".to_string()),
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
        let builder =
            PatchAgentScheduleRequestBuilder::new(Arc::new(config), "agent_123".to_string());

        assert_eq!(builder.agent_id, "agent_123");
        assert!(builder.work_date.is_none());
    }
}
