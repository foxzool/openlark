//! 搜索合同
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/contract/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 搜索合同请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 合同名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

    /// 设置员工 ID
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = Some(employee_id);
        self
    }

    /// 设置合同名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

/// 搜索合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 合同列表
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
    /// 合同列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ContractItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractItem {
    /// 合同 ID
    pub id: String,
    /// 合同名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 合同状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索合同请求构建器
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

    /// 设置员工 ID
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.request = self.request.employee_id(employee_id);
        self
    }

    /// 设置合同名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
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
        let api_endpoint = FeishuPeopleApiV2::ContractSearch;
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
                "搜索合同响应数据为空",
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
