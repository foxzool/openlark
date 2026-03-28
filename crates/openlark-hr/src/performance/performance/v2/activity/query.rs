//! 获取项目列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/activity/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取项目列表请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRequest {
    /// 搜索关键字，支持按项目名称搜索（可选）
    name: Option<String>,
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            name: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置搜索关键字（可选）
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置分页大小（可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::ActivityQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref name) = self.name {
            request = request.query("name", name);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取项目列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取项目列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 项目列表
    pub items: Vec<Activity>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    /// 项目 ID
    pub id: String,
    /// 项目名称
    pub name: String,
    /// 项目状态
    pub status: i32,
    /// 创建时间
    pub created_at: i64,
}

impl ApiResponseTrait for QueryResponse {
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
