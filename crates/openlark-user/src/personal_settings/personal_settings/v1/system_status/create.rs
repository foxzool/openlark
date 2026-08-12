//! 创建系统状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::models::{SystemStatus, SystemStatusI18nName, SystemStatusSyncSetting};

/// 创建系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusCreateRequest {
    config: Arc<Config>,
    body: SystemStatusCreateBody,
}

/// 创建系统状态请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusCreateBody {
    /// 系统状态标题（必填，1～20 字符）。
    pub title: String,
    /// 系统状态国际化标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<SystemStatusI18nName>,
    /// 图标 key（必填）。
    pub icon_key: String,
    /// 颜色。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// 优先级。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 同步设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_setting: Option<SystemStatusSyncSetting>,
}

impl SystemStatusCreateBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.title.trim(), "title 不能为空");
        validate_required!(self.icon_key.trim(), "icon_key 不能为空");
        Ok(())
    }
}

/// 创建系统状态响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusCreateResponse {
    /// 创建后的系统状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

impl ApiResponseTrait for SystemStatusCreateResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SystemStatusCreateRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SystemStatusCreateBody::default(),
        }
    }

    /// 设置标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    /// 设置国际化标题。
    pub fn i18n_title(mut self, i18n_title: SystemStatusI18nName) -> Self {
        self.body.i18n_title = Some(i18n_title);
        self
    }

    /// 设置图标 key。
    pub fn icon_key(mut self, icon_key: impl Into<String>) -> Self {
        self.body.icon_key = icon_key.into();
        self
    }

    /// 设置颜色。
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.body.color = Some(color.into());
        self
    }

    /// 设置优先级。
    pub fn priority(mut self, priority: i32) -> Self {
        self.body.priority = Some(priority);
        self
    }

    /// 设置同步设置。
    pub fn sync_setting(mut self, sync_setting: SystemStatusSyncSetting) -> Self {
        self.body.sync_setting = Some(sync_setting);
        self
    }

    /// 执行创建系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusCreateResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SystemStatusCreateResponse> =
            ApiRequest::post("/open-apis/personal_settings/v1/system_statuses")
                .body(body)
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "创建系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../system_statuses + body → 响应 `system_status`。
    #[tokio::test]
    async fn test_create_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/personal_settings/v1/system_statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "system_status": {
                        "system_status_id": "ss_new",
                        "title": "出差",
                        "icon_key": "GeneralBusinessTrip",
                        "color": "BLUE",
                        "priority": 1
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = SystemStatusCreateRequest::new(config)
            .title("出差")
            .icon_key("GeneralBusinessTrip")
            .color("BLUE")
            .priority(1)
            .execute()
            .await
            .expect("创建系统状态应成功");
        assert_eq!(
            resp.system_status
                .as_ref()
                .unwrap()
                .system_status_id
                .as_deref(),
            Some("ss_new")
        );

        let received = server.received_requests().await.unwrap_or_default();
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["title"], "出差");
        assert_eq!(sent["icon_key"], "GeneralBusinessTrip");
    }
}
