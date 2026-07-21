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

use crate::hire::hire::common_models::WebsiteDeliveryTaskResult;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `delivery_task` 字段。
    pub delivery_task: Option<WebsiteDeliveryTaskResult>,
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

    /// 端到端：POST /open-apis/hire/v1/websites/test001/deliveries/create_by_resume
    #[tokio::test]
    async fn test_create_by_resume_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/websites/test001/deliveries/create_by_resume",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        CreateByResumeRequest::new(config)
            .website_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
