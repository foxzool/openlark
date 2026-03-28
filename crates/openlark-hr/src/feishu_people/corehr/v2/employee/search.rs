//! 搜索员工信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employee/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 搜索员工信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置电话
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// 设置员工状态
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}

/// 搜索员工信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 员工列表
    pub data: Option<SearchResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponseData {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 员工列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeItem {
    /// 员工 ID
    pub id: String,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索员工信息请求构建器
#[derive(Debug, Clone)]
pub struct SearchRequestBuilder {
    config: Config,
    request: SearchRequest,
}

impl SearchRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: SearchRequest::new(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.request = self.request.department_id(department_id);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.request = self.request.email(email);
        self
    }

    /// 设置电话
    pub fn phone(mut self, phone: String) -> Self {
        self.request = self.request.phone(phone);
        self
    }

    /// 设置员工状态
    pub fn status(mut self, status: String) -> Self {
        self.request = self.request.status(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::EmployeeSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "搜索员工信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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
