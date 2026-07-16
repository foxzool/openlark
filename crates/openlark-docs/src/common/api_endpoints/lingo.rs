//! Lingo API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// Lingo语言服务 API v1 端点
#[derive(Debug, Clone, PartialEq)]
pub enum LingoApiV1 {
    /// 草稿管理
    DraftCreate,
    /// 更新草稿（参数：draft_id）
    DraftUpdate(String), // draft_id

    /// 词条管理
    EntityCreate,
    /// 更新词条（参数：entity_id）
    EntityUpdate(String), // entity_id
    /// 删除词条（参数：entity_id）
    EntityDelete(String), // entity_id
    /// 获取词条（参数：entity_id）
    EntityGet(String), // entity_id
    /// 列出词条
    EntityList,
    /// 词条匹配
    EntityMatch,
    /// 搜索词条
    EntitySearch,
    /// 词条高亮
    EntityHighlight,
    /// 批量获取词条
    EntityBatchGet,
    /// 批量更新词条
    EntityBatchUpdate,
    /// 词条搜索推荐
    EntitySearchRecommend,
    /// 提取潜在词条
    EntityExtract,
    /// 获取词条历史（参数：entity_id）
    EntityHistoryGet(String), // entity_id
    /// 列出词条历史
    EntityHistoryList,

    /// 分类管理
    ClassificationList,

    /// 词库管理
    RepoList,

    /// 文件管理
    FileUpload,
    /// 下载文件（参数：file_token）
    FileDownload(String), // file_token

    /// 智能处理
    GenerateSummary,
    /// 提取关键词
    ExtractKeywords,
    /// 翻译文本
    TranslateText,
}

impl LingoApiV1 {
    /// 提供 `to_url` 能力。
    pub fn to_url(&self) -> String {
        match self {
            LingoApiV1::DraftCreate => "/open-apis/lingo/v1/drafts".to_string(),
            LingoApiV1::DraftUpdate(draft_id) => {
                format!("/open-apis/lingo/v1/drafts/{draft_id}")
            }
            LingoApiV1::EntityCreate => "/open-apis/lingo/v1/entities".to_string(),
            LingoApiV1::EntityUpdate(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityDelete(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityGet(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}")
            }
            LingoApiV1::EntityList => "/open-apis/lingo/v1/entities".to_string(),
            LingoApiV1::EntityMatch => "/open-apis/baike/v1/entities/match".to_string(),
            LingoApiV1::EntitySearch => "/open-apis/lingo/v1/entities/search".to_string(),
            LingoApiV1::EntityHighlight => "/open-apis/lingo/v1/entities/highlight".to_string(),
            LingoApiV1::EntityBatchGet => "/open-apis/lingo/v1/entities:batchGet".to_string(),
            LingoApiV1::EntityBatchUpdate => "/open-apis/lingo/v1/entities:batchUpdate".to_string(),
            LingoApiV1::EntitySearchRecommend => {
                "/open-apis/lingo/v1/entities:searchRecommend".to_string()
            }
            LingoApiV1::EntityExtract => "/open-apis/baike/v1/entities/extract".to_string(),
            LingoApiV1::EntityHistoryGet(entity_id) => {
                format!("/open-apis/lingo/v1/entities/{entity_id}/history")
            }
            LingoApiV1::EntityHistoryList => "/open-apis/lingo/v1/entityHistory".to_string(),
            LingoApiV1::ClassificationList => "/open-apis/lingo/v1/classifications".to_string(),
            LingoApiV1::RepoList => "/open-apis/lingo/v1/repos".to_string(),
            LingoApiV1::FileUpload => "/open-apis/lingo/v1/files/upload".to_string(),
            LingoApiV1::FileDownload(file_token) => {
                format!("/open-apis/lingo/v1/files/{file_token}/download")
            }
            LingoApiV1::GenerateSummary => "/open-apis/lingo/v1/text:generateSummary".to_string(),
            LingoApiV1::ExtractKeywords => "/open-apis/lingo/v1/text:extractKeywords".to_string(),
            LingoApiV1::TranslateText => "/open-apis/lingo/v1/text:translate".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }

    /// 返回端点的 HTTP 方法。
    pub fn method(&self) -> HttpMethod {
        match self {
            Self::EntityGet(_)
            | Self::EntityList
            | Self::EntityHistoryGet(_)
            | Self::EntityHistoryList
            | Self::ClassificationList
            | Self::RepoList
            | Self::FileDownload(_) => HttpMethod::Get,
            Self::DraftUpdate(_) | Self::EntityUpdate(_) => HttpMethod::Put,
            Self::EntityDelete(_) => HttpMethod::Delete,
            Self::DraftCreate
            | Self::EntityCreate
            | Self::EntityMatch
            | Self::EntitySearch
            | Self::EntityHighlight
            | Self::EntityBatchGet
            | Self::EntityBatchUpdate
            | Self::EntitySearchRecommend
            | Self::FileUpload
            | Self::GenerateSummary
            | Self::ExtractKeywords
            | Self::TranslateText => HttpMethod::Post,
            Self::EntityExtract => HttpMethod::Post,
        }
    }
}

impl CatalogEndpoint for LingoApiV1 {
    fn to_url(&self) -> String {
        LingoApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        LingoApiV1::method(self)
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}
