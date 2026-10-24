//! 编辑试用期
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation/edit>

use super::search::CustomFieldData;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 编辑试用期请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditProbationBody {
    /// 试用期人员的雇佣 ID，类型与查询参数 user_id_type 取值一致
    pub employment_id: String,
    /// 试用期开始日期，格式："YYYY-MM-DD"（需与 probation_expected_end_date 一同填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    /// 试用期结果：passed 通过 / failed 未通过 / delayed 延期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_outcome: Option<String>,
    /// 试用期实际结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
    /// 延长后试用期预计结束日期，格式："YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_extend_expected_end_date: Option<String>,
    /// 试用期预计延长时长（取值范围 1～500）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_probation_period_duration: Option<i64>,
    /// 试用期延长时长单位：day 天 / week 周 / month 月
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_probation_period_unit: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 员工自评
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_review: Option<String>,
    /// 自定义字段（当前不支持附件类型字段写入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 编辑试用的响应（官方 data 为空对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EditProbationResponse {}

impl ApiResponseTrait for EditProbationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑试用的请求。
#[derive(Debug, Clone)]
pub struct EditProbationRequest {
    config: Arc<Config>,
    /// 请求体
    body: EditProbationBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

/// 编辑试用期请求构建器实现。
impl EditProbationRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: EditProbationBody::default(),
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: EditProbationBody) -> Self {
        self.body = body;
        self
    }

    /// 设置试用期人员的雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置试用期开始日期（需与试用期预计结束日期一同填写）。
    pub fn probation_start_date(mut self, probation_start_date: impl Into<String>) -> Self {
        self.body.probation_start_date = Some(probation_start_date.into());
        self
    }

    /// 设置试用期预计结束日期。
    pub fn probation_expected_end_date(
        mut self,
        probation_expected_end_date: impl Into<String>,
    ) -> Self {
        self.body.probation_expected_end_date = Some(probation_expected_end_date.into());
        self
    }

    /// 设置试用期结果（passed/failed/delayed）。
    pub fn probation_outcome(mut self, probation_outcome: impl Into<String>) -> Self {
        self.body.probation_outcome = Some(probation_outcome.into());
        self
    }

    /// 设置试用期实际结束日期。
    pub fn actual_probation_end_date(
        mut self,
        actual_probation_end_date: impl Into<String>,
    ) -> Self {
        self.body.actual_probation_end_date = Some(actual_probation_end_date.into());
        self
    }

    /// 设置延长后试用期预计结束日期。
    pub fn probation_extend_expected_end_date(
        mut self,
        probation_extend_expected_end_date: impl Into<String>,
    ) -> Self {
        self.body.probation_extend_expected_end_date =
            Some(probation_extend_expected_end_date.into());
        self
    }

    /// 设置试用期预计延长时长（1～500）。
    pub fn extended_probation_period_duration(
        mut self,
        extended_probation_period_duration: i64,
    ) -> Self {
        self.body.extended_probation_period_duration = Some(extended_probation_period_duration);
        self
    }

    /// 设置试用期延长时长单位（day/week/month）。
    pub fn extended_probation_period_unit(
        mut self,
        extended_probation_period_unit: impl Into<String>,
    ) -> Self {
        self.body.extended_probation_period_unit = Some(extended_probation_period_unit.into());
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

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行编辑试用期请求。
    pub async fn execute(self) -> SDKResult<EditProbationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EditProbationResponse> {
        validate_required!(self.body.employment_id, "employment_id 不能为空");

        let request =
            ApiRequest::<EditProbationResponse>::post("/open-apis/corehr/v2/probation/edit")
                .query_opt("user_id_type", self.user_id_type)
                .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "编辑试用期").await
    }
}
