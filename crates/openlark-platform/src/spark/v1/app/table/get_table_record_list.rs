//! 查询数据表数据记录
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 查询数据表数据记录请求。
#[derive(Debug, Clone)]
pub struct AppTableGetTableRecordListRequest {
    config: Arc<Config>,
    app_id: String,
    table_name: String,
}

impl AppTableGetTableRecordListRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
            table_name: String::new(),
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置路径参数 `table_name`。
    pub fn table_name(mut self, table_name: impl Into<String>) -> Self {
        self.table_name = table_name.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        let path = format!(
            "/open-apis/spark/v1/apps/{}/tables/{}/records",
            self.app_id, self.table_name
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        Transport::request_typed(req, &self.config, Some(option), "查询数据表数据记录").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppTableGetTableRecordListRequest::new(config);
    }
}
