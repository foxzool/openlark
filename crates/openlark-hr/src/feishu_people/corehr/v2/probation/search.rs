//! 搜索试用期信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 搜索试用期信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBody {
    /// 雇佣 ID 列表，可通过【搜索员工信息】接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_ids: Option<Vec<String>>,
    /// 部门 ID 列表，可通过【批量查询部门】接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 试用期开始日期 - 搜索范围开始，需要与搜索范围结束一同使用，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date_start: Option<String>,
    /// 试用期开始日期 - 搜索范围结束，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date_end: Option<String>,
    /// 试用期预计结束日期 - 搜索范围开始，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date_start: Option<String>,
    /// 试用期预计结束日期 - 搜索范围结束，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date_end: Option<String>,
    /// 试用期实际结束日期 - 搜索范围开始，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date_start: Option<String>,
    /// 试用期实际结束日期 - 搜索范围结束，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date_end: Option<String>,
    /// 转正发起日期 - 搜索范围开始，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_time_start: Option<String>,
    /// 转正发起日期 - 搜索范围结束，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_time_end: Option<String>,
    /// 试用期状态：pending/rejected/waiting/approved/converted/offboarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_status: Option<String>,
    /// 试用期最终考核结果：approved/rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_result: Option<String>,
    /// 试用期最终考核等级，枚举值 api_name 可通过【获取字段详情】接口查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_grade: Option<String>,
}

/// 搜索试用期信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: SearchBody,
    /// 分页大小（必填，取值范围 1～100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 部门 ID 类型
    department_id_type: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: SearchBody::default(),
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: SearchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置雇佣 ID 列表。
    pub fn employment_ids(mut self, employment_ids: Vec<String>) -> Self {
        self.body.employment_ids = Some(employment_ids);
        self
    }

    /// 设置部门 ID 列表。
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.body.department_ids = Some(department_ids);
        self
    }

    /// 设置试用期状态（pending/rejected/waiting/approved/converted/offboarded）。
    pub fn probation_status(mut self, probation_status: impl Into<String>) -> Self {
        self.body.probation_status = Some(probation_status.into());
        self
    }

    /// 设置试用期最终考核结果（approved/rejected）。
    pub fn final_assessment_result(mut self, final_assessment_result: impl Into<String>) -> Self {
        self.body.final_assessment_result = Some(final_assessment_result.into());
        self
    }

    /// 设置试用期最终考核等级。
    pub fn final_assessment_grade(mut self, final_assessment_grade: impl Into<String>) -> Self {
        self.body.final_assessment_grade = Some(final_assessment_grade.into());
        self
    }

    /// 设置分页大小（必填，取值范围 1～100）。
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

    /// 设置部门 ID 类型。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }

        let api_endpoint = FeishuPeopleApiV2::ProbationSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url())
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索试用期信息响应数据为空",
        )
        .await
    }
}

/// 搜索试用期信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的试用期信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ProbationItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 试用期信息条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbationItem {
    /// 雇佣 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 试用期信息 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_id: Option<String>,
    /// 试用期开始日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    /// 试用期实际结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
    /// 转正发起日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_time: Option<String>,
    /// 发起方
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_type: Option<EnumValue>,
    /// 转正发起人的雇佣 ID，当系统发起转正时该字段为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 试用期状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_status: Option<EnumValue>,
    /// 员工自评
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_review: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 流程实例 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 是否通过 BPM 转正
    #[serde(skip_serializing_if = "Option::is_none")]
    pub converted_via_bpm: Option<bool>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 试用期考核最终状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_status: Option<EnumValue>,
    /// 试用期考核最终结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_result: Option<EnumValue>,
    /// 试用期考核最终得分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_score: Option<f64>,
    /// 试用期考核最终等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_grade: Option<EnumValue>,
    /// 试用期考核最终评语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_comment: Option<String>,
    /// 最终考核结果页面超链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_assessment_detail: Option<String>,
    /// 试用期考核结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessments: Option<Vec<ProbationAssessment>>,
    /// 试用期延长后的预计结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_extend_expected_end_date: Option<String>,
    /// 试用期延长时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_probation_period_duration: Option<i64>,
    /// 试用期延长时间单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_probation_period_unit: Option<EnumValue>,
    /// 试用期结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_outcome: Option<EnumValue>,
}

