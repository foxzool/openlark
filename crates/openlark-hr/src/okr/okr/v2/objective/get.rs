//! 获取目标详细信息
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

use crate::common::api_endpoints::OkrApiV2;
use crate::okr::okr::v2::common::models::Objective;

/// 获取目标详细信息请求。
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
    pub async fn execute(self) -> SDKResult<GetObjectiveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetObjectiveResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = OkrApiV2::ObjectiveGet(self.objective_id).to_url();
        let req: ApiRequest<GetObjectiveResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标详细信息", "响应数据为空")
        })
    }
}

/// 获取目标详细信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse {
    /// 目标详情。
    pub objective: Objective,
}

impl ApiResponseTrait for GetObjectiveResponse {
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
    fn test_get_objective_response_deserialize() {
        let json = serde_json::json!({
            "objective": {
                "id": "O-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "cycle_id": "C-1",
                "position": 1,
                "score": 0.8,
                "weight": 0.5,
                "deadline": "1700000000000",
                "category_id": "cat-1"
            }
        });
        let resp: GetObjectiveResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.objective.id, "O-123");
        assert_eq!(resp.objective.position, 1);
        assert_eq!(resp.objective.owner.owner_type, "user");
        assert_eq!(resp.objective.score, Some(0.8));
        assert!(resp.objective.content.is_none());
    }
}
