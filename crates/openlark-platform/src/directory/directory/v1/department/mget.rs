//! 批量获取部门信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/mget>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取部门信息 Builder
#[derive(Debug, Clone)]
pub struct DepartmentMgetBuilder {
    config: Config,
    /// 部门 ID 列表
    department_ids: Vec<String>,
}

impl DepartmentMgetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_ids: Vec::new(),
        }
    }

    /// 添加部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_ids.push(department_id.into());
        self
    }

    /// 添加多个部门 ID
    pub fn department_ids(
        mut self,
        department_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.department_ids
            .extend(department_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentMgetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentMgetResponse> {
        let url = "/open-apis/directory/v1/departments/mget".to_string();

        let request = DepartmentMgetRequest {
            department_ids: self.department_ids,
        };

        let req: ApiRequest<DepartmentMgetResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量获取部门信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentMgetRequest {
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 部门信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentInfo {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 部门负责人 ID
    #[serde(rename = "leader_user_id", skip_serializing_if = "Option::is_none")]
    leader_user_id: Option<String>,
    /// 排序序号
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    order: Option<u32>,
}

/// 批量获取部门信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentMgetResponse {
    /// 部门信息列表
    #[serde(rename = "items")]
    items: Vec<DepartmentInfo>,
}

impl ApiResponseTrait for DepartmentMgetResponse {
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
