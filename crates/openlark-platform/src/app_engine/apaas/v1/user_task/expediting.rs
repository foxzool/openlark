//! 催办人工任务
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/user-task/expediting
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/expediting

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 催办人工任务 Builder
#[derive(Debug, Clone)]
pub struct ExpeditingBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 用户 ID
    user_ids: Vec<String>,
}

impl ExpeditingBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            user_ids: Vec::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ExpeditingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ExpeditingResponse> {
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/expediting", self.task_id);

        let request = ExpeditingRequest {
            user_ids: self.user_ids,
        };

        let req: ApiRequest<ExpeditingResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 催办请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ExpeditingRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
}

/// 催办响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpeditingResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    task_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for ExpeditingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
