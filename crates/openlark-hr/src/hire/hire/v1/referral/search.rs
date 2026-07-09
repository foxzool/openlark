//! 查询人才内推信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral/search>

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

use crate::hire::hire::common_models::IdNameObject;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SearchRequestBody {
    talent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<String>,
}

/// `SearchRequest` 请求。
#[derive(Debug, Clone)]
pub struct SearchRequest {
    config: Config,
    user_id_type: Option<String>,
    talent_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
}

impl SearchRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            talent_id: String::new(),
            start_time: None,
            end_time: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置 `talent_id`。
    pub fn talent_id(mut self, talent_id: impl Into<String>) -> Self {
        self.talent_id = talent_id.into();
        self
    }

    /// 设置 `start_time`。
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 设置 `end_time`。
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        if self.talent_id.trim().is_empty() {
            return Err(error::validation_error("talent_id", "talent_id 不能为空"));
        }

        let mut request = ApiRequest::<SearchResponse>::post("/open-apis/hire/v1/referrals/search");
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        request = request.body(
            serde_json::to_value(SearchRequestBody {
                talent_id: self.talent_id,
                start_time: self.start_time,
                end_time: self.end_time,
            })
            .map_err(|e| {
                error::validation_error("request_body", format!("无法序列化请求体: {e}"))
            })?,
        );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("查询人才内推信息响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `ReferralItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_ids` 字段。
    pub application_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `referral_user` 字段。
    pub referral_user: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `SearchResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<ReferralItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for SearchResponse {
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

    /// 端到端：POST /open-apis/hire/v1/referrals/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/referrals/search"))
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

        SearchRequest::new(config)
            .talent_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
