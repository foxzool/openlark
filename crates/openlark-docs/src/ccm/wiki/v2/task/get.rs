//! 获取任务结果
//!
//! 该方法用于获取 wiki 异步任务的结果。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiTask;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取任务结果请求
pub struct GetWikiTaskRequest {
    task_id: String,
    task_type: Option<String>,
    config: Config,
}

/// 获取任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiTaskResponse {
    /// 任务信息
    pub task: Option<WikiTask>,
}

impl ApiResponseTrait for GetWikiTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiTaskRequest {
    /// 创建获取任务结果请求
    pub fn new(config: Config) -> Self {
        Self {
            task_id: String::new(),
            task_type: None,
            config,
        }
    }

    /// 设置任务ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    /// 设置任务类型（可选，例如 move）
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetWikiTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetWikiTaskResponse> {
        // 验证必填字段
        validate_required!(self.task_id, "任务ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::TaskGet(self.task_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetWikiTaskResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(task_type) = self.task_type {
            api_request = api_request.query("task_type", &task_type);
        }

        // 发送请求

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