/// 试用期考核结果条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbationAssessment {
    /// 考核结果 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    /// 考核状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<EnumValue>,
    /// 试用期考核结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_result: Option<EnumValue>,
    /// 考核得分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_score: Option<f64>,
    /// 试用期考核等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_grade: Option<EnumValue>,
    /// 考核评语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_comment: Option<String>,
    /// 考核结果页面超链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_detail: Option<String>,
    /// 是否为最终考核结果（官方字段名即此拼写）
    #[serde(rename = "is_final_asssessment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final_asssessment: Option<bool>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：{zh_cn, en_us}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 custom_field_data 结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/probation/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/probation/search"))
            .and(query_param("page_size", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "6893014062142064135",
                            "probation_id": "6893014062142064132",
                            "probation_start_date": "2022-05-20",
                            "probation_expected_end_date": "2022-05-28",
                            "actual_probation_end_date": "2022-06-28",
                            "initiating_time": "2022-07-28",
                            "submission_type": {
                                "enum_name": "self_submission",
                                "display": [{"lang": "zh-CN", "value": "员工发起"}]
                            },
                            "initiator_id": "6893014062142061135",
                            "probation_status": {
                                "enum_name": "converted",
                                "display": [{"lang": "zh-CN", "value": "已转正"}]
                            },
                            "self_review": "符合预期",
                            "notes": "高潜",
                            "process_id": "6893014062142164135",
                            "converted_via_bpm": false,
                            "custom_fields": [
                                {
                                    "custom_api_name": "name",
                                    "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            ],
                            "final_assessment_status": {
                                "enum_name": "completed",
                                "display": [{"lang": "zh-CN", "value": "已完成"}]
                            },
                            "final_assessment_result": {
                                "enum_name": "approved",
                                "display": [{"lang": "zh-CN", "value": "通过"}]
                            },
                            "final_assessment_score": 99.9,
                            "final_assessment_grade": {
                                "enum_name": "grade_a",
                                "display": [{"lang": "zh-CN", "value": "A"}]
                            },
                            "final_assessment_comment": "超出预期",
                            "final_assessment_detail": "https://example.feishuapp.cn/ae/records/1",
                            "assessments": [
                                {
                                    "assessment_id": "6893014061142064135",
                                    "assessment_status": {
                                        "enum_name": "completed",
                                        "display": [{"lang": "zh-CN", "value": "已完成"}]
                                    },
                                    "assessment_result": {
                                        "enum_name": "approved",
                                        "display": [{"lang": "zh-CN", "value": "通过"}]
                                    },
                                    "assessment_score": 99.9,
                                    "assessment_grade": {
                                        "enum_name": "grade_a",
                                        "display": [{"lang": "zh-CN", "value": "A"}]
                                    },
                                    "assessment_comment": "超出预期",
                                    "assessment_detail": "https://example.feishuapp.cn/ae/records/2",
                                    "is_final_asssessment": true
                                }
                            ],
                            "probation_extend_expected_end_date": "2022-06-20",
                            "extended_probation_period_duration": 1,
                            "extended_probation_period_unit": {
                                "enum_name": "month",
                                "display": [{"lang": "zh-CN", "value": "月"}]
                            },
                            "probation_outcome": {
                                "enum_name": "passed",
                                "display": [{"lang": "zh-CN", "value": "通过"}]
                            }
                        }
                    ],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more": true
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = SearchRequest::new(config)
            .page_size(100)
            .employment_ids(vec!["7140964208476371111".to_string()])
            .probation_status("approved")
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0]
                .probation_status
                .as_ref()
                .unwrap()
                .enum_name
                .as_deref(),
            Some("converted")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
