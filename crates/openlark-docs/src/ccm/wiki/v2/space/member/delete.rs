//! 删除知识空间成员
//!
//! 从知识空间删除成员。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceMember;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 删除知识空间成员请求
pub struct DeleteWikiSpaceMemberRequest {
    space_id: String,
    member_id: String,
    config: Config,
}

/// 删除知识空间成员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWikiSpaceMemberParams {
    /// 成员角色（例如 admin、member）
    pub member_role: String,
    /// 成员标识类型（例如 openid、userid）
    pub member_type: String,
    /// 成员主体类型（例如 user）
    #[serde(rename = "type")]
    pub member_kind: String,
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWikiSpaceMemberResponse {
    /// 删除的成员信息
    pub member: Option<WikiSpaceMember>,
}

impl ApiResponseTrait for DeleteWikiSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteWikiSpaceMemberRequest {
    /// 创建删除知识空间成员请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            member_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_id = member_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: DeleteWikiSpaceMemberParams,
    ) -> SDKResult<DeleteWikiSpaceMemberResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: DeleteWikiSpaceMemberParams,
        option: RequestOption,
    ) -> SDKResult<DeleteWikiSpaceMemberResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.member_id, "成员ID不能为空");
        validate_required!(params.member_role, "成员角色不能为空");
        validate_required!(params.member_type, "成员类型不能为空");
        validate_required!(params.member_kind, "成员主体类型不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint =
            WikiApiV2::SpaceMemberDelete(self.space_id.clone(), self.member_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteWikiSpaceMemberResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&params, "删除知识空间成员")?);

        // 发送请求
        Transport::request_typed(api_request, &self.config, Some(option), "删除知识空间成员").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../spaces/{space_id}/members/{member_id} → DeleteWikiSpaceMemberResponse。
    #[tokio::test]
    async fn test_delete_wiki_space_member_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/wiki/v2/spaces/sp001/members/mb001"))
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
        let resp = DeleteWikiSpaceMemberRequest::new(config)
            .space_id("sp001")
            .member_id("mb001")
            .execute(DeleteWikiSpaceMemberParams {
                member_role: "member".into(),
                member_type: "openid".into(),
                member_kind: "user".into(),
            })
            .await
            .expect("删除知识空间成员应成功");
        assert!(resp.member.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/wiki/v2/spaces/sp001/members/mb001"
        );
    }
}
