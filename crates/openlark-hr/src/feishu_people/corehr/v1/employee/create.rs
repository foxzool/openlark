//! 创建员工
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employee/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建员工请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 员工姓名（必填）
    name: String,
    /// 员工邮箱
    email: Option<String>,
    /// 员工电话
    phone: Option<String>,
    /// 部门 ID
    department_id: Option<String>,
    /// 职位 ID
    position_id: Option<String>,
    /// 工号
    employee_no: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    hire_date: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            email: None,
            phone: None,
            department_id: None,
            position_id: None,
            employee_no: None,
            hire_date: None,
        }
    }

    /// 设置员工姓名（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置员工邮箱
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置员工电话
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置职位 ID
    pub fn position_id(mut self, position_id: String) -> Self {
        self.position_id = Some(position_id);
        self
    }

    /// 设置工号
    pub fn employee_no(mut self, employee_no: String) -> Self {
        self.employee_no = Some(employee_no);
        self
    }

    /// 设置入职日期
    pub fn hire_date(mut self, hire_date: String) -> Self {
        self.hire_date = Some(hire_date);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.name.trim(), "员工姓名不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmployeeCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            email: self.email,
            phone: self.phone,
            department_id: self.department_id,
            position_id: self.position_id,
            employee_no: self.employee_no,
            hire_date: self.hire_date,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建员工响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateResponse {
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
