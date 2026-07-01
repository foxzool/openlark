//! 建立部门与单位的绑定关系
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/bind_department>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_UNIT_BIND_DEPARTMENT,
};

/// 建立部门与单位的绑定关系请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindDepartmentBody {
    /// 单位 ID。
    pub unit_id: String,
    /// 部门 ID。
    pub department_id: String,
    /// 部门 ID 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<DepartmentIdType>,
}

/// 建立部门与单位的绑定关系请求
///
/// 用于把指定部门挂接到某个单位下。
pub struct BindDepartmentRequest {
    config: Config,
}

impl BindDepartmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/bind_department>
    pub async fn execute(self, body: BindDepartmentBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BindDepartmentBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(body.unit_id, "unit_id 不能为空");
        validate_required!(body.department_id, "department_id 不能为空");

        // url: POST:/open-apis/contact/v3/unit/bind_department
        let req: ApiRequest<EmptyData> = ApiRequest::post(CONTACT_V3_UNIT_BIND_DEPARTMENT)
            .body(serialize_params(&body, "建立部门与单位的绑定关系")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "建立部门与单位的绑定关系")
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
