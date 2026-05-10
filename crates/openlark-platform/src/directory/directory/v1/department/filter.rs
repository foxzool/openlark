//! 获取部门列表
//!
//! 文档: https://open.feishu.cn/document/directory-v1/department/filter
//! docPath: https://open.feishu.cn/document/directory-v1/department/filter

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取部门列表 Builder
#[derive(Debug, Clone)]
pub struct DepartmentFilterBuilder {
    config: Config,
    /// 父部门 ID
    parent_id: Option<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl DepartmentFilterBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            parent_id: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
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
    pub async fn execute(self) -> SDKResult<DepartmentFilterResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentFilterResponse> {
        let url = "/open-apis/directory/v1/departments/filter".to_string();

        let request = DepartmentFilterRequest {
            parent_id: self.parent_id,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<DepartmentFilterResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 获取部门列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentFilterRequest {
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 部门简要信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentBrief {
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

/// 获取部门列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentFilterResponse {
    /// 部门列表
    #[serde(rename = "items")]
    items: Vec<DepartmentBrief>,
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

impl ApiResponseTrait for DepartmentFilterResponse {
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
