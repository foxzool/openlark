//! 获取自定义字段列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/custom_field/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::CustomFieldItem;

/// 获取自定义字段列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl QueryRequest {
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
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::CustomFieldQuery;
        let request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取自定义字段列表响应数据为空",
        )
        .await
    }
}

/// 获取自定义字段列表响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 自定义字段列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CustomFieldItem>>,
    /// 分页令牌
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for QueryResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/custom_fields/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/custom_fields/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "custom_api_name": "custom_field_33__c",
                            "name": {"zh_cn": "cn", "en_us": "en"},
                            "description": {"zh_cn": "cn", "en_us": "en"},
                            "is_open": true,
                            "is_required": true,
                            "is_unique": true,
                            "object_api_name": "offboarding_info",
                            "type": 1,
                            "common_schema_config": {
                                "text_field_setting": {
                                    "is_multilingual": true,
                                    "is_multiline": true,
                                    "max_length": 1,
                                    "is_url_type": true
                                }
                            },
                            "create_time": "1625542287",
                            "update_time": "1625542639"
                        }
                    ]
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

        let resp = QueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解析出自定义字段列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].custom_api_name.as_deref(),
            Some("custom_field_33__c")
        );
        assert_eq!(items[0].is_open, Some(true));
        assert_eq!(items[0].r#type, Some(1));
        assert_eq!(
            items[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("cn")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
