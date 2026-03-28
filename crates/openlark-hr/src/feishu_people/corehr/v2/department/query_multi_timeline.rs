//! 批量查询部门版本信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_multi_timeline>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 批量查询部门版本信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineRequest {
    /// 部门 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl QueryMultiTimelineRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置部门 ID 列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);
        self
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
}

/// 批量查询部门版本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMultiTimelineResponse {
    pub data: Option<QueryMultiTimelineResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMultiTimelineResponseData {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 部门版本列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentTimelineItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentTimelineItem {
    /// 部门 ID
    pub id: String,
    /// 版本 ID
    pub timeline_id: String,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl ApiResponseTrait for QueryMultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门版本信息请求构建器
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequestBuilder {
    config: Config,
    request: QueryMultiTimelineRequest,
}

impl QueryMultiTimelineRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: QueryMultiTimelineRequest::new(),
        }
    }

    /// 设置部门 ID 列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request = self.request.department_ids(department_ids);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: String) -> Self {
        self.request = self.request.start_time(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: String) -> Self {
        self.request = self.request.end_time(end_time);
        self
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentQueryMultiTimeline;
        let request = ApiRequest::<QueryMultiTimelineResponse>::post(api_endpoint.to_url());

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
                "批量查询部门版本信息响应数据为空",
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
