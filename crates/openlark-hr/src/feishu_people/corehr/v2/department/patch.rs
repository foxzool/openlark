//! 更新部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::query_timeline::{EnumValue, I18nText};

/// 更新部门请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 部门子类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    /// 部门负责人（员工雇佣 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 是否保密
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    /// 层级通用信息（上级部门 ID、名称、启停用状态、编码、描述）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    /// 版本生效日期（必填，填写格式：YYYY-MM-DD 00:00:00）
    pub effective_time: String,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 部门默认成本中心
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 岗职管理模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    /// 是否优先使用手工编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_prefer_manual_encoding: Option<bool>,
}

/// 层级通用信息（官方 `hiberarchy_common` 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct HiberarchyCommon {
    /// 上级部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门名称（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
}

/// 自定义字段数据（官方 `object_field_data` 结构：{field_name, value}）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 自定义字段 API Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// PatchRequest
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    department_id: Option<String>,
    /// 请求体
    body: PatchBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: None,
            body: PatchBody::default(),
            client_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置 `department_id`。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: PatchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置部门 ID。
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.body.id = Some(id.into());
        self
    }

    /// 设置部门子类型。
    pub fn sub_type(mut self, sub_type: EnumValue) -> Self {
        self.body.sub_type = Some(sub_type);
        self
    }

    /// 设置部门负责人（员工雇佣 ID）。
    pub fn manager(mut self, manager: impl Into<String>) -> Self {
        self.body.manager = Some(manager.into());
        self
    }

    /// 设置是否保密。
    pub fn is_confidential(mut self, is_confidential: bool) -> Self {
        self.body.is_confidential = Some(is_confidential);
        self
    }

    /// 设置层级通用信息。
    pub fn hiberarchy_common(mut self, hiberarchy_common: HiberarchyCommon) -> Self {
        self.body.hiberarchy_common = Some(hiberarchy_common);
        self
    }

    /// 设置版本生效日期（必填，格式：YYYY-MM-DD 00:00:00）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置自定义字段列表。
    pub fn custom_fields(mut self, custom_fields: Vec<ObjectFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置部门默认成本中心。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.body.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置岗职管理模式。
    pub fn staffing_model(mut self, staffing_model: EnumValue) -> Self {
        self.body.staffing_model = Some(staffing_model);
        self
    }

    /// 设置是否优先使用手工编码。
    pub fn is_prefer_manual_encoding(mut self, is_prefer_manual_encoding: bool) -> Self {
        self.body.is_prefer_manual_encoding = Some(is_prefer_manual_encoding);
        self
    }

    /// 设置 client_token（判断是否为同一请求）。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
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
        let department_id = self.department_id.unwrap_or_default();
        validate_required!(department_id.trim(), "department_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/departments/{department_id}"
        ))
        .query_opt("client_token", self.client_token)
        .query_opt("user_id_type", self.user_id_type)
        .query_opt("department_id_type", self.department_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// PatchResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/corehr/v2/departments/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/departments/test001"))
            .and(query_param("department_id_type", "open_department_id"))
            .and(body_json(json!({
                "id": "4719456877659520852",
                "sub_type": { "enum_name": "department" },
                "manager": "6893013238632416776",
                "is_confidential": true,
                "hiberarchy_common": {
                    "parent_id": "4719168654814483759",
                    "name": [{ "lang": "zh-CN", "value": "中文示例" }],
                    "active": true,
                    "code": "12456",
                    "description": [{ "lang": "zh-CN", "value": "中文示例" }]
                },
                "effective_time": "2020-05-01 00:00:00",
                "custom_fields": [{ "field_name": "name", "value": "Sandy" }],
                "cost_center_id": "7142384817131652652",
                "staffing_model": { "enum_name": "job" },
                "is_prefer_manual_encoding": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PatchRequest::new(config)
            .department_id("test001".to_string())
            .id("4719456877659520852")
            .sub_type(EnumValue {
                enum_name: Some("department".to_string()),
                display: None,
            })
            .manager("6893013238632416776")
            .is_confidential(true)
            .hiberarchy_common(HiberarchyCommon {
                parent_id: Some("4719168654814483759".to_string()),
                name: Some(vec![I18nText {
                    lang: Some("zh-CN".to_string()),
                    value: Some("中文示例".to_string()),
                }]),
                active: Some(true),
                code: Some("12456".to_string()),
                description: Some(vec![I18nText {
                    lang: Some("zh-CN".to_string()),
                    value: Some("中文示例".to_string()),
                }]),
            })
            .effective_time("2020-05-01 00:00:00")
            .custom_fields(vec![ObjectFieldData {
                field_name: Some("name".to_string()),
                value: Some("Sandy".to_string()),
            }])
            .cost_center_id("7142384817131652652")
            .staffing_model(EnumValue {
                enum_name: Some("job".to_string()),
                display: None,
            })
            .is_prefer_manual_encoding(false)
            .department_id_type("open_department_id")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
