//! 内推账户提现数据对账
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/reconciliation>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::BonusAmount;

/// `TradeDetail`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeDetail {
    /// `account_id` 字段。
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `total_recharge_reward_info` 字段。
    pub total_recharge_reward_info: Option<BonusAmount>,
}

impl TradeDetail {
    /// 创建新的请求实例。
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            total_recharge_reward_info: None,
        }
    }

    /// 设置 `total_recharge_reward_info`。
    pub fn total_recharge_reward_info(mut self, total_recharge_reward_info: BonusAmount) -> Self {
        self.total_recharge_reward_info = Some(total_recharge_reward_info);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ReconciliationRequestBody {
    start_trans_time: String,
    end_trans_time: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    trade_details: Vec<TradeDetail>,
}

/// `ReconciliationRequest` 请求。
#[derive(Debug, Clone)]
pub struct ReconciliationRequest {
    config: Config,
    start_trans_time: String,
    end_trans_time: String,
    trade_details: Vec<TradeDetail>,
}

impl ReconciliationRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            start_trans_time: String::new(),
            end_trans_time: String::new(),
            trade_details: Vec::new(),
        }
    }

    /// 设置 `start_trans_time`。
    pub fn start_trans_time(mut self, start_trans_time: impl Into<String>) -> Self {
        self.start_trans_time = start_trans_time.into();
        self
    }

    /// 设置 `end_trans_time`。
    pub fn end_trans_time(mut self, end_trans_time: impl Into<String>) -> Self {
        self.end_trans_time = end_trans_time.into();
        self
    }

    /// 设置 `trade_details`。
    pub fn trade_details(mut self, trade_details: Vec<TradeDetail>) -> Self {
        self.trade_details = trade_details;
        self
    }

    /// 设置 `add_trade_detail`。
    pub fn add_trade_detail(mut self, trade_detail: TradeDetail) -> Self {
        self.trade_details.push(trade_detail);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ReconciliationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ReconciliationResponse> {
        if self.start_trans_time.trim().is_empty() {
            return Err(error::validation_error(
                "start_trans_time",
                "start_trans_time 不能为空",
            ));
        }
        if self.end_trans_time.trim().is_empty() {
            return Err(error::validation_error(
                "end_trans_time",
                "end_trans_time 不能为空",
            ));
        }

        let request = ApiRequest::<ReconciliationResponse>::post(
            "/open-apis/hire/v1/referral_account/reconciliation",
        )
        .body(
            serde_json::to_value(ReconciliationRequestBody {
                start_trans_time: self.start_trans_time,
                end_trans_time: self.end_trans_time,
                trade_details: self.trade_details,
            })
            .map_err(|e| {
                error::validation_error("request_body", format!("无法序列化请求体: {e}"))
            })?,
        );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "内推账户提现数据对账响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `CheckFailedAccountInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CheckFailedAccountInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `total_withdraw_reward_info` 字段。
    pub total_withdraw_reward_info: Option<BonusAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `total_recharge_reward_info` 字段。
    pub total_recharge_reward_info: Option<BonusAmount>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ReconciliationResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReconciliationResponse {
    #[serde(default)]
    /// `check_failed_list` 字段。
    pub check_failed_list: Vec<CheckFailedAccountInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ReconciliationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
