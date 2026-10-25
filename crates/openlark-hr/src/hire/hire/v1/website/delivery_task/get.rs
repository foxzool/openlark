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
use serde_json::Value;
use std::collections::HashMap;

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

/// 官网投递信息（官方 `website_delivery_dto`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// ID（官方标注为废弃字段，请勿使用）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位广告 ID。
    pub job_post_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网简历 ID。
    pub portal_resume_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网用户 ID。
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取招聘官网投递任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 任务状态：0 新建 / 1 处理中 / 2 成功 / 3 失败。
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递信息。
    pub delivery: Option<WebsiteDeliveryDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 状态信息，仅在任务状态为失败（3）时返回。
    pub status_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附加信息，失败且重复投递时返回重复投递的 ID。
    pub extra_info: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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
                "code": 0, "msg": "ok", "data": {
                    "status": 2,
                    "delivery": {
                        "application_id": "6960663240925956657",
                        "id": "6960663240925956655",
                        "job_id": "6960663240925956659",
                        "job_post_id": "6960663240925956658",
                        "portal_resume_id": "6960663240925956660",
                        "user_id": "6960663240925956656",
                        "talent_id": "7095600054216542508"
                    },
                    "status_msg": " ",
                    "extra_info": "  "
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

        let resp = GetRequest::new(config)
            .website_id("test001".to_string())
            .delivery_task_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.status, Some(2));
        let delivery = resp.delivery.as_ref().expect("delivery 应存在");
        assert_eq!(
            delivery.application_id.as_deref(),
            Some("6960663240925956657")
        );
        assert_eq!(
            delivery.portal_resume_id.as_deref(),
            Some("6960663240925956660")
        );
        assert_eq!(delivery.talent_id.as_deref(), Some("7095600054216542508"));
        assert_eq!(resp.status_msg.as_deref(), Some(" "));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
