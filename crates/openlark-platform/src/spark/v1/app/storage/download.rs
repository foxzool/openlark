//! 下载文件
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 下载文件请求。
#[derive(Debug, Clone)]
pub struct AppStorageDownloadRequest {
    config: Arc<Config>,
    app_id: String,
}

impl AppStorageDownloadRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        let path = format!("/open-apis/spark/v1/apps/{}/storage", self.app_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        Transport::request_typed(req, &self.config, Some(option), "下载文件").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppStorageDownloadRequest::new(config);
    }
}
