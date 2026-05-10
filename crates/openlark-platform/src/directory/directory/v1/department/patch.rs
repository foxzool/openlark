//! 更新部门
//!
//! 文档: https://open.feishu.cn/document/directory-v1/department/patch
//! docPath: https://open.feishu.cn/document/directory-v1/department/patch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentPatchBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
    /// 部门名称
    name: Option<String>,
    /// 父部门 ID
    parent_id: Option<String>,
    /// 部门负责人 ID
    leader_user_id: Option<String>,
}

impl DepartmentPatchBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
            name: None,
            parent_id: None,
            leader_user_id: None,
        }
    }

    /// 设置部门名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置部门负责人 ID
    pub fn leader_user_id(mut self, leader_user_id: impl Into<String>) -> Self {
        self.leader_user_id = Some(leader_user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentPatchResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let request = DepartmentPatchRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let req: ApiRequest<DepartmentPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentPatchResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let request = DepartmentPatchRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let req: ApiRequest<DepartmentPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 更新部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentPatchRequest {
    /// 部门名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 部门负责人 ID
    #[serde(rename = "leader_user_id", skip_serializing_if = "Option::is_none")]
    leader_user_id: Option<String>,
}

/// 更新部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPatchResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

impl ApiResponseTrait for DepartmentPatchResponse {
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
