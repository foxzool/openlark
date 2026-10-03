//! 启用内推账户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/enable>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::ReferralAccountOperationResult;

/// 启用内推账户请求
#[derive(Debug, Clone)]
pub struct EnableRequest {
    /// 配置信息
    config: Config,
    account_id: String,
}

impl EnableRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            account_id: String::new(),
        }
    }

    /// 设置 `account_id`。
    pub fn account_id(mut self, account_id: String) -> Self {
        self.account_id = account_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EnableResponse> {
        validate_required!(self.account_id.trim(), "内推账户 ID 不能为空");

        let request =
            ApiRequest::<EnableResponse>::post("/open-apis/hire/v1/referral_account/enable");
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "启用内推账户响应数据为空",
        )
        .await
    }
}

/// 启用内推账户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnableResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ReferralAccountOperationResult,
}

impl ApiResponseTrait for EnableResponse {
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

    /// 端到端：POST /open-apis/hire/v1/referral_account/enable
    #[tokio::test]
    async fn test_enable_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/referral_account/enable"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "operation": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        EnableRequest::new(config)
            .account_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
