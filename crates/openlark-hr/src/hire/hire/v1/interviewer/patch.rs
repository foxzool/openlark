//! 更新面试官信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interviewer/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::InterviewerOperationResult;

/// 更新面试官信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    interviewer_id: String,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            interviewer_id: String::new(),
        }
    }

    /// 设置 `interviewer_id`。
    pub fn interviewer_id(mut self, interviewer_id: impl Into<String>) -> Self {
        self.interviewer_id = interviewer_id.into();
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
        validate_required!(self.interviewer_id.trim(), "interviewer_id 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/hire/v1/interviewers/{}",
            self.interviewer_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新面试官信息响应数据为空",
        )
        .await
    }
}

/// 更新面试官信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchResponse {
    #[serde(flatten)]
    /// `interviewer` 字段。
    pub interviewer: InterviewerOperationResult,
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

    /// 端到端：PATCH /open-apis/hire/v1/interviewers/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/hire/v1/interviewers/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "interviewer": {  } }
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
            .interviewer_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
