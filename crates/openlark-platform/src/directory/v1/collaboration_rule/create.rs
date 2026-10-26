//! 新增可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/create>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/create>
//!
//! 官方请求体：subjects / objects 均为必填（collaboration_rule_entities）；
//! 必填查询参数 target_tenant_key；成功响应 data 仅含 add_rule_id。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::CollaborationRuleEntities;

/// 新增可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleCreateRequestBuilder {
    config: Config,
    /// 对方组织的 tenant key（必填查询参数）
    target_tenant_key: String,
    /// 请求体
    body: CollaborationRuleCreateBody,
}

impl CollaborationRuleCreateRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `target_tenant_key` 为对方组织的 tenant key，可通过
    /// 「管理员获取所有关联组织列表」接口获取。
    pub fn new(config: Config, target_tenant_key: impl Into<String>) -> Self {
        Self {
            config,
            target_tenant_key: target_tenant_key.into(),
            body: CollaborationRuleCreateBody {
                subjects: CollaborationRuleEntities::new(),
                objects: CollaborationRuleEntities::new(),
            },
        }
    }

    /// 设置规则主体（必填，需至少包含一个实体）
    pub fn subjects(mut self, subjects: CollaborationRuleEntities) -> Self {
        self.body.subjects = subjects;
        self
    }

    /// 设置规则客体（必填，需至少包含一个实体）
    pub fn objects(mut self, objects: CollaborationRuleEntities) -> Self {
        self.body.objects = objects;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleCreateResponse> {
        validate_required!(self.target_tenant_key, "target_tenant_key 不能为空");
        validate_entities("subjects", &self.body.subjects)?;
        validate_entities("objects", &self.body.objects)?;

        let url = "/open-apis/directory/v1/collaboration_rules";

        let req: ApiRequest<CollaborationRuleCreateResponse> = ApiRequest::post(url)
            .body(serde_json::to_value(&self.body)?)
            .query("target_tenant_key", self.target_tenant_key);
        Transport::request_typed(req, &self.config, Some(option), "新增可搜可见规则").await
    }
}

/// 校验主体/客体至少设置一个实体（官方错误码 2223106 拒绝空实体）。
fn validate_entities(label: &str, entities: &CollaborationRuleEntities) -> SDKResult<()> {
    if entities.is_empty() {
        return Err(CoreError::validation_msg(format!(
            "{label} 需至少设置一个实体（open_user_ids/open_department_ids/open_group_ids）"
        )));
    }
    Ok(())
}

/// 新增可搜可见规则请求体（subjects / objects 均必填）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationRuleCreateBody {
    /// 规则主体
    pub subjects: CollaborationRuleEntities,
    /// 规则客体
    pub objects: CollaborationRuleEntities,
}

/// 新增可搜可见规则响应（data 仅含添加的规则 ID）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleCreateResponse {
    /// 添加的规则 ID
    #[serde(rename = "add_rule_id")]
    pub add_rule_id: String,
}

impl ApiResponseTrait for CollaborationRuleCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleCreateRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleCreateBuilder = CollaborationRuleCreateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST 官方 subjects/objects 请求体 + 必填查询参数 → 强类型 add_rule_id。
    #[tokio::test]
    async fn test_create_collaboration_rule_sends_official_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/collaboration_rules"))
            .and(query_param("target_tenant_key", "test_key"))
            .and(body_json(json!({
                "subjects": {
                    "open_user_ids": ["od-112121"]
                },
                "objects": {
                    "open_department_ids": ["od-12121212"]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "add_rule_id": "1212121"
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

        let resp = CollaborationRuleCreateRequestBuilder::new(config, "test_key")
            .subjects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-112121"]))
            .objects(CollaborationRuleEntities::new().with_open_department_ids(vec!["od-12121212"]))
            .execute()
            .await
            .expect("新增可搜可见规则应成功");
        assert_eq!(resp.add_rule_id, "1212121");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "POST");
    }

    /// 必填校验：target_tenant_key 为空时不发请求。
    #[tokio::test]
    async fn test_create_collaboration_rule_validates_required_query() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CollaborationRuleCreateRequestBuilder::new(config, " ")
            .subjects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-1"]))
            .objects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-2"]))
            .execute()
            .await;
        assert!(result.is_err());
    }

    /// 必填校验：主体/客体未设置实体时快速失败（官方错误码 2223106）。
    #[tokio::test]
    async fn test_create_collaboration_rule_rejects_empty_entities() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CollaborationRuleCreateRequestBuilder::new(config, "test_key")
            .execute()
            .await
            .expect_err("空实体应返回错误");
        assert!(err.to_string().contains("subjects"));
    }
}
