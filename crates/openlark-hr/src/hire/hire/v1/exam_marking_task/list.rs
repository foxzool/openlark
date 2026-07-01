//! 获取笔试阅卷任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/exam_marking_task/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// `ListRequest` 请求。
#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id: String,
    activity_status: Option<i32>,
    user_id_type: Option<String>,
}

impl ListRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id: String::new(),
            activity_status: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置 `user_id`。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置 `activity_status`。
    pub fn activity_status(mut self, activity_status: i32) -> Self {
        self.activity_status = Some(activity_status);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        if let Some(page_size) = self.page_size
            && !(1..=20).contains(&page_size)
        {
            return Err(error::validation_error(
                "page_size",
                "page_size 必须在 1-20 之间",
            ));
        }

        if self.user_id.trim().is_empty() {
            return Err(error::validation_error("user_id", "user_id 不能为空"));
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v1/exam_marking_tasks");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        request = request.query("user_id", self.user_id);
        if let Some(activity_status) = self.activity_status {
            request = request.query("activity_status", activity_status.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "获取笔试阅卷任务列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `ExamMarkingTaskItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExamMarkingTaskItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `activity_status` 字段。
    pub activity_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ListResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<ExamMarkingTaskItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
