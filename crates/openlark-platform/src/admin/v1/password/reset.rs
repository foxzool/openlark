//! 重置用户密码 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/password/reset>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 重置用户密码的请求构建器。
pub struct ResetPasswordRequestBuilder {
    user_id: String,
    password: PasswordParam,
    /// 用户 ID 类型（查询参数，可选 open_id/union_id/user_id，默认 open_id）
    user_id_type: Option<String>,
    config: Config,
}

impl ResetPasswordRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            user_id: String::new(),
            password: PasswordParam::default(),
            user_id_type: None,
            config,
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置新密码（官方请求体嵌套在 password.ent_email_password）。
    pub fn new_password(mut self, new_password: impl Into<String>) -> Self {
        self.password.ent_email_password = new_password.into();
        self
    }

    /// 设置用户 ID 类型（查询参数，可选 open_id/union_id/user_id，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ResetPasswordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ResetPasswordResponse> {
        validate_required!(self.user_id, "用户ID不能为空");
        validate_required!(self.password.ent_email_password, "新密码不能为空");

        let request_body = ResetPasswordRequest {
            user_id: self.user_id,
            password: self.password,
        };

        let api_request: ApiRequest<ResetPasswordResponse> =
            ApiRequest::post("/open-apis/admin/v1/password/reset")
                .body(serde_json::to_value(&request_body)?)
                .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(api_request, &self.config, Some(option), "重置用户密码").await
    }
}

/// 重置用户密码请求体（官方形状：密码嵌套在 password 对象内）。
#[derive(Debug, Serialize)]
struct ResetPasswordRequest {
    user_id: String,
    password: PasswordParam,
}

/// 密码参数（官方嵌套容器）。
#[derive(Debug, Default, Serialize)]
struct PasswordParam {
    /// 企业邮箱密码
    ent_email_password: String,
}

/// 重置用户密码的响应（官方成功响应 data 为空对象，无业务字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResponse {}

impl ApiResponseTrait for ResetPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应无业务字段，data 为空对象 `{}` 或整个缺失时给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ResetPasswordRequestBuilder, will be removed in v1.0 (#271)")]
pub type ResetPasswordBuilder = ResetPasswordRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST 官方嵌套请求体 + user_id_type 查询参数 → 成功响应 data 为空对象。
    #[tokio::test]
    async fn test_reset_password_sends_official_body_and_returns_empty_data() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/password/reset"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "password": { "ent_email_password": "NewP@ssw0rd!" },
                "user_id": "u_001"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ResetPasswordRequestBuilder::new(config)
            .user_id("u_001")
            .new_password("NewP@ssw0rd!")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("重置用户密码应成功");
        let _ = resp;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/admin/v1/password/reset");
    }
}
