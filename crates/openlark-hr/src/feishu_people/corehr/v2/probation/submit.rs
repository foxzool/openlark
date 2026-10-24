//! 发起转正
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation/submit>

use super::search::{CustomFieldData, EnumValue};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 发起转正请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBody {
    /// 试用期人员的雇佣 ID，可通过【搜索员工信息】接口获取
    pub employment_id: String,
    /// 转正方式：1 直接转正 / 2 发起转正
    pub conversion_mode: i32,
    /// 实际结束日期，如果为空则默认填入试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
    /// 发起方：self_submission 员工 / system 系统 / hr_submission HR
    pub submission_type: String,
    /// 发起人 ID，当发起方为 HR 时填写，为其他发起方时该字段会自动计算
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 备注，当为直接转正时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 员工自评
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_review: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 发起转正请求
#[derive(Debug, Clone)]
pub struct SubmitRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: SubmitBody,
    /// 用于幂等控制的 client_token
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl SubmitRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: SubmitBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: SubmitBody) -> Self {
        self.body = body;
        self
    }

    /// 设置试用期人员的雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置转正方式（1 直接转正 / 2 发起转正）。
    pub fn conversion_mode(mut self, conversion_mode: i32) -> Self {
        self.body.conversion_mode = conversion_mode;
        self
    }

    /// 设置实际结束日期，格式："YYYY-MM-DD"。
    pub fn actual_probation_end_date(
        mut self,
        actual_probation_end_date: impl Into<String>,
    ) -> Self {
        self.body.actual_probation_end_date = Some(actual_probation_end_date.into());
        self
    }

    /// 设置发起方（self_submission/system/hr_submission）。
    pub fn submission_type(mut self, submission_type: impl Into<String>) -> Self {
        self.body.submission_type = submission_type.into();
        self
    }

    /// 设置发起人 ID。
    pub fn initiator_id(mut self, initiator_id: impl Into<String>) -> Self {
        self.body.initiator_id = Some(initiator_id.into());
        self
    }

    /// 设置备注。
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.body.notes = Some(notes.into());
        self
    }

    /// 设置员工自评。
    pub fn self_review(mut self, self_review: impl Into<String>) -> Self {
        self.body.self_review = Some(self_review.into());
        self
    }

    /// 设置自定义字段。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置用于幂等控制的 client_token。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubmitResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubmitResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        validate_required!(self.body.employment_id, "employment_id 不能为空");
        validate_required!(self.body.submission_type, "submission_type 不能为空");

        let api_endpoint = FeishuPeopleApiV2::ProbationSubmit;
        let request = ApiRequest::<SubmitResponse>::post(api_endpoint.to_url())
            .query_opt("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type)
            .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "发起转正响应数据为空").await
    }
}

/// 发起转正响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitResponse {
    /// 试用期信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_info: Option<ProbationInfoForSubmit>,
}

/// 发起转正返回的试用期信息（probation_info_for_submit，按官方 20 个字段建模）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbationInfoForSubmit {
    /// 雇佣 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 试用期信息 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_id: Option<String>,
    /// 试用期开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期预计结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    /// 试用期实际结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
    /// 转正发起日期
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
}

impl ApiResponseTrait for SubmitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/probation/submit
    #[tokio::test]
    async fn test_submit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/probation/submit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "probation_info": {
                        "employment_id": "6893014062142064135",
                        "probation_id": "6893014062142064132",
                        "probation_start_date": "2022-05-20",
                        "probation_expected_end_date": "2022-05-28",
                        "actual_probation_end_date": "2022-06-28",
                        "initiating_time": "2022-07-28",
                        "submission_type": {
                            "enum_name": "system",
                            "display": [{"lang": "zh-CN", "value": "系统发起"}]
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
                        "custom_fields": [],
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
                        "final_assessment_detail": "暂无示例"
                    }
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

        let resp = SubmitRequest::new(config)
            .employment_id("7140964208476371111")
            .conversion_mode(1)
            .submission_type("system")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(
            resp.probation_info.unwrap().probation_id.unwrap(),
            "6893014062142064132"
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
