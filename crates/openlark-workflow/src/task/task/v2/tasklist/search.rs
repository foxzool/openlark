//! 搜索清单
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist/search

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 搜索清单请求
pub struct SearchTasklistRequest {
    config: Arc<Config>,
}

impl SearchTasklistRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
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
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(TaskApiV2::TasklistSearch.to_url())
                .body(serialize_params(&body, "搜索清单")?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索清单")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = SearchTasklistRequest::new(config);
    }
}
