//! 用户数据维度解绑
//!
//! 文档: <https://open.feishu.cn/document/server-docs/mdm-v1/user_auth_data_relation/unbind>
//! docPath: <https://open.feishu.cn/document/server-docs/mdm-v1/user_auth_data_relation/unbind>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 用户数据维度解绑 Builder
#[derive(Debug, Clone)]
pub struct UserAuthDataRelationUnbindRequestBuilder {
    config: Config,
    user_ids: Vec<String>,
    data_type: String,
}

impl UserAuthDataRelationUnbindRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            data_type: String::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置数据类型
    pub fn data_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = data_type.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserAuthDataRelationUnbindResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserAuthDataRelationUnbindResponse> {
        let url = "/open-apis/mdm/v1/user_auth_data_relations/unbind";

        let request = UserAuthDataRelationUnbindRequest {
            user_ids: self.user_ids,
            data_type: self.data_type,
        };

        let req: ApiRequest<UserAuthDataRelationUnbindResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 用户数据维度解绑请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserAuthDataRelationUnbindRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
    /// 数据类型
    #[serde(rename = "data_type")]
    data_type: String,
}

/// 用户数据维度解绑响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuthDataRelationUnbindResponse {
    /// 解绑结果
    pub items: Vec<UserAuthDataRelationResult>,
}

/// 用户数据维度解绑结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAuthDataRelationResult {
    /// 用户 ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 解绑是否成功
    #[serde(rename = "is_success")]
    pub is_success: bool,
    /// 失败原因
    #[serde(rename = "fail_reason")]
    pub fail_reason: Option<String>,
}

impl ApiResponseTrait for UserAuthDataRelationUnbindResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to UserAuthDataRelationUnbindRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type UserAuthDataRelationUnbindBuilder = UserAuthDataRelationUnbindRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../mdm/v1/user_auth_data_relations/unbind → 强类型 UserAuthDataRelationUnbindResponse。
    #[tokio::test]
    async fn test_unbind_user_auth_data_relation_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mdm/v1/user_auth_data_relations/unbind"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "user_id": "u_001", "is_success": true }
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

        let resp = UserAuthDataRelationUnbindRequestBuilder::new(config)
            .user_id("u_001")
            .data_type("department")
            .execute()
            .await
            .expect("用户数据维度解绑应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].user_id, "u_001");
        assert!(resp.items[0].is_success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mdm/v1/user_auth_data_relations/unbind"
        );
    }
}
