//! Docx API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// Docx API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocxApiV1 {
    // 群公告相关API (7个)
    /// 获取群公告基本信息
    ChatAnnouncementGet(String),
    /// 获取群公告所有块
    ChatAnnouncementBlockList(String),
    /// 在群公告中创建块
    ChatAnnouncementBlockChildrenCreate(String, String),
    /// 批量更新群公告块的内容
    ChatAnnouncementBlockBatchUpdate(String),
    /// 获取群公告块的内容
    ChatAnnouncementBlockGet(String, String),
    /// 获取所有子块
    ChatAnnouncementBlockChildrenGet(String, String),
    /// 删除群公告中的块
    ChatAnnouncementBlockChildrenBatchDelete(String, String),

    // 文档相关API (12个)
    /// 创建文档
    DocumentCreate,
    /// 获取文档基本信息
    DocumentGet(String),
    /// 获取文档纯文本内容
    DocumentRawContent(String),
    /// 获取文档所有块
    DocumentBlockList(String),
    /// 创建块
    DocumentBlockChildrenCreate(String, String),
    /// 创建嵌套块
    DocumentBlockDescendantCreate(String, String),
    /// 更新块的内容
    DocumentBlockPatch(String, String),
    /// 获取块的内容
    DocumentBlockGet(String, String),
    /// 批量更新块的内容
    DocumentBlockBatchUpdate(String),
    /// 获取所有子块
    DocumentBlockChildrenGet(String, String),
    /// 删除块
    DocumentBlockChildrenBatchDelete(String, String),
    /// Markdown/HTML 内容转换为文档块
    DocumentConvert,
}

impl DocxApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // 群公告相关API (7个)
            DocxApiV1::ChatAnnouncementGet(chat_id) => {
                format!("/open-apis/docx/v1/chats/{chat_id}/announcement")
            }
            DocxApiV1::ChatAnnouncementBlockList(chat_id) => {
                format!("/open-apis/docx/v1/chats/{chat_id}/announcement/blocks")
            }
            DocxApiV1::ChatAnnouncementBlockChildrenCreate(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{chat_id}/announcement/blocks/{block_id}/children"
                )
            }
            DocxApiV1::ChatAnnouncementBlockBatchUpdate(chat_id) => {
                format!("/open-apis/docx/v1/chats/{chat_id}/announcement/blocks/batch_update")
            }
            DocxApiV1::ChatAnnouncementBlockGet(chat_id, block_id) => {
                format!("/open-apis/docx/v1/chats/{chat_id}/announcement/blocks/{block_id}")
            }
            DocxApiV1::ChatAnnouncementBlockChildrenGet(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{chat_id}/announcement/blocks/{block_id}/children"
                )
            }
            DocxApiV1::ChatAnnouncementBlockChildrenBatchDelete(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{chat_id}/announcement/blocks/{block_id}/children/batch_delete"
                )
            }

            // 文档相关API (12个)
            DocxApiV1::DocumentCreate => "/open-apis/docx/v1/documents".to_string(),
            DocxApiV1::DocumentGet(document_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}")
            }
            DocxApiV1::DocumentRawContent(document_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/raw_content")
            }
            DocxApiV1::DocumentBlockList(document_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks")
            }
            DocxApiV1::DocumentBlockChildrenCreate(document_id, block_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/children")
            }
            DocxApiV1::DocumentBlockDescendantCreate(document_id, block_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/descendant")
            }
            DocxApiV1::DocumentBlockPatch(document_id, block_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}")
            }
            DocxApiV1::DocumentBlockGet(document_id, block_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}")
            }
            DocxApiV1::DocumentBlockBatchUpdate(document_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/batch_update")
            }
            DocxApiV1::DocumentBlockChildrenGet(document_id, block_id) => {
                format!("/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/children")
            }
            DocxApiV1::DocumentBlockChildrenBatchDelete(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/children/batch_delete"
                )
            }
            // 注意：该接口虽然归类在 docx-v1 文档下，但实际 HTTP URL 不包含 /v1
            DocxApiV1::DocumentConvert => "/open-apis/docx/documents/blocks/convert".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for DocxApiV1 {
    fn to_url(&self) -> String {
        DocxApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::ChatAnnouncementGet(_)
            | Self::ChatAnnouncementBlockList(_)
            | Self::ChatAnnouncementBlockGet(_, _)
            | Self::ChatAnnouncementBlockChildrenGet(_, _)
            | Self::DocumentGet(_)
            | Self::DocumentRawContent(_)
            | Self::DocumentBlockList(_)
            | Self::DocumentBlockGet(_, _)
            | Self::DocumentBlockChildrenGet(_, _) => HttpMethod::Get,
            Self::ChatAnnouncementBlockChildrenCreate(_, _)
            | Self::DocumentCreate
            | Self::DocumentBlockChildrenCreate(_, _)
            | Self::DocumentBlockDescendantCreate(_, _)
            | Self::DocumentConvert => HttpMethod::Post,
            Self::ChatAnnouncementBlockBatchUpdate(_)
            | Self::DocumentBlockPatch(_, _)
            | Self::DocumentBlockBatchUpdate(_) => HttpMethod::Patch,
            Self::ChatAnnouncementBlockChildrenBatchDelete(_, _)
            | Self::DocumentBlockChildrenBatchDelete(_, _) => HttpMethod::Delete,
        }
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}
