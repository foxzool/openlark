//! 上传妙搭应用图标
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 上传妙搭应用图标请求。
#[derive(Debug, Clone)]
pub struct UploadSparkAppIconRequest {
    config: Arc<Config>,
}

impl UploadSparkAppIconRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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
        let req = ApiRequest::<serde_json::Value>::post("/open-apis/spark/v1/icon").body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("上传妙搭应用图标", "响应数据为空")
        })
    }
}
