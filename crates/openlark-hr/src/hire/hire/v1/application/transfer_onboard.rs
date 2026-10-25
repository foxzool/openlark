//! 操作候选人入职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/transfer_onboard>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ApplicationOperationResult;

/// 操作候选人入职请求
#[derive(Debug, Clone)]
pub struct TransferOnboardRequest {
    /// 配置信息
    config: Config,
    application_id: String,
    request_body: Value,
}

impl TransferOnboardRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            request_body: Value::Null,
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferOnboardResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransferOnboardResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");
        if self.request_body.is_null() {
            return Err(openlark_core::error::validation_error(
                "请求体不能为空",
                "操作候选人入职时 request_body 为必填参数",
            ));
        }

        let api_endpoint = HireApiV1::ApplicationTransferOnboard(self.application_id.clone());
        let request = ApiRequest::<TransferOnboardResponse>::post(api_endpoint.to_url())
            .body(self.request_body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "操作候选人入职响应数据为空",
        )
        .await
    }
}

/// 操作候选人入职响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferOnboardResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `employee` 字段（官方响应 data 下的员工信息容器）。
    pub employee: Option<TransferOnboardEmployee>,
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ApplicationOperationResult,
}

/// 操作候选人入职返回的员工信息（官方 `employee` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferOnboardEmployee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 员工 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职状态（1：已入职 2：已离职）。
    pub onboard_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 转正状态（1：未转正 2：已转正）。
    pub conversion_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际入职时间，毫秒时间戳（int64 类型）。
    pub onboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 预期转正时间，毫秒时间戳（int64 类型）。
    pub expected_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际转正时间，毫秒时间戳（int64 类型）。
    pub actual_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职时间，毫秒时间戳（int64 类型）。
    pub overboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职原因。
    pub overboard_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人办公地点 ID。
    pub onboard_city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人入职部门 ID，与入参 `department_id_type` 类型一致。
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人直属上级 UserID，与入参 `user_id_type` 类型一致。
    pub leader: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人序列 ID，与入参 `job_family_id_type` 类型一致。
    pub sequence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人职级 ID，与入参 `job_level_id_type` 类型一致。
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人入职人员类型 ID，与入参 `employee_type_id_type` 类型一致。
    pub employee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求 ID。
    pub job_requirement_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for TransferOnboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/applications/test001/transfer_onboard
    #[tokio::test]
    async fn test_transfer_onboard_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/applications/test001/transfer_onboard",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "employee": {
                        "id": "7095600054216542508",
                        "application_id": "7073372582620416300",
                        "onboard_status": 1,
                        "conversion_status": 1,
                        "onboard_time": 1637596800000i64,
                        "expected_conversion_time": 1637596800000i64,
                        "actual_conversion_time": 1637596800000i64,
                        "overboard_time": 1637596800000i64,
                        "overboard_note": "职业发展考虑",
                        "onboard_city_code": "CT_2",
                        "department": "6966123381141866028",
                        "leader": "ou-xxx",
                        "sequence": "6937934036379650311",
                        "level": "7006234385490345986",
                        "employee_type": "1",
                        "job_requirement_id": "123123123213"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = TransferOnboardRequest::new(config)
            .application_id("test001".to_string())
            .request_body(json!({"k": "v"}))
            .execute()
            .await
            .expect("请求应成功");

        let employee = resp.employee.as_ref().expect("employee 容器应存在");
        assert_eq!(employee.id.as_deref(), Some("7095600054216542508"));
        assert_eq!(employee.onboard_status, Some(1));
        assert_eq!(employee.conversion_status, Some(1));
        assert_eq!(employee.onboard_time, Some(1637596800000));
        assert_eq!(employee.overboard_note.as_deref(), Some("职业发展考虑"));
        assert_eq!(employee.onboard_city_code.as_deref(), Some("CT_2"));
        assert_eq!(employee.leader.as_deref(), Some("ou-xxx"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
