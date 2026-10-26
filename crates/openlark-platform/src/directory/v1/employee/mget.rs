//! 批量获取员工信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/mget>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/mget>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeeMgetRequestBuilder {
    config: Config,
    /// 员工 ID 列表
    employee_ids: Vec<String>,
    /// 需要查询的字段列表
    required_fields: Vec<String>,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl EmployeeMgetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: Vec::new(),
            required_fields: Vec::new(),
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 添加员工 ID
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.employee_ids.push(employee_id.into());
        self
    }

    /// 添加多个员工 ID
    pub fn employee_ids(
        mut self,
        employee_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.employee_ids
            .extend(employee_ids.into_iter().map(Into::into));
        self
    }

    /// 添加需要查询的字段（如 "base_info.mobile"）
    pub fn required_field(mut self, field: impl Into<String>) -> Self {
        self.required_fields.push(field.into());
        self
    }

    /// 设置需要查询的字段列表
    pub fn required_fields(mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.required_fields = fields.into_iter().map(Into::into).collect();
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
    pub async fn execute(self) -> SDKResult<EmployeeMgetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeMgetResponse> {
        validate_required_list!(
            self.employee_ids,
            100,
            "employee_ids 不能为空且不能超过 100 个"
        );
        validate_required!(self.required_fields, "required_fields 不能为空");

        let url = "/open-apis/directory/v1/employees/mget";

        let body = EmployeeMgetBody {
            employee_ids: self.employee_ids,
            required_fields: self.required_fields,
        };

        let req: ApiRequest<EmployeeMgetResponse> = ApiRequest::post(url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "批量获取员工信息").await
    }
}

/// 批量获取员工信息请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeMgetBody {
    /// 员工 ID 列表，与 employee_id_type 类型保持一致（1~100 个）
    pub employee_ids: Vec<String>,
    /// 需要查询的字段列表，如 ["base_info.mobile"]（不传则不会返回任何字段）
    pub required_fields: Vec<String>,
}

/// 员工信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeInfo {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    pub name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    pub mobile: String,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    pub department_ids: Vec<String>,
    /// 状态
    #[serde(rename = "status")]
    pub status: String,
}

/// 批量获取员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeMgetResponse {
    /// 员工信息列表
    #[serde(rename = "items")]
    pub items: Vec<EmployeeInfo>,
}

impl ApiResponseTrait for EmployeeMgetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeMgetRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeMgetBuilder = EmployeeMgetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST 官方 employee_ids + required_fields 请求体 → 强类型 EmployeeMgetResponse。
    #[tokio::test]
    async fn test_mget_employees_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/mget"))
            .and(query_param("employee_id_type", "open_id"))
            .and(body_json(json!({
                "employee_ids": ["emp_001", "emp_002"],
                "required_fields": ["base_info.mobile"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employee_id": "emp_001",
                            "name": "alice",
                            "mobile": "13800000001",
                            "email": "alice@example.com",
                            "department_ids": ["dept_001"],
                            "status": "active"
                        },
                        {
                            "employee_id": "emp_002",
                            "name": "bob",
                            "mobile": "13800000002",
                            "department_ids": ["dept_002"],
                            "status": "active"
                        }
                    ]
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

        let resp = EmployeeMgetRequestBuilder::new(config)
            .employee_ids(["emp_001", "emp_002"])
            .required_fields(["base_info.mobile"])
            .employee_id_type("open_id")
            .execute()
            .await
            .expect("批量获取员工信息应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].employee_id, "emp_001");
        assert_eq!(resp.items[0].name, "alice");
        assert_eq!(resp.items[1].employee_id, "emp_002");
        assert_eq!(resp.items[1].email, None);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/mget"
        );
        assert_eq!(received[0].method, "POST");
    }

    /// 必填校验：employee_ids 为空时不发请求。
    #[tokio::test]
    async fn test_mget_employees_validates_required_fields() {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = EmployeeMgetRequestBuilder::new(config)
            .required_fields(["base_info.mobile"])
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
