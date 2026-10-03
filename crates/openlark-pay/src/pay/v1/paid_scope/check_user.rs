//! 查询用户是否在应用开通范围
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uATNwUjLwUDM14CM1ATN>
//!
//! Authorization 仅支持 `tenant_access_token`。`open_id` / `user_id` 二选一条件必填，
//! 同时传入时由服务端取 `open_id`，SDK 不阻止同传。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::PayApiV1;

/// 用户开通范围状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaidScopeStatus {
    /// 在开通范围内且许可有效。
    Valid,
    /// 不在开通范围内。
    NotInScope,
    /// 无有效许可。
    NoActiveLicense,
    /// 超出最大使用人数。
    ExceedsMaximumLimit,
}

/// 查询用户是否在应用开通范围响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CheckUserPaidScopeResponse {
    /// 开通状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaidScopeStatus>,
    /// 付费方案 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_plan_id: Option<String>,
    /// 是否试用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trial: Option<bool>,
    /// 服务停止时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_stop_time: Option<String>,
}

impl ApiResponseTrait for CheckUserPaidScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户是否在应用开通范围请求。
#[derive(Debug, Clone)]
pub struct CheckUserPaidScopeRequest {
    config: Config,
    open_id: Option<String>,
    user_id: Option<String>,
}

impl CheckUserPaidScopeRequest {
    /// 创建请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            open_id: None,
            user_id: None,
        }
    }

    /// 设置用户 open_id。
    pub fn open_id(mut self, open_id: impl Into<String>) -> Self {
        self.open_id = Some(open_id.into());
        self
    }

    /// 设置用户 user_id。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CheckUserPaidScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CheckUserPaidScopeResponse> {
        validate_check_user_ids(self.open_id.as_deref(), self.user_id.as_deref())?;

        let mut req: ApiRequest<CheckUserPaidScopeResponse> =
            ApiRequest::get(PayApiV1::PaidScopeCheckUser.to_url())
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        req = req.query_opt("open_id", self.open_id);
        req = req.query_opt("user_id", self.user_id);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "查询用户是否在应用开通范围",
        )
        .await
    }
}

fn validate_check_user_ids(open_id: Option<&str>, user_id: Option<&str>) -> SDKResult<()> {
    let open_empty = open_id.map(str::trim).unwrap_or("").is_empty();
    let user_empty = user_id.map(str::trim).unwrap_or("").is_empty();
    if open_empty && user_empty {
        return Err(CoreError::validation_msg("open_id 与 user_id 必须包含其一"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path, query_param},
    };

    fn test_config(base_url: impl Into<String>) -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(base_url)
            .enable_token_cache(false)
            .build()
    }

    fn tenant_option() -> RequestOption {
        RequestOption::builder()
            .tenant_access_token("tenant-token")
            .build()
    }

    async fn assert_no_http(server: &MockServer) {
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn check_user_rejects_missing_ids_before_sending_request() {
        let server = MockServer::start().await;
        let result = CheckUserPaidScopeRequest::new(test_config(server.uri()))
            .execute()
            .await;
        let error = result.expect_err("open_id 与 user_id 都缺时应在发请求前失败");
        assert!(error.to_string().contains("open_id"));
        assert_no_http(&server).await;
    }

    #[tokio::test]
    async fn check_user_rejects_blank_ids_before_sending_request() {
        let server = MockServer::start().await;
        let result = CheckUserPaidScopeRequest::new(test_config(server.uri()))
            .open_id("  ")
            .user_id("")
            .execute()
            .await;
        result.expect_err("空白 open_id/user_id 应视为缺失");
        assert_no_http(&server).await;
    }

    #[tokio::test]
    async fn check_user_allows_both_ids_and_parses_typed_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/pay/v1/paid_scope/check_user"))
            .and(query_param("open_id", "ou_1"))
            .and(query_param("user_id", "ou_2"))
            .and(header("authorization", "Bearer tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "status": "valid",
                    "price_plan_id": "plan_1",
                    "is_trial": false,
                    "service_stop_time": "1700000000"
                }
            })))
            .mount(&server)
            .await;

        let resp = CheckUserPaidScopeRequest::new(test_config(server.uri()))
            .open_id("ou_1")
            .user_id("ou_2")
            .execute_with_options(tenant_option())
            .await
            .expect("同传 open_id/user_id 不应被 SDK 阻止");
        assert_eq!(resp.status, Some(PaidScopeStatus::Valid));
        assert_eq!(resp.price_plan_id.as_deref(), Some("plan_1"));
        assert_eq!(resp.is_trial, Some(false));
    }

    #[test]
    fn paid_scope_status_four_values() {
        for value in [
            "valid",
            "not_in_scope",
            "no_active_license",
            "exceeds_maximum_limit",
        ] {
            let parsed: PaidScopeStatus = serde_json::from_value(json!(value)).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), json!(value));
        }
    }
}
