//! 更新待入职信息（不推荐）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::{EnumValue, PreHire, SupportCostCenterItem};

/// 更新待入职信息（不推荐）请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    pre_hire_id: String,
    body: PatchBody,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pre_hire_id: String::new(),
            body: PatchBody::default(),
        }
    }

    /// 设置 `pre_hire_id`。
    pub fn pre_hire_id(mut self, pre_hire_id: String) -> Self {
        self.pre_hire_id = pre_hire_id;
        self
    }

    /// 设置入职状态 `onboarding_status`（必填，枚举值如 `preboarding`/`day_one`/`completed`/`withdrawn`/`deleted`）。
    pub fn onboarding_status(mut self, enum_name: String) -> Self {
        self.body.onboarding_status = EnumValue {
            enum_name: Some(enum_name),
            display: None,
        };
        self
    }

    /// 设置招聘投递 ID `ats_application_id`。
    pub fn ats_application_id(mut self, ats_application_id: String) -> Self {
        self.body.ats_application_id = Some(ats_application_id);
        self
    }

    /// 设置入职日期 `hire_date`（格式 `YYYY-MM-DD`）。
    pub fn hire_date(mut self, hire_date: String) -> Self {
        self.body.hire_date = Some(hire_date);
        self
    }

    /// 设置雇佣类型 `employee_type`（枚举值）。
    pub fn employee_type(mut self, enum_name: String) -> Self {
        self.body.employee_type = Some(EnumValue {
            enum_name: Some(enum_name),
            display: None,
        });
        self
    }

    /// 设置人员编号 `worker_id`。
    pub fn worker_id(mut self, worker_id: String) -> Self {
        self.body.worker_id = Some(worker_id);
        self
    }

    /// 设置雇佣类型 `employee_type_id`。
    pub fn employee_type_id(mut self, employee_type_id: String) -> Self {
        self.body.employee_type_id = Some(employee_type_id);
        self
    }

    /// 设置个人信息 ID `person_id`。
    pub fn person_id(mut self, person_id: String) -> Self {
        self.body.person_id = Some(person_id);
        self
    }

    /// 设置自定义字段 `custom_fields`。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置成本中心分摊信息 `cost_center_rate`。
    pub fn cost_center_rate(mut self, cost_center_rate: Vec<SupportCostCenterItem>) -> Self {
        self.body.cost_center_rate = Some(cost_center_rate);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.pre_hire_id.trim(), "待入职ID不能为空");
        let enum_name = self
            .body
            .onboarding_status
            .enum_name
            .clone()
            .unwrap_or_default();
        validate_required!(enum_name, "onboarding_status.enum_name 不能为空");

        let api_endpoint = FeishuPeopleApiV1::PreHirePatch(self.pre_hire_id);
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新待入职信息响应数据为空",
        )
        .await
    }
}

/// 更新待入职信息请求体（官方字段为扁平结构，`onboarding_status` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 入职状态（必填）：preboarding 待入职 / deleted 已删除 / day_one 准备就绪 / withdrawn 已撤销 / completed 已完成
    pub onboarding_status: EnumValue,
    /// 招聘投递 ID（仅部分租户可用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    /// 入职日期，格式 `YYYY-MM-DD`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 雇佣类型（枚举）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<EnumValue>,
    /// 人员编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    /// 雇佣类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 引用 Person ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 成本中心分摊信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_rate: Option<Vec<SupportCostCenterItem>>,
}

/// 请求体自定义字段（object_field_data：`field_name`/`value` 均必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 字段名
    pub field_name: String,
    /// 字段值（json 转义后的字符串，如 `"\"Sandy\""`）
    pub value: String,
}

/// 更新待入职信息（不推荐）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 待入职数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire: Option<PreHire>,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/corehr/v1/pre_hires/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/pre_hires/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "pre_hire": {
                        "ats_application_id": "4719168654814483759",
                        "id": "154545454",
                        "hire_date": "2020-01-01",
                        "employee_type": {
                            "enum_name": "type_1",
                            "display": [{ "lang": "zh-CN", "value": "刘梓新" }]
                        },
                        "worker_id": "1245646",
                        "employee_type_id": "正式",
                        "person_id": "656464648662",
                        "onboarding_status": {
                            "enum_name": "preboarding",
                            "display": [{ "lang": "zh-CN", "value": "待入职" }]
                        }
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

        let data = PatchRequest::new(config)
            .pre_hire_id("test001".to_string())
            .onboarding_status("preboarding".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let pre_hire = data.pre_hire.expect("pre_hire 应存在");
        assert_eq!(
            pre_hire.ats_application_id,
            Some("4719168654814483759".to_string())
        );
        assert_eq!(pre_hire.worker_id, Some("1245646".to_string()));
        assert_eq!(pre_hire.person_id, Some("656464648662".to_string()));
        assert_eq!(pre_hire.id, Some("154545454".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：扁平结构（无 pre_hire 包装），onboarding_status 必填
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({ "onboarding_status": { "enum_name": "preboarding" } })
        );
    }

    /// 官方请求体形状：扁平字段 + 自定义字段 + 成本中心分摊
    #[tokio::test]
    async fn test_patch_sends_full_official_body_shape() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/pre_hires/test002"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "pre_hire": { "id": "2" } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PatchRequest::new(config)
            .pre_hire_id("test002".to_string())
            .onboarding_status("day_one".to_string())
            .ats_application_id("4719168654814483759".to_string())
            .hire_date("2020-01-01".to_string())
            .employee_type("type_1".to_string())
            .worker_id("1245646".to_string())
            .employee_type_id("正式".to_string())
            .person_id("656464648662".to_string())
            .custom_fields(vec![CustomFieldData {
                field_name: "name".to_string(),
                value: "\"Sandy\"".to_string(),
            }])
            .cost_center_rate(vec![SupportCostCenterItem {
                cost_center_id: Some("6950635856373745165".to_string()),
                rate: Some(100),
            }])
            .execute()
            .await
            .expect("请求应成功");
        assert!(resp.pre_hire.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "onboarding_status": { "enum_name": "day_one" },
                "ats_application_id": "4719168654814483759",
                "hire_date": "2020-01-01",
                "employee_type": { "enum_name": "type_1" },
                "worker_id": "1245646",
                "employee_type_id": "正式",
                "person_id": "656464648662",
                "custom_fields": [{ "field_name": "name", "value": "\"Sandy\"" }],
                "cost_center_rate": [{ "cost_center_id": "6950635856373745165", "rate": 100 }]
            })
        );
    }

    /// 必填校验：缺 onboarding_status 快速失败
    #[tokio::test]
    async fn test_patch_missing_onboarding_status_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = PatchRequest::new(config)
            .pre_hire_id("test001".to_string())
            .execute()
            .await
            .expect_err("缺 onboarding_status 应被拒绝");
        assert!(err.to_string().contains("onboarding_status"), "实际: {err}");
    }
}
