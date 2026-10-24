//! 启用/停用试用期考核功能
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation/enable_disable_assessment>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 启用/停用试用期考核功能请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableDisableAssessmentBody {
    /// 启用 / 停用状态。启用后可在试用期管理页面中可见试用期考核相关的字段
    pub active: bool,
    /// 试用期考核系统入口链接，当启用功能（active=true）时该字段必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
}

/// 启用/停用试用期考核功能请求
#[derive(Debug, Clone)]
pub struct EnableDisableAssessmentRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: EnableDisableAssessmentBody,
}

impl EnableDisableAssessmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: EnableDisableAssessmentBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: EnableDisableAssessmentBody) -> Self {
        self.body = body;
        self
    }

    /// 设置启用 / 停用状态。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = active;
        self
    }

    /// 设置试用期考核系统入口链接（active=true 时必填）。
    pub fn app_url(mut self, app_url: impl Into<String>) -> Self {
        self.body.app_url = Some(app_url.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnableDisableAssessmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EnableDisableAssessmentResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 官方约束：当启用功能（active=true）时 app_url 必填
        if self.body.active
            && self
                .body
                .app_url
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "启用功能时 app_url 不能为空",
            ));
        }

        let api_endpoint = FeishuPeopleApiV2::ProbationEnableDisableAssessment;
        let request = ApiRequest::<EnableDisableAssessmentResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "启用/停用试用期考核功能响应数据为空",
        )
        .await
    }
}

/// 启用/停用试用期考核功能响应（官方 data 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnableDisableAssessmentResponse {}

impl ApiResponseTrait for EnableDisableAssessmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/probation/enable_disable_assessment
    #[tokio::test]
    async fn test_enable_disable_assessment_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/probation/enable_disable_assessment",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        EnableDisableAssessmentRequest::new(config)
            .active(true)
            .app_url("https://example.feishuapp.cn/ae/apps/1")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
