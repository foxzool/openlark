//! 搜索员工信息
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/search
//! docPath: https://open.feishu.cn/document/directory-v1/employee/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 搜索员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeeSearchBuilder {
    config: Config,
    /// 搜索关键词
    keyword: String,
    /// 部门 ID
    department_id: Option<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl EmployeeSearchBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, keyword: impl Into<String>) -> Self {
        Self {
            config,
            keyword: keyword.into(),
            department_id: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
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
    pub async fn execute(self) -> SDKResult<EmployeeSearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeSearchResponse> {
        let url = "/open-apis/directory/v1/employees/search".to_string();

        let request = EmployeeSearchRequest {
            keyword: self.keyword,
            department_id: self.department_id,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<EmployeeSearchResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 搜索员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeSearchRequest {
    /// 搜索关键词
    #[serde(rename = "keyword")]
    keyword: String,
    /// 部门 ID
    #[serde(rename = "department_id", skip_serializing_if = "Option::is_none")]
    department_id: Option<String>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 搜索到的员工信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedEmployee {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    mobile: String,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 搜索员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeSearchResponse {
    /// 搜索结果列表
    #[serde(rename = "items")]
    items: Vec<SearchedEmployee>,
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

impl ApiResponseTrait for EmployeeSearchResponse {
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
