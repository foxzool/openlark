//! 获取所有 OKR 分类
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/category/list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 获取所有 OKR 分类请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = ApiRequest::get("/open-apis/okr/v2/categories");
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取所有 OKR 分类", "响应数据为空")
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
