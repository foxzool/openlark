//! 获取妙搭产品使用权限
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-available_scope/open_api_get_miaoda_available_scope>
//!
//! Authorization **仅** `user_access_token`。权限点 `spark:admin:read`，仅 Custom App。

use crate::common::constants::endpoints::SPARK_V1_AVAILABLE_SCOPE;
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

/// 获取妙搭产品使用权限响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GetMiaodaAvailableScopeResponse {
    /// 使用权限模式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_scope: Option<AvailableScopeMode>,
    /// 部门 ID 列表（仅 PART 模式有意义）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户 ID 列表（仅 PART 模式有意义）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for GetMiaodaAvailableScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取妙搭产品使用权限请求。
#[derive(Debug, Clone)]
pub struct GetMiaodaAvailableScopeRequest {
    config: Arc<Config>,
}

impl GetMiaodaAvailableScopeRequest {
    /// 创建请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMiaodaAvailableScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMiaodaAvailableScopeResponse> {
        let req: ApiRequest<GetMiaodaAvailableScopeResponse> =
            ApiRequest::get(SPARK_V1_AVAILABLE_SCOPE)
                .with_supported_access_token_types(vec![AccessTokenType::User]);
        Transport::request_typed(req, &self.config, Some(option), "获取妙搭产品使用权限").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
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
    async fn get_available_scope_uses_user_token_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/spark/v1/available_scope"))
            .and(header("authorization", "Bearer user-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "available_scope": "ALLOW_PART",
                    "department_ids": ["od_1"],
                    "user_ids": ["ou_1"]
                }
            })))
            .mount(&server)
            .await;

        let resp = GetMiaodaAvailableScopeRequest::new(test_config(server.uri()))
            .execute_with_options(user_option())
            .await
            .expect("获取使用权限应成功");
        assert_eq!(resp.available_scope, Some(AvailableScopeMode::AllowPart));
        assert_eq!(
            resp.department_ids.as_deref(),
            Some(["od_1".to_string()].as_slice())
        );
    }
}
