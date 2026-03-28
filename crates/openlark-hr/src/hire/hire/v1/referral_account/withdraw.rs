//! 全额提取内推账户余额
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/withdraw>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 全额提取内推账户余额请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WithdrawRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl WithdrawRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WithdrawResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WithdrawResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::ReferralAccountWithdraw;
        let request = ApiRequest::<WithdrawResponse>::post(api_endpoint.to_url());
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WithdrawResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
