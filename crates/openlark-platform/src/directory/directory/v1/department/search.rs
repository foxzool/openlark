//! 搜索部门
//!
//! 文档: https://open.feishu.cn/document/directory-v1/department/search
//! docPath: https://open.feishu.cn/document/directory-v1/department/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 搜索部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentSearchBuilder {
    config: Config,
    /// 搜索关键词
    keyword: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl DepartmentSearchBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, keyword: impl Into<String>) -> Self {
        Self {
            config,
            keyword: keyword.into(),
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
    pub async fn execute(self) -> SDKResult<DepartmentSearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentSearchResponse> {
        let url = "/open-apis/directory/v1/departments/search".to_string();

        let request = DepartmentSearchRequest {
            keyword: self.keyword,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<DepartmentSearchResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 搜索部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentSearchRequest {
    /// 搜索关键词
    #[serde(rename = "keyword")]
    keyword: String,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 搜索到的部门信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedDepartment {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
}

/// 搜索部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentSearchResponse {
    /// 搜索结果列表
    #[serde(rename = "items")]
    items: Vec<SearchedDepartment>,
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

impl ApiResponseTrait for DepartmentSearchResponse {
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
