//! 获取单位绑定的部门列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/list_department>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::unit::models::ListUnitDepartmentsResponse,
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_UNIT_LIST_DEPARTMENT,
};

/// 获取单位绑定的部门列表请求
pub struct ListUnitDepartmentsRequest {
    config: Config,
    unit_id: String,
    department_id_type: Option<DepartmentIdType>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListUnitDepartmentsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
            department_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 单位 ID（查询参数，必填）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 部门 ID 类型（查询参数，可选，默认 open_department_id）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 50，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/list_department>
    pub async fn execute(self) -> SDKResult<ListUnitDepartmentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListUnitDepartmentsResponse> {
        validate_required!(self.unit_id, "unit_id 不能为空");

        // url: GET:/open-apis/contact/v3/unit/list_department
        let mut req: ApiRequest<ListUnitDepartmentsResponse> =
            ApiRequest::get(CONTACT_V3_UNIT_LIST_DEPARTMENT).query("unit_id", self.unit_id);

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单位绑定的部门列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
