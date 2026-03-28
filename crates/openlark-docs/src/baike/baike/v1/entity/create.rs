//! 创建免审词条
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/entity/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::{Entity, OuterInfo, RelatedMeta, Term, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEntityReq {
    /// 词条名
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本格式词条释义（description 与 rich_text 至少有一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更多相关信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 外部系统关联数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 富文本格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

/// 创建免审词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    user_id_type: Option<UserIdType>,
    req: CreateEntityReq,
}

impl CreateEntityRequest {
    pub fn new(config: Config, req: CreateEntityReq) -> Self {
        Self {
            config,
            user_id_type: None,
            req,
        }
    }

    /// 设置用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateEntityResp> {
        // ===== 参数校验 =====
        validate_required!(self.req.main_keys, "main_keys 不能为空");
        if self.req.main_keys.len() > 1 {
            return Err(openlark_core::error::validation_error(
                "main_keys",
                "main_keys 最大长度为 1",
            ));
        }
        for (idx, term) in self.req.main_keys.iter().enumerate() {
            if term.key.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    &format!("main_keys[{}].key", idx),
                    "key 不能为空",
                ));
            }
        }
        if let Some(aliases) = &self.req.aliases {
            for (idx, term) in aliases.iter().enumerate() {
                if term.key.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("aliases[{}].key", idx),
                        "key 不能为空",
                    ));
                }
            }
        }
        if self
            .req
            .description
            .as_deref()
            .unwrap_or_default()
            .is_empty()
            && self.req.rich_text.as_deref().unwrap_or_default().is_empty()
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "description 与 rich_text 至少填写一个",
            ));
        }

        // ===== 构建请求 =====

        let mut api_request: ApiRequest<CreateEntityResp> =
            ApiRequest::post(&BaikeApiV1::EntityCreate.to_url())
                .body(serde_json::to_value(&self.req)?);
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
    use crate::baike::baike::v1::models::{DisplayStatus, Term, UserIdType};

    #[test]
    fn test_create_entity_request_builder() {
        let config = Config::default();
        let req = CreateEntityReq {
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
        let request = CreateEntityRequest::new(config, req).user_id_type(UserIdType::OpenId);

        assert_eq!(request.req.main_keys.len(), 1);
        assert_eq!(request.req.description, Some("词条描述".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_create_entity_request_with_aliases() {
        let config = Config::default();
        let req = CreateEntityReq {
            main_keys: vec![Term {
                key: "public_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            aliases: Some(vec![Term {
                key: "alias_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }]),
            rich_text: Some("<p>富文本内容</p>".to_string()),
            ..Default::default()
        };
        let request = CreateEntityRequest::new(config, req);

        assert!(request.req.aliases.is_some());
        assert_eq!(
            request.req.aliases.as_ref().map(|v| v.len()).unwrap_or(0),
            1
        );
    }

    #[tokio::test]
    async fn test_create_entity_request_validation() {
        let config = Config::default();

        // 测试 main_keys 为空
        let req = CreateEntityReq {
            main_keys: vec![],
            ..Default::default()
        };
        let request = CreateEntityRequest::new(config.clone(), req);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 description 和 rich_text 都为空
        let req2 = CreateEntityReq {
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
        let request2 = CreateEntityRequest::new(config, req2);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateEntityResp::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_entity_req_default() {
        let req = CreateEntityReq::default();
        assert!(req.main_keys.is_empty());
        assert!(req.aliases.is_none());
        assert!(req.description.is_none());
        assert!(req.rich_text.is_none());
    }
}
