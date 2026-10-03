//! 流转入职任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pre_hire/transit_task>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `TransitTaskRequest` 请求。
#[derive(Debug, Clone)]
pub struct TransitTaskRequest {
    config: Config,
    pre_hire_id: Option<String>,
    request_body: Option<Value>,
}

impl TransitTaskRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pre_hire_id: None,
            request_body: None,
        }
    }

    /// 设置 `pre_hire_id`。
    pub fn pre_hire_id(mut self, pre_hire_id: impl Into<String>) -> Self {
        self.pre_hire_id = Some(pre_hire_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TransitTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransitTaskResponse> {
        let pre_hire_id = self.pre_hire_id.unwrap_or_default();
        validate_required!(pre_hire_id.trim(), "pre_hire_id 不能为空");
        let mut request = ApiRequest::<TransitTaskResponse>::post(format!(
            "/open-apis/corehr/v2/pre_hires/{pre_hire_id}/transit_task"
        ));

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "流转入职任务响应数据为空",
        )
        .await
    }
}

/// `TransitTaskResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransitTaskResponse {
    /// 原始响应数据。
    pub data: Value,
}

impl ApiResponseTrait for TransitTaskResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/pre_hires/test001/transit_task
    #[tokio::test]
    async fn test_transit_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/pre_hires/test001/transit_task"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        TransitTaskRequest::new(config)
            .pre_hire_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
