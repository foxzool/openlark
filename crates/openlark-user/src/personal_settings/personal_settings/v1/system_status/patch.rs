//! 修改系统状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::models::SystemStatus;

/// 更新系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusPatchRequest {
    config: Arc<Config>,
    /// 路径参数 `system_status_id`。
    system_status_id: String,
    body: SystemStatusPatchBody,
}

/// 更新系统状态请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusPatchBody {
    /// 待更新的系统状态字段。
    pub system_status: SystemStatus,
    /// 需要更新的字段枚举列表（如 `TITLE` / `ICON` / `COLOR` 等）。
    pub update_fields: Vec<String>,
}

impl SystemStatusPatchBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required_list!(self.update_fields, 100, "update_fields 不能为空");
        Ok(())
    }
}

/// 更新系统状态响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusPatchResponse {
    /// 更新后的系统状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

impl ApiResponseTrait for SystemStatusPatchResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SystemStatusPatchRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            system_status_id: String::new(),
            body: SystemStatusPatchBody::default(),
        }
    }

    /// 设置系统状态 ID（路径参数）。
    pub fn system_status_id(mut self, system_status_id: impl Into<String>) -> Self {
        self.system_status_id = system_status_id.into();
        self
    }

    /// 设置待更新的系统状态内容。
    pub fn system_status(mut self, system_status: SystemStatus) -> Self {
        self.body.system_status = system_status;
        self
    }

    /// 设置需要更新的字段列表。
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 执行更新系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusPatchResponse> {
        validate_required!(self.system_status_id.trim(), "system_status_id 不能为空");
        self.body.validate()?;
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            self.system_status_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SystemStatusPatchResponse> = ApiRequest::patch(&path)
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "修改系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../system_statuses/{id} + body{system_status,update_fields}。
    #[tokio::test]
    async fn test_patch_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "system_status": {
                        "system_status_id": "ss_001",
                        "title": "出差",
                        "icon_key": "GeneralBusinessTrip",
                        "color": "BLUE"
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

        let resp = SystemStatusPatchRequest::new(config)
            .system_status_id("ss_001")
            .system_status(SystemStatus {
                icon_key: Some("GeneralBusinessTrip".into()),
                color: Some("BLUE".into()),
                ..Default::default()
            })
            .update_fields(vec!["ICON".into(), "COLOR".into()])
            .execute()
            .await
            .expect("更新系统状态应成功");
        assert_eq!(
            resp.system_status
                .as_ref()
                .unwrap()
                .system_status_id
                .as_deref(),
            Some("ss_001")
        );

        let received = server.received_requests().await.unwrap_or_default();
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["system_status"]["icon_key"], "GeneralBusinessTrip");
        assert_eq!(sent["update_fields"].as_array().unwrap().len(), 2);
    }
}
