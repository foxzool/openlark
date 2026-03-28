//! 部门群转为普通群
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/unbind_department_chat>

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
    endpoints::CONTACT_V3_DEPARTMENTS_UNBIND_DEPARTMENT_CHAT,
};

/// 部门群转为普通群请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentChatBody {
    pub department_id: String,
}

impl UnbindDepartmentChatBody {
    pub fn new(department_id: impl Into<String>) -> Self {
        Self {
            department_id: department_id.into(),
        }
    }
}

/// 部门群转为普通群请求
///
/// 用于将部门群转换为普通群。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `department_id`: 部门 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = UnbindDepartmentChatBody::new("dept_xxx");
/// let request = UnbindDepartmentChatRequest::new(config)
///     .department_id_type(DepartmentIdType::OpenDepartmentId);
/// ```
pub struct UnbindDepartmentChatRequest {
    config: Config,
    department_id_type: Option<DepartmentIdType>,
}

impl UnbindDepartmentChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id_type: None,
        }
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/unbind_department_chat>
    pub async fn execute(self, body: UnbindDepartmentChatBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UnbindDepartmentChatBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(body.department_id, "department_id 不能为空");

        // url: POST:/open-apis/contact/v3/departments/unbind_department_chat
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::post(CONTACT_V3_DEPARTMENTS_UNBIND_DEPARTMENT_CHAT)
                .body(serialize_params(&body, "部门群转为普通群")?);

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "部门群转为普通群")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_unbind_department_chat_request_builder() {
        let config = Config::default();
        let request = UnbindDepartmentChatRequest::new(config);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_unbind_department_chat_request_with_department_id_type() {
        let config = Config::default();
        let request = UnbindDepartmentChatRequest::new(config)
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }

    #[test]
    fn test_unbind_department_chat_body_builder() {
        let body = UnbindDepartmentChatBody::new("dept_xxx");
        assert_eq!(body.department_id, "dept_xxx");
    }

    #[test]
    fn test_unbind_department_chat_request_with_all_options() {
        let config = Config::default();
        let request = UnbindDepartmentChatRequest::new(config)
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }
}
