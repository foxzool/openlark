//! 获取认证信息
//!
//! 文档: <https://open.feishu.cn/document/server-docs/verification-v1/get>
//! docPath: <https://open.feishu.cn/document/server-docs/verification-v1/get>

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
pub struct VerificationGetRequestBuilder {
    config: Config,
}

impl VerificationGetRequestBuilder {
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
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 认证信息响应
///
/// 官方响应 data 层为 `{ "verification": { "name": ..., "has_verification": ... } }`，
/// 参见 <https://open.feishu.cn/document/server-docs/verification-v1/get>
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerificationGetResponse {
    /// 认证信息
    pub verification: VerificationInfo,
}

/// 企业认证信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerificationInfo {
    /// 企业主体名称
    pub name: String,
    /// 企业是否完成认证
    pub has_verification: bool,
}

impl ApiResponseTrait for VerificationGetResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to VerificationGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type VerificationGetBuilder = VerificationGetRequestBuilder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    /// 官方示例响应 data 层：{"verification":{"has_verification":true,"name":"..."}}
    #[test]
    fn test_deserialize_official_example() {
        let response: VerificationGetResponse = serde_json::from_value(json!({
            "verification": {
                "has_verification": true,
                "name": "无限未来科技公司"
            }
        }))
        .expect("官方示例 JSON 反序列化失败");

        assert_eq!(response.verification.name, "无限未来科技公司");
        assert!(response.verification.has_verification);
    }

    /// 序列化后键名与官方字段一致，且不再包含幻觉字段
    #[test]
    fn test_serialization_uses_official_keys() {
        let response = VerificationGetResponse {
            verification: VerificationInfo {
                name: "飞书科技有限公司".to_string(),
                has_verification: true,
            },
        };

        let json = serde_json::to_string(&response).expect("JSON 序列化失败");
        assert!(json.contains("\"verification\""));
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"has_verification\""));
        assert!(!json.contains("company_name"));
        assert!(!json.contains("is_verified"));
        assert!(!json.contains("verification_time"));
    }
}
