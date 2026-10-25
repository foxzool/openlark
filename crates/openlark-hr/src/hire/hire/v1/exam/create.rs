//! 添加笔试结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/exam/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 添加笔试结果请求
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
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::ExamCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "添加笔试结果响应数据为空",
        )
        .await
    }
}

/// 添加笔试结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试 ID。
    pub exam_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试卷名称。
    pub exam_resource_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试分数。
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件 ID。
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 操作人用户 open_id。
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 操作时间戳（单位：毫秒）。
    pub operate_time: Option<String>,
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

    /// 端到端：POST /open-apis/hire/v1/exams
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/exams"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "exam_id": "7031822096580868396",
                    "application_id": "7031822096580868396",
                    "exam_resource_name": "测试名字",
                    "score": 95.5,
                    "uuid": "6949805467799537964",
                    "operator_id": "ou_xxx",
                    "operate_time": "1637295516451"
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
        assert_eq!(resp.exam_id.as_deref(), Some("7031822096580868396"));
        assert_eq!(resp.exam_resource_name.as_deref(), Some("测试名字"));
        assert_eq!(resp.score, Some(95.5));
        assert_eq!(resp.uuid.as_deref(), Some("6949805467799537964"));
        assert_eq!(resp.operator_id.as_deref(), Some("ou_xxx"));
        assert_eq!(resp.operate_time.as_deref(), Some("1637295516451"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
