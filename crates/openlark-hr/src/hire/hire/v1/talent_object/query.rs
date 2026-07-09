//! 获取人才字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent_object/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::I18nText;

/// 获取人才字段请求。
///
/// 官方文档未定义请求参数，因此该请求仅持有配置并直接发起调用。
#[derive(Debug, Clone)]
pub struct QueryRequest {
    config: Config,
}

impl QueryRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        let request = ApiRequest::<QueryResponse>::get("/open-apis/hire/v1/talent_objects/query");
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取人才字段响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `TalentObjectOption`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentObjectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 键。
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentObjectConfig`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentObjectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `options` 字段。
    pub options: Option<Vec<TalentObjectOption>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentObjectSetting`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentObjectSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 对象类型。
    pub object_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `config` 字段。
    pub config: Option<TalentObjectConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentObjectChild`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentObjectChild {
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
    /// `setting` 字段。
    pub setting: Option<TalentObjectSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `parent_id` 字段。
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_customized` 字段。
    pub is_customized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_required` 字段。
    pub is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentObjectItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentObjectItem {
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
    /// `setting` 字段。
    pub setting: Option<TalentObjectSetting>,
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
    /// `children_list` 字段。
    pub children_list: Option<Vec<TalentObjectChild>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `QueryResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<TalentObjectItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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

    /// 端到端：GET /open-apis/hire/v1/talent_objects/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/talent_objects/query"))
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
