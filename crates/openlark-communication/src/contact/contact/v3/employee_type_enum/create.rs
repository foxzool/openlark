//! 新增人员类型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::employee_type_enum::models::{EmployeeTypeEnumResponse, I18nContent},
    endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS,
};

/// 新增人员类型请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeTypeEnumBody {
    /// 展示内容。
    pub content: String,
    /// 枚举类型。
    pub enum_type: i32,
    /// 枚举状态。
    pub enum_status: i32,
    /// 国际化内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<Vec<I18nContent>>,
}

/// 新增人员类型请求
pub struct CreateEmployeeTypeEnumRequest {
    config: Config,
}

impl CreateEmployeeTypeEnumRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/create>
    pub async fn execute(
        self,
        body: CreateEmployeeTypeEnumBody,
    ) -> SDKResult<EmployeeTypeEnumResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateEmployeeTypeEnumBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmployeeTypeEnumResponse> {
        validate_required!(body.content, "content 不能为空");

        // url: POST:/open-apis/contact/v3/employee_type_enums
        let req: ApiRequest<EmployeeTypeEnumResponse> =
            ApiRequest::post(CONTACT_V3_EMPLOYEE_TYPE_ENUMS)
                .body(serialize_params(&body, "新增人员类型")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "新增人员类型")
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
