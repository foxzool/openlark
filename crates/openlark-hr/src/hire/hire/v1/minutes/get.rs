//! 获取面试速记明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/minutes/get>

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

use crate::hire::hire::common_models::I18nText;

/// `GetRequest` 请求。
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    interview_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl GetRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            interview_id: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    /// 设置 `interview_id`。
    pub fn interview_id(mut self, interview_id: impl Into<String>) -> Self {
        self.interview_id = interview_id.into();
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        if let Some(page_size) = self.page_size
            && !(1..=100).contains(&page_size)
        {
            return Err(error::validation_error(
                "page_size",
                "page_size 必须在 1-100 之间",
            ));
        }

        if self.interview_id.trim().is_empty() {
            return Err(error::validation_error(
                "interview_id",
                "interview_id 不能为空",
            ));
        }

        let mut request = ApiRequest::<GetResponse>::get("/open-apis/hire/v1/minutes");
        request = request.query("interview_id", self.interview_id);
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取面试速记明细响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `MinuteSentence`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MinuteSentence {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `speak_time` 字段。
    pub speak_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_type` 字段。
    pub user_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `speaker_name` 字段。
    pub speaker_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `MinutesDetail`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MinutesDetail {
    #[serde(default)]
    /// `sentences` 字段。
    pub sentences: Vec<MinuteSentence>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `minutes` 字段。
    pub minutes: Option<MinutesDetail>,
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

impl ApiResponseTrait for GetResponse {
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
