/// 获取云文档权限设置
///
/// 该接口用于根据 token 获取云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::PermissionPublic;
use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取云文档权限设置请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicRequest {
    /// 文档 token。
    pub token: String,
    /// 文档类型。
    pub r#type: String,
}

impl GetPermissionPublicRequest {
    /// 创建新的权限查询请求。
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
        }
    }
}

/// 获取云文档权限设置响应 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicResponse {
    /// 权限设置详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_public: Option<PermissionPublic>,
}

impl ApiResponseTrait for GetPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置。
pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
) -> SDKResult<GetPermissionPublicResponse> {
    get_permission_public_with_options(
        request,
        config,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

/// 使用指定请求选项获取云文档权限设置。
pub async fn get_permission_public_with_options(
    request: GetPermissionPublicRequest,
    config: &Config,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<GetPermissionPublicResponse> {
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

    let api_endpoint = DriveApi::GetPublicPermissionV2(request.token);
    let api_request: ApiRequest<GetPermissionPublicResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("type", &request.r#type);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取云文档权限设置")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../permissions/{token}/public?type=doc → GetPermissionPublicResponse。
    #[tokio::test]
    async fn test_get_permission_public_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
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
        let resp =
            get_permission_public(GetPermissionPublicRequest::new("token001", "doc"), &config)
                .await
                .expect("获取云文档权限应成功");
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
