//! 删除关键结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/key_result/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 删除关键结果请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    key_result_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            key_result_id: String::new(),
        }
    }

    /// 设置路径参数 `key_result_id`。
    pub fn key_result_id(mut self, val: impl Into<String>) -> Self {
        self.key_result_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteKeyResultResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteKeyResultResponse> {
        validate_required!(self.key_result_id, "key_result_id 不能为空");
        let path = format!("/open-apis/okr/v2/key_results/{}", self.key_result_id);
        let req: ApiRequest<DeleteKeyResultResponse> = ApiRequest::delete(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除关键结果", "响应数据为空"))
    }
}

/// 删除关键结果响应。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteKeyResultResponse {}

impl ApiResponseTrait for DeleteKeyResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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

    #[test]
    fn test_delete_key_result_response_deserialize() {
        let json = serde_json::json!({});
        let resp: DeleteKeyResultResponse = serde_json::from_value(json).expect("反序列化失败");
        let _ = resp;
    }
}
