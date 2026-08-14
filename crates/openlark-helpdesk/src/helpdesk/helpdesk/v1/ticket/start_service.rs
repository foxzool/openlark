//! 创建服务台对话
//!
//! 创建一个新的服务台对话（工单）。
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/start_service>

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建服务台对话请求
#[derive(Debug, Clone)]
pub struct StartServiceRequest {
    config: Config,
    body: StartServiceBody,
}

/// 创建服务台对话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartServiceBody {
    /// 是否直接转人工服务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_service: Option<bool>,
    /// 指定接待客服的 open_id 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_agents: Option<Vec<String>>,
    /// 用户 open_id
    pub open_id: String,
    /// 自定义信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_info: Option<String>,
}

impl StartServiceBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.open_id, "open_id 不能为空");
        Ok(())
    }
}

/// 创建服务台对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceResponse {
    /// 工单 ID
    pub ticket_id: String,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl StartServiceRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: StartServiceBody::default(),
        }
    }

    /// 设置是否直接转人工服务。
    pub fn human_service(mut self, human_service: bool) -> Self {
        self.body.human_service = Some(human_service);
        self
    }

    /// 设置指定接待客服的 open_id 列表。
    pub fn appointed_agents(mut self, appointed_agents: Vec<String>) -> Self {
        self.body.appointed_agents = Some(appointed_agents);
        self
    }

    /// 设置用户 open_id。
    pub fn open_id(mut self, open_id: impl Into<String>) -> Self {
        self.body.open_id = open_id.into();
        self
    }

    /// 设置自定义信息。
    pub fn customized_info(mut self, customized_info: impl Into<String>) -> Self {
        self.body.customized_info = Some(customized_info.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<StartServiceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StartServiceResponse> {
        self.body.validate()?;

        let req: ApiRequest<StartServiceResponse> =
            ApiRequest::post(HelpdeskApiV1::TicketStartService.to_url())
                .body(serialize_params(&self.body, "创建服务台对话")?);

        Transport::request_typed(req, &self.config, Some(option), "创建服务台对话").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_requires_open_id() {
        let body = StartServiceBody::default();
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serialization_matches_official_shape() {
        let body = StartServiceBody {
            human_service: Some(false),
            appointed_agents: Some(vec!["ou_agent".to_string()]),
            open_id: "ou_user".to_string(),
            customized_info: Some("自定义信息".to_string()),
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["open_id"], "ou_user");
        assert_eq!(value["appointed_agents"][0], "ou_agent");
        assert!(value.get("question").is_none());
        assert!(value.get("service_id").is_none());
        assert!(value.get("user_id").is_none());
    }

    /// 端到端：POST .../start_service → 强类型响应解析。
    #[tokio::test]
    async fn test_start_service_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/start_service"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "ticket_id": "tk_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = StartServiceRequest::new(config)
            .human_service(false)
            .appointed_agents(vec!["ou_agent".to_string()])
            .open_id("ou_001")
            .customized_info("自定义信息")
            .execute()
            .await
            .expect("创建服务台对话应成功");
        assert_eq!(resp.ticket_id, "tk_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/start_service"
        );
    }
}
