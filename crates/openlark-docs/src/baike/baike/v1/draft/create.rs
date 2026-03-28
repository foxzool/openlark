//! 创建草稿
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/draft/create>

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
pub struct CreateDraftReq {
    /// 词条 ID（更新已有词条时填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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

/// 创建草稿响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDraftResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Draft {
    pub draft_id: String,
    pub entity: Entity,
}

impl ApiResponseTrait for CreateDraftResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建草稿请求
pub struct CreateDraftRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    req: CreateDraftReq,
}

impl CreateDraftRequest {
    pub fn new(config: Config, req: CreateDraftReq) -> Self {
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

    pub async fn execute(self) -> SDKResult<CreateDraftResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateDraftResp> {
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
        let mut api_request: ApiRequest<CreateDraftResp> =
            ApiRequest::post(&BaikeApiV1::DraftCreate.to_url())
                .body(serde_json::to_value(&self.req)?);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<CreateDraftResp> =
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
    fn test_create_draft_request_builder() {
        let config = Config::default();
        let req = CreateDraftReq {
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
        let request = CreateDraftRequest::new(config, req).user_id_type(UserIdType::OpenId);

        assert_eq!(request.req.main_keys.len(), 1);
        assert_eq!(request.req.description, Some("词条描述".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_create_draft_request_with_id() {
        let config = Config::default();
        let req = CreateDraftReq {
            id: Some("entity_123".to_string()),
            main_keys: vec![Term {
                key: "update_key".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            rich_text: Some("<p>富文本内容</p>".to_string()),
            ..Default::default()
        };
        let request = CreateDraftRequest::new(config, req);

        assert_eq!(request.req.id, Some("entity_123".to_string()));
    }

    #[tokio::test]
    async fn test_create_draft_request_validation() {
        let config = Config::default();

        // 测试 main_keys 为空
        let req = CreateDraftReq {
            main_keys: vec![],
            ..Default::default()
        };
        let request = CreateDraftRequest::new(config.clone(), req);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 main_keys 超过 1 个
        let req2 = CreateDraftReq {
            main_keys: vec![
                Term {
                    key: "key1".to_string(),
                    display_status: DisplayStatus {
                        allow_highlight: true,
                        allow_search: true,
                    },
                },
                Term {
                    key: "key2".to_string(),
                    display_status: DisplayStatus {
                        allow_highlight: false,
                        allow_search: false,
                    },
                },
            ],
            description: Some("描述".to_string()),
            ..Default::default()
        };
        let request2 = CreateDraftRequest::new(config.clone(), req2);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 description 和 rich_text 都为空
        let req3 = CreateDraftReq {
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
        let request3 = CreateDraftRequest::new(config, req3);
        assert!(request3
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateDraftResp::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_draft_req_default() {
        let req = CreateDraftReq::default();
        assert!(req.main_keys.is_empty());
        assert!(req.aliases.is_none());
        assert!(req.description.is_none());
        assert!(req.rich_text.is_none());
    }
}
