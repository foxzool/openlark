//! 创建外部投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_application/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalApplication;

/// 创建外部投递请求
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
            ApiRequest::<CreateResponse>::post("/open-apis/hire/v1/external_applications");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建外部投递响应数据为空",
        )
        .await
    }
}

/// 创建外部投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递信息。
    pub external_application: Option<ExternalApplication>,
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

    /// 端到端：POST /open-apis/hire/v1/external_applications，响应按官方 `data.external_application` 形状解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/external_applications"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "external_application": {
                        "id": "6989202908470446380",
                        "job_recruitment_type": 1,
                        "job_title": "高级Java",
                        "resume_source": "内推",
                        "stage": "简历初筛阶段",
                        "talent_id": "6960663240925956459",
                        "termination_reason": "不合适",
                        "delivery_type": 1,
                        "modify_time": 1618500278645i64,
                        "create_time": 1618500278644i64,
                        "termination_type": "HR 主动终止"
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
            .request_body(json!({ "talent_id": "6960663240925956459" }))
            .execute()
            .await
            .expect("请求应成功");

        let application = resp
            .external_application
            .expect("应解析出 external_application");
        assert_eq!(application.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(application.job_title.as_deref(), Some("高级Java"));
        assert_eq!(application.resume_source.as_deref(), Some("内推"));
        assert_eq!(application.stage.as_deref(), Some("简历初筛阶段"));
        assert_eq!(application.delivery_type, Some(1));
        assert_eq!(application.create_time, Some(1618500278644));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
