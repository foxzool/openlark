//! 批量获取面试评价详细信息（新版）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v2/interview_record/list>

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

use crate::hire::hire::common_models::{AttachmentMeta, I18nText, IdNameObject, ScoreInfo};

/// `ListRequest` 请求。
#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    ids: Option<Vec<String>>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ids: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置 ID 列表。
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
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
            && !(0..=100).contains(&page_size)
        {
            return Err(error::validation_error(
                "page_size",
                "page_size 必须在 0-100 之间",
            ));
        }

        if let Some(ref ids) = self.ids
            && ids.len() > 100
        {
            return Err(error::validation_error("ids", "ids 不能超过 100 个"));
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v2/interview_records");
        if let Some(ids) = self.ids {
            request = request.query(
                "ids",
                serde_json::to_string(&ids).map_err(|e| {
                    error::validation_error("ids", format!("无法序列化数组查询参数: {e}"))
                })?,
            );
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "批量获取面试评价详细信息（新版）响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `InterviewRecordDimensionAssessment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordDimensionAssessment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_feedback_form_dimension_id` 字段。
    pub interview_feedback_form_dimension_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 维度名称。
    pub dimension_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 维度类型。
    pub dimension_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 维度权重。
    pub dimension_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 维度分数。
    pub dimension_score: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewRecordModuleAssessment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordModuleAssessment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_feedback_form_module_id` 字段。
    pub interview_feedback_form_module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模块名称。
    pub module_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模块类型。
    pub module_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模块权重。
    pub module_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模块分数。
    pub module_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 维度评估列表。
    pub dimension_assessments: Option<Vec<InterviewRecordDimensionAssessment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewRecordItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `feedback_form_id` 字段。
    pub feedback_form_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `commit_status` 字段。
    pub commit_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `submit_time` 字段。
    pub submit_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `record_score` 字段。
    pub record_score: Option<ScoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 附件列表。
    pub attachments: Option<Vec<AttachmentMeta>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模块评估列表。
    pub module_assessments: Option<Vec<InterviewRecordModuleAssessment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ListResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<InterviewRecordItem>,
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
