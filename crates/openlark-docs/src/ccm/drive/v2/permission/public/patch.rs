/// 更新云文档权限设置
///
/// 该接口用于根据 token 更新云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::PermissionPublic;
use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 更新云文档权限设置请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicRequest {
    /// 文档 token。
    pub token: String,
    /// 云文档类型（query 参数 `type`），需要与 token 匹配
    pub r#type: String,
    /// 是否允许内容被分享到组织外
    pub external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载
    pub security_entity: Option<String>,
    /// 谁可以评论
    pub comment_entity: Option<String>,
    /// 从组织维度，设置谁可以查看、添加、移除协作者
    pub share_entity: Option<String>,
    /// 从协作者维度，设置谁可以查看、添加、移除协作者
    pub manage_collaborator_entity: Option<String>,
    /// 链接分享设置
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容
    pub copy_entity: Option<String>,
}

impl UpdatePermissionPublicRequest {
    /// 创建新的权限更新请求。
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
            external_access_entity: None,
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            manage_collaborator_entity: None,
            link_share_entity: None,
            copy_entity: None,
        }
    }
}

/// 更新云文档权限设置响应 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicResponse {
    /// 权限设置详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_public: Option<PermissionPublic>,
}

impl ApiResponseTrait for UpdatePermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePermissionPublicBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_access_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_collaborator_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_entity: Option<String>,
}

/// 更新云文档权限设置。
pub async fn update_permission_public(
    request: UpdatePermissionPublicRequest,
    config: &Config,
) -> SDKResult<UpdatePermissionPublicResponse> {
    update_permission_public_with_options(
        request,
        config,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

/// 使用指定请求选项更新云文档权限设置。
pub async fn update_permission_public_with_options(
    request: UpdatePermissionPublicRequest,
    config: &Config,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<UpdatePermissionPublicResponse> {
    if request.token.is_empty() {
        return Err(openlark_core::error::validation_error(
            "token",
            "token 不能为空",
        ));
    }
    if request.r#type.is_empty() {
        return Err(openlark_core::error::validation_error(
            "type",
            "type 不能为空",
        ));
    }

    let api_endpoint = DriveApi::UpdatePublicPermissionV2(request.token);

    let body = UpdatePermissionPublicBody {
        external_access_entity: request.external_access_entity,
        security_entity: request.security_entity,
        comment_entity: request.comment_entity,
        share_entity: request.share_entity,
        manage_collaborator_entity: request.manage_collaborator_entity,
        link_share_entity: request.link_share_entity,
        copy_entity: request.copy_entity,
    };

    let api_request: ApiRequest<UpdatePermissionPublicResponse> =
        ApiRequest::patch(&api_endpoint.to_url())
            .query("type", &request.r#type)
            .body(serialize_params(&body, "更新云文档权限设置")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新云文档权限设置")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../permissions/{token}/public?type=doc → UpdatePermissionPublicResponse。
    #[tokio::test]
    async fn test_update_permission_public_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/drive/v2/permissions/token001/public"))
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
        let resp = update_permission_public(
            UpdatePermissionPublicRequest::new("token001", "doc"),
            &config,
        )
        .await
        .expect("更新云文档权限应成功");
        assert!(resp.permission_public.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v2/permissions/token001/public"
        );
        assert!(received[0].url.query().unwrap_or("").contains("type=doc"));
    }
}
