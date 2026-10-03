//! 设置会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::serialize_params;

/// 设置会议室配置请求
pub struct CreateScopeConfigRequest {
    config: Config,
}

impl CreateScopeConfigRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/create>
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
        // url: POST:/open-apis/vc/v1/scope_config
        let api_endpoint = VcApiV1::ScopeConfigCreate;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "设置会议室配置")?);

        Transport::request_typed(req, &self.config, Some(option), "设置会议室配置").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateScopeConfigRequest::new(config.clone());
        let _ = request;
    }
}
