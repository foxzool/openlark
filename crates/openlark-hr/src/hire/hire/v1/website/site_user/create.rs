//! 新建招聘官网用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.site_user/create>

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

use crate::hire::hire::common_models::WebsiteSiteUserSummary;

/// 新建招聘官网用户请求体（`website_user` 创建参数）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequestBody {
    /// 外部 ID（必填），幂等字段：同一外部 ID 只会创建 1 个官网用户。
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 用户姓名。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 用户邮箱。
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 电话（若填写，`mobile_country_code` 必填）。
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 国家码（若填写，`mobile` 必填）。
    pub mobile_country_code: Option<String>,
}

impl CreateRequestBody {
    /// 创建请求体（`external_id` 为官方必填字段）。
    pub fn new(external_id: impl Into<String>) -> Self {
        Self {
            external_id: external_id.into(),
            name: None,
            email: None,
            mobile: None,
            mobile_country_code: None,
        }
    }

    /// 设置用户姓名。
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// 设置用户邮箱。
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// 设置电话（需同时设置 `mobile_country_code`）。
    pub fn mobile(mut self, value: impl Into<String>) -> Self {
        self.mobile = Some(value.into());
        self
    }

    /// 设置国家码（需同时设置 `mobile`）。
    pub fn mobile_country_code(mut self, value: impl Into<String>) -> Self {
        self.mobile_country_code = Some(value.into());
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.external_id, "external_id 不能为空");
        // 官方约束：mobile 与 mobile_country_code 成对必填。
        if self.mobile.is_some() != self.mobile_country_code.is_some() {
            return Err(openlark_core::error::validation_error(
                "电话与国家码必须成对填写",
                "填写 mobile 时必须同时填写 mobile_country_code，反之亦然",
            ));
        }

        Ok(())
    }
}

/// 新建招聘官网用户请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
    request_body: Option<CreateRequestBody>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
            request_body: None,
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 设置请求体（官方必填 `external_id`，见 [`CreateRequestBody::new`]）。
    pub fn request_body(mut self, request_body: CreateRequestBody) -> Self {
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
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");

        let mut request = ApiRequest::<CreateResponse>::post(format!(
            "/open-apis/hire/v1/websites/{website_id}/site_users"
        ));
        if let Some(request_body) = self.request_body {
            request_body.validate()?;
            request = request.body(serde_json::to_value(&request_body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "新建招聘官网用户响应数据为空",
        )
        .await
    }
}

/// 新建招聘官网用户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `site_user` 字段。
    pub site_user: Option<WebsiteSiteUserSummary>,
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

    /// 端到端：POST /open-apis/hire/v1/websites/test001/site_users，请求体带官方必填 `external_id`，响应按官方 `data.site_user` 形状解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/websites/test001/site_users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "site_user": {
                        "user_id": "6960663240925956620",
                        "name": "dan27",
                        "email": "dan27@163.com",
                        "external_id": "6960663240925956621",
                        "mobile": "182900291190",
                        "mobile_country_code": "CN_1"
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
            .website_id("test001".to_string())
            .request_body(
                CreateRequestBody::new("6960663240925956621")
                    .name("dan27")
                    .email("dan27@163.com")
                    .mobile("182900291190")
                    .mobile_country_code("CN_1"),
            )
            .execute()
            .await
            .expect("请求应成功");

        let site_user = resp.site_user.expect("应解析出 site_user");
        assert_eq!(site_user.user_id.as_deref(), Some("6960663240925956620"));
        assert_eq!(site_user.name.as_deref(), Some("dan27"));
        assert_eq!(
            site_user.external_id.as_deref(),
            Some("6960663240925956621")
        );
        assert_eq!(site_user.mobile_country_code.as_deref(), Some("CN_1"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({
                "external_id": "6960663240925956621",
                "name": "dan27",
                "email": "dan27@163.com",
                "mobile": "182900291190",
                "mobile_country_code": "CN_1"
            })
        );
    }

    /// `mobile` 与 `mobile_country_code` 未成对填写时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_create_rejects_unpaired_mobile_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CreateRequest::new(config)
            .website_id("test001".to_string())
            .request_body(CreateRequestBody::new("ext_1").mobile("182900291190"))
            .execute()
            .await;
        assert!(result.is_err());
    }
}
