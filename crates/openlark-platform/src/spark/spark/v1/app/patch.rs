//! 更新妙搭应用信息
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 更新妙搭应用信息请求。
#[derive(Debug, Clone)]
pub struct PatchSparkAppRequest {
    config: Arc<Config>,
    app_id: String,
}

impl PatchSparkAppRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id.trim(), "app_id 不能为空");
        let path = format!("/open-apis/spark/v1/apps/{}", self.app_id);
        let req = ApiRequest::<serde_json::Value>::patch(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("更新妙搭应用信息", "响应数据为空")
        })
    }
}
