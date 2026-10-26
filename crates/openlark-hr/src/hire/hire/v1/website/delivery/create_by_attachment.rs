//! 根据简历附件创建招聘官网投递任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.delivery/create_by_attachment>

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

/// 根据简历附件创建招聘官网投递任务请求体（官方必填 `job_post_id`/`user_id`/`resume_file_id`）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CreateByAttachmentBody {
    /// 职位广告 ID（必填）
    pub job_post_id: String,
    /// 官网用户 ID（必填，需在新建招聘官网用户时获取并保存）
    pub user_id: String,
    /// 简历文件 ID（必填，通过创建附件生成）
    pub resume_file_id: String,
    /// 官网推广渠道 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// 意向投递城市列表（最多一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_city_code_list: Option<Vec<String>>,
    /// 电话国际区号（与简历附件不一致时以该参数为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_country_code: Option<String>,
    /// 电话号码（与简历附件不一致时以该参数为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 邮箱（与简历附件不一致时以该参数为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 证件（与简历附件不一致时以该参数为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<WebsiteDeliveryAttachmentIdentification>,
}

/// 简历附件创建投递的证件信息（官方 `website_delivery_attachment_indentification`）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WebsiteDeliveryAttachmentIdentification {
    /// 证件类型：1 中国-居民身份证 / 2 护照 / 3 港澳居民居住证 / 4 台湾居民来往大陆通行证 / 5 其他 / 6 港澳居民来往内地通行证 / 9 台湾居民居住证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
    /// 证件号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
}

/// 根据简历附件创建招聘官网投递任务请求
#[derive(Debug, Clone)]
pub struct CreateByAttachmentRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
    body: CreateByAttachmentBody,
}

impl CreateByAttachmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
            body: CreateByAttachmentBody::default(),
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 设置职位广告 ID（必填，官方 `job_post_id`）。
    pub fn job_post_id(mut self, job_post_id: impl Into<String>) -> Self {
        self.body.job_post_id = job_post_id.into();
        self
    }

    /// 设置官网用户 ID（必填，官方 `user_id`）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置简历文件 ID（必填，官方 `resume_file_id`，通过创建附件生成）。
    pub fn resume_file_id(mut self, resume_file_id: impl Into<String>) -> Self {
        self.body.resume_file_id = resume_file_id.into();
        self
    }

    /// 设置官网推广渠道 ID。
    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.body.channel_id = Some(channel_id.into());
        self
    }

    /// 设置意向投递城市列表（最多一个）。
    pub fn application_preferred_city_code_list(mut self, value: Vec<String>) -> Self {
        self.body.application_preferred_city_code_list = Some(value);
        self
    }

    /// 设置电话国际区号。
    pub fn mobile_country_code(mut self, value: impl Into<String>) -> Self {
        self.body.mobile_country_code = Some(value.into());
        self
    }

    /// 设置电话号码。
    pub fn mobile(mut self, value: impl Into<String>) -> Self {
        self.body.mobile = Some(value.into());
        self
    }

    /// 设置邮箱。
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    /// 设置证件信息。
    pub fn identification(mut self, value: WebsiteDeliveryAttachmentIdentification) -> Self {
        self.body.identification = Some(value);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateByAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateByAttachmentResponse> {
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");
        validate_required!(self.body.job_post_id.trim(), "job_post_id 不能为空");
        validate_required!(self.body.user_id.trim(), "user_id 不能为空");
        validate_required!(self.body.resume_file_id.trim(), "resume_file_id 不能为空");

        let request = ApiRequest::<CreateByAttachmentResponse>::post(format!(
            "/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_attachment"
        ))
        .body(serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "根据简历附件创建招聘官网投递任务响应数据为空",
        )
        .await
    }
}

/// 根据简历附件创建招聘官网投递任务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateByAttachmentResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 异步任务 ID，可根据此 ID 通过获取招聘官网投递任务结果接口查询最终的投递结果。
    pub task_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateByAttachmentResponse {
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

    /// 端到端：POST /open-apis/hire/v1/websites/test001/deliveries/create_by_attachment
    #[tokio::test]
    async fn test_create_by_attachment_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/websites/test001/deliveries/create_by_attachment",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": { "task_id": "f1c2a0f138ec492d99d7ab73594158ad" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateByAttachmentRequest::new(config)
            .website_id("test001".to_string())
            .job_post_id("6960663240925956636".to_string())
            .user_id("6960663240925956634".to_string())
            .resume_file_id("7095614894070434092".to_string())
            .channel_id("6891560630172518670".to_string())
            .application_preferred_city_code_list(vec!["CT_100121".to_string()])
            .mobile_country_code("+86".to_string())
            .mobile("182900291190".to_string())
            .email("foo@bytedance.com".to_string())
            .identification(WebsiteDeliveryAttachmentIdentification {
                identification_type: Some(1),
                identification_number: Some("11002030131312312".to_string()),
            })
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(
            resp.task_id.as_deref(),
            Some("f1c2a0f138ec492d99d7ab73594158ad")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档示例一致：三个必填字段 + 可选覆盖字段
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({
                "job_post_id": "6960663240925956636",
                "user_id": "6960663240925956634",
                "resume_file_id": "7095614894070434092",
                "channel_id": "6891560630172518670",
                "application_preferred_city_code_list": ["CT_100121"],
                "mobile_country_code": "+86",
                "mobile": "182900291190",
                "email": "foo@bytedance.com",
                "identification": {
                    "identification_type": 1,
                    "identification_number": "11002030131312312"
                }
            })
        );
    }

    /// 缺必填 body 字段时快速失败，不发起请求。
    #[tokio::test]
    async fn test_create_by_attachment_rejects_missing_required_body_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CreateByAttachmentRequest::new(config)
            .website_id("test001".to_string())
            .execute()
            .await
            .expect_err("缺 job_post_id 应被拒绝");
        assert!(err.to_string().contains("job_post_id"), "实际: {err}");
    }
}
