//! 查询会议室预定表单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-2>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::endpoints::VC_V1_RESERVE_CONFIGS;

/// 查询会议室预定表单请求
pub struct GetReserveConfigFormRequest {
    config: Config,
    reserve_config_id: String,
    query_params: Vec<(String, String)>,
}

impl GetReserveConfigFormRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-2>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserve_configs/:reserve_config_id/form
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/form",
            VC_V1_RESERVE_CONFIGS, self.reserve_config_id
        ));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "查询会议室预定表单").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserve_configs/{reserve_config_id}/form → serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_reserve_config_form_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reserve_configs/rc_001/form"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "form_id": "f_001", "field_count": 3 }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetReserveConfigFormRequest::new(config)
            .reserve_config_id("rc_001")
            .execute()
            .await
            .expect("查询会议室预定表单应成功");
        assert_eq!(resp["form_id"], "f_001");
        assert_eq!(resp["field_count"], 3);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/rc_001/form"
        );
    }
}
