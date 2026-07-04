//! 删除 OKR 目标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective/get>

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

/// 删除 OKR 目标请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    objective_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            objective_id: String::new(),
        }
    }

    /// 设置路径参数 `objective_id`。
    pub fn objective_id(mut self, val: impl Into<String>) -> Self {
        self.objective_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteObjectiveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteObjectiveResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = format!("/open-apis/okr/v2/objectives/{}", self.objective_id);
        let req: ApiRequest<DeleteObjectiveResponse> = ApiRequest::delete(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除 OKR 目标", "响应数据为空"))
    }
}

/// 删除 OKR 目标响应。
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct DeleteObjectiveResponse {}

impl ApiResponseTrait for DeleteObjectiveResponse {
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
    fn test_delete_objective_response_deserialize() {
        let resp: DeleteObjectiveResponse =
            serde_json::from_value(serde_json::json!({})).expect("空响应反序列化失败");
        assert_eq!(resp, DeleteObjectiveResponse::default());
    }
}
