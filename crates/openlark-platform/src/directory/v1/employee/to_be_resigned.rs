//! 更新在职员工为待离职
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/to_be_resigned>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/to_be_resigned>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新在职员工为待离职 Builder
#[derive(Debug, Clone)]
pub struct EmployeeToBeResignedRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 在职员工流转到待离职
    employee: SetEmployeePreResigned,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl EmployeeToBeResignedRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            employee: SetEmployeePreResigned {
                resign_date: String::new(),
                resign_reason: String::new(),
                resign_type: String::new(),
                resign_remark: None,
            },
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置离职日期（固定格式 yyyy-mm-dd）
    pub fn resign_date(mut self, resign_date: impl Into<String>) -> Self {
        self.employee.resign_date = resign_date.into();
        self
    }

    /// 设置离职原因（官方枚举 "1"~"25"）
    pub fn resign_reason(mut self, resign_reason: impl Into<String>) -> Self {
        self.employee.resign_reason = resign_reason.into();
        self
    }

    /// 设置离职类型：1 主动 / 2 被动 / 3 其他
    pub fn resign_type(mut self, resign_type: impl Into<String>) -> Self {
        self.employee.resign_type = resign_type.into();
        self
    }

    /// 设置离职备注（0~255 字符）
    pub fn resign_remark(mut self, resign_remark: impl Into<String>) -> Self {
        self.employee.resign_remark = Some(resign_remark.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）
    pub fn employee_id_type(mut self, employee_id_type: impl Into<String>) -> Self {
        self.employee_id_type = Some(employee_id_type.into());
        self
    }

    /// 设置部门 ID 类型（查询参数）
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeToBeResignedResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeToBeResignedResponse> {
        validate_required!(self.employee_id, "employee_id 不能为空");
        validate_required!(self.employee.resign_date, "resign_date 不能为空");
        validate_required!(self.employee.resign_reason, "resign_reason 不能为空");
        validate_required!(self.employee.resign_type, "resign_type 不能为空");

        let url = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            self.employee_id
        );

        let body = EmployeeToBeResignedBody {
            employee: self.employee,
        };

        let req: ApiRequest<EmployeeToBeResignedResponse> = ApiRequest::patch(&url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "更新在职员工为待离职").await
    }
}

/// 更新在职员工为待离职请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeToBeResignedBody {
    /// 在职员工流转到待离职
    pub employee: SetEmployeePreResigned,
}

/// 待离职信息（官方 set_employee_pre_resigned）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetEmployeePreResigned {
    /// 离职日期（固定格式 yyyy-mm-dd）
    pub resign_date: String,
    /// 离职原因（官方枚举 "1"~"25"）
    pub resign_reason: String,
    /// 离职类型：1 主动 / 2 被动 / 3 其他
    pub resign_type: String,
    /// 离职备注（0~255 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_remark: Option<String>,
}

/// 更新在职员工为待离职响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeToBeResignedResponse {
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

impl ApiResponseTrait for EmployeeToBeResignedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to EmployeeToBeResignedRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type EmployeeToBeResignedBuilder = EmployeeToBeResignedRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH 官方 `{"employee": {...}}` 请求体 → 强类型 EmployeeToBeResignedResponse。
    #[tokio::test]
    async fn test_to_be_resigned_employee_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/directory/v1/employees/emp_001/to_be_resigned",
            ))
            .and(body_json(json!({
                "employee": {
                    "resign_date": "2024-06-21",
                    "resign_reason": "9",
                    "resign_type": "1",
                    "resign_remark": "contract ended"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "status": "to_be_resigned",
                    "message": "updated to to_be_resigned"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EmployeeToBeResignedRequestBuilder::new(config, "emp_001")
            .resign_date("2024-06-21")
            .resign_reason("9")
            .resign_type("1")
            .resign_remark("contract ended")
            .execute()
            .await
            .expect("更新在职员工为待离职应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.status, "to_be_resigned");
        assert_eq!(resp.message, "updated to to_be_resigned");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001/to_be_resigned"
        );
        assert_eq!(received[0].method, "PATCH");
    }

    /// 必填校验：resign_date 为空时不发请求。
    #[tokio::test]
    async fn test_to_be_resigned_validates_required_fields() {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = EmployeeToBeResignedRequestBuilder::new(config, "emp_001")
            .resign_reason("9")
            .resign_type("1")
            .execute()
            .await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
