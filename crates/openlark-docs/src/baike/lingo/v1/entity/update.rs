//! 更新免审词条
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, EntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 更新免审词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for UpdateEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新免审词条请求
pub struct UpdateEntityRequest {
    config: Config,
    entity_id: String,
    body: EntityInput,
    user_id_type: Option<UserIdType>,
}

impl UpdateEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>, body: EntityInput) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            body,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<UpdateEntityResp> {
        // ===== 参数校验 =====
        validate_required!(self.entity_id, "entity_id 不能为空");
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
            openlark_core::error::serialization_error("序列化更新免审词条请求体失败", Some(e))
        })?;

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<UpdateEntityResp> =
            ApiRequest::put(&LingoApiV1::EntityUpdate(self.entity_id).to_url()).body(body);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<UpdateEntityResp> =
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

    #[test]
    fn test_update_lingo_entity_request_builder() {
        let config = Config::default();
        let body = EntityInput {
            main_keys: vec![Term {
                key: "update_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            description: Some("更新描述".to_string()),
            ..Default::default()
        };
        let request =
            UpdateEntityRequest::new(config, "entity_123", body).user_id_type(UserIdType::OpenId);

        assert_eq!(request.entity_id, "entity_123");
        assert_eq!(request.body.main_keys.len(), 1);
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_update_lingo_entity_request_with_rich_text() {
        let config = Config::default();
        let body = EntityInput {
            main_keys: vec![Term {
                key: "rich_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            rich_text: Some("<p>富文本内容</p>".to_string()),
            ..Default::default()
        };
        let request = UpdateEntityRequest::new(config, "entity_456", body);

        assert_eq!(request.entity_id, "entity_456");
        assert!(request.body.rich_text.is_some());
    }

    #[tokio::test]
    async fn test_update_lingo_entity_request_validation() {
        let config = Config::default();

        // 测试 entity_id 为空
        let body = EntityInput {
            main_keys: vec![Term {
                key: "test_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            description: Some("描述".to_string()),
            ..Default::default()
        };
        let request = UpdateEntityRequest::new(config.clone(), "", body);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 main_keys 为空
        let body2 = EntityInput {
            ..Default::default()
        };
        let request2 = UpdateEntityRequest::new(config.clone(), "entity_123", body2);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 description 和 rich_text 都为空
        let body3 = EntityInput {
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
        let request3 = UpdateEntityRequest::new(config, "entity_123", body3);
        assert!(request3
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateEntityResp::data_format(), ResponseFormat::Data);
    }
}
