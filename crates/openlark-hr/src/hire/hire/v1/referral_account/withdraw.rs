//! 全额提取内推账户余额
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/withdraw>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::{BonusAmount, ReferralAccountOperationResult};

/// 全额提取内推账户余额请求
#[derive(Debug, Clone)]
pub struct WithdrawRequest {
    /// 配置信息
    config: Config,
    account_id: Option<String>,
}

impl WithdrawRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            account_id: None,
        }
    }

    /// 设置 `account_id`。
    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WithdrawResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WithdrawResponse> {
        let account_id = self.account_id.unwrap_or_default();
        validate_required!(account_id.trim(), "内推账户 ID 不能为空");

        let request = ApiRequest::<WithdrawResponse>::post(format!(
            "/open-apis/hire/v1/referral_account/{account_id}/withdraw"
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "全额提取内推账户余额响应数据为空",
        )
        .await
    }
}

/// 全额提取内推账户余额响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WithdrawResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ReferralAccountOperationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 提取单 ID。
    pub external_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 交易时间（毫秒时间戳）。
    pub trans_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 提取详情（官方 `bonus_amount`）。
    pub withdrawal_details: Option<BonusAmount>,
}

impl ApiResponseTrait for WithdrawResponse {
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

    /// 端到端：POST /open-apis/hire/v1/referral_account/test001/withdraw
    #[tokio::test]
    async fn test_withdraw_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/referral_account/test001/withdraw"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "external_order_id": "6942778198054125570",
                    "trans_time": "1683634459543",
                    "withdrawal_details": {
                        "point_bonus": 100,
                        "cash_bonus": [
                            { "currency_type": "CNY", "amount": 100 }
                        ]
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

        let resp = WithdrawRequest::new(config)
            .account_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(
            resp.external_order_id.as_deref(),
            Some("6942778198054125570")
        );
        assert_eq!(resp.trans_time.as_deref(), Some("1683634459543"));
        let details = resp
            .withdrawal_details
            .as_ref()
            .expect("withdrawal_details 应被类型化解析");
        assert_eq!(details.point_bonus, Some(100));
        assert_eq!(
            details
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
