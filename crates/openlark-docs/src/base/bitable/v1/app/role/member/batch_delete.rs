//! Bitable 批量删除协作者（自定义角色）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/batch_delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::RoleMemberId;

/// 批量删除协作者请求。
#[derive(Debug, Clone)]
pub struct BatchDeleteRoleMemberRequest {
    config: Config,
    app_token: String,
    role_id: String,
    member_list: Vec<RoleMemberId>,
}

impl BatchDeleteRoleMemberRequest {
    /// 创建新的批量删除协作者请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            member_list: Vec::new(),
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID。
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置成员列表。
    pub fn member_list(mut self, member_list: Vec<RoleMemberId>) -> Self {
        self.member_list = member_list;
        self
    }

    /// 追加一个成员。
    pub fn add_member(mut self, member: RoleMemberId) -> Self {
        self.member_list.push(member);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteRoleMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteRoleMemberResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");
        validate_required!(self.member_list, "member_list");
        if self.member_list.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "member_list",
                "member_list 最多 100 项",
            ));
        }
        for (idx, item) in self.member_list.iter().enumerate() {
            if item.id.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "member_list",
                    &format!("第 {} 个成员的 id 不能为空", idx + 1),
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RoleMemberBatchDelete(self.app_token.clone(), self.role_id.clone());

        // #439: method 来自 catalog
        let api_request: ApiRequest<BatchDeleteRoleMemberResponse> = api_endpoint
            .to_request::<BatchDeleteRoleMemberResponse>()
            .body(serde_json::to_vec(&BatchDeleteRoleMemberRequestBody {
                member_list: self.member_list,
            })?);

        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Bitable 批量删除协作者（自定义角色）",
        )
        .await
    }
}

/// 批量删除协作者请求体（内部使用）。
#[derive(Serialize)]
struct BatchDeleteRoleMemberRequestBody {
    member_list: Vec<RoleMemberId>,
}

/// 批量删除协作者响应（data 为 {}）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberResponse {}

impl ApiResponseTrait for BatchDeleteRoleMemberResponse {
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

    /// 端到端：POST .../members/batch_delete → BatchDeleteRoleMemberResponse。
    #[tokio::test]
    async fn test_batch_delete_role_member_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/bitable/v1/apps/app001/roles/role001/members/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        BatchDeleteRoleMemberRequest::new(config)
            .app_token("app001".into())
            .role_id("role001".into())
            .member_list(vec![RoleMemberId {
                id_type: Default::default(),
                id: "ou001".into(),
            }])
            .execute()
            .await
            .expect("批量删除角色成员应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/roles/role001/members/batch_delete"
        );
    }
}
