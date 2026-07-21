//! 获取招聘官网投递任务结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.delivery_task/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::WebsiteDeliveryTaskResult;

/// 获取招聘官网投递任务结果请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
    delivery_task_id: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
            delivery_task_id: None,
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 设置 `delivery_task_id`。
    pub fn delivery_task_id(mut self, delivery_task_id: impl Into<String>) -> Self {
        self.delivery_task_id = Some(delivery_task_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let website_id = self.website_id.unwrap_or_default();
        let delivery_task_id = self.delivery_task_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");
        validate_required!(delivery_task_id.trim(), "delivery_task_id 不能为空");

        let request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/hire/v1/websites/{website_id}/delivery_tasks/{delivery_task_id}"
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取招聘官网投递任务结果响应数据为空",
        )
        .await
    }
}

/// 获取招聘官网投递任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(flatten)]
    /// `delivery_task` 字段。
    pub delivery_task: WebsiteDeliveryTaskResult,
}

impl ApiResponseTrait for GetResponse {
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

    /// 端到端：GET /open-apis/hire/v1/websites/test001/delivery_tasks/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/hire/v1/websites/test001/delivery_tasks/test001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "delivery_task": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetRequest::new(config)
            .website_id("test001".to_string())
            .delivery_task_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
