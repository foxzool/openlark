//! 删除免审词条
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::LingoApiV1;

/// 删除免审词条响应（data）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteEntityResp {}

impl ApiResponseTrait for DeleteEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除免审词条请求
pub struct DeleteEntityRequest {
    config: Config,
    entity_id: String,
    provider: Option<String>,
    outer_id: Option<String>,
}

impl DeleteEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            provider: None,
            outer_id: None,
        }
    }

    /// 外部系统（使用时需要将路径中的词条 ID 固定为 enterprise_0，并同时提供 provider 与 outer_id）
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// 词条在外部系统中对应的唯一 ID（使用时需要将路径中的词条 ID 固定为 enterprise_0，并同时提供 provider 与 outer_id）
    pub fn outer_id(mut self, outer_id: impl Into<String>) -> Self {
        self.outer_id = Some(outer_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<DeleteEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<DeleteEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");

        let mut api_request: ApiRequest<DeleteEntityResp> =
            ApiRequest::delete(&LingoApiV1::EntityDelete(self.entity_id).to_url());
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(outer_id) = &self.outer_id {
            api_request = api_request.query("outer_id", outer_id);
        }

        let response: Response<DeleteEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_entity_request_builder() {
        let config = Config::default();
        let request = DeleteEntityRequest::new(config, "entity_123")
            .provider("test_provider")
            .outer_id("outer_456");

        assert_eq!(request.entity_id, "entity_123");
        assert_eq!(request.provider, Some("test_provider".to_string()));
        assert_eq!(request.outer_id, Some("outer_456".to_string()));
    }

    #[test]
    fn test_delete_entity_request_minimal() {
        let config = Config::default();
        let request = DeleteEntityRequest::new(config, "entity_789");

        assert_eq!(request.entity_id, "entity_789");
        assert!(request.provider.is_none());
        assert!(request.outer_id.is_none());
    }

    #[test]
    fn test_delete_entity_request_with_string() {
        let config = Config::default();
        let entity_id = String::from("entity_str");
        let request = DeleteEntityRequest::new(config, entity_id);

        assert_eq!(request.entity_id, "entity_str");
    }

    #[test]
    fn test_delete_entity_request_only_provider() {
        let config = Config::default();
        let request = DeleteEntityRequest::new(config, "entity_001").provider("prov_001");

        assert_eq!(request.entity_id, "entity_001");
        assert_eq!(request.provider, Some("prov_001".to_string()));
        assert!(request.outer_id.is_none());
    }

    #[test]
    fn test_delete_entity_request_only_outer_id() {
        let config = Config::default();
        let request = DeleteEntityRequest::new(config, "entity_002").outer_id("out_002");

        assert_eq!(request.entity_id, "entity_002");
        assert!(request.provider.is_none());
        assert_eq!(request.outer_id, Some("out_002".to_string()));
    }
}
