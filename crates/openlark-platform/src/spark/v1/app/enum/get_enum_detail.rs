//! 获取自定义枚举详细信息
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取自定义枚举详细信息请求。
#[derive(Debug, Clone)]
pub struct AppEnumGetEnumDetailRequest {
    config: Arc<Config>,
    app_id: String,
    enum_name: String,
}

impl AppEnumGetEnumDetailRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
            enum_name: String::new(),
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置路径参数 `enum_name`。
    pub fn enum_name(mut self, enum_name: impl Into<String>) -> Self {
        self.enum_name = enum_name.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.enum_name, "enum_name 不能为空");
        let path = format!(
            "/open-apis/spark/v1/apps/{}/enums/{}",
            self.app_id, self.enum_name
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取自定义枚举详细信息", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppEnumGetEnumDetailRequest::new(config);
    }
}
