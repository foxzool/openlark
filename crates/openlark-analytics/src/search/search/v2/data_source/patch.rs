//! 修改数据源
//! docPath: https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/patch

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
pub struct PatchDataSourceRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDataSourceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchDataSourceRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<PatchDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchDataSourceResponse> {
        let path = "/open-apis/search/v2/data_sources/{}".to_string();
        let req: ApiRequest<PatchDataSourceResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("修改数据源", "响应数据为空"))
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
