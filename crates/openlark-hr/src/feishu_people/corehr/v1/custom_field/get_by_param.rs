//! 获取字段详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/custom_field/get_by_param>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::CustomFieldItem;

/// 获取字段详情请求
#[derive(Debug, Clone)]
pub struct GetByParamRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl GetByParamRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByParamResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByParamResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::CustomFieldGetByParam;
        let request = ApiRequest::<GetByParamResponse>::get(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取字段详情响应数据为空",
        )
        .await
    }
}

/// 获取字段详情响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
/// 官方响应示例与 Go SDK 的内层键为 `data`（字段表记为 `custom_field`），
/// 此处以 `data` 为 wire 键、`custom_field` 作别名，两种键名均兼容。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetByParamResponse {
    /// 字段详情
    #[serde(
        rename = "data",
        alias = "custom_field",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field: Option<CustomFieldItem>,
}

impl ApiResponseTrait for GetByParamResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/custom_fields/get_by_param
    #[tokio::test]
    async fn test_get_by_param_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/custom_fields/get_by_param"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "custom_api_name": "custom_field_33__c",
                        "name": {"zh_cn": "cn", "en_us": "en"},
                        "description": {"zh_cn": "cn", "en_us": "en"},
                        "is_open": true,
                        "is_required": true,
                        "is_unique": true,
                        "object_api_name": "offboarding_info",
                        "type": 4,
                        "common_schema_config": {
                            "enum_field_setting": {
                                "enum_field_option_list": [
                                    {
                                        "api_name": "custom_enum_option_33",
                                        "name": {"zh_cn": "cn", "en_us": "en"},
                                        "description": {"zh_cn": "cn", "en_us": "en"},
                                        "is_open": true
                                    }
                                ],
                                "is_multiple": false
                            }
                        },
                        "create_time": "1625542287",
                        "update_time": "1625542639"
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

        let resp = GetByParamRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let field = resp.custom_field.expect("custom_field 应从 data 键解析");
        assert_eq!(field.custom_api_name.as_deref(), Some("custom_field_33__c"));
        assert_eq!(field.is_unique, Some(true));
        assert_eq!(field.object_api_name.as_deref(), Some("offboarding_info"));
        let enum_setting = field
            .common_schema_config
            .as_ref()
            .and_then(|c| c.enum_field_setting.as_ref())
            .expect("enum_field_setting 应解析");
        assert_eq!(enum_setting.is_multiple, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
