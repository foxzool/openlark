//! 创建千人千面菜单。
//!
//! `POST /open-apis/bot/v3/bot_menu`，只接受 tenant access token。
//! 为指定用户创建仅其可见的机器人菜单。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/set_bot_menu>

use super::BOT_V3_BOT_MENU;
use super::models::{BotMenu, BotMenuItem};
use openlark_core::SDKResult;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

/// 创建千人千面菜单。
#[derive(Debug, Clone)]
pub struct SetBotMenuRequest {
    config: Config,
    user_id_type: Option<String>,
    user_id: Option<String>,
    bot_menu: Option<BotMenu>,
}

/// 创建千人千面菜单的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetBotMenuBody {
    /// 用户 ID。类型由查询参数 `user_id_type` 决定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 仅该用户可见的菜单。
    pub bot_menu: BotMenu,
}

/// 创建千人千面菜单的响应数据。
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct SetBotMenuResponse {
    /// 创建后的菜单。
    #[serde(default)]
    pub bot_menu: Option<BotMenu>,
}

impl ApiResponseTrait for SetBotMenuResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

fn validate_menu(menu: &BotMenu) -> SDKResult<()> {
    if let Some(items) = &menu.bot_menu_items {
        for item in items {
            validate_item(item)?;
        }
    }
    Ok(())
}

fn validate_item(item: &BotMenuItem) -> SDKResult<()> {
    openlark_core::validate_required!(item.name, "name 不能为空");
    if item.i18n_name.is_empty() {
        return Err(openlark_core::error::validation_error(
            "创建千人千面菜单",
            "i18n_name 不能为空",
        ));
    }
    for value in item.i18n_name.values() {
        openlark_core::validate_required!(value, "i18n_name 不能为空");
    }
    if let Some(children) = &item.children {
        for child in children {
            validate_item(child)?;
        }
    }
    Ok(())
}

impl SetBotMenuRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            user_id: None,
            bot_menu: None,
        }
    }

    /// 用户 ID 类型：`open_id`、`union_id` 或 `user_id`。默认 `open_id`。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 用户 ID。类型由查询参数 `user_id_type` 决定。不传则不放进请求体。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 要创建的菜单。
    pub fn bot_menu(mut self, bot_menu: BotMenu) -> Self {
        self.bot_menu = Some(bot_menu);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SetBotMenuResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetBotMenuResponse> {
        let Some(bot_menu) = self.bot_menu else {
            return Err(openlark_core::error::validation_error(
                "创建千人千面菜单",
                "bot_menu 不能为空",
            ));
        };
        validate_menu(&bot_menu)?;
        let body = SetBotMenuBody {
            user_id: self.user_id,
            bot_menu,
        };
        let payload = serde_json::to_value(&body).map_err(|err| {
            openlark_core::error::validation_error(
                "创建千人千面菜单",
                format!("请求体序列化失败: {err}"),
            )
        })?;
        let req: ApiRequest<SetBotMenuResponse> = ApiRequest::post(BOT_V3_BOT_MENU)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant])
            .query_opt("user_id_type", self.user_id_type)
            .body(payload);
        Transport::request_typed(req, &self.config, Some(option), "创建千人千面菜单").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BotService;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn sample_menu() -> BotMenu {
        serde_json::from_value(json!({
            "bot_menu_id": "1",
            "bot_menu_items": [{
                "name": "测试按钮",
                "i18n_name": {"en_us": "New topic"},
                "tag": "kkk111",
                "behaviors": [{
                    "type": "target",
                    "target": {"common_url": "https://www.google.com"},
                    "event_key": "xxx",
                    "is_primary": false
                }]
            }]
        }))
        .expect("菜单样例")
    }

    #[tokio::test]
    async fn set_bot_menu_requires_menu_before_http() {
        let err = BotService::new(Config::default())
            .set_bot_menu()
            .execute()
            .await
            .expect_err("缺少 bot_menu");
        assert!(err.to_string().contains("bot_menu"), "{err}");
    }

    #[tokio::test]
    async fn set_bot_menu_rejects_blank_name_at_any_depth() {
        let menu: BotMenu = serde_json::from_value(json!({
            "bot_menu_items": [{
                "name": "测试按钮",
                "i18n_name": {"en_us": "New topic"},
                "children": [{
                    "name": "  ",
                    "i18n_name": {"en_us": "Child"}
                }]
            }]
        }))
        .expect("菜单");
        let err = SetBotMenuRequest::new(Config::default())
            .bot_menu(menu)
            .execute()
            .await
            .expect_err("子项空白 name");
        assert!(err.to_string().contains("name"), "{err}");
    }

    #[tokio::test]
    async fn set_bot_menu_posts_tenant_body_and_reads_menu() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/bot/v3/bot_menu"))
            .and(query_param("user_id_type", "open_id"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(body_json(json!({
                "user_id": "ou_user",
                "bot_menu": {
                    "bot_menu_id": "1",
                    "bot_menu_items": [{
                        "name": "测试按钮",
                        "i18n_name": {"en_us": "New topic"},
                        "tag": "kkk111",
                        "behaviors": [{
                            "type": "target",
                            "target": {"common_url": "https://www.google.com"},
                            "event_key": "xxx",
                            "is_primary": false
                        }]
                    }]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {"bot_menu": {"bot_menu_id": "1", "bot_menu_items": []}}
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
        let response = SetBotMenuRequest::new(config)
            .user_id_type("open_id")
            .user_id("ou_user")
            .bot_menu(sample_menu())
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("创建菜单应成功");
        assert_eq!(
            response.bot_menu.and_then(|menu| menu.bot_menu_id),
            Some("1".to_string())
        );
    }
}
