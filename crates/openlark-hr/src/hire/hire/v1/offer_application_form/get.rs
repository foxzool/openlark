//! 获取 Offer 申请表信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer_application_form/get>

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

use crate::hire::hire::common_models::I18nText;

/// 获取 Offer 申请表信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    application_form_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_form_id: String::new(),
        }
    }

    /// 设置 `application_form_id`。
    pub fn application_form_id(mut self, application_form_id: String) -> Self {
        self.application_form_id = application_form_id;
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
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_form_id.trim(), "Offer 申请表 ID 不能为空");

        let api_endpoint = HireApiV1::OfferApplicationFormGet(self.application_form_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 Offer 申请表信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 Offer 申请表信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferFormPreObjectConfig`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormPreObjectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operator` 字段。
    pub operator: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 值。
    pub value: Option<Vec<String>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferFormObjectDisplayConfig`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormObjectDisplayConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `display_condition` 字段。
    pub display_condition: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `pre_object_config_list` 字段。
    pub pre_object_config_list: Option<Vec<OfferFormPreObjectConfig>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferFormObjectConfig`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormObjectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `options` 字段。
    pub options: Option<Vec<OfferFormOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `object_display_config` 字段。
    pub object_display_config: Option<OfferFormObjectDisplayConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferFormObject`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `module_id` 字段。
    pub module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_customized` 字段。
    pub is_customized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_required` 字段。
    pub is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `need_approve` 字段。
    pub need_approve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_sensitive` 字段。
    pub is_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 对象类型。
    pub object_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `object_type_v2` 字段。
    pub object_type_v2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `config` 字段。
    pub config: Option<OfferFormObjectConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferFormModule`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferFormModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_customized` 字段。
    pub is_customized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `hint` 字段。
    pub hint: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `object_list` 字段。
    pub object_list: Option<Vec<OfferFormObject>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferApplySchema`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferApplySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `module_list` 字段。
    pub module_list: Option<Vec<OfferFormModule>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferApplyForm`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferApplyForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `schema` 字段。
    pub schema: Option<OfferApplySchema>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(rename = "offer_apply_form", skip_serializing_if = "Option::is_none")]
    /// `offer_apply_form` 字段。
    pub offer_apply_form: Option<OfferApplyForm>,
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

    /// 端到端：GET /open-apis/hire/v1/offer_application_forms/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/offer_application_forms/test001"))
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

        GetRequest::new(config)
            .application_form_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
