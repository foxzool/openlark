//! 删除试用期考核信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation.assessment/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除试用期考核信息请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    assessment_id: Option<String>,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            assessment_id: None,
        }
    }

    /// 设置 `assessment_id`。
    pub fn assessment_id(mut self, assessment_id: String) -> Self {
        self.assessment_id = Some(assessment_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let assessment_id = self.assessment_id.unwrap_or_default();
        validate_required!(assessment_id.trim(), "assessment_id 不能为空");

        let api_endpoint = FeishuPeopleApiV2::ProbationAssessmentDelete(assessment_id);
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除试用期考核信息响应数据为空",
        )
        .await
    }
}

/// 删除试用期考核信息响应（官方 data 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteResponse {}

impl ApiResponseTrait for DeleteResponse {
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

    /// 端到端：DELETE /open-apis/corehr/v2/probation/assessments/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v2/probation/assessments/test001"))
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

        DeleteRequest::new(config)
            .assessment_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
