//! 更新会议室预定管理员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-3>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新会议室预定管理员请求
pub struct PatchReserveConfigAdminRequest {
    config: Config,
    reserve_config_id: String,
}

impl PatchReserveConfigAdminRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-3>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        let api_endpoint = VcApiV1::ReserveConfigAdminPatch(self.reserve_config_id.clone());
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(api_endpoint.to_url())
            .body(serialize_params(&body, "更新会议室预定管理员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会议室预定管理员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../vc/v1/reserve_configs/{reserve_config_id}/admin → serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_patch_reserve_config_admin_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/reserve_configs/rc_001/admin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "admin_user_ids": ["ou_001", "ou_002"] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PatchReserveConfigAdminRequest::new(config)
            .reserve_config_id("rc_001")
            .execute(json!({ "admin_user_ids": ["ou_001", "ou_002"] }))
            .await
            .expect("更新会议室预定管理员应成功");
        assert_eq!(resp["admin_user_ids"][1], "ou_002");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/rc_001/admin"
        );
    }
}
