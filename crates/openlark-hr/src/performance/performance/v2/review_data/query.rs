//! 获取绩效详情数据
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/review_data/query>

use crate::common::shared_models::I18nText;
use crate::performance::performance::v2::models::UserId;
use openlark_core::validate_required_list;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取绩效详情数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 评估周期 ID 列表（必填，≤10）
    semester_ids: Vec<String>,
    /// 被评估人 ID 列表（必填，≤10）
    reviewee_user_ids: Vec<String>,
    /// 环节类型（可选，≤50；与 stage_ids 至少传一个，不传默认不返回环节评估数据）
    stage_types: Option<Vec<String>>,
    /// 评估型环节的执行人角色（可选）
    review_stage_roles: Option<Vec<String>>,
    /// 环节 ID（可选，≤50）
    stage_ids: Option<Vec<String>>,
    /// 是否返回绩效终评数据的具体环节来源（可选）
    need_leader_review_data_source: Option<bool>,
    /// 环节更新时间最早时间，毫秒时间戳（可选）
    updated_later_than: Option<String>,
    /// 环节状态（可选，≤50）
    stage_progresses: Option<Vec<i32>>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            semester_ids: Vec::new(),
            reviewee_user_ids: Vec::new(),
            stage_types: None,
            review_stage_roles: None,
            stage_ids: None,
            need_leader_review_data_source: None,
            updated_later_than: None,
            stage_progresses: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置评估周期 ID 列表（必填，≤10）
    pub fn semester_ids(mut self, semester_ids: Vec<String>) -> Self {
        self.semester_ids = semester_ids;
        self
    }

    /// 追加一个评估周期 ID
    pub fn add_semester_id(mut self, semester_id: String) -> Self {
        self.semester_ids.push(semester_id);
        self
    }

    /// 设置被评估人 ID 列表（必填，≤10）
    pub fn reviewee_user_ids(mut self, reviewee_user_ids: Vec<String>) -> Self {
        self.reviewee_user_ids = reviewee_user_ids;
        self
    }

    /// 追加一个被评估人 ID
    pub fn add_reviewee_user_id(mut self, reviewee_user_id: String) -> Self {
        self.reviewee_user_ids.push(reviewee_user_id);
        self
    }

    /// 设置环节类型列表（可选，≤50）
    pub fn stage_types(mut self, stage_types: Vec<String>) -> Self {
        self.stage_types = Some(stage_types);
        self
    }

    /// 设置评估型环节的执行人角色列表（可选）
    pub fn review_stage_roles(mut self, review_stage_roles: Vec<String>) -> Self {
        self.review_stage_roles = Some(review_stage_roles);
        self
    }

    /// 设置环节 ID 列表（可选，≤50）
    pub fn stage_ids(mut self, stage_ids: Vec<String>) -> Self {
        self.stage_ids = Some(stage_ids);
        self
    }

    /// 设置是否返回绩效终评数据的具体环节来源（可选）
    pub fn need_leader_review_data_source(mut self, need: bool) -> Self {
        self.need_leader_review_data_source = Some(need);
        self
    }

    /// 设置环节更新时间最早时间，毫秒时间戳（可选）
    pub fn updated_later_than(mut self, updated_later_than: String) -> Self {
        self.updated_later_than = Some(updated_later_than);
        self
    }

    /// 设置环节状态列表（可选，≤50）
    pub fn stage_progresses(mut self, stage_progresses: Vec<i32>) -> Self {
        self.stage_progresses = Some(stage_progresses);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required_list!(
            self.semester_ids,
            10,
            "semester_ids 不能为空且不能超过 10 个"
        );
        validate_required_list!(
            self.reviewee_user_ids,
            10,
            "reviewee_user_ids 不能为空且不能超过 10 个"
        );

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::ReviewDataQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            semester_ids: self.semester_ids,
            reviewee_user_ids: self.reviewee_user_ids,
            stage_types: self.stage_types,
            review_stage_roles: self.review_stage_roles,
            stage_ids: self.stage_ids,
            need_leader_review_data_source: self.need_leader_review_data_source,
            updated_later_than: self.updated_later_than,
            stage_progresses: self.stage_progresses,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取绩效详情数据响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 评估周期 ID 列表
    pub semester_ids: Vec<String>,
    /// 被评估人 ID 列表
    pub reviewee_user_ids: Vec<String>,
    /// 环节类型列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_types: Option<Vec<String>>,
    /// 评估型环节的执行人角色列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_stage_roles: Option<Vec<String>>,
    /// 环节 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_ids: Option<Vec<String>>,
    /// 是否返回绩效终评数据的具体环节来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_leader_review_data_source: Option<bool>,
    /// 环节更新时间最早时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_later_than: Option<String>,
    /// 环节状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progresses: Option<Vec<i32>>,
}

