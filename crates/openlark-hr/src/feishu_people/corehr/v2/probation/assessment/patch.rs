//! 更新试用期考核信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/probation.assessment/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新试用期考核信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    assessment_id: Option<String>,
    request_body: Option<Value>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            assessment_id: None,
            request_body: None,
        }
    }

    /// 设置 `assessment_id`。
    pub fn assessment_id(mut self, assessment_id: String) -> Self {
        self.assessment_id = Some(assessment_id);
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let assessment_id = self.assessment_id.unwrap_or_default();
        validate_required!(assessment_id.trim(), "assessment_id 不能为空");

        let api_endpoint = FeishuPeopleApiV2::ProbationAssessmentPatch(assessment_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新试用期考核信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新试用期考核信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
    /// probation信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation: Option<ProbationAssessmentProbation>,
}

/// 试用期对象（含考核信息）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbationAssessmentProbation {
    /// 试用期 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_id: Option<String>,
    /// 雇佣 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 考核项列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessments: Option<Vec<ProbationAssessmentItem>>,
    /// 预留扩展字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

/// 试用期考核项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbationAssessmentItem {
    /// 考核 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    /// 考核名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 考核状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 预留扩展字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl ApiResponseTrait for PatchResponse {
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

    /// 端到端：PATCH /open-apis/corehr/v2/probation/assessments/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/probation/assessments/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequest::new(config)
            .assessment_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
