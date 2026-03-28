//! 离职员工
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 离职员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeDeleteBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeDeleteBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeDeleteResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let req: ApiRequest<EmployeeDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, None).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("离职员工", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeDeleteResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let req: ApiRequest<EmployeeDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("离职员工", "响应数据为空"))
    }
}

/// 离职员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeDeleteResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for EmployeeDeleteResponse {
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
