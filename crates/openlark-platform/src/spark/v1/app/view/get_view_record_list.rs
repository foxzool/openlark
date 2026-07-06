//! 查询视图数据记录
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 查询视图数据记录请求。
#[derive(Debug, Clone)]
pub struct AppViewGetViewRecordListRequest {
    config: Arc<Config>,
    app_id: String,
    view_name: String,
}

impl AppViewGetViewRecordListRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
            view_name: String::new(),
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置路径参数 `view_name`。
    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.view_name = view_name.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.view_name, "view_name 不能为空");
        let path = format!(
            "/open-apis/spark/v1/apps/{}/views/{}/records",
            self.app_id, self.view_name
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询视图数据记录", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppViewGetViewRecordListRequest::new(config);
    }
}
