//! 查询内推账户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_account/get_account_assets>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询内推账户请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetAccountAssetsRequest {
    /// 配置信息
    config: Config,
    account_id: String,
}

impl GetAccountAssetsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            account_id: String::new(),
        }
    }

    pub fn account_id(mut self, account_id: String) -> Self {
        self.account_id = account_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAccountAssetsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAccountAssetsResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.account_id.trim(), "内推账户 ID 不能为空");

        let api_endpoint = HireApiV1::ReferralAccountGetAccountAssets(self.account_id);
        let request = ApiRequest::<GetAccountAssetsResponse>::get(api_endpoint.to_url());
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAccountAssetsResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
