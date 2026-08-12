//! 创建客服工作日程
//!
//! 为指定客服创建工作日程。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent-schedules/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 创建客服工作日程请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentScheduleBody {
    /// 待创建的客服日程列表。
    pub agent_schedules: Vec<AgentSchedule>,
}

/// 客服日程。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSchedule {
    /// 客服 ID。
    pub agent_id: String,
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

impl CreateAgentScheduleBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.agent_schedules, "agent_schedules 不能为空");
        for agent_schedule in &self.agent_schedules {
            validate_required!(agent_schedule.agent_id, "agent_id 不能为空");
            if let Some(schedule) = &agent_schedule.schedule {
                for item in schedule {
                    validate_required!(item.start_time, "start_time 不能为空");
                    validate_required!(item.end_time, "end_time 不能为空");
                }
            }
        }
        Ok(())
    }
}

/// 创建客服工作日程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentScheduleResponse {
    /// 已创建的客服日程列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_schedules: Option<Vec<AgentSchedule>>,
}

impl openlark_core::api::ApiResponseTrait for CreateAgentScheduleResponse {}

/// 创建客服工作日程结果。
pub type CreateAgentScheduleResult = CreateAgentScheduleResponse;

/// 创建客服工作日程请求
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleRequest {
    config: Config,
}

impl CreateAgentScheduleRequest {
    /// 创建新的创建客服工作日程请求
    pub fn new(config: Config) -> Self {
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

        Transport::request_typed(req, &self.config, Some(option), "创建客服工作日程").await
    }
}

/// 创建客服工作日程请求构建器
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleRequestBuilder {
    config: Config,
    agent_schedules: Vec<AgentSchedule>,
}

impl CreateAgentScheduleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            agent_schedules: Vec::new(),
        }
    }

    /// 设置待创建的客服日程列表。
    pub fn agent_schedules(mut self, agent_schedules: Vec<AgentSchedule>) -> Self {
        self.agent_schedules = agent_schedules;
        self
    }

    /// 添加一个客服日程。
    pub fn agent_schedule(mut self, agent_schedule: AgentSchedule) -> Self {
        self.agent_schedules.push(agent_schedule);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> CreateAgentScheduleBody {
        CreateAgentScheduleBody {
            agent_schedules: self.agent_schedules.clone(),
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateAgentScheduleResponse> {
        let body = self.body();
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

    Transport::request_typed(req, config, Some(option), "创建客服工作日程").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = CreateAgentScheduleBody {
            agent_schedules: vec![AgentSchedule {
                agent_id: "agent_123".to_string(),
                schedule: Some(vec![WeekdaySchedule {
                    start_time: "00:00".to_string(),
                    end_time: "24:00".to_string(),
                    weekday: 9,
                }]),
                agent_skill_ids: Some(vec!["test-skill-id".to_string()]),
            }],
        };
        assert!(body.validate().is_ok());
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "agent_schedules": [{
                    "agent_id": "agent_123",
                    "schedule": [{
                        "start_time": "00:00",
                        "end_time": "24:00",
                        "weekday": 9
                    }],
                    "agent_skill_ids": ["test-skill-id"]
                }]
            })
        );
    }

    #[test]
    fn test_body_validation_rejects_empty_agent_id() {
        let body = CreateAgentScheduleBody {
            agent_schedules: vec![AgentSchedule::default()],
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_rejects_empty_list() {
        assert!(CreateAgentScheduleBody::default().validate().is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateAgentScheduleRequestBuilder::new(config);

        assert!(builder.agent_schedules.is_empty());
    }

    /// 端到端：POST .../agent_schedules → 强类型响应解析。
    #[tokio::test]
    async fn test_create_agent_schedule_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/agent_schedules"))
            .and(body_json(json!({
                "agent_schedules": [{
                    "agent_id": "ag_001",
                    "schedule": [{
                        "start_time": "00:00",
                        "end_time": "24:00",
                        "weekday": 9
                    }],
                    "agent_skill_ids": ["test-skill-id"]
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "agent_schedules": [{ "agent_id": "ag_001" }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateAgentScheduleBody {
            agent_schedules: vec![AgentSchedule {
                agent_id: "ag_001".to_string(),
                schedule: Some(vec![WeekdaySchedule {
                    start_time: "00:00".to_string(),
                    end_time: "24:00".to_string(),
                    weekday: 9,
                }]),
                agent_skill_ids: Some(vec!["test-skill-id".to_string()]),
            }],
        };
        let resp = CreateAgentScheduleRequest::new(config)
            .execute(body)
            .await
            .expect("创建客服工作日程应成功");
        assert_eq!(resp.agent_schedules.as_ref().map(Vec::len), Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_schedules"
        );
    }
}
