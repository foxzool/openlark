//! 创建外派信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.international_assignment/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `CreateRequest` 请求。
#[derive(Debug, Clone)]
pub struct CreateRequest {
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::EmployeesInternationalAssignmentCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建外派信息响应数据为空",
        )
        .await
    }
}

/// `CreateResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 外派信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment: Option<EmployeesInternationalAssignmentResp>,
}

/// 外派信息（官方类型 employees.international_assignment_resp，create/patch 响应共用）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeesInternationalAssignmentResp {
    /// 外派工作地点 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    /// 外派任职公司 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    /// 排班类型（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<EnumValue>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 人员类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 周工作时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours_v2: Option<f64>,
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 序列 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 职级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职等 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    /// 薪资类型（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<EnumValue>,
    /// 直属上级雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    /// 虚线上级雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    /// 工作日历 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendar_id: Option<String>,
    /// 岗位 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 外派原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_reason: Option<String>,
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 预计结束日期（yyyy-mm-dd）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_expected_end_date: Option<String>,
    /// 外派状态（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_status: Option<EnumValue>,
    /// 外派类型（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_type: Option<EnumValue>,
    /// 开始日期（yyyy-mm-dd）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 结束日期（yyyy-mm-dd，外派未结束时默认 9999-12-31）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 外派 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 object_field_data 结构：{field_name, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/employees/international_assignments
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/employees/international_assignments",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "international_assignment": {
                        "work_location_id": "7127921432117937708",
                        "service_company": "7127921432117937708",
                        "work_shift": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "weekly_working_hours_v2": 8,
                        "department_id": "7127921432117937708",
                        "compensation_type": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "employment_id": "7127921432117937708",
                        "custom_fields": [
                            { "field_name": "name", "value": "Sandy" }
                        ],
                        "international_assignment_status": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "effective_time": "2024-01-02",
                        "expiration_time": "2024-01-02",
                        "id": "7127921432117937708"
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

        let resp = CreateRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let assignment = resp.international_assignment.expect("应解析外派信息");
        assert_eq!(
            assignment.work_location_id,
            Some("7127921432117937708".to_string())
        );
        assert_eq!(
            assignment.service_company,
            Some("7127921432117937708".to_string())
        );
        assert_eq!(
            assignment
                .work_shift
                .as_ref()
                .and_then(|v| v.enum_name.clone()),
            Some("phone_type".to_string())
        );
        assert_eq!(assignment.weekly_working_hours_v2, Some(8.0));
        assert_eq!(
            assignment.custom_fields.as_deref(),
            Some(
                &[ObjectFieldData {
                    field_name: Some("name".to_string()),
                    value: Some("Sandy".to_string()),
                }][..]
            )
        );
        assert_eq!(assignment.id, Some("7127921432117937708".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
