//! 查询外部面试列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_interview/batch_query>

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

/// 查询外部面试列表请求
#[derive(Debug, Clone)]
pub struct BatchQueryRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl BatchQueryRequest {
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
    pub async fn execute(self) -> SDKResult<BatchQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchQueryResponse> {
        let mut request = ApiRequest::<BatchQueryResponse>::post(
            "/open-apis/hire/v1/external_interviews/batch_query",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询外部面试列表响应数据为空",
        )
        .await
    }
}

/// 查询外部面试列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchQueryResponse {
    #[serde(default, alias = "records")]
    /// 结果项列表。
    pub items: Vec<ExternalInterview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for BatchQueryResponse {
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

    /// 端到端：POST /open-apis/hire/v1/external_interviews/batch_query
    #[tokio::test]
    async fn test_batch_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/external_interviews/batch_query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "items": [{
                        "external_application_id": "6960663240925956437",
                        "id": "6960663240925956436",
                        "participate_status": 1,
                        "begin_time": 1618500278638i64,
                        "end_time": 1618500278639i64,
                        "interview_assessments": [{
                            "id": "6989181065243969836",
                            "username": "张三",
                            "conclusion": 1,
                            "assessment_dimension_list": [{
                                "score": 99,
                                "option": "选项1",
                                "options": ["选项1"],
                                "content": "这是一个测试内容",
                                "assessment_type": 1,
                                "title": "心理测试",
                                "description": "心理测试题目"
                            }],
                            "content": "这是一个心理测试题目"
                        }]
                    }],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more": true
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

        let resp = BatchQueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        // 官方 item 含 interview_assessments 嵌套（.md L54-72）
        let item = resp.items.first().expect("items 应反序列化");
        assert_eq!(item.id.as_deref(), Some("6960663240925956436"));
        assert_eq!(item.participate_status, Some(1));
        assert_eq!(item.begin_time, Some(1618500278638));
        let assessment = item
            .interview_assessments
            .as_ref()
            .and_then(|v| v.first())
            .expect("interview_assessments 应反序列化");
        assert_eq!(assessment.conclusion, Some(1));
        let dimension = assessment
            .assessment_dimension_list
            .as_ref()
            .and_then(|v| v.first())
            .expect("assessment_dimension_list 应反序列化");
        assert_eq!(dimension.score, Some(99));
        assert_eq!(dimension.assessment_type, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
