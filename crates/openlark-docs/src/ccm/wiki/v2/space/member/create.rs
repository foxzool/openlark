//! 添加知识空间成员
//!
//! 为知识空间添加成员。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create>

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

/// 添加知识空间成员请求
pub struct CreateWikiSpaceMemberRequest {
    space_id: String,
    need_notification: Option<bool>,
    config: Config,
}

/// 添加知识空间成员请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceMemberParams {
    /// 成员标识类型（例如 openid、userid）
    pub member_type: String,
    /// 成员 ID（与 member_type 搭配）
    pub member_id: String,
    /// 成员角色（例如 admin、member）
    pub member_role: String,
}

/// 添加知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceMemberResponse {
    /// 成员信息
    pub member: Option<WikiSpaceMember>,
}

impl ApiResponseTrait for CreateWikiSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceMemberRequest {
    /// 创建添加知识空间成员请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            need_notification: None,
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 是否需要通知（可选）
    pub fn need_notification(mut self, need_notification: bool) -> Self {
        self.need_notification = Some(need_notification);
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: CreateWikiSpaceMemberParams,
    ) -> SDKResult<CreateWikiSpaceMemberResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: CreateWikiSpaceMemberParams,
        option: RequestOption,
    ) -> SDKResult<CreateWikiSpaceMemberResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(params.member_type, "成员类型不能为空");
        validate_required!(params.member_id, "成员ID不能为空");
        validate_required!(params.member_role, "成员角色不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceMemberCreate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateWikiSpaceMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "添加知识空间成员")?);

        if let Some(need_notification) = self.need_notification {
            api_request = api_request.query("need_notification", &need_notification.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "添加知识空间成员")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../spaces/{space_id}/members → CreateWikiSpaceMemberResponse。
    #[tokio::test]
    async fn test_create_wiki_space_member_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/wiki/v2/spaces/sp001/members"))
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
        let resp = CreateWikiSpaceMemberRequest::new(config)
            .space_id("sp001")
            .execute(CreateWikiSpaceMemberParams {
                member_type: "openid".into(),
                member_id: "ou001".into(),
                member_role: "member".into(),
            })
            .await
            .expect("添加知识空间成员应成功");
        assert!(resp.member.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/wiki/v2/spaces/sp001/members"
        );
    }
}
