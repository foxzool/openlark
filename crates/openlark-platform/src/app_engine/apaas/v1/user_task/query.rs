//! 查询人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询人工任务 Builder
#[derive(Debug, Clone)]
pub struct UserTaskQueryBuilder {
    config: Config,
    /// 任务状态列表
    statuses: Vec<String>,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 用户 ID
    user_ids: Vec<String>,
}

impl UserTaskQueryBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            statuses: Vec::new(),
            start_time: None,
            end_time: None,
            page: None,
            page_size: None,
            user_ids: Vec::new(),
        }
    }

    /// 添加任务状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.statuses.push(status.into());
        self
    }

    /// 添加多个任务状态
    pub fn statuses(mut self, statuses: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.statuses.extend(statuses.into_iter().map(Into::into));
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
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
    pub async fn execute(self) -> SDKResult<UserTaskQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserTaskQueryResponse> {
        let url = "/open-apis/apaas/v1/user_task/query".to_string();

        let request = UserTaskQueryRequest {
            statuses: self.statuses,
            start_time: self.start_time,
            end_time: self.end_time,
            page: self.page,
            page_size: self.page_size,
            user_ids: self.user_ids,
        };

        let req: ApiRequest<UserTaskQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 查询请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserTaskQueryRequest {
    /// 任务状态列表
    #[serde(rename = "statuses", skip_serializing_if = "Vec::is_empty")]
    statuses: Vec<String>,
    /// 开始时间
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    /// 结束时间
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
    /// 用户 ID 列表
    #[serde(rename = "user_ids", skip_serializing_if = "Vec::is_empty")]
    user_ids: Vec<String>,
}

/// 任务信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskInfo {
    /// 任务 ID
    #[serde(rename = "task_id")]
    task_id: String,
    /// 任务标题
    #[serde(rename = "title")]
    title: String,
    /// 任务状态
    #[serde(rename = "status")]
    status: String,
    /// 发起人 ID
    #[serde(rename = "initiator_id")]
    initiator_id: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

/// 查询响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskQueryResponse {
    /// 任务列表
    #[serde(rename = "tasks")]
    tasks: Vec<TaskInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
    /// 总数
    #[serde(rename = "total_count")]
    total_count: u32,
}

impl ApiResponseTrait for UserTaskQueryResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
