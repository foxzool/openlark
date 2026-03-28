//! 更新部门 ID
//!
//! docPath: <https://open.feishu.cn/document/contact-v3/department/update_department_id>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 更新部门 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentIdBody {
    pub new_department_id: String,
}

impl UpdateDepartmentIdBody {
    pub fn new(new_department_id: impl Into<String>) -> Self {
        Self {
            new_department_id: new_department_id.into(),
        }
    }
}

/// 更新部门 ID 请求
///
/// 用于更新部门的自定义 ID。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `department_id`: 部门 ID，必填
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `new_department_id`: 新部门 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = UpdateDepartmentIdBody::new("new_dept_id_123");
/// let request = UpdateDepartmentIdRequest::new(config)
///     .department_id("old_dept_id")
///     .department_id_type(DepartmentIdType::OpenDepartmentId);
/// ```
pub struct UpdateDepartmentIdRequest {
    config: Config,
    department_id: String,
    department_id_type: Option<DepartmentIdType>,
}

impl UpdateDepartmentIdRequest {
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
    /// docPath: <https://open.feishu.cn/document/contact-v3/department/update_department_id>
    pub async fn execute(self, body: UpdateDepartmentIdBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateDepartmentIdBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.department_id, "department_id 不能为空");
        validate_required!(body.new_department_id, "new_department_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/departments/:department_id/update_department_id
        let mut req: ApiRequest<EmptyData> = ApiRequest::patch(format!(
            "{}/{}/update_department_id",
            CONTACT_V3_DEPARTMENTS, self.department_id
        ))
        .body(serialize_params(&body, "更新部门 ID")?);

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新部门 ID")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_update_department_id_request_builder() {
        let config = Config::default();
        let request = UpdateDepartmentIdRequest::new(config).department_id("old_dept_id");
        assert_eq!(request.department_id, "old_dept_id");
    }

    #[test]
    fn test_update_department_id_request_with_department_id_type() {
        let config = Config::default();
        let request = UpdateDepartmentIdRequest::new(config)
            .department_id("old_dept_id")
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }

    #[test]
    fn test_update_department_id_body_builder() {
        let body = UpdateDepartmentIdBody::new("new_dept_id_123");
        assert_eq!(body.new_department_id, "new_dept_id_123");
    }

    #[test]
    fn test_update_department_id_request_default_values() {
        let config = Config::default();
        let request = UpdateDepartmentIdRequest::new(config);
        assert_eq!(request.department_id, "");
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_update_department_id_request_with_all_options() {
        let config = Config::default();
        let request = UpdateDepartmentIdRequest::new(config)
            .department_id("dept_123")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(request.department_id, "dept_123");
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }
}
