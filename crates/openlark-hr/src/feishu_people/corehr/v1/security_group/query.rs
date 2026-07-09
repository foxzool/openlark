//! 查询部门 / 地点的 HRBP / 属地 BP
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/security_group/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询部门 / 地点的 HRBP / 属地 BP请求
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

        let api_endpoint = FeishuPeopleApiV1::SecurityGroupQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url()).body(self.body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询部门或地点 HRBP/属地 BP 响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询部门 / 地点的 HRBP / 属地 BP响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SecurityGroupItem>,
}

/// 角色（安全组）条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SecurityGroupItem {
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

    /// 端到端：POST /open-apis/corehr/v1/security_groups/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/security_groups/query"))
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

        QueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
