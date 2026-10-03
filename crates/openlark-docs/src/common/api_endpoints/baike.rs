//! Baike（旧版词典）API 端点目录。
//!
//! 飞书 baike 与 lingo 并行存在：路径前缀分别为 `/open-apis/baike/v1/` 与
//! `/open-apis/lingo/v1/`。历史上 `BaikeApiV1` 曾是 `LingoApiV1` 的 re-export 别名，
//! 导致 baike 实现错误打到 lingo 路径（#568 contract 解析后暴露）。本模块提供独立
//! catalog，保证 baike typed API 的 method/path 与官网 CSV 一致。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};

/// Baike 词典 API v1 端点（`/open-apis/baike/v1/...`）。
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum BaikeApiV1 {
    /// 创建草稿
    DraftCreate,
    /// 更新草稿（参数：draft_id）
    DraftUpdate(String),

    /// 创建免审词条
    EntityCreate,
    /// 更新免审词条（参数：entity_id）
    EntityUpdate(String),
    /// 获取词条详情（参数：entity_id）
    EntityGet(String),
    /// 获取词条列表
    EntityList,
    /// 精准搜索词条
    EntityMatch,
    /// 模糊搜索词条
    EntitySearch,
    /// 词条高亮
    EntityHighlight,
    /// 提取潜在词条
    EntityExtract,

    /// 获取词典分类
    ClassificationList,

    /// 上传图片
    FileUpload,
    /// 下载图片（参数：file_token）
    FileDownload(String),
}

impl BaikeApiV1 {
    /// 生成对应的 URL（baike 前缀）。
    pub fn to_url(&self) -> String {
        match self {
            BaikeApiV1::DraftCreate => "/open-apis/baike/v1/drafts".to_string(),
            BaikeApiV1::DraftUpdate(draft_id) => {
                format!("/open-apis/baike/v1/drafts/{draft_id}")
            }
            BaikeApiV1::EntityCreate => "/open-apis/baike/v1/entities".to_string(),
            BaikeApiV1::EntityUpdate(entity_id) => {
                format!("/open-apis/baike/v1/entities/{entity_id}")
            }
            BaikeApiV1::EntityGet(entity_id) => {
                format!("/open-apis/baike/v1/entities/{entity_id}")
            }
            BaikeApiV1::EntityList => "/open-apis/baike/v1/entities".to_string(),
            BaikeApiV1::EntityMatch => "/open-apis/baike/v1/entities/match".to_string(),
            BaikeApiV1::EntitySearch => "/open-apis/baike/v1/entities/search".to_string(),
            BaikeApiV1::EntityHighlight => "/open-apis/baike/v1/entities/highlight".to_string(),
            BaikeApiV1::EntityExtract => "/open-apis/baike/v1/entities/extract".to_string(),
            BaikeApiV1::ClassificationList => "/open-apis/baike/v1/classifications".to_string(),
            BaikeApiV1::FileUpload => "/open-apis/baike/v1/files/upload".to_string(),
            BaikeApiV1::FileDownload(file_token) => {
                format!("/open-apis/baike/v1/files/{file_token}/download")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for BaikeApiV1 {
    fn to_url(&self) -> String {
        BaikeApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::EntityGet(_)
            | Self::EntityList
            | Self::ClassificationList
            | Self::FileDownload(_) => HttpMethod::Get,
            Self::DraftUpdate(_) | Self::EntityUpdate(_) => HttpMethod::Put,
            Self::DraftCreate
            | Self::EntityCreate
            | Self::EntityMatch
            | Self::EntitySearch
            | Self::EntityHighlight
            | Self::EntityExtract
            | Self::FileUpload => HttpMethod::Post,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::test_support::catalog_semantics_snapshot;

    #[test]
    fn baike_catalog_semantics_snapshot() {
        insta::assert_snapshot!(catalog_semantics_snapshot::<BaikeApiV1>());
    }
}
