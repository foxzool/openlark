//! 新建招聘官网投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.delivery/create_by_resume>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::WebsiteDeliveryDto;

/// 新建招聘官网投递请求
#[derive(Debug, Clone)]
pub struct CreateByResumeRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
}

impl CreateByResumeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateByResumeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateByResumeResponse> {
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");

        let request = ApiRequest::<CreateByResumeResponse>::post(format!(
            "/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_resume"
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "新建招聘官网投递响应数据为空",
        )
        .await
    }
}

/// 新建招聘官网投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateByResumeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递信息。
    pub delivery: Option<WebsiteDeliveryDto>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateByResumeResponse {
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

    /// 端到端：POST /open-apis/hire/v1/websites/test001/deliveries/create_by_resume，响应按官方 `data.delivery` 形状解析。
    #[tokio::test]
    async fn test_create_by_resume_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/websites/test001/deliveries/create_by_resume",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "delivery": {
                        "application_id": "6960663240925956657",
                        "id": "6960663240925956655",
                        "job_id": "6960663240925956659",
                        "job_post_id": "6960663240925956658",
                        "portal_resume_id": "6960663240925956660",
                        "user_id": "6960663240925956656",
                        "talent_id": "7095600054216542508"
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

        let resp = CreateByResumeRequest::new(config)
            .website_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let delivery = resp.delivery.expect("应解析出 delivery");
        assert_eq!(
            delivery.application_id.as_deref(),
            Some("6960663240925956657")
        );
        assert_eq!(delivery.id.as_deref(), Some("6960663240925956655"));
        assert_eq!(delivery.job_id.as_deref(), Some("6960663240925956659"));
        assert_eq!(delivery.job_post_id.as_deref(), Some("6960663240925956658"));
        assert_eq!(
            delivery.portal_resume_id.as_deref(),
            Some("6960663240925956660")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
