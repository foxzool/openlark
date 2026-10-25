//! 获取绩效结果
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/review_data/query>

use crate::performance::performance::v1::models::{I18nName, UserId};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use openlark_core::{validate_required, validate_required_list};
use serde::{Deserialize, Serialize};

/// 获取绩效结果请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 查询范围开始时间，毫秒时间戳（必填）
    start_time: String,
    /// 查询范围结束时间，毫秒时间戳（必填）
    end_time: String,
    /// 环节类型（必填，≤50：leader_review/communication_and_open_result/view_result）
    stage_types: Vec<String>,
    /// 环节状态（可选，≤50）
    stage_progress: Option<Vec<i32>>,
    /// 评估周期 ID 列表（可选，≤50）
    semester_id_list: Option<Vec<String>>,
    /// 被评估人 ID 列表（必填，≤50）
    reviewee_user_id_list: Vec<String>,
    /// 环节更新时间最早时间，毫秒时间戳（可选）
    updated_later_than: Option<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            stage_types: Vec::new(),
            stage_progress: None,
            semester_id_list: None,
            reviewee_user_id_list: Vec::new(),
            updated_later_than: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置查询范围开始时间，毫秒时间戳（必填）
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = start_time;
        self
    }

    /// 设置查询范围结束时间，毫秒时间戳（必填）
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = end_time;
        self
    }

    /// 设置环节类型列表（必填，≤50）
    pub fn stage_types(mut self, stage_types: Vec<String>) -> Self {
        self.stage_types = stage_types;
        self
    }

    /// 追加一个环节类型
    pub fn add_stage_type(mut self, stage_type: String) -> Self {
        self.stage_types.push(stage_type);
        self
    }

    /// 设置环节状态列表（可选，≤50）
    pub fn stage_progress(mut self, stage_progress: Vec<i32>) -> Self {
        self.stage_progress = Some(stage_progress);
        self
    }

    /// 设置评估周期 ID 列表（可选，≤50）
    pub fn semester_id_list(mut self, semester_id_list: Vec<String>) -> Self {
        self.semester_id_list = Some(semester_id_list);
        self
    }

    /// 设置被评估人 ID 列表（必填，≤50）
    pub fn reviewee_user_id_list(mut self, reviewee_user_id_list: Vec<String>) -> Self {
        self.reviewee_user_id_list = reviewee_user_id_list;
        self
    }

    /// 追加一个被评估人 ID
    pub fn add_reviewee_user_id(mut self, reviewee_user_id: String) -> Self {
        self.reviewee_user_id_list.push(reviewee_user_id);
        self
    }

    /// 设置环节更新时间最早时间，毫秒时间戳（可选）
    pub fn updated_later_than(mut self, updated_later_than: String) -> Self {
        self.updated_later_than = Some(updated_later_than);
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
        validate_required!(self.start_time.trim(), "start_time");
        validate_required!(self.end_time.trim(), "end_time");
        validate_required_list!(self.stage_types, 50, "stage_types 不能为空且不能超过 50 个");
        validate_required_list!(
            self.reviewee_user_id_list,
            50,
            "reviewee_user_id_list 不能为空且不能超过 50 个"
        );

        let request =
            ApiRequest::<QueryResponse>::post("/open-apis/performance/v1/review_datas/query")
                .query_opt("user_id_type", self.user_id_type);

        let request_body = QueryRequestBody {
            start_time: self.start_time,
            end_time: self.end_time,
            stage_types: self.stage_types,
            stage_progress: self.stage_progress,
            semester_id_list: self.semester_id_list,
            reviewee_user_id_list: self.reviewee_user_id_list,
            updated_later_than: self.updated_later_than,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取绩效结果响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 查询范围开始时间，毫秒时间戳
    pub start_time: String,
    /// 查询范围结束时间，毫秒时间戳
    pub end_time: String,
    /// 环节类型列表
    pub stage_types: Vec<String>,
    /// 环节状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress: Option<Vec<i32>>,
    /// 评估周期 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semester_id_list: Option<Vec<String>>,
    /// 被评估人 ID 列表
    pub reviewee_user_id_list: Vec<String>,
    /// 环节更新时间最早时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_later_than: Option<String>,
}

/// 获取绩效结果响应（官方 review_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 周期列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semesters: Option<Vec<Semester>>,
    /// 绩效评估项目列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<Activity>>,
    /// 评估项列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicators: Option<Vec<Indicator>>,
    /// 评估模板列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<Template>>,
    /// 评估内容列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<Unit>>,
    /// 评估字段列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    /// 评估数据列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datas: Option<Vec<ReviewProfile>>,
}

/// 周期信息（官方 semester 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Semester {
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 周期年份
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// 周期类型分组
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_group: Option<String>,
    /// 周期类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 周期名称（i18n，键为 zh-CN/en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 周期状态：initiating / enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 周期开始时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 周期结束时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 周期创建时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 周期更新时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 周期创建人 ID（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user_id: Option<String>,
    /// 周期更新人 ID（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_user_id: Option<String>,
}

/// 绩效评估项目（官方 activity 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Activity {
    /// 项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 项目名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
}

/// 评估项（官方 indicator 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Indicator {
    /// 评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评估项名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 评估项等级列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<IndicatorOption>>,
}

