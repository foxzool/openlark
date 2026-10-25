//! 创建外部面试
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_interview/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalInterview;

/// 创建外部面试请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        let mut request =
            ApiRequest::<CreateResponse>::post("/open-apis/hire/v1/external_interviews");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建外部面试响应数据为空",
        )
        .await
    }
}

/// 创建外部面试响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部面试信息。
    pub external_interview: Option<ExternalInterview>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/external_interviews，响应按官方 `data.external_interview` 形状解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/external_interviews"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "external_interview": {
                        "external_application_id": "6960663240925956437",
                        "id": "6960663240925956436",
                        "participate_status": 1,
                        "begin_time": 1618500278638i64,
                        "end_time": 1618500278639i64,
                        "interview_assessments": [
                            {
                                "id": "6989181065243969836",
                                "username": "张三",
                                "conclusion": 1,
                                "assessment_dimension_list": [
                                    {
                                        "score": 99,
                                        "option": "选项1",
                                        "options": ["选项1"],
                                        "content": "第一次面试记录",
                                        "assessment_type": 1,
                                        "title": "性格测试",
                                        "description": "这是性格测试相关的"
                                    }
                                ],
                                "content": "内容输出"
                            }
                        ]
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let interview = resp
            .external_interview
            .expect("应解析出 external_interview");
        assert_eq!(interview.id.as_deref(), Some("6960663240925956436"));
        assert_eq!(interview.participate_status, Some(1));
        assert_eq!(interview.begin_time, Some(1618500278638));
        let assessment = interview
            .interview_assessments
            .as_ref()
            .and_then(|l| l.first())
            .expect("应解析出 interview_assessments");
        assert_eq!(assessment.username.as_deref(), Some("张三"));
        assert_eq!(assessment.conclusion, Some(1));
        let dimension = assessment
            .assessment_dimension_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应解析出 assessment_dimension_list");
        assert_eq!(dimension.score, Some(99));
        assert_eq!(dimension.assessment_type, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
