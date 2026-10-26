//! 更新自定义组织信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/custom_org/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::OrgRoleUpdate;
use super::query::{CustomFieldData, I18nText};

/// 更新自定义组织信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 组织类型编码（必填，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看）
    pub object_api_name: String,
    /// 组织名称（i18n 多语，最多 5 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 自定义组织编码（不能与其他记录的编码重复；未开启自动编码时不传不会自动生成）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级组织 ID（上级自定义组织的状态必须是启用的）
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
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 更新自定义组织信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    org_id: Option<String>,
    /// 请求体
    body: PatchBody,
    /// 根据 client_token 是否一致来判断是否为同一请求
    client_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            org_id: None,
            body: PatchBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置 `org_id`。
    pub fn org_id(mut self, org_id: String) -> Self {
        self.org_id = Some(org_id);
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: PatchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置组织类型编码（必填）。
    pub fn object_api_name(mut self, object_api_name: impl Into<String>) -> Self {
        self.body.object_api_name = object_api_name.into();
        self
    }

    /// 设置组织名称（i18n 多语）。
    pub fn names(mut self, names: Vec<I18nText>) -> Self {
        self.body.names = Some(names);
        self
    }

    /// 设置自定义组织编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置上级组织 ID。
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
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        let org_id = self.org_id.unwrap_or_default();
        validate_required!(org_id.trim(), "org_id 不能为空");
        validate_required!(self.body.object_api_name, "object_api_name 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/custom_orgs/{org_id}"
        ))
        .query_opt("client_token", self.client_token)
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 更新自定义组织信息响应
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

    /// 端到端：PATCH /open-apis/corehr/v2/custom_orgs/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/custom_orgs/test001"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .and(body_json(json!({
                "object_api_name": "custom_org_01",
                "names": [{ "lang": "zh-CN", "value": "中文示例" }],
                "code": "MDPD00000023",
                "parent_id": "6862995757234914824",
                "manager_ids": ["6862995757234914824"],
                "description": [{ "lang": "zh-CN", "value": "中文示例" }],
                "effective_time": "2020-01-01",
                "org_roles": [{
                    "api_name": "hcm_corehr_xxxxxx",
                    "employment_ids": ["6862995757234914824"]
                }],
                "custom_fields": [{ "custom_api_name": "name", "value": "\"231\"" }]
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
            .org_id("test001".to_string())
            .object_api_name("custom_org_01")
            .names(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .code("MDPD00000023")
            .parent_id("6862995757234914824")
            .manager_ids(vec!["6862995757234914824".to_string()])
            .description(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .effective_time("2020-01-01")
            .org_roles(vec![OrgRoleUpdate {
                api_name: Some("hcm_corehr_xxxxxx".to_string()),
                security_group_id: None,
                employment_ids: Some(vec!["6862995757234914824".to_string()]),
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

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
