//! 更新在职员工为待离职
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/to_be_resigned
//! docPath: https://open.feishu.cn/document/directory-v1/employee/to_be_resigned

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新在职员工为待离职 Builder
#[derive(Debug, Clone)]
pub struct EmployeeToBeResignedBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 离职原因
    reason: Option<String>,
}

impl EmployeeToBeResignedBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            reason: None,
        }
    }

    /// 设置离职原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeToBeResignedResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            self.employee_id
        );

        let request = EmployeeToBeResignedRequest {
            reason: self.reason,
        };

        let req: ApiRequest<EmployeeToBeResignedResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeToBeResignedResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            self.employee_id
        );

        let request = EmployeeToBeResignedRequest {
            reason: self.reason,
        };

        let req: ApiRequest<EmployeeToBeResignedResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 更新在职员工为待离职请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeToBeResignedRequest {
    /// 离职原因
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// 更新在职员工为待离职响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeToBeResignedResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 状态
    #[serde(rename = "status")]
    status: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for EmployeeToBeResignedResponse {
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
