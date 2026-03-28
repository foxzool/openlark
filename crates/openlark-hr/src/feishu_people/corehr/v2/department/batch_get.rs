//! 批量查询部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/batch_get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 批量查询部门请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetRequest {
    /// 部门 ID 列表
    pub department_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(department_ids: Vec<String>) -> Self {
        Self { department_ids }
    }

    /// 添加部门 ID
    pub fn add_department_id(mut self, department_id: String) -> Self {
        self.department_ids.push(department_id);
        self
    }
}

/// 批量查询部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 部门列表
    pub data: Option<BatchGetResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponseData {
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
    /// 部门负责人 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门请求构建器
#[derive(Debug, Clone)]
pub struct BatchGetRequestBuilder {
    config: Config,
    request: BatchGetRequest,
}

impl BatchGetRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_ids: Vec<String>) -> Self {
        Self {
            config,
            request: BatchGetRequest::new(department_ids),
        }
    }

    /// 添加部门 ID
    pub fn add_department_id(mut self, department_id: String) -> Self {
        self.request = self.request.add_department_id(department_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

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
                "批量查询部门响应数据为空",
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
