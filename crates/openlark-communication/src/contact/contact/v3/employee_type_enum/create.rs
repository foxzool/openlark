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
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/contact/v3/employee_type_enums
    #[tokio::test]
    async fn test_create_employee_type_enum_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/contact/v3/employee_type_enums"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "employee_type_enum": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: CreateEmployeeTypeEnumBody = serde_json::from_value(
            json!({ "content": "test001", "enum_type": 0, "enum_status": 0 }),
        )
        .expect("body 构造");
        CreateEmployeeTypeEnumRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
