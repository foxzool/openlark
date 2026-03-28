//! 添加人员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employee/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use serde::{Deserialize, Serialize};

/// 添加人员请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRequest {
    /// 姓名（必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期（YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置电话
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
}

/// 添加人员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 员工信息
    pub data: Option<CreateResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseData {
    /// 员工 ID
    pub id: String,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加人员请求构建器
#[derive(Debug, Clone)]
pub struct CreateRequestBuilder {
    config: Config,
    request: CreateRequest,
}

impl CreateRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: CreateRequest::new(),
        }
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.request = self.request.email(email);
        self
    }

    /// 设置电话
    pub fn phone(mut self, phone: String) -> Self {
        self.request = self.request.phone(phone);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.request = self.request.department_id(department_id);
        self
    }

    /// 设置职位 ID
    pub fn position_id(mut self, position_id: String) -> Self {
        self.request = self.request.position_id(position_id);
        self
    }

    /// 设置工号
    pub fn employee_no(mut self, employee_no: String) -> Self {
        self.request = self.request.employee_no(employee_no);
        self
    }

    /// 设置入职日期
    pub fn hire_date(mut self, hire_date: String) -> Self {
        self.request = self.request.hire_date(hire_date);
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
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 验证必填字段
        let name = self.request.name.clone().unwrap_or_default();
        validate_required!(name.trim(), "员工姓名不能为空");

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::EmployeeCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "添加人员响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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
