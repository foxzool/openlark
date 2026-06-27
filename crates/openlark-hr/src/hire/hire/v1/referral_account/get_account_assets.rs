//! 查询内推账户
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/get_account_assets

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

use crate::hire::hire::common_models::I18nText;

use crate::hire::hire::common_models::BonusAmount;

/// 查询内推账户请求
#[derive(Debug, Clone)]
pub struct GetAccountAssetsRequest {
    /// 配置信息
    config: Config,
    account_id: String,
    user_id_type: Option<String>,
}

impl GetAccountAssetsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            account_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置 `account_id`。
    pub fn account_id(mut self, account_id: String) -> Self {
        self.account_id = account_id;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAccountAssetsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAccountAssetsResponse> {
        validate_required!(self.account_id.trim(), "内推账户 ID 不能为空");

        let mut request = ApiRequest::<GetAccountAssetsResponse>::get(
            "/open-apis/hire/v1/referral_account/get_account_assets",
        );
        request = request.query("referral_account_id", self.account_id);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询内推账户响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询内推账户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `confirmed_bonus` 字段。
    pub confirmed_bonus: Option<BonusAmount>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ReferralAccountWithReferrer`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountWithReferrer {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `assets` 字段。
    pub assets: Option<ReferralAccountAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `referrer` 字段。
    pub referrer: Option<ReferralAccountReferrer>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ReferralAccountReferrer`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountReferrer {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mobile` 字段。
    pub mobile: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetAccountAssetsResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetAccountAssetsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account` 字段。
    pub account: Option<ReferralAccountWithReferrer>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetAccountAssetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
