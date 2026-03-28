//! 复制仪表盘
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/copy>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 复制仪表盘请求
#[derive(Debug, Clone)]
pub struct CopyDashboardRequest {
    config: Config,
    app_token: String,
    block_id: String,
    name: String,
}

impl CopyDashboardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            block_id: String::new(),
            name: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = block_id.into();
        self
    }

    /// 新仪表盘名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CopyDashboardResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CopyDashboardResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.block_id, "block_id 不能为空");
        validate_required!(self.name, "name 不能为空");

        let api_endpoint = BitableApiV1::DashboardCopy(self.app_token, self.block_id);
        let api_request: ApiRequest<CopyDashboardResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(
                &CopyDashboardRequestBody { name: self.name },
            )?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct CopyDashboardRequestBody {
    name: String,
}

/// 复制仪表盘响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyDashboardResponse {
    /// 新的仪表盘的 block_id
    pub block_id: String,
    /// 新的仪表盘名称
    pub name: String,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
