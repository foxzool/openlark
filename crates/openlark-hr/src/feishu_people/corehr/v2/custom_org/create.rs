//! 创建自定义组织
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/custom_org/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::OrgRoleUpdate;
use super::query::{CustomFieldData, I18nText, MatchRuleGroup};

/// 创建自定义组织请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 组织类型编码（必填，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看）
    pub object_api_name: String,
    /// 自定义组织的名称（必填，i18n 多语，最多 5 条）
    pub names: Vec<I18nText>,
    /// 自定义组织编码（不能与其他记录的编码重复；未开启自动编码时不传不会自动生成）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级自定义组织 ID（上级自定义组织的状态必须是启用的）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 负责人 ID 列表（employment_id 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_ids: Option<Vec<String>>,
    /// 自定义组织描述（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 自定义组织生效时间（必填，格式：YYYY-MM-DD）
    pub effective_time: String,
    /// 自动给「按自定义组织授权的角色」授权（最多 64 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_roles: Option<Vec<OrgRoleUpdate>>,
    /// 自动匹配的规则组（组间并集、组内交集，最多 16 组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_rule_groups: Option<Vec<MatchRuleGroup>>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 创建自定义组织请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置组织类型编码（必填）。
    pub fn object_api_name(mut self, object_api_name: impl Into<String>) -> Self {
        self.body.object_api_name = object_api_name.into();
        self
    }

    /// 设置自定义组织的名称（必填，i18n 多语）。
    pub fn names(mut self, names: Vec<I18nText>) -> Self {
        self.body.names = names;
        self
    }

    /// 设置自定义组织编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置上级自定义组织 ID。
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = Some(parent_id.into());
        self
    }

    /// 设置负责人 ID 列表。
    pub fn manager_ids(mut self, manager_ids: Vec<String>) -> Self {
        self.body.manager_ids = Some(manager_ids);
        self
    }

    /// 设置自定义组织描述（i18n 多语）。
    pub fn description(mut self, description: Vec<I18nText>) -> Self {
        self.body.description = Some(description);
        self
    }

    /// 设置自定义组织生效时间（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置自动授权的组织角色列表。
    pub fn org_roles(mut self, org_roles: Vec<OrgRoleUpdate>) -> Self {
        self.body.org_roles = Some(org_roles);
        self
    }

    /// 设置自动匹配的规则组。
    pub fn match_rule_groups(mut self, match_rule_groups: Vec<MatchRuleGroup>) -> Self {
        self.body.match_rule_groups = Some(match_rule_groups);
        self
    }

    /// 设置自定义字段列表。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        validate_required!(self.body.object_api_name, "object_api_name 不能为空");
        validate_required!(self.body.names, "names 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/custom_orgs")
            .query_opt("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 创建自定义组织响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 自定义组织的 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::super::query::MatchRule;
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/custom_orgs
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/custom_orgs"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .and(body_json(json!({
                "object_api_name": "custom_org_01",
                "names": [{ "lang": "zh-CN", "value": "飞书人事" }],
                "code": "000001",
                "parent_id": "6862995757234914824",
                "manager_ids": ["6862995757234914824"],
                "description": [{ "lang": "zh-CN", "value": "中文示例" }],
                "effective_time": "2020-01-01",
                "org_roles": [{
                    "api_name": "hcm_corehr_xxxxxx",
                    "security_group_id": "7034393015968122400",
                    "employment_ids": ["6862995757234914824"]
                }],
                "match_rule_groups": [{
                    "match_rules": [{
                        "left_value": "department",
                        "operator": "contains",
                        "right_values": ["6862995757234914824"]
                    }]
                }],
                "custom_fields": [{ "custom_api_name": "name", "value": "\"231\"" }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "org_id": "6862995757234914824" }
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
            .object_api_name("custom_org_01")
            .names(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("飞书人事".to_string()),
            }])
            .code("000001")
            .parent_id("6862995757234914824")
            .manager_ids(vec!["6862995757234914824".to_string()])
            .description(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .effective_time("2020-01-01")
            .org_roles(vec![OrgRoleUpdate {
                api_name: Some("hcm_corehr_xxxxxx".to_string()),
                security_group_id: Some("7034393015968122400".to_string()),
                employment_ids: Some(vec!["6862995757234914824".to_string()]),
            }])
            .match_rule_groups(vec![MatchRuleGroup {
                match_rules: Some(vec![MatchRule {
                    left_value: Some("department".to_string()),
                    operator: Some("contains".to_string()),
                    right_values: Some(vec!["6862995757234914824".to_string()]),
                }]),
            }])
            .custom_fields(vec![CustomFieldData {
                custom_api_name: Some("name".to_string()),
                name: None,
                r#type: None,
                value: Some("\"231\"".to_string()),
            }])
            .user_id_type("people_corehr_id")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.org_id.as_deref(), Some("6862995757234914824"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
