//! 操作员工离职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/offboarding/submit_v2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 操作员工离职请求
#[derive(Debug, Clone)]
pub struct SubmitV2Request {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl SubmitV2Request {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubmitV2Response> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubmitV2Response> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::OffboardingSubmitV2;
        let mut request = ApiRequest::<SubmitV2Response>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "操作员工离职响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 操作员工离职响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitV2Response {
    /// 响应数据
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OffboardingInfo>,
}

/// 操作离职返回信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OffboardingInfo {
    /// 离职 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 雇佣 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 离职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 操作状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 操作时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operated_at: Option<String>,
    /// 预留扩展字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl ApiResponseTrait for SubmitV2Response {
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

    /// 端到端：POST /open-apis/corehr/v2/offboardings/submit_v2
    #[tokio::test]
    async fn test_submit_v2_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/offboardings/submit_v2"))
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

        SubmitV2Request::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
