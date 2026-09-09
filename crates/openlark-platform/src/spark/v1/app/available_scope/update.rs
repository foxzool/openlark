//! 修改妙搭产品使用权限
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-available_scope/open_api_update_miaoda_available_scope>
//!
//! Authorization **仅** `user_access_token`。权限点 `spark:admin:write`，仅 Custom App。
//! 成功响应 `data` 为空对象 `{}`。`department_ids` / `user_ids` 长度 0-1000 由服务端校验。

use crate::common::{api_utils::serialize_params, constants::endpoints::SPARK_V1_AVAILABLE_SCOPE};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::AvailableScopeMode;

/// 修改妙搭产品使用权限请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMiaodaAvailableScopeBody {
    /// 使用权限模式（必填）。
    pub available_scope: AvailableScopeMode,
    /// 部门 ID 列表（仅 PART 模式生效）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户 ID 列表（仅 PART 模式生效）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

/// 修改妙搭产品使用权限响应 `data`（空对象）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateMiaodaAvailableScopeResponse {}

impl ApiResponseTrait for UpdateMiaodaAvailableScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改妙搭产品使用权限请求。
#[derive(Debug, Clone)]
pub struct UpdateMiaodaAvailableScopeRequest {
    config: Arc<Config>,
}

impl UpdateMiaodaAvailableScopeRequest {
    /// 创建请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: UpdateMiaodaAvailableScopeBody,
    ) -> SDKResult<UpdateMiaodaAvailableScopeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateMiaodaAvailableScopeBody,
        option: RequestOption,
    ) -> SDKResult<UpdateMiaodaAvailableScopeResponse> {
        let req: ApiRequest<UpdateMiaodaAvailableScopeResponse> =
            ApiRequest::put(SPARK_V1_AVAILABLE_SCOPE)
                .body(serialize_params(&body, "修改妙搭产品使用权限")?)
                .with_supported_access_token_types(vec![AccessTokenType::User]);
        Transport::request_typed(req, &self.config, Some(option), "修改妙搭产品使用权限").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    fn test_config(base_url: impl Into<String>) -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(base_url)
                .enable_token_cache(false)
                .build(),
        )
    }

    fn user_option() -> RequestOption {
        RequestOption::builder()
            .user_access_token("user-token")
            .build()
    }

    #[tokio::test]
    async fn update_available_scope_uses_user_token_and_empty_data() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/spark/v1/available_scope"))
            .and(header("authorization", "Bearer user-token"))
            .and(body_json(json!({
                "available_scope": "DENY_ALL"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let resp = UpdateMiaodaAvailableScopeRequest::new(test_config(server.uri()))
            .execute_with_options(
                UpdateMiaodaAvailableScopeBody {
                    available_scope: AvailableScopeMode::DenyAll,
                    department_ids: None,
                    user_ids: None,
                },
                user_option(),
            )
            .await
            .expect("修改使用权限应成功");
        let _ = resp;
    }
}
