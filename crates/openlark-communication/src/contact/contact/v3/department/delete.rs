//! 删除部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 删除部门请求
///
/// 用于删除通讯录中的部门。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `department_id`: 部门 ID，必填
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeleteDepartmentRequest::new(config)
///     .department_id("dept_xxx")
///     .department_id_type(DepartmentIdType::OpenDepartmentId);
/// ```
pub struct DeleteDepartmentRequest {
    config: Config,
    department_id: String,
    department_id_type: Option<DepartmentIdType>,
}

impl DeleteDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            department_id_type: None,
        }
    }

    /// 部门 ID（路径参数）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = department_id.into();
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.department_id, "department_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/departments/:department_id
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_DEPARTMENTS, self.department_id));

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除部门")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_department_request_builder() {
        let config = Config::default();
        let request = DeleteDepartmentRequest::new(config).department_id("dept_xxx");
        assert_eq!(request.department_id, "dept_xxx");
    }

    #[test]
    fn test_delete_department_request_with_department_id_type() {
        let config = Config::default();
        let request = DeleteDepartmentRequest::new(config)
            .department_id("dept_xxx")
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }

    #[test]
    fn test_delete_department_request_default_values() {
        let config = Config::default();
        let request = DeleteDepartmentRequest::new(config);
        assert_eq!(request.department_id, "");
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_delete_department_request_with_all_options() {
        let config = Config::default();
        let request = DeleteDepartmentRequest::new(config)
            .department_id("dept_123")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(request.department_id, "dept_123");
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }
}
