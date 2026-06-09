//! 获取用户 OKR 周期内的目标
//!
//! docPath: https://open.feishu.cn/document/server-docs/okr-v2/cycle.objective/get

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取用户 OKR 周期内的目标请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    cycle_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            cycle_id: String::new(),
        }
    }

    /// 设置路径参数 `cycle_id`。
    pub fn cycle_id(mut self, val: impl Into<String>) -> Self {
        self.cycle_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.cycle_id, "cycle_id 不能为空");
        let path = format!("/open-apis/okr/v2/{}/objectives", self.cycle_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户 OKR 周期内的目标", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }
}
