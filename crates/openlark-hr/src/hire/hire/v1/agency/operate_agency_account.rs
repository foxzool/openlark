//! 禁用/取消禁用猎头
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/agency/operate_agency_account>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 禁用/取消禁用猎头请求
#[derive(Debug, Clone)]
pub struct OperateAgencyAccountRequest {
    /// 配置信息
    config: Config,
    agency_id: String,
    request_body: Option<Value>,
}

impl OperateAgencyAccountRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            agency_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `agency_id`。
    pub fn agency_id(mut self, agency_id: String) -> Self {
        self.agency_id = agency_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OperateAgencyAccountResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OperateAgencyAccountResponse> {
        let mut request = ApiRequest::<OperateAgencyAccountResponse>::post(
            "/open-apis/hire/v1/agencies/operate_agency_account",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "禁用/取消禁用猎头响应数据为空",
        )
        .await
    }
}

/// 禁用/取消禁用猎头响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OperateAgencyAccountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_account_id` 字段。
    pub agency_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for OperateAgencyAccountResponse {
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

    /// 端到端：POST /open-apis/hire/v1/agencies/operate_agency_account
    #[tokio::test]
    async fn test_operate_agency_account_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/agencies/operate_agency_account"))
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

        OperateAgencyAccountRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
