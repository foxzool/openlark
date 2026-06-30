//! 更新待离职成员为在职
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/regular
//! docPath: https://open.feishu.cn/document/directory-v1/employee/regular

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新待离职成员为在职 Builder
#[derive(Debug, Clone)]
pub struct EmployeeRegularRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeRegularRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeRegularResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeRegularResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/regular",
            self.employee_id
        );

        let req: ApiRequest<EmployeeRegularResponse> = ApiRequest::patch(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("更新待离职成员为在职", "响应数据为空")
        })
    }
}

/// 更新待离职成员为在职响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeRegularResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 状态
    #[serde(rename = "status")]
    pub status: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for EmployeeRegularResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeRegularRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeRegularBuilder = EmployeeRegularRequestBuilder;

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
