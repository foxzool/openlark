//! API端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。
//!
//! # 使用场景
//!
//! ## 生产代码（推荐）
//! 使用枚举端点获得编译时类型检查和动态 URL 生成能力：
//! ```rust
//! use openlark_docs::common::api_endpoints::BitableApiV1;
//!
//! let app_token = "app_token".to_string();
//! let table_id = "table_id".to_string();
//! let endpoint = BitableApiV1::RecordCreate(app_token, table_id);
//! let url = endpoint.to_url(); // 类型安全，动态生成
//! assert!(url.contains("/open-apis/bitable/v1/"));
//! ```
//!
//! # 特性
//! - ✅ **类型安全**：编译时验证参数
//! - ✅ **动态生成**：支持参数化 URL
//! - ✅ **易于维护**：集中管理端点定义
//! - ✅ **避免错误**：消除字符串拼接错误
//!
//! # 与常量端点系统的关系
//!
//! 本模块与 `endpoints/mod.rs` 中的常量端点系统配合使用：
//! - **枚举端点**：用于生产代码（推荐）
//! - **常量端点**：用于测试和文档示例
//!
//! 不建议混合使用两个系统，应根据场景选择合适的端点方式。

use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// 端点 catalog 的通用语义接口（#424 / #438）。
/// 允许 to_request 等逻辑共享，减少重复。
pub trait CatalogEndpoint {
    /// 返回端点 URL。
    fn to_url(&self) -> String;

    /// 返回 HTTP 方法。
    fn method(&self) -> HttpMethod;

    /// 稳定的访问令牌要求（默认 None）。
    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        None
    }

    /// 构建带正确方法的请求。
    fn to_request<R>(&self) -> ApiRequest<R> {
        self.to_request_with_url(self.to_url())
    }

    /// 使用调用方补充了动态 query 的 URL 构建请求，同时保留 catalog 的 method/auth 语义。
    fn to_request_with_url<R>(&self, url: impl Into<String>) -> ApiRequest<R> {
        let url = url.into();
        let mut req = match self.method() {
            HttpMethod::Get => ApiRequest::get(url),
            HttpMethod::Post => ApiRequest::post(url),
            HttpMethod::Put => ApiRequest::put(url),
            HttpMethod::Delete => ApiRequest::delete(url),
            HttpMethod::Patch => ApiRequest::patch(url),
            _ => ApiRequest::get(self.to_url()),
        };
        if let Some(tokens) = self.supported_access_token_types() {
            req = req.with_supported_access_token_types(tokens);
        }
        req
    }
}

pub mod base;
pub use base::BaseApiV2;

pub mod bitable;
pub use bitable::BitableApiV1;

pub mod docs;
pub use docs::DocsApiV1;

pub mod docx;
pub use docx::DocxApiV1;

/// CCM Doc API Old V1 端点枚举
/// 对应 meta.project = ccm_doc, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocApiOld {
    /// 创建旧版文档
    Create,
    /// 获取旧版文档元信息
    Meta(String), // doc_token
    /// 获取旧版文档中的电子表格元数据
    SheetMeta(String), // doc_token
    /// 获取旧版文档纯文本内容
    RawContent(String), // doc_token
    /// 获取旧版文档富文本内容
    Content(String), // doc_token
    /// 编辑旧版文档内容
    BatchUpdate(String), // doc_token
}

impl CcmDocApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocApiOld::Create => "/open-apis/doc/v2/create".to_string(),
            CcmDocApiOld::Meta(doc_token) => {
                format!("/open-apis/doc/v2/meta/{doc_token}")
            }
            CcmDocApiOld::SheetMeta(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/sheet_meta")
            }
            CcmDocApiOld::RawContent(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/raw_content")
            }
            CcmDocApiOld::Content(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/content")
            }
            CcmDocApiOld::BatchUpdate(doc_token) => {
                format!("/open-apis/doc/v2/{doc_token}/batch_update")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for CcmDocApiOld {
    fn to_url(&self) -> String {
        CcmDocApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::Create | Self::BatchUpdate(_) => HttpMethod::Post,
            Self::Meta(_) | Self::SheetMeta(_) | Self::RawContent(_) | Self::Content(_) => {
                HttpMethod::Get
            }
        }
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}

/// CCM Docs API Old V1 端点枚举
/// 对应 meta.project = ccm_docs, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocsApiOld {
    /// 搜索云文档
    SearchObject,
    /// 获取元数据
    Meta,
}

impl CcmDocsApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocsApiOld::SearchObject => "/open-apis/suite/docs-api/search/object".to_string(),
            CcmDocsApiOld::Meta => "/open-apis/suite/docs-api/meta".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for CcmDocsApiOld {
    fn to_url(&self) -> String {
        CcmDocsApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::SearchObject => HttpMethod::Post,
            Self::Meta => HttpMethod::Get,
        }
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}

pub mod drive;
pub use drive::{
    CcmDriveExplorerApi, CcmDriveExplorerApiOld, DriveApi, PermissionApi, PermissionApiOld,
};

