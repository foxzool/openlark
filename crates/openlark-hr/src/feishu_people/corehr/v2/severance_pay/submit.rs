//! 发起被动解除补偿金流程。
//!
//! `POST /open-apis/corehr/v2/severance_pays/submit`，只接受 tenant access token。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/severance_pay/submit>

use super::models::{
    EnumValue, ObjectFieldData, SubmitSeverancePayInfo, require_enum, require_object_fields,
    take_required_text,
};
use crate::common::api_endpoints::FeishuPeopleApiV2;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use openlark_core::{SDKResult, validate_allow_empty_list};
use serde::{Deserialize, Serialize};

/// 发起被动解除补偿金流程。
#[derive(Debug, Clone)]
pub struct SubmitRequest {
    config: Config,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    employment_id: Option<String>,
    offboarding_date: Option<String>,
    offboarding_reason_unique_identifier: Option<String>,
    initiator_id: Option<String>,
    severance_pay_info: Option<SubmitSeverancePayInfo>,
    custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 发起被动解除补偿金流程的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitRequestBody {
    /// 被解除员工的雇佣 ID。
    pub employment_id: String,
    /// 离职日期，`YYYY-MM-DD`。
    pub offboarding_date: String,
    /// 被动解除原因的唯一标识。
    pub offboarding_reason_unique_identifier: String,
    /// 发起人雇佣 ID。
    pub initiator_id: String,
    /// 补偿金信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severance_pay_info: Option<SubmitSeverancePayInfo>,
    /// 自定义字段。允许空数组，最多 1000 项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 发起被动解除补偿金流程的响应数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SubmitResponse {
    /// 补偿金业务单 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_id: Option<String>,
    /// 员工雇佣 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 补偿金状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnumValue>,
    /// BPM 流程 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 创建时间。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

impl ApiResponseTrait for SubmitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SubmitRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            employment_id: None,
            offboarding_date: None,
            offboarding_reason_unique_identifier: None,
            initiator_id: None,
            severance_pay_info: None,
            custom_fields: None,
        }
    }

    /// 用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 部门 ID 类型。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 被解除员工的雇佣 ID。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.employment_id = Some(employment_id.into());
        self
    }

    /// 离职日期，`YYYY-MM-DD`。
    pub fn offboarding_date(mut self, offboarding_date: impl Into<String>) -> Self {
        self.offboarding_date = Some(offboarding_date.into());
        self
    }

    /// 被动解除原因的唯一标识。
    pub fn offboarding_reason_unique_identifier(mut self, value: impl Into<String>) -> Self {
        self.offboarding_reason_unique_identifier = Some(value.into());
        self
    }

    /// 发起人雇佣 ID。
    pub fn initiator_id(mut self, initiator_id: impl Into<String>) -> Self {
        self.initiator_id = Some(initiator_id.into());
        self
    }

    /// 补偿金信息。
    pub fn severance_pay_info(mut self, severance_pay_info: SubmitSeverancePayInfo) -> Self {
        self.severance_pay_info = Some(severance_pay_info);
        self
    }

    /// 自定义字段。
    pub fn custom_fields(mut self, custom_fields: Vec<ObjectFieldData>) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SubmitResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<SubmitResponse> {
        let employment_id = take_required_text(self.employment_id, "employment_id 不能为空")?;
        let offboarding_date =
            take_required_text(self.offboarding_date, "offboarding_date 不能为空")?;
        let offboarding_reason_unique_identifier = take_required_text(
            self.offboarding_reason_unique_identifier,
            "offboarding_reason_unique_identifier 不能为空",
        )?;
        let initiator_id = take_required_text(self.initiator_id, "initiator_id 不能为空")?;
        if let Some(info) = &self.severance_pay_info
            && let Some(pay_type) = &info.severance_pay_type
        {
            require_enum(pay_type)?;
        }
        if let Some(custom_fields) = &self.custom_fields {
            validate_allow_empty_list!(custom_fields, 1000, "custom_fields 不能超过 1000 个");
            require_object_fields(custom_fields)?;
        }
        let body = SubmitRequestBody {
            employment_id,
            offboarding_date,
            offboarding_reason_unique_identifier,
            initiator_id,
            severance_pay_info: self.severance_pay_info,
            custom_fields: self.custom_fields,
        };
        let request =
            ApiRequest::<SubmitResponse>::post(FeishuPeopleApiV2::SeverancePaySubmit.to_url())
                .with_supported_access_token_types(vec![AccessTokenType::Tenant])
                .query_opt("user_id_type", self.user_id_type)
                .query_opt("department_id_type", self.department_id_type)
                .body(serde_json::to_value(&body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "发起被动解除补偿金流程",
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
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn config(server: &MockServer) -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build()
    }

    fn ready(server: &MockServer) -> SubmitRequest {
        SubmitRequest::new(config(server))
            .employment_id("7659617611689510444")
            .offboarding_date("2026-08-26")
            .offboarding_reason_unique_identifier("positive")
            .initiator_id("7249355547343013420")
    }

    #[test]
    fn submit_endpoint_is_severance_pay_submit() {
        assert_eq!(
            FeishuPeopleApiV2::SeverancePaySubmit.to_url(),
            "/open-apis/corehr/v2/severance_pays/submit"
        );
    }

    #[tokio::test]
    async fn submit_requires_employment_id_before_http() {
        let server = MockServer::start().await;
        let err = SubmitRequest::new(config(&server))
            .execute()
            .await
            .expect_err("缺少 employment_id");
        assert!(err.to_string().contains("employment_id"), "{err}");
        let received = server.received_requests().await.unwrap_or_default();
        assert!(received.is_empty());
    }

    #[tokio::test]
    async fn submit_posts_tenant_body_and_reads_process() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/severance_pays/submit"))
            .and(query_param("user_id_type", "open_id"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(body_json(json!({
                "employment_id": "7659617611689510444",
                "offboarding_date": "2026-08-26",
                "offboarding_reason_unique_identifier": "positive",
                "initiator_id": "7249355547343013420",
                "severance_pay_info": {
                    "severance_pay_type": { "enum_name": "n_plus_one" },
                    "currency": "THB"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "severance_pay_id": "7660423757252773420",
                    "employment_id": "7659617611689510444",
                    "status": {
                        "enum_name": "approving",
                        "display": [{ "lang": "zh-CN", "value": "审批中" }]
                    },
                    "process_id": "7660486572667979308",
                    "created_time": "1783994400000"
                }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let info = SubmitSeverancePayInfo {
            severance_pay_type: Some(EnumValue {
                enum_name: Some("n_plus_one".to_string()),
                display: None,
            }),
            currency: Some("THB".to_string()),
            ..SubmitSeverancePayInfo::default()
        };
        let response = ready(&server)
            .user_id_type("open_id")
            .severance_pay_info(info)
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("发起应成功");
        assert_eq!(
            response.severance_pay_id.as_deref(),
            Some("7660423757252773420")
        );
        assert_eq!(
            response.status.and_then(|status| status.enum_name),
            Some("approving".to_string())
        );
    }
}
