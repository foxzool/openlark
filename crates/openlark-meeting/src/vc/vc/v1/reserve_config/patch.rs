//! 更新会议室预定限制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::serialize_params;

/// 更新会议室预定限制请求
pub struct PatchReserveConfigRequest {
    config: Config,
    reserve_config_id: String,
}

impl PatchReserveConfigRequest {
    /// 创建请求实例。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch>
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

        // url: PATCH:/open-apis/vc/v1/reserve_configs/:reserve_config_id
        let api_endpoint = VcApiV1::ReserveConfigPatch(self.reserve_config_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(api_endpoint.to_url())
            .body(serialize_params(&body, "更新会议室预定限制")?);

        Transport::request_typed(req, &self.config, Some(option), "更新会议室预定限制").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../vc/v1/reserve_configs/{reserve_config_id} → serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_patch_reserve_config_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/reserve_configs/rc_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "scope_config_id": "rc_001", "updated": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PatchReserveConfigRequest::new(config)
            .reserve_config_id("rc_001")
            .execute(json!({ "scope_config_id": "rc_001" }))
            .await
            .expect("更新会议室预定限制应成功");
        assert_eq!(resp["scope_config_id"], "rc_001");
        assert_eq!(resp["updated"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/rc_001"
        );
    }
}
