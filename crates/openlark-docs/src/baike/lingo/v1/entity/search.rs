//! 模糊搜索词条
//!
//! docPath: <https://open.feishu.cn/document/lingo-v1/entity/search>

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

/// 分类筛选（classification_filter）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationFilter {
    /// 需要获取的分类
    pub include: Option<Vec<String>>,
    /// 需要排除的分类
    pub exclude: Option<Vec<String>>,
}

/// 模糊搜索词条请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityBody {
    /// 搜索关键词
    pub query: Option<String>,
    /// 分类筛选
    pub classification_filter: Option<ClassificationFilter>,
    /// 词条的创建来源，1：用户主动创建，2：批量导入，3：官方词，4：OpenAPI 创建
    pub sources: Option<Vec<i32>>,
    /// 创建者
    pub creators: Option<Vec<String>>,
}

/// 模糊搜索词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityResp {
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

impl ApiResponseTrait for SearchEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 模糊搜索词条请求
pub struct SearchEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
    body: SearchEntityBody,
}

impl SearchEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            repo_id: None,
            user_id_type: None,
            body: SearchEntityBody {
                query: None,
                classification_filter: None,
                sources: None,
                creators: None,
            },
        }
    }

    /// 搜索关键词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 分类筛选
    pub fn classification_filter(mut self, filter: ClassificationFilter) -> Self {
        self.body.classification_filter = Some(filter);
        self
    }

    /// 创建来源过滤
    pub fn sources(mut self, sources: Vec<i32>) -> Self {
        self.body.sources = Some(sources);
        self
    }

    /// 创建者过滤
    pub fn creators(mut self, creators: Vec<String>) -> Self {
        self.body.creators = Some(creators);
        self
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

    /// 词库ID（不传时默认在全员词库内搜索）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<SearchEntityResp> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~100 之间",
                ));
            }
        }

        // ===== 构建请求 =====
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化模糊搜索词条请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<SearchEntityResp> =
            ApiRequest::post(&LingoApiV1::EntitySearch.to_url()).body(body);
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        // ===== 发送请求 =====
        let response: Response<SearchEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_search_entity_builder() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config)
            .query("搜索关键词")
            .page_size(20);

        assert_eq!(request.body.query, Some("搜索关键词".to_string()));
        assert_eq!(request.page_size, Some(20));
    }

    /// 测试分类筛选
    #[test]
    fn test_classification_filter() {
        let config = Config::default();
        let filter = ClassificationFilter {
            include: Some(vec!["分类1".to_string(), "分类2".to_string()]),
            exclude: Some(vec!["分类3".to_string()]),
        };

        let request = SearchEntityRequest::new(config).classification_filter(filter);

        assert!(request.body.classification_filter.is_some());
    }

    /// 测试创建来源过滤
    #[test]
    fn test_sources_filter() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config).sources(vec![1, 2, 3]);

        assert_eq!(request.body.sources, Some(vec![1, 2, 3]));
    }

    /// 测试创建者过滤
    #[test]
    fn test_creators_filter() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config)
            .creators(vec!["user1".to_string(), "user2".to_string()]);

        assert_eq!(
            request.body.creators,
            Some(vec!["user1".to_string(), "user2".to_string()])
        );
    }

    /// 测试响应数据结构
    #[test]
    fn test_search_entity_response() {
        let response = SearchEntityResp {
            entities: vec![],
            page_token: Some("next_token".to_string()),
            has_more: Some(true),
        };

        assert!(response.entities.is_empty());
        assert_eq!(response.has_more, Some(true));
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(SearchEntityResp::data_format(), ResponseFormat::Data);
    }

    /// 测试边界page_size
    #[test]
    fn test_page_size_boundaries() {
        let config = Config::default();
        let min_request = SearchEntityRequest::new(config.clone()).page_size(1);
        assert_eq!(min_request.page_size, Some(1));

        let max_request = SearchEntityRequest::new(config).page_size(100);
        assert_eq!(max_request.page_size, Some(100));
    }

    /// 测试分页token
    #[test]
    fn test_pagination_token() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config).page_token("token_123");

        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    /// 测试词库ID
    #[test]
    fn test_repo_id() {
        let config = Config::default();
        let request = SearchEntityRequest::new(config).repo_id("repo_abc");

        assert_eq!(request.repo_id, Some("repo_abc".to_string()));
    }
}
