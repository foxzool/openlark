//! 添加访客
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/visitor/create>
//!
//! 文档核对：`POST /open-apis/acs/v1/visitors?user_id_type={...}`，body 为
//! `{"user": {"user_type": 11, ...}}`（`user` 必填，其内 `user_type` 必填）；
//! 查询参数 `user_id_type` 可选（默认 `open_id`）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 添加访客请求 body。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVisitorBody {
    /// 访客信息。
    pub user: VisitorUser,
}

/// 访客信息（官方 user_external）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorUser {
    /// 用户类型（必填；访客固定为 `11`，可选 `1` 员工/`2` 部门/`10` 全体员工/`11` 访客）。
    pub user_type: i32,
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 电话号码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_num: Option<String>,
    /// 部门 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

/// 添加访客请求
#[derive(Debug)]
pub struct CreateVisitorRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: CreateVisitorBody,
    /// 用户 ID 类型（查询参数，可选；如 `open_id`/`user_id`/`union_id`，默认 `open_id`）。
    user_id_type: Option<String>,
}

impl CreateVisitorRequest {
    /// 创建新的请求构建器（`user_type` 需另行设置，访客为 `11`）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateVisitorBody {
                user: VisitorUser {
                    user_type: 0,
                    user_id: None,
                    user_name: None,
                    phone_num: None,
                    department_id: None,
                },
            },
            user_id_type: None,
        }
    }

    /// 设置用户类型（访客为 `11`）。
    pub fn user_type(mut self, user_type: i32) -> Self {
        self.body.user.user_type = user_type;
        self
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user.user_id = Some(user_id.into());
        self
    }

    /// 设置用户名称。
    pub fn user_name(mut self, user_name: impl Into<String>) -> Self {
        self.body.user.user_name = Some(user_name.into());
        self
    }

    /// 设置电话号码。
    pub fn phone_num(mut self, phone_num: impl Into<String>) -> Self {
        self.body.user.phone_num = Some(phone_num.into());
        self
    }

    /// 设置部门 ID。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.user.department_id = Some(department_id.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        if self.body.user.user_type <= 0 {
            return Err(validation_error(
                "添加访客",
                "user_type 不能为空（访客为 11）",
            ));
        }

        let req: ApiRequest<serde_json::Value> = ApiRequest::post("/open-apis/acs/v1/visitors")
            .query_opt("user_id_type", self.user_id_type.as_ref())
            .body(
                serde_json::to_value(&self.body)
                    .map_err(|e| validation_error("添加访客", format!("序列化失败: {e}")))?,
            )
            .with_supported_access_token_types(vec![AccessTokenType::User]);

        Transport::request_typed(req, &self.config, Some(option), "添加访客").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_create_visitor_rejects_missing_user_type() {
        let req = CreateVisitorRequest::new(test_config());
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_type"));
    }

    /// 端到端：POST .../visitors + body {"user": {...}} 包装 + 响应解析。
    #[tokio::test]
    async fn test_create_visitor_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/acs/v1/visitors"))
            .and(body_json(json!({
                "user": {
                    "user_type": 11,
                    "user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
                    "user_name": "张三",
                    "phone_num": "1357890001"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "visitor_id": "6939433228970082568" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateVisitorRequest::new(config)
            .user_type(11)
            .user_id("ou_7dab8a3d3cdcc9da365777c7ad535d62")
            .user_name("张三")
            .phone_num("1357890001")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("添加访客应成功");
        assert_eq!(data["visitor_id"], "6939433228970082568");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));
        // body 应为 {"user": {...}} 包装
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["user"]["user_type"], 11);
        assert_eq!(sent["user"]["user_name"], "张三");
    }
}
