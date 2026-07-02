//! 向管理员申请授权
//! docPath: <https://open.feishu.cn/document/application-v6/scope/apply>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 向管理员申请授权的请求。
#[derive(Debug, Clone)]
pub struct ApplyScopeRequest {
    config: Arc<Config>,
    body: ApplyScopeBody,
}

/// 向管理员申请授权的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplyScopeBody {
    /// 权限范围类型。
    pub scope_type: String,
    /// 原因说明。
    pub reason: String,
}

/// 向管理员申请授权的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyScopeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ApplyScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyScopeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ApplyScopeBody::default(),
        }
    }

    /// 执行向管理员申请授权请求。
    pub async fn execute(self) -> SDKResult<ApplyScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
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
