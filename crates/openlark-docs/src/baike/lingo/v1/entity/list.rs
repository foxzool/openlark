//! 获取词条列表
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 获取词条列表响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEntityResp {
    /// 词条列表
    #[serde(default)]
    pub entities: Vec<Entity>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条列表请求
pub struct ListEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    provider: Option<String>,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl ListEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            provider: None,
            repo_id: None,
            user_id_type: None,
        }
    }

    /// 每页返回的词条量（默认 20，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 外部系统过滤（可选）
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// 词库 id（不传默认返回全员词库数据）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<ListEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListEntityResp> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
                ));
            }
        }

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<ListEntityResp> =
            ApiRequest::get(&LingoApiV1::EntityList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求并返回结果 =====
        let response: Response<ListEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baike::lingo::v1::models::UserIdType;

    #[test]
    fn test_list_lingo_entity_request_builder() {
        let config = Config::default();
        let request = ListEntityRequest::new(config)
            .page_size(20)
            .page_token("token123")
            .repo_id("repo_456")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.repo_id, Some("repo_456".to_string()));
        assert!(request.user_id_type.is_some());
    }

    #[test]
    fn test_list_lingo_entity_request_with_provider() {
        let config = Config::default();
        let request = ListEntityRequest::new(config)
            .provider("my_system")
            .page_size(50);

        assert_eq!(request.provider, Some("my_system".to_string()));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_lingo_entity_request_minimal() {
        let config = Config::default();
        let request = ListEntityRequest::new(config);

        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
        assert!(request.provider.is_none());
        assert!(request.repo_id.is_none());
    }

    #[tokio::test]
    async fn test_list_lingo_entity_request_validation() {
        let config = Config::default();

        // 测试 page_size 超出范围
        let request = ListEntityRequest::new(config.clone()).page_size(0);
        assert!(request
            .execute_with_options(RequestOption::default())
            .await
            .is_err());

        let request2 = ListEntityRequest::new(config.clone()).page_size(101);
        assert!(request2
            .execute_with_options(RequestOption::default())
            .await
            .is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListEntityResp::data_format(), ResponseFormat::Data);
    }
}
