//! 创建外部背调
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_background_check/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalBackgroundCheck;

/// 创建外部背调请求
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
            ApiRequest::<CreateResponse>::post("/open-apis/hire/v1/external_background_checks");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建外部背调响应数据为空",
        )
        .await
    }
}

/// 创建外部背调响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部背调信息。
    pub external_background_check: Option<ExternalBackgroundCheck>,
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

    /// 端到端：POST /open-apis/hire/v1/external_background_checks，响应按官方 `data.external_background_check` 形状解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/external_background_checks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "external_background_check": {
                        "id": "6989202908470446380",
                        "external_application_id": "6989202908470446380",
                        "date": 1626602069393i64,
                        "name": "张三的背调",
                        "result": "已通过",
                        "attachment_list": [
                            {
                                "id": "6987954043925432620",
                                "name": "test_resume.pdf",
                                "size": 126371
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

        let check = resp
            .external_background_check
            .expect("应解析出 external_background_check");
        assert_eq!(check.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(check.name.as_deref(), Some("张三的背调"));
        assert_eq!(check.result.as_deref(), Some("已通过"));
        assert_eq!(check.date, Some(1626602069393));
        let attachment = check
            .attachment_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应解析出 attachment_list");
        assert_eq!(attachment.name.as_deref(), Some("test_resume.pdf"));
        assert_eq!(attachment.size, Some(126371));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