pub mod sheets;
pub use sheets::{CcmSheetApiOld, SheetsApiV3};

pub mod wiki;
pub use wiki::{WikiApi, WikiApiV1, WikiApiV2};

pub mod lingo;
pub use lingo::LingoApiV1;
pub use lingo::LingoApiV1 as BaikeApiV1;

pub mod minutes;
pub use minutes::MinutesApiV1;

/// Lingo API v1 端点基础路径（兼容保留）。
pub const LINGO_API_V1: &str = "/open-apis/lingo/v1";

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::{ApiRequest, HttpMethod};
    use openlark_core::constants::AccessTokenType;

    // ========== BaseApiV2 Tests ==========
    #[test]
    fn test_base_api_v2_role_create() {
        let endpoint = BaseApiV2::RoleCreate("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles"
        );
        assert_eq!(endpoint.method(), HttpMethod::Post);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Post);
        // #438: catalog 拥有认证要求
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_update() {
        let endpoint =
            BaseApiV2::RoleUpdate("app_token_123".to_string(), "role_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles/role_id_456"
        );
        assert_eq!(endpoint.method(), HttpMethod::Put);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Put);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_list() {
        let endpoint = BaseApiV2::RoleList("app_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles"
        );
        assert_eq!(endpoint.method(), HttpMethod::Get);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Get);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_role_delete() {
        let endpoint =
            BaseApiV2::RoleDelete("app_token_123".to_string(), "role_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/base/v2/apps/app_token_123/roles/role_id_456"
        );
        assert_eq!(endpoint.method(), HttpMethod::Delete);
        let req: ApiRequest<()> = endpoint.to_request();
        assert_eq!(req.method(), &HttpMethod::Delete);
        assert_eq!(
            endpoint.supported_access_token_types(),
            Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
        );
        assert_eq!(
            req.supported_access_token_types(),
            vec![AccessTokenType::User, AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_base_api_v2_with_special_chars() {
        let endpoint = BaseApiV2::RoleCreate("app-token_123".to_string());
        assert!(endpoint.to_url().contains("app-token_123"));
    }

    // ========== MinutesApiV1 Tests ==========
    #[test]
    fn test_minutes_api_v1_get() {
        let endpoint = MinutesApiV1::Get("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123"
        );
    }

    #[test]
    fn test_minutes_api_v1_media_get() {
        let endpoint = MinutesApiV1::MediaGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/media"
        );
    }

    #[test]
    fn test_minutes_api_v1_transcript_get() {
        let endpoint = MinutesApiV1::TranscriptGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/transcript"
        );
    }

    #[test]
    fn test_minutes_api_v1_statistics_get() {
        let endpoint = MinutesApiV1::StatisticsGet("minute_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/minutes/v1/minutes/minute_token_123/statistics"
        );
    }

    #[test]
    fn minute_subscription_issue_194_endpoints() {
        assert_eq!(
            MinutesApiV1::Subscription.to_url(),
            "/open-apis/minutes/v1/minutes/subscription"
        );
        assert_eq!(
            MinutesApiV1::Unsubscription.to_url(),
            "/open-apis/minutes/v1/minutes/unsubscription"
        );
    }

    // ========== WikiApiV1 Tests ==========
    #[test]
    fn test_wiki_api_v1_node_search() {
        let endpoint = WikiApiV1::NodeSearch;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v1/nodes/search");
    }

    // ========== DocsApiV1 Tests ==========
    #[test]
    fn test_docs_api_v1_content_get() {
        let endpoint = DocsApiV1::ContentGet;
        assert_eq!(endpoint.to_url(), "/open-apis/docs/v1/content");
    }

    // ========== DocxApiV1 Tests ==========
    #[test]
    fn test_docx_api_v1_document_create() {
        let endpoint = DocxApiV1::DocumentCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/docx/v1/documents");
    }

    #[test]
    fn test_docx_api_v1_document_get() {
        let endpoint = DocxApiV1::DocumentGet("doc_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/docx/v1/documents/doc_id_123");
    }

    #[test]
    fn test_docx_api_v1_document_block_list() {
        let endpoint = DocxApiV1::DocumentBlockList("doc_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/documents/doc_id_123/blocks"
        );
    }

    #[test]
    fn test_docx_api_v1_chat_announcement_get() {
        let endpoint = DocxApiV1::ChatAnnouncementGet("chat_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/chats/chat_id_123/announcement"
        );
    }

    #[test]
    fn test_docx_api_v1_document_convert() {
        let endpoint = DocxApiV1::DocumentConvert;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/documents/blocks/convert"
        );
    }

    #[test]
    fn test_docx_api_v1_document_block_children_create() {
        let endpoint = DocxApiV1::DocumentBlockChildrenCreate(
            "doc_id_123".to_string(),
            "block_id_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/docx/v1/documents/doc_id_123/blocks/block_id_456/children"
        );
    }

    // ========== WikiApiV2 Tests ==========
    #[test]
    fn test_wiki_api_v2_space_list() {
        let endpoint = WikiApiV2::SpaceList;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces");
    }

    #[test]
    fn test_wiki_api_v2_space_get() {
        let endpoint = WikiApiV2::SpaceGet("space_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces/space_id_123");
    }

    #[test]
    fn test_wiki_api_v2_space_create() {
        let endpoint = WikiApiV2::SpaceCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/spaces");
    }

    #[test]
    fn test_wiki_api_v2_space_node_list() {
        let endpoint = WikiApiV2::SpaceNodeList("space_id_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/wiki/v2/spaces/space_id_123/nodes"
        );
    }

    #[test]
    fn test_wiki_api_v2_space_member_delete() {
        let endpoint =
            WikiApiV2::SpaceMemberDelete("space_id_123".to_string(), "member_id_456".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/wiki/v2/spaces/space_id_123/members/member_id_456"
        );
    }

    #[test]
    fn test_wiki_api_v2_task_get() {
        let endpoint = WikiApiV2::TaskGet("task_id_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/wiki/v2/tasks/task_id_123");
    }

    // ========== CcmDocApiOld Tests ==========
    #[test]
    fn test_ccm_doc_api_old_create() {
        let endpoint = CcmDocApiOld::Create;
        assert_eq!(endpoint.to_url(), "/open-apis/doc/v2/create");
    }

    #[test]
    fn test_ccm_doc_api_old_meta() {
        let endpoint = CcmDocApiOld::Meta("doc_token_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/doc/v2/meta/doc_token_123");
    }

    #[test]
    fn test_ccm_doc_api_old_raw_content() {
        let endpoint = CcmDocApiOld::RawContent("doc_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/doc/v2/doc_token_123/raw_content"
        );
    }

    #[test]
    fn test_ccm_doc_api_old_batch_update() {
        let endpoint = CcmDocApiOld::BatchUpdate("doc_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/doc/v2/doc_token_123/batch_update"
        );
    }

    // ========== CcmDocsApiOld Tests ==========
    #[test]
    fn test_ccm_docs_api_old_search_object() {
        let endpoint = CcmDocsApiOld::SearchObject;
        assert_eq!(endpoint.to_url(), "/open-apis/suite/docs-api/search/object");
    }

    #[test]
    fn test_ccm_docs_api_old_meta() {
        let endpoint = CcmDocsApiOld::Meta;
        assert_eq!(endpoint.to_url(), "/open-apis/suite/docs-api/meta");
    }

    // ========== CcmDriveExplorerApiOld Tests ==========
    #[test]
    fn test_ccm_drive_explorer_api_old_root_folder_meta() {
        let endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/root_folder/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_old_folder_meta() {
        let endpoint = CcmDriveExplorerApiOld::FolderMeta("folder_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/folder/folder_token_123/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_old_file_copy() {
        let endpoint = CcmDriveExplorerApiOld::FileCopy("file_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/explorer/v2/file/copy/files/file_token_123"
        );
    }

    // ========== CcmDriveExplorerApi Tests ==========
    #[test]
    fn test_ccm_drive_explorer_api_root_folder_meta() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/explorer/root_folder/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_folder_meta() {
        let endpoint = CcmDriveExplorerApi::FolderMeta("folder_token_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/explorer/folder/folder_token_123/meta"
        );
    }

    #[test]
    fn test_ccm_drive_explorer_api_folder() {
        let endpoint = CcmDriveExplorerApi::Folder;
        assert_eq!(endpoint.to_url(), "/open-apis/drive/v1/explorer/folder");
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_params() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params = vec![("key", "value".to_string())];
        let url = endpoint.to_url_with_params(&params);
        assert!(url.contains("?"));
        assert!(url.contains("key=value"));
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_empty_params() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params: Vec<(&str, String)> = vec![];
        let url = endpoint.to_url_with_params(&params);
        assert!(!url.contains("?"));
    }

    #[test]
    fn test_ccm_drive_explorer_api_to_url_with_special_chars() {
        let endpoint = CcmDriveExplorerApi::RootFolderMeta;
        let params = vec![("query", "hello world".to_string())];
        let url = endpoint.to_url_with_params(&params);
        assert!(url.contains("%20"));
    }

    // ========== PermissionApi Tests ==========
    #[test]
    fn test_permission_api_member_permitted() {
        let endpoint = PermissionApi::MemberPermitted;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/permitted"
        );
    }

    #[test]
    fn test_permission_api_member_transfer() {
        let endpoint = PermissionApi::MemberTransfer;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/transfer"
        );
    }

    #[test]
    fn test_permission_api_public() {
        let endpoint = PermissionApi::Public;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/v2/public/"
        );
    }

    // ========== PermissionApiOld Tests ==========
    #[test]
    fn test_permission_api_old_member_permitted() {
        let endpoint = PermissionApiOld::MemberPermitted;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/member/permitted"
        );
    }

    #[test]
    fn test_permission_api_old_public() {
        let endpoint = PermissionApiOld::Public;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/drive/v1/permission/v2/public/"
        );
    }
}
