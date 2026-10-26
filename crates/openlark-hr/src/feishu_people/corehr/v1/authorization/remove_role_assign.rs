//! 移除用户被授权的角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/remove_role_assign>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 移除用户被授权的角色请求
#[derive(Debug, Clone)]
pub struct RemoveRoleAssignRequest {
    /// 配置信息
    config: Config,
    /// 查询参数（官方无请求体，employment_id/user_id_type/role_id 均为查询参数）
    query: RemoveRoleAssignQuery,
}

/// 移除用户被授权的角色查询参数
///
/// employment_id/role_id 必填；user_id_type 可选（默认 people_corehr_id）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleAssignQuery {
    /// 用户的 ID，ID 类型与 user_id_type 的取值意义一致，默认为飞书人事中的 employment_id
    pub employment_id: String,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id，默认 people_corehr_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 移除的角色 ID，一次仅可移除一个角色
    pub role_id: String,
}

impl RemoveRoleAssignRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: RemoveRoleAssignQuery {
                employment_id: String::new(),
                user_id_type: None,
                role_id: String::new(),
            },
        }
    }

    /// 设置用户的 ID
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.query.employment_id = employment_id.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置移除的角色 ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.query.role_id = role_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveRoleAssignResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveRoleAssignResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.query.employment_id, "employment_id 不能为空");
        validate_required!(self.query.role_id, "role_id 不能为空");

        let api_endpoint = FeishuPeopleApiV1::AuthorizationRemoveRoleAssign;
        let RemoveRoleAssignQuery {
            employment_id,
            user_id_type,
            role_id,
        } = self.query;
        let request = ApiRequest::<RemoveRoleAssignResponse>::post(api_endpoint.to_url())
            .query("employment_id", employment_id)
            .query_opt("user_id_type", user_id_type)
            .query("role_id", role_id);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "移除用户被授权角色响应数据为空",
        )
        .await
    }
}

/// 移除用户被授权的角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveRoleAssignResponse {
    /// 授权 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_id: Option<String>,
}

impl ApiResponseTrait for RemoveRoleAssignResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/authorizations/remove_role_assign
    /// （官方无请求体，employment_id/role_id 为必填查询参数）
    #[tokio::test]
    async fn test_remove_role_assign_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v1/authorizations/remove_role_assign",
            ))
            .and(query_param("employment_id", "7303877149017292801"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .and(query_param("role_id", "7358732070333613619"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "assign_id": "67489937334909845" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RemoveRoleAssignRequest::new(config)
            .employment_id("7303877149017292801")
            .user_id_type("people_corehr_id")
            .role_id("7358732070333613619")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.assign_id.as_deref(), Some("67489937334909845"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("employment_id=7303877149017292801"));
        assert!(query.contains("role_id=7358732070333613619"));
    }
}
