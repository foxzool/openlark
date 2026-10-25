//! 停用内推账户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/deactivate>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::ReferralAccountOperationResult;
use crate::hire::hire::v1::referral_account::create::ReferralAccount;

/// 停用内推账户请求
#[derive(Debug, Clone)]
pub struct DeactivateRequest {
    /// 配置信息
    config: Config,
    account_id: String,
}

impl DeactivateRequest {
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
    pub async fn execute(self) -> SDKResult<DeactivateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeactivateResponse> {
        validate_required!(self.account_id.trim(), "内推账户 ID 不能为空");

        let request = ApiRequest::<DeactivateResponse>::post(format!(
            "/open-apis/hire/v1/referral_account/{}/deactivate",
            self.account_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "停用内推账户响应数据为空",
        )
        .await
    }
}

/// 停用内推账户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeactivateResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ReferralAccountOperationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推账户信息（官方 `data.account` 容器）。
    pub account: Option<ReferralAccount>,
}

impl ApiResponseTrait for DeactivateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/referral_account/test001/deactivate
    #[tokio::test]
    async fn test_deactivate_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/referral_account/test001/deactivate",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "account": {
                        "account_id": "6942778198054125570",
                        "assets": {
                            "confirmed_bonus": {
                                "point_bonus": 100,
                                "cash_bonus": [
                                    { "currency_type": "CNY", "amount": 100 }
                                ]
                            }
                        },
                        "status": 2
                    }
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

        let resp = DeactivateRequest::new(config)
            .account_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        let account = resp.account.as_ref().expect("account 应被类型化解析");
        assert_eq!(account.account_id.as_deref(), Some("6942778198054125570"));
        assert_eq!(account.status, Some(2));
        let assets = account.assets.as_ref().expect("assets 应被类型化解析");
        let confirmed_bonus = assets
            .confirmed_bonus
            .as_ref()
            .expect("confirmed_bonus 应被类型化解析");
        assert_eq!(confirmed_bonus.point_bonus, Some(100));
        assert_eq!(
            confirmed_bonus
                .cash_bonus
                .as_ref()
                .and_then(|l| l.first())
                .and_then(|c| c.currency_type.as_deref()),
            Some("CNY")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
