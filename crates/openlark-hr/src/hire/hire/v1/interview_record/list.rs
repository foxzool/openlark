//! 批量获取面试评价详细信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/interview_record/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::{I18nText, IdNameObject};

/// `ListRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 批量获取面试评价详细信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        if let Some(size) = self.page_size
            && !(1..=100).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-100 之间",
            ));
        }

        let api_endpoint = HireApiV1::InterviewRecordList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量获取面试评价详细信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量获取面试评价详细信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewLevelScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `level` 字段。
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_description` 字段。
    pub zh_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_description` 字段。
    pub en_description: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewAssessmentScore`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAssessmentScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `calculate_type` 字段。
    pub calculate_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `full_score` 字段。
    pub full_score: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewAbility`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAbility {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewQuestion`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `ability_list` 字段。
    pub ability_list: Option<Vec<InterviewAbility>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewRecordImage`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mime` 字段。
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InterviewRecordDimensionAssessment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordDimensionAssessment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<f64>,
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
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `commit_status` 字段。
    pub commit_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `feedback_submit_time` 字段。
    pub feedback_submit_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `conclusion` 字段。
    pub conclusion: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_score` 字段。
    pub interview_score: Option<InterviewLevelScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `assessment_score` 字段。
    pub assessment_score: Option<InterviewAssessmentScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `question_list` 字段。
    pub question_list: Option<Vec<InterviewQuestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code_question_list` 字段。
    pub code_question_list: Option<Vec<InterviewQuestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `image_list` 字段。
    pub image_list: Option<Vec<InterviewRecordImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `dimension_assessment_list` 字段。
    pub dimension_assessment_list: Option<Vec<InterviewRecordDimensionAssessment>>,
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
