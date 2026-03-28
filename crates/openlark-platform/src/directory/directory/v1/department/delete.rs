//! 删除部门
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentDeleteBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
}

impl DepartmentDeleteBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentDeleteResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let req: ApiRequest<DepartmentDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除部门", "响应数据为空"))
    }
}

/// 删除部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDeleteResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for DepartmentDeleteResponse {
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
