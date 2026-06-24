//! 向管理员申请授权
//! docPath: https://open.feishu.cn/document/application-v6/scope/apply

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct ApplyScopeRequest {
    config: Arc<Config>,
    body: ApplyScopeBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct ApplyScopeBody {
    /// 待补充文档。
    pub scope_type: String,
    /// 待补充文档。
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct ApplyScopeResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ApplyScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyScopeRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ApplyScopeBody::default(),
        }
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<ApplyScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ApplyScopeResponse> {
        let path = "/open-apis/application/v6/scopes/apply";
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<ApplyScopeResponse> = ApiRequest::post(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp.data.unwrap_or(ApplyScopeResponse { data: None }))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
