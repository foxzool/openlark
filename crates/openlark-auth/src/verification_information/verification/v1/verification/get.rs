//! 获取认证信息
//!
//! 文档: https://open.feishu.cn/document/server-docs/verification-v1/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取认证信息 Builder
#[derive(Debug, Clone)]
pub struct VerificationGetBuilder {
    config: Config,
}

impl VerificationGetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<VerificationGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VerificationGetResponse> {
        let req: ApiRequest<VerificationGetResponse> =
            ApiRequest::get("/open-apis/verification/v1/verification");
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 认证信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerificationGetResponse {
    /// 企业主体名称
    #[serde(rename = "company_name")]
    pub company_name: String,
    /// 是否完成企业认证
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    /// 认证时间（毫秒时间戳）
    #[serde(rename = "verification_time")]
    pub verification_time: Option<String>,
    /// 认证有效期（毫秒时间戳）
    #[serde(rename = "verification_expire_time")]
    pub verification_expire_time: Option<String>,
}

impl ApiResponseTrait for VerificationGetResponse {}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        let response = VerificationGetResponse {
            company_name: "飞书科技有限公司".to_string(),
            is_verified: true,
            verification_time: Some("1710000000000".to_string()),
            verification_expire_time: None,
        };

        let json = serde_json::to_string(&response).expect("JSON 序列化失败");
        let parsed: VerificationGetResponse =
            serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(parsed.company_name, "飞书科技有限公司");
        assert!(parsed.is_verified);
    }
}
