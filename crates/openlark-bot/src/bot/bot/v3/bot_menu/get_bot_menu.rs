//! 获取千人千面菜单。
//!
//! `GET /open-apis/bot/v3/bot_menu`，只接受 tenant access token。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/get_bot_menu>

use super::BOT_V3_BOT_MENU;
use super::models::BotMenu;
use openlark_core::SDKResult;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use serde::Deserialize;

/// 获取千人千面菜单。
#[derive(Debug, Clone)]
pub struct GetBotMenuRequest {
    config: Config,
    user_id: Option<String>,
    user_id_type: Option<String>,
}

/// 获取千人千面菜单的响应数据。
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct GetBotMenuResponse {
    /// 该用户可见的菜单。
    #[serde(default)]
    pub bot_menu: Option<BotMenu>,
}

impl ApiResponseTrait for GetBotMenuResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetBotMenuRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: None,
            user_id_type: None,
        }
    }

    /// 用户 ID。类型由 `user_id_type` 决定。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 用户 ID 类型：`open_id`、`union_id` 或 `user_id`。默认 `open_id`。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetBotMenuResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetBotMenuResponse> {
        let req = ApiRequest::<GetBotMenuResponse>::get(BOT_V3_BOT_MENU)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant])
            .query_opt("user_id", self.user_id)
            .query_opt("user_id_type", self.user_id_type);
        Transport::request_typed(req, &self.config, Some(option), "获取千人千面菜单").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn get_bot_menu_reads_menu_for_user() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/bot/v3/bot_menu"))
            .and(query_param("user_id", "ou_user"))
            .and(query_param("user_id_type", "union_id"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "bot_menu": {
                        "bot_menu_id": "menu-9",
                        "bot_menu_items": [{
                            "name": "测试按钮",
                            "i18n_name": {"en_us": "New topic"},
                            "children": [{
                                "name": "子按钮",
                                "behaviors": {
                                    "type": "event_key",
                                    "event_key": "open"
                                }
                            }]
                        }]
                    }
                }
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
        let response = GetBotMenuRequest::new(config)
            .user_id("ou_user")
            .user_id_type("union_id")
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("获取菜单应成功");
        let menu = response.bot_menu.expect("bot_menu");
        assert_eq!(menu.bot_menu_id.as_deref(), Some("menu-9"));
        let items = menu.bot_menu_items.expect("items");
        let children = items[0].children.as_ref().expect("children");
        let behavior = &children[0].behaviors.as_ref().expect("behaviors")[0];
        assert_eq!(behavior.behavior_type.as_deref(), Some("event_key"));
        assert_eq!(behavior.event_key.as_deref(), Some("open"));
    }
}
