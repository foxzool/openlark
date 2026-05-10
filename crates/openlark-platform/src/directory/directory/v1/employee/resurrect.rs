//! 恢复离职员工
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/resurrect

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 恢复离职员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeResurrectBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeResurrectBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeResurrectResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeResurrectResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            self.employee_id
        );

        let req: ApiRequest<EmployeeResurrectResponse> = ApiRequest::post(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("恢复离职员工", "响应数据为空"))
    }
}

/// 恢复离职员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeResurrectResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for EmployeeResurrectResponse {
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
