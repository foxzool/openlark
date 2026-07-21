//! 获取 Offer 申请表详细信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer_schema/get>

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

/// 获取 Offer 申请表详细信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    schema_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            schema_id: String::new(),
        }
    }

    /// 设置 `schema_id`。
    pub fn schema_id(mut self, schema_id: String) -> Self {
        self.schema_id = schema_id;
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

        validate_required!(self.schema_id.trim(), "Offer 模板 ID 不能为空");

        let api_endpoint = HireApiV1::OfferSchemaGet(self.schema_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 Offer 申请表详细信息响应数据为空",
        )
        .await
    }
}

/// 获取 Offer 申请表详细信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSchemaOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `index` 字段。
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferSchemaObject`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSchemaObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    /// 对象类型。
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_customized` 字段。
    pub is_customized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 选项列表。
    pub option_list: Option<Vec<OfferSchemaOption>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `scenario` 字段。
    pub scenario: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `version` 字段。
    pub version: Option<i32>,
    #[serde(default)]
    /// `object_list` 字段。
    pub object_list: Vec<OfferSchemaObject>,
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

    /// 端到端：GET /open-apis/hire/v1/offer_schemas/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/offer_schemas/test001"))
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
            .schema_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