/// 获取绩效详情数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 评估数据列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datas: Option<Vec<ReviewProfile>>,
}

/// 评估数据（官方 review_profile 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewProfile {
    /// 被评估人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    /// 绩效评估周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 绩效评估项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// 被评估人在该周期对应的后台评估模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_template_id: Option<String>,
    /// 本周期内各环节内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<ReviewStage>>,
}

/// 环节内容（官方 review_stage 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewStage {
    /// 环节 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    /// 环节类型：summarize_key_outputs / review / communication_and_open_result / view_result / reconsideration / leader_review
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 该环节对应的环节模板的 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 评估内容记录
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<ReviewRecord>>,
    /// 评估型环节的执行人角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_role: Option<String>,
}

/// 评估内容记录（官方 review_record 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewRecord {
    /// 评估人的环节状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 评估记录中的评估内容明细
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<ReviewUnit>>,
    /// 360° 评估记录的信息
    ///
    /// 注：官方字段表将该结构列于评估题（data）之下，响应示例却嵌于
    /// 评估记录（records）之下，两处均建模以兼容两种真实形状。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_review_record_info: Option<InvitedReviewRecordInfo>,
    /// 合作项目中上级的评估记录信息（仅「项目直属上级环节」有值）
    ///
    /// 注：同 `invited_review_record_info`，字段表与示例的嵌套位置不一致，两处均建模。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_project_leader_record_info: Option<DirectProjectLeaderRecordInfo>,
    /// 评估记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

/// 评估内容明细（官方 review_unit 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewUnit {
    /// 评估内容 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 是否为不了解（评估人选不了解时为 true）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unknown: Option<bool>,
    /// 评估题列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ReviewDetail>>,
}

/// 评估题（官方 review_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewDetail {
    /// 评估题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 评估人 ID（360 匿名评估对全部查看者匿名时不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_user_id: Option<UserId>,
    /// 该评估题的最后提交时间，毫秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 评估项 ID（option_id 或 score 有值时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 评估等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// 评分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 填写项填写的文本
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 标签填写题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_based_question_id: Option<String>,
    /// 标签填写项的内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_text_item_data: Option<Vec<TagText>>,
    /// 绩效系数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf_coefficient_value: Option<String>,
    /// 子评估项内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_indicator_data: Option<Vec<SubIndicator>>,
    /// 评估的目标数据（对 O/KR 评估时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_data: Option<Vec<ObjectiveData>>,
    /// 评估的指标（对指标评估时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricData>>,
    /// 终评环节填写内容的来源：review / calibaration / reconsideration（仅终评环节有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_review_data_source: Option<String>,
    /// 工作/总结类型的文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_texts: Option<Vec<String>>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
    /// 富文本格式的填写内容（多条）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_richtexts: Option<Vec<String>>,
    /// 该评估题是否是首要评估项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_principal_review_item: Option<bool>,
    /// 360° 评估记录的信息
    ///
    /// 注：官方字段表将该结构列于评估题之下，响应示例却嵌于评估记录
    /// （records）之下，两处均建模以兼容两种真实形状。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_review_record_info: Option<InvitedReviewRecordInfo>,
    /// 合作项目中上级的评估记录信息（仅「项目直属上级环节」有值）
    ///
    /// 注：同 `invited_review_record_info`，字段表与示例的嵌套位置不一致，两处均建模。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_project_leader_record_info: Option<DirectProjectLeaderRecordInfo>,
}

/// 标签填写项内容（官方 tag_text 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagText {
    /// 标签 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_text_id: Option<String>,
    /// 评估人在该标签下填写的文本
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_text: Option<String>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_richtext: Option<String>,
}

/// 子评估项内容（官方 sub_indicator 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SubIndicator {
    /// 子评估题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 子评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 子评估项的评估等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// 子评估项的评分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 子评估项填写的文本
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

/// 目标数据（官方 objective_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ObjectiveData {
    /// 目标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,
    /// 目标的评分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 评估人在该填写项填写的文本
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 评估的关键举措
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyresult_data: Option<Vec<KeyresultData>>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

/// 关键举措数据（官方 keyresult_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct KeyresultData {
    /// 关键举措 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyresult_id: Option<String>,
    /// 关键举措的评分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 该关键举措的填写项内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

