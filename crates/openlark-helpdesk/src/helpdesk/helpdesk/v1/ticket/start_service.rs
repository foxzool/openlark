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
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建服务台对话请求
#[derive(Debug, Clone)]
pub struct StartServiceRequest {
    config: Arc<Config>,
    body: StartServiceBody,
}

/// 创建服务台对话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartServiceBody {
    /// 用户 ID
    pub user_id: String,
    /// 服务台 ID
    pub service_id: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
}

impl StartServiceBody {
    fn validate(&self) -> SDKResult<()> {
        if self.user_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error("用户ID不能为空", ""));
        }
        if self.service_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "服务台ID不能为空",
                "",
            ));
        }
        Ok(())
    }
}

/// 创建服务台对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceResponse {
    /// 响应数据。
    pub data: Option<StartServiceData>,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建服务台对话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceData {
    /// 工单 ID
    pub ticket_id: String,
}

impl StartServiceRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: StartServiceBody::default(),
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置服务 ID。
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.body.service_id = service_id.into();
        self
    }

    /// 设置问题内容。
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.body.question = Some(question.into());
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建服务台对话", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../start_service → 强类型 StartServiceResponse 解析（双层 data 信封）。
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
                "data": { "data": { "ticket_id": "tk_001" } }
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

        let resp = StartServiceRequest::new(config)
            .user_id("ou_001")
            .service_id("svc_001")
            .execute()
            .await
            .expect("创建服务台对话应成功");
        assert!(resp.data.is_some());
        assert_eq!(resp.data.unwrap().ticket_id, "tk_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/start_service"
        );
    }
}
