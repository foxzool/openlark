//! 编辑被动解除补偿金信息。
//!
//! `POST /open-apis/corehr/v2/severance_pays/edit`，只接受 tenant access token。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/severance_pay/edit>

use super::models::{ObjectFieldData, optional_text, require_object_fields, take_bounded_text};
use crate::common::api_endpoints::FeishuPeopleApiV2;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use openlark_core::{CoreError, SDKResult, validate_allow_empty_list};
use serde::{Deserialize, Serialize};

/// 编辑被动解除补偿金信息。
#[derive(Debug, Clone)]
pub struct EditRequest {
    config: Config,
    user_id_type: Option<String>,
    severance_pay_id: Option<String>,
    operator_id: Option<String>,
    update_data: Option<Vec<ObjectFieldData>>,
}

/// 编辑被动解除补偿金信息的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditRequestBody {
    /// 补偿金业务单 ID。长度 1 到 64。
    pub severance_pay_id: String,
    /// 操作人雇佣 ID。为空时服务端按系统操作处理。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 待更新字段。允许空数组，最多 10000 项。
    pub update_data: Vec<ObjectFieldData>,
}

/// 编辑被动解除补偿金信息的响应数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EditResponse {
    /// 编辑后的字段。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ObjectFieldData>>,
}

impl ApiResponseTrait for EditResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EditRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            severance_pay_id: None,
            operator_id: None,
            update_data: None,
        }
    }

    /// 用户 ID 类型。不传则不放进查询参数。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 补偿金业务单 ID。
    pub fn severance_pay_id(mut self, severance_pay_id: impl Into<String>) -> Self {
        self.severance_pay_id = Some(severance_pay_id.into());
        self
    }

    /// 操作人雇佣 ID。
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.operator_id = Some(operator_id.into());
        self
    }

    /// 待更新字段。空数组合法。
    pub fn update_data(mut self, update_data: Vec<ObjectFieldData>) -> Self {
        self.update_data = Some(update_data);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<EditResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EditResponse> {
        let severance_pay_id = take_bounded_text(
            self.severance_pay_id,
            64,
            "severance_pay_id 不能为空且不能超过 64 个字符",
        )?;
        optional_text(
            &self.operator_id,
            64,
            "operator_id 不能为空且不能超过 64 个字符",
        )?;
        let Some(update_data) = self.update_data else {
            return Err(CoreError::validation_msg("update_data 必须提供"));
        };
        validate_allow_empty_list!(update_data, 10000, "update_data 不能超过 10000 个");
        require_object_fields(&update_data)?;
        let body = EditRequestBody {
            severance_pay_id,
            operator_id: self.operator_id,
            update_data,
        };
        let request =
            ApiRequest::<EditResponse>::post(FeishuPeopleApiV2::SeverancePayEdit.to_url())
                .with_supported_access_token_types(vec![AccessTokenType::Tenant])
                .query_opt("user_id_type", self.user_id_type)
                .body(serde_json::to_value(&body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "编辑被动解除补偿金信息",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    fn field(name: &str, value: &str) -> ObjectFieldData {
        ObjectFieldData {
            field_name: Some(name.to_string()),
            value: Some(value.to_string()),
        }
    }

    fn config(server: &MockServer) -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build()
    }

    #[test]
    fn edit_endpoint_is_severance_pay_edit() {
        assert_eq!(
            FeishuPeopleApiV2::SeverancePayEdit.to_url(),
            "/open-apis/corehr/v2/severance_pays/edit"
        );
    }

    #[tokio::test]
    async fn edit_requires_id_and_update_data_before_http() {
        let server = MockServer::start().await;
        let err = EditRequest::new(config(&server))
            .execute()
            .await
            .expect_err("缺少必填字段");
        assert!(err.to_string().contains("severance_pay_id"), "{err}");
        let received = server.received_requests().await.unwrap_or_default();
        assert!(received.is_empty());
    }

    #[tokio::test]
    async fn edit_posts_empty_update_data_and_reads_fields() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/severance_pays/edit"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(body_json(json!({
                "severance_pay_id": "7660423757252773420",
                "operator_id": "7659617611689510444",
                "update_data": [{ "field_name": "name", "value": "Sandy" }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": [{ "field_name": "name", "value": "Sandy" }] }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let response = EditRequest::new(config(&server))
            .severance_pay_id("7660423757252773420")
            .operator_id("7659617611689510444")
            .update_data(vec![field("name", "Sandy")])
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("编辑应成功");
        let fields = response.data.expect("data");
        assert_eq!(fields[0].field_name.as_deref(), Some("name"));
        assert_eq!(fields[0].value.as_deref(), Some("Sandy"));
    }

    #[tokio::test]
    async fn edit_sends_empty_update_list() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/severance_pays/edit"))
            .and(body_json(json!({
                "severance_pay_id": "7660423757252773420",
                "update_data": []
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": [] }
            })))
            .expect(1)
            .mount(&server)
            .await;

        EditRequest::new(config(&server))
            .severance_pay_id("7660423757252773420")
            .update_data(Vec::new())
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("空 update_data 应发出");
    }
}
