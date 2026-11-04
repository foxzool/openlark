//! 删除千人千面菜单。
//!
//! `DELETE /open-apis/bot/v3/bot_menu`，只接受 tenant access token。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/delete_bot_menu>

use super::BOT_V3_BOT_MENU;
use openlark_core::SDKResult;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

/// 删除千人千面菜单。
#[derive(Debug, Clone)]
pub struct DeleteBotMenuRequest {
    config: Config,
    user_id_type: Option<String>,
    user_id: Option<String>,
}

/// 删除千人千面菜单的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteBotMenuBody {
    /// 用户 ID。类型由查询参数 `user_id_type` 决定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 删除成功时 `data` 为空对象。
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct DeleteBotMenuResponse {}

impl ApiResponseTrait for DeleteBotMenuResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

impl DeleteBotMenuRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            user_id: None,
        }
    }

    /// 用户 ID 类型：`open_id`、`union_id` 或 `user_id`。默认 `open_id`。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 要删除其菜单的用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteBotMenuResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteBotMenuResponse> {
        let body = DeleteBotMenuBody {
            user_id: self.user_id,
        };
        let payload = serde_json::to_value(&body).map_err(|err| {
            openlark_core::error::validation_error(
                "删除千人千面菜单",
                format!("请求体序列化失败: {err}"),
            )
        })?;
        let req: ApiRequest<DeleteBotMenuResponse> = ApiRequest::delete(BOT_V3_BOT_MENU)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant])
            .query_opt("user_id_type", self.user_id_type)
            .body(payload);
        Transport::request_typed(req, &self.config, Some(option), "删除千人千面菜单").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn delete_bot_menu_sends_user_and_accepts_empty_data() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/bot/v3/bot_menu"))
            .and(query_param("user_id_type", "user_id"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(body_json(json!({"user_id": "u_1"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .expect(1)
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        DeleteBotMenuRequest::new(config)
            .user_id_type("user_id")
            .user_id("u_1")
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("删除菜单应成功");
    }
}
