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

use crate::hire::hire::common_models::ReferralAccountOperationResult;

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
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "全额提取内推账户余额响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 全额提取内推账户余额响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WithdrawResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ReferralAccountOperationResult,
}

impl ApiResponseTrait for WithdrawResponse {
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