/// 指标数据（官方 metric_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricData {
    /// 指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 指标评分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
}

/// 360° 评估记录信息（官方 invited_review_record_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InvitedReviewRecordInfo {
    /// 评估人 ID（360 匿名评估对全部查看者匿名时不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<UserId>,
    /// 是否拒绝
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_rejected: Option<bool>,
    /// 360° 评估人拒绝评估的理由
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_reason: Option<String>,
    /// 360° 评估人的评估尺度标签：1 严格 / 2 适中 / 3 宽松
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribute_type: Option<i32>,
    /// 360° 评估人的评估尺度数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_diff: Option<String>,
    /// 360° 评估人的与被评估人关系
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_with_reviewee: Option<String>,
    /// 360° 评估人的邀请人类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitedby: Option<String>,
}

/// 合作项目中上级的评估记录信息（官方 direct_project_leader_record_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DirectProjectLeaderRecordInfo {
    /// 评估人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<UserId>,
    /// 评估人作为直属项目上级所在的项目
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooperation_projects: Option<Vec<CooperationProject>>,
    /// 评估依据的项目
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_depend_projects: Option<Vec<CooperationProject>>,
    /// 共同参与的项目
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participated_projects: Option<Vec<CooperationProject>>,
}

/// 合作项目（官方 cooperation_project 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CooperationProject {
    /// 合作项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 合作项目的名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 项目角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<CooperationRole>>,
    /// 评估人项目角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_roles: Option<Vec<CooperationUserRole>>,
    /// 被评估人项目角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underling_roles: Option<Vec<CooperationUserRole>>,
}

/// 项目角色（官方 cooperation_role 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CooperationRole {
    /// 评估人在合作项目中的角色（未配置时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_role: Option<CooperationUserRole>,
    /// 被评估人在合作项目中的角色（未配置时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_role: Option<CooperationUserRole>,
}

