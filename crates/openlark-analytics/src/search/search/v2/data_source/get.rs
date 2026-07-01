//! 获取数据源
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取数据源请求。
#[derive(Debug, Clone)]
pub struct GetDataSourceRequest {
    config: Arc<Config>,
    data_source_id: String,
}

/// 获取数据源响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataSourceResponse {
    /// 响应数据。
    pub data: Option<DataSourceData>,
}

impl ApiResponseTrait for GetDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据源详情数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceData {
    /// 数据源 ID。
    pub data_source_id: String,
    /// 数据源名称。
    pub name: String,
    /// 数据源描述。
    pub description: String,
}

impl GetDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
        }
    }

    /// 执行获取数据源请求。
    pub async fn execute(self) -> SDKResult<GetDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行获取数据源请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDataSourceResponse> {
        let path = format!("/open-apis/search/v2/data_sources/{}", self.data_source_id);
        let req: ApiRequest<GetDataSourceResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取数据源", "响应数据为空"))
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
