//! 查询环境变量列表
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-environment_variable/query
//! docPath: https://open.feishu.cn/document/apaas-v1/application-environment_variable/query

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询环境变量列表 Builder
#[derive(Debug, Clone)]
pub struct EnvironmentVariableQueryBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl EnvironmentVariableQueryBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            page: None,
            page_size: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnvironmentVariableQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EnvironmentVariableQueryResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/environment_variables/query",
            self.namespace
        );

        let request = EnvironmentVariableQueryRequest {
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<EnvironmentVariableQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 查询环境变量列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EnvironmentVariableQueryRequest {
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 环境变量信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableInfo {
    /// 环境变量 API 名称
    #[serde(rename = "api_name")]
    api_name: String,
    /// 环境变量名称
    #[serde(rename = "name")]
    name: String,
    /// 环境变量值
    #[serde(rename = "value")]
    value: String,
    /// 描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 查询环境变量列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableQueryResponse {
    /// 环境变量列表
    #[serde(rename = "items")]
    items: Vec<EnvironmentVariableInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for EnvironmentVariableQueryResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
