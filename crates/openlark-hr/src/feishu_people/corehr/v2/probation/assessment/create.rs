//! 新增试用期考核信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation.assessment/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 新增试用期考核信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAssessmentBody {
    /// 试用期人员的雇佣 ID
    pub employment_id: String,
    /// 试用期考核结果列表
    pub assessments: Vec<AssessmentForCreate>,
}

/// 试用期考核结果条目（assessment_for_create）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssessmentForCreate {
    /// 考核状态：not_started/in_process/completed/no_need
    pub assessment_status: String,
    /// 试用期考核结果：approved/rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_result: Option<String>,
    /// 考核得分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_score: Option<f64>,
    /// 试用期考核等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_grade: Option<String>,
    /// 考核评语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_comment: Option<String>,
    /// 考核结果页面超链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_detail: Option<String>,
    /// 是否为最终考核结果（官方字段名即此拼写）
    pub is_final_asssessment: bool,
}

/// 新增试用期考核信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateAssessmentBody,
    /// 用于幂等控制的 client_token
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateAssessmentBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: CreateAssessmentBody) -> Self {
        self.body = body;
        self
    }

    /// 设置试用期人员的雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置试用期考核结果列表。
    pub fn assessments(mut self, assessments: Vec<AssessmentForCreate>) -> Self {
        self.body.assessments = assessments;
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
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        validate_required!(self.body.employment_id, "employment_id 不能为空");
        validate_required!(self.body.assessments, "assessments 不能为空");

        let api_endpoint = FeishuPeopleApiV2::ProbationAssessmentCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .query_opt("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "新增试用期考核信息响应数据为空",
        )
        .await
    }
}

/// 新增试用期考核信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 创建的试用期考核记录 ID 列表，有序返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/probation/assessments
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/probation/assessments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "assessment_ids": ["7140964208476371991"]
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

        let resp = CreateRequest::new(config)
            .employment_id("7140964208476371111")
            .assessments(vec![AssessmentForCreate {
                assessment_status: "completed".to_string(),
                assessment_result: Some("approved".to_string()),
                assessment_score: Some(99.9),
                assessment_grade: Some("grade_a".to_string()),
                assessment_comment: Some("超出预期".to_string()),
                assessment_detail: Some("暂无示例".to_string()),
                is_final_asssessment: false,
            }])
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(
            resp.assessment_ids.unwrap(),
            vec!["7140964208476371991".to_string()]
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
