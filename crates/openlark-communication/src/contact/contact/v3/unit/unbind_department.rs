//! 解除部门与单位的绑定关系
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/unbind_department>

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
    endpoints::CONTACT_V3_UNIT_UNBIND_DEPARTMENT,
};

/// 解除部门与单位的绑定关系请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentBody {
    /// 单位 ID。
    pub unit_id: String,
    /// 部门 ID。
    pub department_id: String,
    /// 部门 ID 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<DepartmentIdType>,
}

/// 解除部门与单位的绑定关系请求
///
/// 用于把某个部门从指定单位下解绑。
pub struct UnbindDepartmentRequest {
    config: Config,
}

impl UnbindDepartmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/unbind_department>
    pub async fn execute(self, body: UnbindDepartmentBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UnbindDepartmentBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(body.unit_id, "unit_id 不能为空");
        validate_required!(body.department_id, "department_id 不能为空");

        // url: POST:/open-apis/contact/v3/unit/unbind_department
        let req: ApiRequest<EmptyData> = ApiRequest::post(CONTACT_V3_UNIT_UNBIND_DEPARTMENT)
            .body(serialize_params(&body, "解除部门与单位的绑定关系")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "解除部门与单位的绑定关系")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/contact/v3/unit/unbind_department
    #[tokio::test]
    async fn test_unbind_department_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/contact/v3/unit/unbind_department"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: UnbindDepartmentBody =
            serde_json::from_value(json!({ "unit_id": "test001", "department_id": "test001" }))
                .expect("body 构造");
        UnbindDepartmentRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
