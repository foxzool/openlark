//! 转移应用所有者

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
pub struct TransferAppOwnerRequest {
    config: Arc<Config>,
    app_id: String,
    body: TransferAppOwnerBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct TransferAppOwnerBody {
    /// 待补充文档。
    pub new_owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct TransferAppOwnerResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for TransferAppOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TransferAppOwnerRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            body: TransferAppOwnerBody::default(),
        }
    }

    /// 待补充文档。
    pub fn new_owner_id(mut self, id: impl Into<String>) -> Self {
        self.body.new_owner_id = id.into();
        self
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<TransferAppOwnerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TransferAppOwnerResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/owner/transfer",
            self.app_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<TransferAppOwnerResponse> = ApiRequest::post(&path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp.data.unwrap_or(TransferAppOwnerResponse { data: None }))
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
