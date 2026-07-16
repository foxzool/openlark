//! 创建免审词条
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, EntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 创建免审词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 创建成功的词条实体信息
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for CreateEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建免审词条请求
pub struct CreateEntityRequest {
    config: Config,
    body: EntityInput,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl CreateEntityRequest {
    /// 创建新的实例。
    pub fn new(config: Config, body: EntityInput) -> Self {
        Self {
            config,
            body,
            repo_id: None,
            user_id_type: None,
        }
    }

    /// 词库 ID（不传默认创建至全员词库）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateEntityResp> {
        // ===== 参数校验 =====
        validate_required!(self.body.main_keys, "main_keys 不能为空");
        if self
            .body
            .description
            .as_deref()
            .unwrap_or_default()
            .is_empty()
            && self
                .body
                .rich_text
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "description 与 rich_text 至少填写一个",
            ));
        }

        // ===== 序列化请求体 =====
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化创建免审词条请求体失败", Some(e))
        })?;

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<CreateEntityResp> =
            LingoApiV1::EntityCreate.to_request().body(body);
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<CreateEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baike::lingo::v1::models::{DisplayStatus, Term, UserIdType};
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_create_lingo_entity_request_builder() {
        let config = Config::default();
        let body = EntityInput {
            main_keys: vec![Term {
                key: "test_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            description: Some("词条描述".to_string()),
            ..Default::default()
        };
        let request = CreateEntityRequest::new(config, body)
            .repo_id("repo_123")
            .user_id_type(UserIdType::OpenId);

        assert!(request.repo_id.is_some());
        assert_eq!(request.repo_id, Some("repo_123".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_create_lingo_entity_request_without_repo() {
        let config = Config::default();
        let body = EntityInput {
            main_keys: vec![Term {
                key: "public_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            rich_text: Some("<p>富文本内容</p>".to_string()),
            ..Default::default()
        };
        let request = CreateEntityRequest::new(config, body);

        assert!(request.repo_id.is_none());
    }

    #[tokio::test]
    async fn test_create_lingo_entity_request_validation() {
        let config = Config::default();

        // 测试 main_keys 为空
        let body = EntityInput {
            main_keys: vec![],
            ..Default::default()
        };
        let request = CreateEntityRequest::new(config.clone(), body);
        assert!(
            request
                .execute_with_options(RequestOption::default())
                .await
                .is_err()
        );

        // 测试 description 和 rich_text 都为空
        let body2 = EntityInput {
            main_keys: vec![Term {
                key: "test_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            description: None,
            rich_text: None,
            ..Default::default()
        };
        let request2 = CreateEntityRequest::new(config, body2);
        assert!(
            request2
                .execute_with_options(RequestOption::default())
                .await
                .is_err()
        );
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateEntityResp::data_format(), ResponseFormat::Data);
    }

    #[tokio::test]
    async fn create_entity_uses_catalog_request_semantics() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/lingo/v1/entities"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "entity": null }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let body = EntityInput {
            main_keys: vec![Term {
                key: "测试词条".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            description: Some("词条描述".to_string()),
            ..Default::default()
        };

        let response = CreateEntityRequest::new(config, body)
            .repo_id("repo_123")
            .user_id_type(UserIdType::OpenId)
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("创建词条应成功");
        assert!(response.entity.is_none());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query: std::collections::HashMap<_, _> = received[0]
            .url
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();
        assert_eq!(query.get("repo_id").map(String::as_str), Some("repo_123"));
        assert_eq!(
            query.get("user_id_type").map(String::as_str),
            Some("open_id")
        );
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["main_keys"][0]["key"], "测试词条");
        assert_eq!(body["description"], "词条描述");
    }
}
