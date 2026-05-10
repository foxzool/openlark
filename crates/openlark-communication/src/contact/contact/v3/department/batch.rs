//! 批量获取部门信息
//!
//! docPath: https://open.feishu.cn/document/contact-v3/department/batch

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required_list,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        department::models::BatchGetDepartmentsResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS_BATCH,
};

/// 批量获取部门信息请求
///
/// 用于通过部门 ID 批量获取部门详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `department_ids`: 部门 ID 列表，必填（1-50 个）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = BatchGetDepartmentsRequest::new(config)
///     .push_department_id("dept_1")
///     .push_department_id("dept_2")
///     .department_id_type(DepartmentIdType::OpenDepartmentId);
/// ```
pub struct BatchGetDepartmentsRequest {
    config: Config,
    department_ids: Vec<String>,
    department_id_type: Option<DepartmentIdType>,
    user_id_type: Option<UserIdType>,
}

impl BatchGetDepartmentsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_ids: Vec::new(),
            department_id_type: None,
            user_id_type: None,
        }
    }

    /// 追加一个部门 ID（查询参数 department_ids，可多次传递，1-50 个）
    pub fn push_department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_ids.push(department_id.into());
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/contact-v3/department/batch
    pub async fn execute(self) -> SDKResult<BatchGetDepartmentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetDepartmentsResponse> {
        // === 必填字段验证 ===
        validate_required_list!(self.department_ids, 50, "department_ids 不能为空");

        // url: GET:/open-apis/contact/v3/departments/batch
        let mut req: ApiRequest<BatchGetDepartmentsResponse> =
            ApiRequest::get(CONTACT_V3_DEPARTMENTS_BATCH);

        for department_id in self.department_ids {
            req = req.query("department_ids", department_id);
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "批量获取部门信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_departments_request_builder() {
        let config = Config::default();
        let request = BatchGetDepartmentsRequest::new(config)
            .push_department_id("dept_1")
            .push_department_id("dept_2");
        assert_eq!(request.department_ids.len(), 2);
    }

    #[test]
    fn test_batch_get_departments_request_with_department_id_type() {
        let config = Config::default();
        let request = BatchGetDepartmentsRequest::new(config)
            .push_department_id("dept_1")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }

    #[test]
    fn test_batch_get_departments_request_with_user_id_type() {
        let config = Config::default();
        let request = BatchGetDepartmentsRequest::new(config)
            .push_department_id("dept_1")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_batch_get_departments_request_default_values() {
        let config = Config::default();
        let request = BatchGetDepartmentsRequest::new(config);
        assert_eq!(request.department_ids.len(), 0);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_batch_get_departments_request_with_all_options() {
        let config = Config::default();
        let request = BatchGetDepartmentsRequest::new(config)
            .push_department_id("dept_1")
            .push_department_id("dept_2")
            .department_id_type(DepartmentIdType::OpenDepartmentId)
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.department_ids.len(), 2);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}
