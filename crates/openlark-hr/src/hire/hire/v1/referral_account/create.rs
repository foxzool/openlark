//! 注册内推账户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/create>

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

/// `Mobile`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mobile {
    /// `code` 字段。
    pub code: String,
    /// 数值。
    pub number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct CreateRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<Mobile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

/// `CreateRequest` 请求。
#[derive(Debug, Clone)]
pub struct CreateRequest {
    config: Config,
    mobile: Option<Mobile>,
    email: Option<String>,
}

impl CreateRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            mobile: None,
            email: None,
        }
    }

    /// 设置 `mobile`。
    pub fn mobile(mut self, code: impl Into<String>, number: impl Into<String>) -> Self {
        self.mobile = Some(Mobile {
            code: code.into(),
            number: number.into(),
        });
        self
    }

    /// 设置 `email`。
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        if self.mobile.is_none() && self.email.as_deref().unwrap_or("").trim().is_empty() {
            return Err(error::validation_error(
                "contact",
                "mobile 和 email 至少需要提供一个",
            ));
        }

        let request = ApiRequest::<CreateResponse>::post("/open-apis/hire/v1/referral_account")
            .body(
                serde_json::to_value(CreateRequestBody {
                    mobile: self.mobile,
                    email: self.email,
                })
                .map_err(|e| {
                    error::validation_error("request_body", format!("无法序列化请求体: {e}"))
                })?,
            );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("注册内推账户响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `ReferralAccountAssets`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `confirmed_bonus` 字段。
    pub confirmed_bonus: Option<BonusAmount>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ReferralAccount`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `assets` 字段。
    pub assets: Option<ReferralAccountAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `CreateResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account` 字段。
    pub account: Option<ReferralAccount>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
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
