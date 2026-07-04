//! 获取目标下的所有关键结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.key_result/get>

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

use super::super::super::key_result::get::KeyResult;

/// 获取目标下的所有关键结果请求。
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
    pub async fn execute(self) -> SDKResult<ListObjectiveKeyResultResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListObjectiveKeyResultResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = format!(
            "/open-apis/okr/v2/objectives/{}/key_results",
            self.objective_id
        );
        let req: ApiRequest<ListObjectiveKeyResultResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标下的所有关键结果", "响应数据为空")
        })
    }
}

/// 获取目标下的所有关键结果响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectiveKeyResultResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 关键结果列表。
    #[serde(default)]
    pub items: Option<Vec<KeyResult>>,
}

impl ApiResponseTrait for ListObjectiveKeyResultResponse {
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
    fn test_list_objective_key_result_response_deserialize() {
        let json = serde_json::json!({
            "has_more": false,
            "page_token": "token-1",
            "items": [
                {
                    "id": "KR-1",
                    "create_time": "1700000000000",
                    "update_time": "1700000000000",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "objective_id": "O-123",
                    "position": 1,
                    "score": 0.8,
                    "weight": 0.5,
                    "deadline": "1700000000000"
                }
            ]
        });
        let resp: ListObjectiveKeyResultResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.has_more, Some(false));
        let items = resp.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "KR-1");
        assert_eq!(items[0].objective_id, "O-123");
        assert_eq!(items[0].position, 1);
    }
}
