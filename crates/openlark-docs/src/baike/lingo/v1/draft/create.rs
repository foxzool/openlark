//! 创建草稿
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/draft/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Draft, DraftEntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 创建草稿响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDraftResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

impl ApiResponseTrait for CreateDraftResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建草稿请求
pub struct CreateDraftRequest {
    config: Config,
    body: DraftEntityInput,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl CreateDraftRequest {
    pub fn new(config: Config, body: DraftEntityInput) -> Self {
        Self {
            config,
            body,
            repo_id: None,
            user_id_type: None,
        }
    }

    /// 词库ID（不传默认创建至全员词库）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateDraftResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateDraftResp> {
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
            openlark_core::error::serialization_error("序列化创建草稿请求体失败", Some(e))
        })?;

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<CreateDraftResp> =
            ApiRequest::post(&LingoApiV1::DraftCreate.to_url()).body(body);
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
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
    use crate::baike::lingo::v1::models::{DisplayStatus, Term, UserIdType};

    #[test]
    fn test_create_lingo_draft_request_builder() {
        let config = Config::default();
        let body = DraftEntityInput {
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
        let request = CreateDraftRequest::new(config, body)
            .repo_id("repo_123")
            .user_id_type(UserIdType::OpenId);

        assert!(request.repo_id.is_some());
        assert_eq!(request.repo_id, Some("repo_123".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_create_lingo_draft_request_without_repo() {
        let config = Config::default();
        let body = DraftEntityInput {
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
        let request = CreateDraftRequest::new(config, body);

        assert!(request.repo_id.is_none());
    }

    #[tokio::test]
    async fn test_create_lingo_draft_request_validation() {
        let config = Config::default();

        // 测试 main_keys 为空
        let body = DraftEntityInput {
            main_keys: vec![],
            ..Default::default()
        };
        let request = CreateDraftRequest::new(config.clone(), body);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        // 测试 description 和 rich_text 都为空
        let body2 = DraftEntityInput {
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
        let request2 = CreateDraftRequest::new(config, body2);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateDraftResp::data_format(), ResponseFormat::Data);
    }
}
