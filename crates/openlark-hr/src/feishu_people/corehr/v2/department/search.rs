//! 搜索部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 搜索部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
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

    /// 设置部门名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// 设置部门编码
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

/// 搜索部门信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 部门列表
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
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentItem {
    /// 部门 ID
    pub id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索部门信息请求构建器
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

    /// 设置部门名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.request = self.request.parent_id(parent_id);
        self
    }

    /// 设置部门编码
    pub fn code(mut self, code: String) -> Self {
        self.request = self.request.code(code);
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
        let api_endpoint = FeishuPeopleApiV2::DepartmentSearch;
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
                "搜索部门信息响应数据为空",
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
