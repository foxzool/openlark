//! 判断用户云文档权限
//!
//! 该接口用于根据 filetoken 判断当前登录用户是否具有某权限。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth>

use openlark_core::{
    SDKResult,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 判断用户云文档权限请求
#[derive(Debug, Clone)]
pub struct AuthPermissionMemberRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 操作类型（query 参数 `action`）
    ///
    /// 可选值：`view`、`edit`、`share`、`comment`、`export`、`copy`、`print`、`manage_public`
    pub action: String,
}

impl AuthPermissionMemberRequest {
    /// 创建新的权限判断请求。
    /// 创建判断用户云文档权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `action` - 操作类型（query 参数 `action`）
    pub fn new(
        config: Config,
        token: impl Into<String>,
        file_type: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            action: action.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<AuthPermissionMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AuthPermissionMemberResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.token, "token 不能为空");
        validate_required!(self.file_type, "file_type 不能为空");
        validate_required!(self.action, "action 不能为空");
        // ===== 验证字段枚举值 =====
        match self.action.as_str() {
            "view" | "edit" | "share" | "comment" | "export" | "copy" | "print"
            | "manage_public" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "action",
                    "action 必须为 view/edit/share/comment/export/copy/print/manage_public",
                ));
            }
        }

        let api_endpoint = DriveApi::AuthPermissionMember(self.token.clone());

        let api_request = api_endpoint
            .to_request::<AuthPermissionMemberResponse>()
            .query("type", &self.file_type)
            .query("action", &self.action);

        Transport::request_typed(api_request, &self.config, Some(option), "授权").await
    }
}

/// 判断用户云文档权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPermissionMemberResponse {
    /// 是否有权限
    pub auth_result: bool,
}

impl ApiResponseTrait for AuthPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_auth_permission_member_request_builder() {
        let config = Config::default();
        let request = AuthPermissionMemberRequest::new(config, "file_token", "docx", "view");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.action, "view");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            AuthPermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试支持的 action 类型
    #[test]
    fn test_supported_actions() {
        let config = Config::default();

        for action in [
            "view",
            "edit",
            "share",
            "comment",
            "export",
            "copy",
            "print",
            "manage_public",
        ] {
            let request = AuthPermissionMemberRequest::new(
                config.clone(),
                "token",
                "docx",
                action.to_string(),
            );
            assert_eq!(request.action, action);
        }
    }

    /// 测试响应结构
    #[test]
    fn test_response_structure() {
        let response = AuthPermissionMemberResponse { auth_result: true };
        assert!(response.auth_result);

        let response2 = AuthPermissionMemberResponse { auth_result: false };
        assert!(!response2.auth_result);
    }

    /// 端到端：GET /open-apis/drive/v1/permissions/{token}/members/auth → AuthPermissionMemberResponse。
    #[tokio::test]
    async fn test_auth_permission_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/drive/v1/permissions/file_token_001/members/auth",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "auth_result": true
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

        let resp = AuthPermissionMemberRequest::new(config, "file_token_001", "docx", "view")
            .execute()
            .await
            .expect("权限判断应成功");
        assert!(resp.auth_result);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/permissions/file_token_001/members/auth"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(
            query.contains("type=docx"),
            "query 应携带 type=docx，实际：{query}"
        );
        assert!(
            query.contains("action=view"),
            "query 应携带 action=view，实际：{query}"
        );
    }
}
