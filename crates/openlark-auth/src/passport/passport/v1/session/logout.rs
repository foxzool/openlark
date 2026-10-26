//! 退出登录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/logout>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 退出登录请求体
#[derive(Debug, Serialize)]
pub struct LogoutRequestBody {
    /// 登出的方式：1-UserID（开放平台维度）；2-IdpCredentialID；3-Session 标识符
    pub logout_type: i32,
    /// 开放平台的用户标识，类型与查询参数 user_id_type 一致，logout_type = 1 时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// idp 侧的唯一标识，logout_type = 2 时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_credential_id: Option<String>,
    /// 需要精确登出的 session 标识符，logout_type = 3 时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// 登出的客户端类型，不传默认全部登出
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<Vec<i32>>,
    /// 登出提示语：34-已修改登录密码；35-登录态已失效；36-密码已过期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_reason: Option<i32>,
}

/// 退出登录请求
pub struct LogoutRequest {
    config: Config,
    logout_type: i32,
    user_id: Option<String>,
    idp_credential_id: Option<String>,
    sid: Option<String>,
    terminal_type: Option<Vec<i32>>,
    logout_reason: Option<i32>,
    /// 用户 ID 类型（查询参数）：open_id / union_id / user_id，默认 open_id
    user_id_type: Option<String>,
}

impl LogoutRequest {
    /// 创建退出登录请求实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            logout_type: 0,
            user_id: None,
            idp_credential_id: None,
            sid: None,
            terminal_type: None,
            logout_reason: None,
            user_id_type: None,
        }
    }

    /// 设置登出的方式（必填）：1-UserID；2-IdpCredentialID；3-Session 标识符
    pub fn logout_type(mut self, logout_type: i32) -> Self {
        self.logout_type = logout_type;
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 设置 idp 侧的唯一标识（logout_type = 2 时使用）
    pub fn idp_credential_id(mut self, idp_credential_id: impl Into<String>) -> Self {
        self.idp_credential_id = Some(idp_credential_id.into());
        self
    }

    /// 设置需要精确登出的 session 标识符（logout_type = 3 时使用）
    pub fn sid(mut self, sid: impl Into<String>) -> Self {
        self.sid = Some(sid.into());
        self
    }

    /// 设置登出的客户端类型，不传默认全部登出
    pub fn terminal_type(mut self, terminal_type: Vec<i32>) -> Self {
        self.terminal_type = Some(terminal_type);
        self
    }

    /// 设置登出提示语
    pub fn logout_reason(mut self, logout_reason: i32) -> Self {
        self.logout_reason = Some(logout_reason);
        self
    }

    /// 设置用户 ID 类型（查询参数）：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行退出登录请求
    pub async fn execute(self) -> SDKResult<LogoutResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行退出登录请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<LogoutResponse> {
        // logout_type 必填，合法取值 1/2/3（数值字段无 Validatable 实现，按 import_task 范式手动校验）
        if !(1..=3).contains(&self.logout_type) {
            return Err(openlark_core::error::validation_error(
                "logout_type",
                "logout_type 不能为空，可选值：1（UserID）、2（IdpCredentialID）、3（Session 标识符）",
            ));
        }

        let body = LogoutRequestBody {
            logout_type: self.logout_type,
            user_id: self.user_id,
            idp_credential_id: self.idp_credential_id,
            sid: self.sid,
            terminal_type: self.terminal_type,
            logout_reason: self.logout_reason,
        };

        let req: ApiRequest<LogoutResponse> = ApiRequest::post(PassportApiV1::SessionLogout.path())
            .body(serde_json::to_value(&body)?)
            .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "logout").await
    }
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 退出结果
    pub result: String,
}

impl ApiResponseTrait for LogoutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/passport/v1/sessions/logout
    #[tokio::test]
    async fn test_logout_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/passport/v1/sessions/logout"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "logout_type": 1,
                "user_id": "test001"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "result": "success" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        LogoutRequest::new(config)
            .logout_type(1)
            .user_id("test001")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    /// 失败路径：未设置 logout_type（或非法值）时本地校验报错，不发起请求
    #[tokio::test]
    async fn test_logout_rejects_invalid_logout_type() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = LogoutRequest::new(config).execute().await;
        assert!(result.is_err());
    }
}