/// 合作项目角色（官方 cooperation_user_role 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CooperationUserRole {
    /// 角色 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_review_data_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（datas 容器 + stages/records/units/data 深层嵌套）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "datas": [
                    {
                        "user_id": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" },
                        "semester_id": "7343513161666707459",
                        "activity_id": "7343513161666707459",
                        "review_template_id": "7343513161666707459",
                        "stages": [
                            {
                                "stage_id": "7343513161666707459",
                                "stage_type": "review",
                                "template_id": "7343513161666707459",
                                "records": [
                                    {
                                        "progress": 1,
                                        "units": [
                                            {
                                                "unit_id": "7343513161666707459",
                                                "data": [
                                                    {
                                                        "field_id": "7343513161666707459",
                                                        "reviewer_user_id": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" },
                                                        "submit_time": "7343513161666707459",
                                                        "indicator_id": "7343513161666707459",
                                                        "option_id": "7343513161666707459",
                                                        "score": "1.1",
                                                        "text": "qwert",
                                                        "tag_based_question_id": "7343513161666707459",
                                                        "tag_text_item_data": [
                                                            {
                                                                "tag_text_id": "7343513161666707459",
                                                                "tag_text": "qwertyu",
                                                                "tag_richtext": "{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}"
                                                            }
                                                        ],
                                                        "perf_coefficient_value": "1.1",
                                                        "sub_indicator_data": [
                                                            {
                                                                "field_id": "7343513161666707459",
                                                                "indicator_id": "7343513161666707459",
                                                                "option_id": "7343513161666707459",
                                                                "score": "1.1",
                                                                "text": "qwertyuiop",
                                                                "richtext": "{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}"
                                                            }
                                                        ],
                                                        "objective_data": [
                                                            {
                                                                "objective_id": "7343513161666707459",
                                                                "score": "1.1",
                                                                "text": "qwertyu",
                                                                "keyresult_data": [
                                                                    {
                                                                        "keyresult_id": "7343513161666707459",
                                                                        "score": "1.1",
                                                                        "text": "qwerty",
                                                                        "richtext": "{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}"
                                                                    }
                                                                ],
                                                                "richtext": "{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}"
                                                            }
                                                        ],
                                                        "metric_data": [
                                                            { "id": "7343513161666707459", "score": "1.1" }
                                                        ],
                                                        "leader_review_data_source": "review",
                                                        "multi_texts": ["qwerty"],
                                                        "richtext": "{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}",
                                                        "multi_richtexts": ["{\"ops\":[{\"name\":\"insert\",\"text\":\"qwerty\"}]}"],
                                                        "is_principal_review_item": true
                                                    }
                                                ]
                                            }
                                        ],
                                        "invited_review_record_info": {
                                            "reviewer_id": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" },
                                            "is_rejected": false,
                                            "rejected_reason": "test",
                                            "distribute_type": 1,
                                            "avg_diff": "1.23",
                                            "relationship_with_reviewee": "direct_report",
                                            "invitedby": "system_default"
                                        },
                                        "direct_project_leader_record_info": {
                                            "reviewer_id": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" },
                                            "cooperation_projects": [
                                                {
                                                    "id": "7309457114076807188",
                                                    "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                                    "roles": [
                                                        {
                                                            "reviewer_role": { "role_id": "7213434603057807379", "name": { "zh_cn": "体验", "en_us": "Interactive experience" } },
                                                            "reviewee_role": { "role_id": "7213434603057807379", "name": { "zh_cn": "体验", "en_us": "Interactive experience" } }
                                                        }
                                                    ],
                                                    "user_roles": [
                                                        { "role_id": "7213434603057807379", "name": { "zh_cn": "体验", "en_us": "Interactive experience" } }
                                                    ],
                                                    "underling_roles": [
                                                        { "role_id": "7213434603057807379", "name": { "zh_cn": "体验", "en_us": "Interactive experience" } }
                                                    ]
                                                }
                                            ],
                                            "review_depend_projects": [],
                                            "participated_projects": []
                                        },
                                        "record_id": "7385000219907457024-7385000219907457025"
                                    }
                                ],
                                "review_stage_role": "reviewee"
                            }
                        ]
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/review_datas/query"))
            .and(body_json(json!({
                "semester_ids": ["6992035450862224940"],
                "reviewee_user_ids": ["ou_3245842393d09e9428ad4655da6e30b3"],
                "stage_types": ["leader_review"],
                "review_stage_roles": ["reviewee"],
                "stage_ids": ["7343513161666707459"],
                "need_leader_review_data_source": true,
                "updated_later_than": "1630425599999",
                "stage_progresses": [1]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(config)
            .add_semester_id("6992035450862224940".to_string())
            .add_reviewee_user_id("ou_3245842393d09e9428ad4655da6e30b3".to_string())
            .stage_types(vec!["leader_review".to_string()])
            .review_stage_roles(vec!["reviewee".to_string()])
            .stage_ids(vec!["7343513161666707459".to_string()])
            .need_leader_review_data_source(true)
            .updated_later_than("1630425599999".to_string())
            .stage_progresses(vec![1])
            .execute()
            .await
            .expect("performance_v2_review_data_query 应成功");

        let datas = data.datas.as_ref().expect("datas 应存在");
        assert_eq!(datas.len(), 1);
        assert_eq!(
            datas[0].user_id.as_ref().and_then(|u| u.open_id.as_deref()),
            Some("od-asd2dasdasd")
        );
        assert_eq!(
            datas[0].review_template_id.as_deref(),
            Some("7343513161666707459")
        );
        let stages = datas[0].stages.as_ref().expect("stages 应存在");
        assert_eq!(stages[0].stage_type.as_deref(), Some("review"));
        assert_eq!(stages[0].review_stage_role.as_deref(), Some("reviewee"));
        let records = stages[0].records.as_ref().expect("records 应存在");
        assert_eq!(records[0].progress, Some(1));
        assert_eq!(
            records[0].record_id.as_deref(),
            Some("7385000219907457024-7385000219907457025")
        );
        let invited = records[0]
            .invited_review_record_info
            .as_ref()
            .expect("invited_review_record_info 应存在");
        assert_eq!(invited.distribute_type, Some(1));
        assert_eq!(invited.avg_diff.as_deref(), Some("1.23"));
        let units = records[0].units.as_ref().expect("units 应存在");
        let detail = &units[0].data.as_ref().expect("data 应存在")[0];
        assert_eq!(detail.score.as_deref(), Some("1.1"));
        assert_eq!(detail.leader_review_data_source.as_deref(), Some("review"));
        assert_eq!(detail.is_principal_review_item, Some(true));
        let tag_texts = detail
            .tag_text_item_data
            .as_ref()
            .expect("tag_text_item_data 应存在");
        assert_eq!(tag_texts[0].tag_text.as_deref(), Some("qwertyu"));
        let projects = records[0]
            .direct_project_leader_record_info
            .as_ref()
            .and_then(|d| d.cooperation_projects.as_ref())
            .expect("cooperation_projects 应存在");
        assert_eq!(projects[0].id.as_deref(), Some("7309457114076807188"));
        assert_eq!(
            projects[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/review_datas/query"
        );
    }
}
