//! 更新草稿
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/draft/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Draft, DraftUpdateEntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 更新草稿响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDraftResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

impl ApiResponseTrait for UpdateDraftResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新草稿请求
pub struct UpdateDraftRequest {
    config: Config,
    draft_id: String,
    body: DraftUpdateEntityInput,
    user_id_type: Option<UserIdType>,
}

impl UpdateDraftRequest {
    pub fn new(config: Config, draft_id: impl Into<String>, body: DraftUpdateEntityInput) -> Self {
        Self {
            config,
            draft_id: draft_id.into(),
            body,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateDraftResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<UpdateDraftResp> {
        // ===== 参数校验 =====
        validate_required!(self.draft_id, "draft_id 不能为空");
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

        // ===== 构建请求 =====
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化更新草稿请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<UpdateDraftResp> =
            ApiRequest::put(&LingoApiV1::DraftUpdate(self.draft_id).to_url()).body(body);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求 =====
        let response: Response<UpdateDraftResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baike::lingo::v1::models::{
        DisplayStatus, DraftUpdateEntityInput, Term, UserIdType,
    };

    /// 测试构建器模式
    #[test]
    fn test_update_draft_request_builder() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
            main_keys: vec![],
            description: Some("测试描述".to_string()),
            rich_text: None,
            ..Default::default()
        };
        let request = UpdateDraftRequest::new(config, "draft_123", body);

        assert_eq!(request.draft_id, "draft_123");
    }

    /// 测试设置user_id_type
    #[test]
    fn test_update_draft_with_user_id_type() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
            main_keys: vec![],
            description: Some("描述".to_string()),
            rich_text: None,
            ..Default::default()
        };
        let request =
            UpdateDraftRequest::new(config, "draft_123", body).user_id_type(UserIdType::UnionId);

        assert!(request.user_id_type.is_some());
    }

    /// 测试响应数据结构
    #[test]
    fn test_response_structure() {
        let response = UpdateDraftResp { draft: None };

        assert!(response.draft.is_none());
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateDraftResp::data_format(), ResponseFormat::Data);
    }

    /// 测试rich_text场景
    #[test]
    fn test_update_with_rich_text() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
            main_keys: vec![],
            description: None,
            rich_text: Some("<p>富文本内容</p>".to_string()),
            ..Default::default()
        };
        let request = UpdateDraftRequest::new(config, "draft_456", body);

        assert_eq!(request.draft_id, "draft_456");
        assert_eq!(
            request.body.rich_text,
            Some("<p>富文本内容</p>".to_string())
        );
    }

    /// 测试同时设置description和rich_text
    #[test]
    fn test_update_with_both_fields() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
            main_keys: vec![],
            description: Some("描述".to_string()),
            rich_text: Some("<p>富文本</p>".to_string()),
            ..Default::default()
        };
        let request = UpdateDraftRequest::new(config, "draft_789", body);

        assert!(request.body.description.is_some());
        assert!(request.body.rich_text.is_some());
    }

    /// 测试空main_keys
    #[test]
    fn test_empty_main_keys() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
            main_keys: vec![],
            description: Some("描述".to_string()),
            rich_text: None,
            ..Default::default()
        };
        let request = UpdateDraftRequest::new(config, "draft_abc", body);

        assert_eq!(request.body.main_keys.len(), 0);
    }

    /// 测试非空main_keys
    #[test]
    fn test_non_empty_main_keys() {
        let config = Config::default();
        let body = DraftUpdateEntityInput {
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
            rich_text: None,
            ..Default::default()
        };
        let request = UpdateDraftRequest::new(config, "draft_xyz", body);

        assert_eq!(request.body.main_keys.len(), 2);
    }
}
