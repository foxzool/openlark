//! 获取父部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/parents>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 获取父部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParentsRequest {
    /// 部门 ID
    pub department_id: String,
}

impl ParentsRequest {
    /// 创建请求
    pub fn new(department_id: String) -> Self {
        Self { department_id }
    }
}

/// 获取父部门信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponse {
    /// 父部门列表
    pub data: Option<ParentsResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponseData {
    /// 父部门列表（按层级从顶部门到直接父部门排序）
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
}

impl ApiResponseTrait for ParentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取父部门信息请求构建器
#[derive(Debug, Clone)]
pub struct ParentsRequestBuilder {
    config: Config,
    department_id: String,
}

impl ParentsRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_id: String) -> Self {
        Self {
            config,
            department_id,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ParentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ParentsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentParents(self.department_id.clone());
        let request = ApiRequest::<ParentsResponse>::get(api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取父部门信息响应数据为空",
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