/// 评估项等级（官方 indicator_option 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IndicatorOption {
    /// 等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 等级名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 等级代号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 评估模板（官方 template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Template {
    /// 评估模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 环节名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 环节类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// 评估内容（官方 unit 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Unit {
    /// 评估内容 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评估内容名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
}

/// 评估字段（官方 field 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Field {
    /// 评估字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评估字段名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 父级评估字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_field_id: Option<String>,
}

/// 评估数据（官方 review_profile 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewProfile {
    /// 被评估人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// 环节信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<ReviewStage>>,
}

/// 环节信息（官方 review_stage 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewStage {
    /// 环节类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 环节状态：0 未开始 / 1 待完成 / 2 已截止 / 3 已完成 / 4 已复议
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 环节填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ReviewDetail>>,
}

/// 环节填写内容（官方 review_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewDetail {
    /// 评估模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 评估内容 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 评估字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 评估人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_user_id: Option<UserId>,
    /// 最后提交时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// 评估项 ID（option_id 或 score 有值时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 评估项结果等级 ID（评级型评估项有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// 评分型评估项填写内容（评分型评估项有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    /// 填写项填写内容（填写项有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 绩效系数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf_coefficient_result: Option<String>,
    /// 富文本格式的填写内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
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
    async fn test_performance_v1_review_data_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（semesters/activities/indicators/templates/
        // units/fields/datas 七键，i18n 键为 zh-CN/en-US），防止字段名回归
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "semesters": [
                    {
                        "id": "6992035450862224940",
                        "name": { "en-US": "", "zh-CN": "sr" },
                        "start_time": "1625068800000",
                        "end_time": "1640966399999"
                    }
                ],
                "activities": [
                    {
                        "id": "6992035450862323244",
                        "name": { "en-US": "", "zh-CN": "rino" },
                        "semester_id": "6992035450862224940"
                    }
                ],
                "indicators": [
                    {
                        "id": "6966127279593784876",
                        "name": { "en-US": "", "zh-CN": "业绩" },
                        "options": [
                            {
                                "id": "6966127279593653804",
                                "name": { "en-US": "", "zh-CN": "符合预期" },
                                "label": "B"
                            }
                        ]
                    }
                ],
                "templates": [
                    {
                        "id": "6982759008789366313",
                        "name": { "en-US": "Manager Review", "zh-CN": "上级评估" },
                        "stage": "leader_review"
                    }
                ],
                "units": [
                    {
                        "id": "6982759009396508196",
                        "name": { "en-US": "", "zh-CN": "业绩" }
                    }
                ],
                "fields": [
                    {
                        "id": "6982759006887888417",
                        "name": { "en-US": "", "zh-CN": "业绩" }
                    }
                ],
                "datas": [
                    {
                        "user_id": {
                            "open_id": "ou_3245842393d09e9428ad4655da6e30b3",
                            "user_id": "6932009399175086099"
                        },
                        "semester_id": "6992035450862224940",
                        "activity_id": "6992035450862323244",
                        "stages": [
                            {
                                "data": [
                                    {
                                        "field_id": "6982759006887888417",
                                        "reviewer_user_id": {
                                            "open_id": "ou_f0ff33850fc47236e34360d5eed99405",
                                            "user_id": "6924863940618878996"
                                        },
                                        "submit_time": "1627977100000",
                                        "template_id": "6982759008789366313",
                                        "unit_id": "6982759009396508196",
                                        "indicator_id": "6966127279593784876",
                                        "option_id": "6966127279593653804"
                                    }
                                ],
                                "progress": 3,
                                "stage_type": "leader_review"
                            }
                        ]
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v1/review_datas/query"))
            .and(body_json(json!({
                "start_time": "1430425599999",
                "end_time": "1630425599999",
                "stage_types": ["leader_review"],
                "reviewee_user_id_list": ["ou_3245842393d09e9428ad4655da6e30b3"]
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
            .start_time("1430425599999".to_string())
            .end_time("1630425599999".to_string())
            .add_stage_type("leader_review".to_string())
            .add_reviewee_user_id("ou_3245842393d09e9428ad4655da6e30b3".to_string())
            .execute()
            .await
            .expect("performance_v1_review_data_query 应成功");

        let semesters = data.semesters.as_ref().expect("semesters 应存在");
        assert_eq!(semesters[0].id.as_deref(), Some("6992035450862224940"));
        assert_eq!(
            semesters[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("sr")
        );
        let activities = data.activities.as_ref().expect("activities 应存在");
        assert_eq!(
            activities[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("rino")
        );
        let indicators = data.indicators.as_ref().expect("indicators 应存在");
        assert_eq!(indicators[0].options.as_ref().map(|o| o.len()), Some(1));
        let datas = data.datas.as_ref().expect("datas 应存在");
        assert_eq!(datas.len(), 1);
        assert_eq!(
            datas[0].user_id.as_ref().and_then(|u| u.open_id.as_deref()),
            Some("ou_3245842393d09e9428ad4655da6e30b3")
        );
        let stages = datas[0].stages.as_ref().expect("stages 应存在");
        assert_eq!(stages[0].stage_type.as_deref(), Some("leader_review"));
        assert_eq!(stages[0].progress, Some(3));
        let detail = &stages[0].data.as_ref().expect("data 应存在")[0];
        assert_eq!(detail.option_id.as_deref(), Some("6966127279593653804"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v1/review_datas/query"
        );
    }
}
