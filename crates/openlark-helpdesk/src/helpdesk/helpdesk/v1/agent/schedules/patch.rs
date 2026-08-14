//! 更新指定客服的工作日程
//!
//! 更新指定客服的工作日程信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 更新客服工作日程请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAgentScheduleBody {
    /// 客服日程。
    pub agent_schedule: AgentSchedule,
}

/// 客服日程。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSchedule {
    /// 每周工作时间段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Vec<WeekdaySchedule>>,
    /// 客服技能 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_skill_ids: Option<Vec<String>>,
}

/// 每周工作时间段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekdaySchedule {
    /// 开始时间，格式为 HH:mm。
    pub start_time: String,
    /// 结束时间，格式为 HH:mm。
    pub end_time: String,
    /// 星期标识。
    pub weekday: i32,
}

impl PatchAgentScheduleBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(schedule) = &self.agent_schedule.schedule {
            for item in schedule {
                validate_required!(item.start_time, "start_time 不能为空");
                validate_required!(item.end_time, "end_time 不能为空");
            }
        }
        Ok(())
    }
}

/// 更新客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentScheduleResponse {
    /// 客服ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// 客服日程。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_schedule: Option<AgentSchedule>,
}

impl openlark_core::api::ApiResponseTrait for PatchAgentScheduleResponse {}

/// 更新客服工作日程结果。
pub type PatchAgentScheduleResult = PatchAgentScheduleResponse;

/// 更新客服工作日程请求
#[derive(Debug, Clone)]
pub struct PatchAgentScheduleRequest {
    config: Config,
    agent_id: String,
}

impl PatchAgentScheduleRequest {
    /// 创建新的更新客服工作日程请求
    pub fn new(config: Config, agent_id: String) -> Self {
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
        body.validate()?;

        let req: ApiRequest<PatchAgentScheduleResponse> =
            ApiRequest::patch(HelpdeskApiV1::AgentSchedulePatch(self.agent_id.clone()).to_url())
                .body(serialize_params(&body, "更新客服工作日程")?);

        Transport::request_typed(req, &self.config, Some(option), "更新客服工作日程").await
    }
}

/// 更新客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct PatchAgentScheduleRequestBuilder {
    config: Config,
    agent_id: String,
    schedule: Option<Vec<WeekdaySchedule>>,
    agent_skill_ids: Option<Vec<String>>,
}

impl PatchAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config, agent_id: String) -> Self {
        Self {
            config,
            agent_id,
            schedule: None,
            agent_skill_ids: None,
        }
    }

    /// 设置每周工作时间段。
    pub fn schedule(mut self, schedule: Vec<WeekdaySchedule>) -> Self {
        self.schedule = Some(schedule);
        self
    }

    /// 设置客服技能 ID 列表。
    pub fn agent_skill_ids(mut self, agent_skill_ids: Vec<String>) -> Self {
        self.agent_skill_ids = Some(agent_skill_ids);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchAgentScheduleBody {
        PatchAgentScheduleBody {
            agent_schedule: AgentSchedule {
                schedule: self.schedule.clone(),
                agent_skill_ids: self.agent_skill_ids.clone(),
            },
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
    body.validate()?;

    let req: ApiRequest<PatchAgentScheduleResponse> =
        ApiRequest::patch(HelpdeskApiV1::AgentSchedulePatch(agent_id).to_url())
            .body(serialize_params(&body, "更新客服工作日程")?);

    Transport::request_typed(req, config, Some(option), "更新客服工作日程").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_body_validation_empty_schedule() {
        let body = PatchAgentScheduleBody::default();
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = PatchAgentScheduleBody {
            agent_schedule: AgentSchedule {
                schedule: Some(vec![WeekdaySchedule {
                    start_time: "00:00".to_string(),
                    end_time: "24:00".to_string(),
                    weekday: 9,
                }]),
                agent_skill_ids: Some(vec!["test-skill-id".to_string()]),
            },
        };
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "agent_schedule": {
                    "schedule": [{
                        "start_time": "00:00",
                        "end_time": "24:00",
                        "weekday": 9
                    }],
                    "agent_skill_ids": ["test-skill-id"]
                }
            })
        );
    }

    #[test]
    fn test_body_validation_rejects_empty_start_time() {
        let body = PatchAgentScheduleBody {
            agent_schedule: AgentSchedule {
                schedule: Some(vec![WeekdaySchedule {
                    start_time: " ".to_string(),
                    end_time: "24:00".to_string(),
                    weekday: 9,
                }]),
                agent_skill_ids: None,
            },
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchAgentScheduleRequestBuilder::new(config, "agent_123".to_string());

        assert_eq!(builder.agent_id, "agent_123");
        assert!(builder.schedule.is_none());
    }

    /// 端到端：PATCH .../agents/{agent_id}/schedules → 强类型响应解析。
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/agents/ag_001/schedules"))
            .and(body_json(json!({
                "agent_schedule": {
                    "schedule": [{
                        "start_time": "00:00",
                        "end_time": "24:00",
                        "weekday": 9
                    }],
                    "agent_skill_ids": ["test-skill-id"]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "agent_id": "ag_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = PatchAgentScheduleBody {
            agent_schedule: AgentSchedule {
                schedule: Some(vec![WeekdaySchedule {
                    start_time: "00:00".to_string(),
                    end_time: "24:00".to_string(),
                    weekday: 9,
                }]),
                agent_skill_ids: Some(vec!["test-skill-id".to_string()]),
            },
        };
        let resp = PatchAgentScheduleRequest::new(config, "ag_001".to_string())
            .execute(body)
            .await
            .expect("更新客服工作日程应成功");
        assert_eq!(resp.agent_id.as_deref(), Some("ag_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agents/ag_001/schedules"
        );
    }
}
