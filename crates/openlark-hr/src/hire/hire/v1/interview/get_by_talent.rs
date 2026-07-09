//! 获取人才面试信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview/get_by_talent>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::InterviewSummary;

/// 获取人才面试信息请求
#[derive(Debug, Clone)]
pub struct GetByTalentRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
}

impl GetByTalentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
        }
    }

    /// 设置 `talent_id`。
    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByTalentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByTalentResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.talent_id.trim(), "候选人 ID 不能为空");

        let api_endpoint = HireApiV1::InterviewGetByTalent(self.talent_id.clone());
        let request = ApiRequest::<GetByTalentResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取人才面试信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取人才面试信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetByTalentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview` 字段。
    pub interview: Option<InterviewSummary>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetByTalentResponse {
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

    /// 端到端：GET /open-apis/hire/v1/interviews/get_by_talent
    #[tokio::test]
    async fn test_get_by_talent_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interviews/get_by_talent"))
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

        GetByTalentRequest::new(config)
            .talent_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
