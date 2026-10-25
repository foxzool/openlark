//! 获取面试满意度问卷列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/questionnaire/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 获取面试满意度问卷列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
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

        let api_endpoint = HireApiV1::QuestionnaireList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试满意度问卷列表响应数据为空",
        )
        .await
    }
}

/// 评分题回答（官方 `five_start_scoring_result`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FiveStartScoringResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高分中文描述。
    pub highest_score_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高分英文描述。
    pub highest_score_en_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低分中文描述。
    pub lowest_score_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低分英文描述。
    pub lowest_score_en_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评分分数（整数）。
    pub score_result: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 选项题回答（官方 `select_option_result`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SelectOptionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项中文名称。
    pub option_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项英文名称。
    pub option_en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项中文描述。
    pub option_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项英文描述。
    pub option_en_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否选中。
    pub is_selected: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 问卷题目（官方 `question`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuestionnaireQuestion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目 ID。
    pub question_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目中文名称。
    pub question_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目英文名称。
    pub question_en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目中文描述。
    pub question_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目英文描述。
    pub question_en_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目类型：1 单选题 / 2 多选题 / 3 描述题 / 4 评分题。
    pub question_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目是否必填。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项题回答列表，当题目类型为单选题或多选题时该字段有值。
    pub select_option_result_list: Option<Vec<SelectOptionResult>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评分题回答，当题目类型为评分题时该字段有值。
    pub five_start_scoring_result: Option<FiveStartScoringResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 描述题回答，当题目类型为描述题时该字段有值。
    pub description_result: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 满意度问卷（官方 `questionnaire`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuestionnaireItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 满意度问卷 ID。
    pub questionnaire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID。
    pub interview_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 满意度问卷版本。
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 题目列表。
    pub questions: Option<Vec<QuestionnaireQuestion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否完成作答。
    pub has_answers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 满意度问卷更新时间，毫秒时间戳。
    pub update_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取面试满意度问卷列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 满意度问卷列表。
    pub items: Vec<QuestionnaireItem>,
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
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/questionnaires
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/questionnaires"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEsInRpbWVzdGFtcCI6MTcyMzU1OTE4OTg1NCwiaWQiOm51bGx9",
                    "items": [
                        {
                            "questionnaire_id": "7037986982531778860",
                            "application_id": "6985833807195212076",
                            "interview_id": "7038435261598763308",
                            "version": 59,
                            "questions": [
                                {
                                    "question_id": "6950930570311829804",
                                    "question_name": "你的总体评价如何？",
                                    "question_en_name": "How would you describe your overall experience?",
                                    "question_desc": "测试描述内容",
                                    "question_en_desc": "Test Text",
                                    "question_type": 4,
                                    "is_required": true,
                                    "select_option_result_list": [
                                        {
                                            "option_id": "6950930570311993644",
                                            "option_name": "选项一",
                                            "option_en_name": "Option 1",
                                            "option_desc": "测试选项描述",
                                            "option_en_desc": "Test Option Desc",
                                            "is_selected": true
                                        }
                                    ],
                                    "five_start_scoring_result": {
                                        "highest_score_desc": "非常好",
                                        "highest_score_en_desc": "Very Good",
                                        "lowest_score_desc": "非常差",
                                        "lowest_score_en_desc": "Very Bad",
                                        "score_result": 4
                                    },
                                    "description_result": "测试描述内容"
                                }
                            ],
                            "has_answers": true,
                            "update_time": "1644412504000"
                        }
                    ]
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(
            item.questionnaire_id.as_deref(),
            Some("7037986982531778860")
        );
        assert_eq!(item.version, Some(59));
        assert_eq!(item.has_answers, Some(true));
        assert_eq!(item.update_time.as_deref(), Some("1644412504000"));
        let question = item
            .questions
            .as_ref()
            .expect("questions 应存在")
            .first()
            .unwrap();
        assert_eq!(question.question_id.as_deref(), Some("6950930570311829804"));
        assert_eq!(question.question_type, Some(4));
        assert_eq!(question.is_required, Some(true));
        let option = question
            .select_option_result_list
            .as_ref()
            .expect("select_option_result_list 应存在")
            .first()
            .unwrap();
        assert_eq!(option.option_id.as_deref(), Some("6950930570311993644"));
        assert_eq!(option.is_selected, Some(true));
        let scoring = question
            .five_start_scoring_result
            .as_ref()
            .expect("five_start_scoring_result 应存在");
        assert_eq!(scoring.score_result, Some(4.0));
        assert_eq!(question.description_result.as_deref(), Some("测试描述内容"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
