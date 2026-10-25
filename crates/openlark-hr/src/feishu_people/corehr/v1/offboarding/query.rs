//! 查询员工离职原因列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询员工离职原因列表请求
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

        let api_endpoint = FeishuPeopleApiV1::OffboardingQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询员工离职原因列表响应数据为空",
        )
        .await
    }
}

/// 查询员工离职原因列表响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 离职原因列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OffboardingItem>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 离职原因条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OffboardingItem {
    /// 离职原因唯一标识，可作为操作员工离职、搜索离职信息等接口的入参
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    /// 名称（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 当前离职原因的父级原因唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_offboarding_reason_unique_identifier: Option<String>,
    /// 创建时间，格式："YYYY-MM-DD HH-mm-ss"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间，格式："YYYY-MM-DD HH-mm-ss"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
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

    /// 端到端：POST /open-apis/corehr/v1/offboardings/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/offboardings/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
                            "name": [{"lang": "zh-CN", "value": "张三"}],
                            "active": true,
                            "parent_offboarding_reason_unique_identifier": "offboarding_reason_1",
                            "created_time": "2021-08-20 20:28:23",
                            "updated_time": "2022-01-07 17:21:06"
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

        let items = resp.items.expect("items 应解析出离职原因列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].offboarding_reason_unique_identifier.as_deref(),
            Some("reason_for_offboarding_option8")
        );
        assert_eq!(items[0].active, Some(true));
        assert_eq!(
            items[0]
                .name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
