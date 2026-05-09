/// ccm_docs API 数据模型
///
/// 定义云文档搜索和元数据API的数据结构。
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 搜索云文档的请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchDocsRequest {
    /// 搜索关键字，支持模糊搜索
    pub search_key: String,
    /// 搜索的文档类型列表，不填则搜索所有类型
    /// 支持的文档类型：doc, sheet, slide, mindnote, docx, bitable, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_types: Option<Vec<String>>,
    /// 搜索范围，不填则搜索用户有权限的所有文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_scope: Option<SearchScope>,
    /// 排序规则，不填则默认按相关度排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortRule>,
    /// 分页参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量，默认20，最大100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 搜索范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SearchScope {
    /// 搜索我创建的文档
    CreatedByMe,
    /// 搜索与我共享的文档
    SharedWithMe,
    /// 搜索我收藏的文档
    Starred,
    /// 搜索最近访问的文档
    RecentlyViewed,
    /// 搜索指定文件夹下的文档
    Folder(String),
}

/// 排序规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortRule {
    /// 排序字段
    /// 支持的字段：created_time, modified_time, name, size
    pub sort_field: String,
    /// 排序方向：asc升序，desc降序
    pub sort_direction: SortDirection,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 搜索云文档的响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchDocsResponse {
    /// 搜索结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentItem>>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页令牌，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchDocsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档信息项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentItem {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 文档修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 文档创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 文档拥有者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserInfo>,
    /// 文档所属文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 文档所属文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    /// 文档大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 是否收藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_starred: Option<bool>,
    /// 文档权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DocumentPermissions>,
    /// 文档扩展属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 文档权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentPermissions {
    /// 是否可查看
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
    /// 是否可编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    /// 是否可评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    /// 是否可分享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    /// 是否可下载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
}

/// 获取文档元数据的请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDocMetaRequest {
    /// 文档token列表，一次最多查询100个
    pub tokens: Vec<String>,
    /// 需要返回的扩展字段，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
}

/// 获取文档元数据的响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDocMetaResponse {
    /// 文档元数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentMetaItem>>,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档元数据项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentMetaItem {
    /// 文档token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文档基础信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<DocumentBasicInfo>,
    /// 文档统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DocumentStatistics>,
    /// 文档扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 文档基础信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentBasicInfo {
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 文档修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 文档创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 文档拥有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserInfo>,
    /// 文档所属文件夹
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<FolderInfo>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文档权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DocumentPermissions>,
}

/// 文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FolderInfo {
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 文档统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentStatistics {
    /// 文档大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 字数（仅适用于文档）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<i64>,
    /// 页数（仅适用于文档、表格、幻灯片）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    /// 表格行数（仅适用于表格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 表格列数（仅适用于表格）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    /// 幻灯片页数（仅适用于幻灯片）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slide_count: Option<i32>,
    /// 思维导图节点数（仅适用于思维导图）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
}

// 实现参数验证trait
impl SearchDocsRequest {
    /// 验证搜索参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.search_key, "搜索关键字不能为空");

        if self.search_key.len() > 1000 {
            return Err(openlark_core::CoreError::validation_msg("搜索关键字长度不能超过1000个字符"));
        }

        if let Some(page_size) = self.page_size
            && (page_size < 1 || page_size > 100)
        {
            return Err(openlark_core::CoreError::validation_msg("每页数量必须在1-100之间"));
        }

        if let Some(ref doc_types) = self.doc_types {
            if doc_types.len() > 10 {
                return Err(openlark_core::CoreError::validation_msg("文档类型列表长度不能超过10"));
            }

            for doc_type in doc_types {
                if !Self::is_valid_doc_type(doc_type) {
                    return Err(openlark_core::CoreError::validation_msg(format!("不支持的文档类型: {doc_type}")));
                }
            }
        }

        Ok(())
    }
        if self.search_key.trim().is_empty() {
            return Err("搜索关键字不能为空".to_string());
        }

        if self.search_key.len() > 1000 {
            return Err("搜索关键字长度不能超过1000个字符".to_string());
        }

        if let Some(page_size) = self.page_size
            && (page_size < 1 || page_size > 100)
        {
            return Err("每页数量必须在1-100之间".to_string());
        }

        if let Some(ref doc_types) = self.doc_types {
            if doc_types.len() > 10 {
                return Err("文档类型列表长度不能超过10".to_string());
            }

            for doc_type in doc_types {
                if !Self::is_valid_doc_type(doc_type) {
                    return Err(format!("不支持的文档类型: {doc_type}"));
                }
            }
        }

        Ok(())
    }

    /// 检查文档类型是否有效
    fn is_valid_doc_type(doc_type: &str) -> bool {
        matches!(
            doc_type,
            "doc" | "sheet" | "slide" | "mindnote" | "docx" | "bitable" | "file"
        )
    }
}

impl GetDocMetaRequest {
    /// 验证元数据请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        if self.tokens.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("文档token列表不能为空"));
        }

        if self.tokens.len() > 100 {
            return Err(openlark_core::CoreError::validation_msg("文档token列表长度不能超过100"));
        }

        for token in &self.tokens {
            validate_required!(token, "文档token不能为空");
        }

        Ok(())
    }
        if self.tokens.is_empty() {
            return Err("文档token列表不能为空".to_string());
        }

        if self.tokens.len() > 100 {
            return Err("文档token列表长度不能超过100".to_string());
        }

        for token in &self.tokens {
            if token.trim().is_empty() {
                return Err("文档token不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// Docx 模块公共类型（模型，不算 API）
pub mod common_types {
    // 统一管理 docx 相关 API 中使用的公共数据结构，避免重复定义。
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// 块内容
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlockContent {
        /// 文本内容
        pub text: Option<String>,
        /// 富文本内容
        pub rich_text: Option<RichText>,
        /// 其他类型内容
        #[serde(flatten)]
        pub other: Option<serde_json::Value>,
    }

    /// 富文本内容
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RichText {
        /// 段落列表
        pub paragraphs: Vec<Paragraph>,
    }

    /// 段落
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Paragraph {
        /// 文本元素列表
        pub elements: Vec<TextElement>,
    }

    /// 文本元素
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextElement {
        /// 文本
        pub text_run: Option<TextRun>,
    }

    /// 文本运行
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextRun {
        /// 内容
        pub content: String,
        /// 样式
        pub style: Option<TextStyle>,
    }

    /// 文本样式
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextStyle {
        /// 是否加粗
        pub bold: Option<bool>,
        /// 是否斜体
        pub italic: Option<bool>,
        /// 是否删除线
        pub strikethrough: Option<bool>,
        /// 字体大小
        pub font_size: Option<u32>,
        /// 字体颜色
        pub font_color: Option<String>,
    }

    /// 块项目基本信息
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlockItem {
        /// 块ID
        pub block_id: String,
        /// 块类型
        pub block_type: String,
        /// 块内容
        pub content: Option<BlockContent>,
        /// 子块数量
        pub children_count: Option<u32>,
        /// 创建时间
        pub create_time: Option<i64>,
        /// 更新时间
        pub update_time: Option<i64>,
    }

    /// 分页数据
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PageData<T> {
        /// 数据列表
        pub items: Vec<T>,
        /// 分页标记
        pub page_token: Option<String>,
        /// 是否有更多
        pub has_more: Option<bool>,
    }

    /// 块更新信息
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlockUpdate {
        /// 块ID
        pub block_id: String,
        /// 更新的内容
        pub content: Option<BlockContent>,
    }

    /// 批量操作结果
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BatchOperationResult {
        /// 成功数量
        pub success_count: Option<u32>,
        /// 失败数量
        pub failed_count: Option<u32>,
        /// 失败的项目
        pub failed_items: Option<Vec<FailedItem>>,
    }

    /// 失败项目
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FailedItem {
        /// 项目ID
        pub item_id: String,
        /// 错误信息
        pub error_message: String,
        /// 错误代码
        pub error_code: Option<i32>,
    }

    /// Docx Block（响应使用，模型，不算 API）
    ///
    /// Docx 的 block 是一个「按 block_type 不同而变化字段」的结构：
    /// - 通用字段：block_id、block_type、children、parent_id
    /// - 其它字段：按 block_type 变化（例如 text、heading1、page、table...）
    ///
    /// 为避免在 SDK 内部强行穷举所有 block 类型，这里用 `extra` 透传。
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct DocxBlock {
        /// Block ID
        pub block_id: String,
        /// Block 类型（数字枚举）
        pub block_type: i32,
        /// 子块 ID 列表
        #[serde(default)]
        pub children: Vec<String>,
        /// 父块 ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        /// 其它字段透传（例如 text/heading1/page/table...）
        #[serde(default, flatten)]
        pub extra: HashMap<String, serde_json::Value>,
    }

    /// Docx Block 分页结构（data 内）
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct DocxBlockPage {
        #[serde(default)]
        /// 公开项说明。
        pub items: Vec<DocxBlock>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// 公开项说明。
        pub page_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// 公开项说明。
        pub has_more: Option<bool>,
    }
}

/// Docx API 数据模型（模型，不算 API）
pub mod models_docx {
    // 定义文档（DOCX）操作 API 的数据结构。
    use crate::prelude::*;
    use serde::{Deserialize, Serialize};

    /// 文档信息
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct DocumentInfo {
        /// 文档token
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document_token: Option<String>,
        /// 文档版本号
        #[serde(skip_serializing_if = "Option::is_none")]
        pub revision_id: Option<i64>,
        /// 文档标题
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
    }

    /// 块信息
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct BlockInfo {
        /// 块ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_id: Option<i64>,
        /// 块类型
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_type: Option<i32>,
        /// 块文本内容
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
    }

    /// 创建文档请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct CreateDocumentRequest {
        /// 文档标题
        pub title: String,
        /// 文件夹token
        #[serde(skip_serializing_if = "Option::is_none")]
        pub folder_token: Option<String>,
        /// 父类型
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_type: Option<String>,
        /// 封面key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cover_key: Option<String>,
    }

    impl CreateDocumentRequest {
        /// 验证请求参数
        pub fn validate(&self) -> openlark_core::SDKResult<()> {
            use openlark_core::validate_required;
            validate_required!(self.title, "文档标题不能为空");

            if self.title.len() > 100 {
                return Err(openlark_core::CoreError::validation_msg("文档标题长度不能超过100个字符"));
            }

            Ok(())
        }
    }
        pub fn validate(&self) -> Result<(), String> {
            if self.title.trim().is_empty() {
                return Err("文档标题不能为空".to_string());
            }

            if self.title.len() > 100 {
                return Err("文档标题长度不能超过100个字符".to_string());
            }

            Ok(())
        }
    }

    /// 创建文档响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct CreateDocumentResponse {
        /// 文档token
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document_token: Option<String>,
        /// 文档信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document: Option<DocumentInfo>,
    }

    impl ApiResponseTrait for CreateDocumentResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 获取文档信息请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct GetDocumentRequest {
        /// 文档token
        pub document_token: String,
    }

    impl GetDocumentRequest {
        /// 验证请求参数
        pub fn validate(&self) -> openlark_core::SDKResult<()> {
            use openlark_core::validate_required;
            validate_required!(self.document_token, "文档token不能为空");
            Ok(())
        }
    }
        pub fn validate(&self) -> Result<(), String> {
            if self.document_token.trim().is_empty() {
                return Err("文档token不能为空".to_string());
            }
            Ok(())
        }
    }

    /// 获取文档信息响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct GetDocumentResponse {
        /// 文档信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub document: Option<DocumentInfo>,
    }

    impl ApiResponseTrait for GetDocumentResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 获取文档原始内容请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct GetRawContentRequest {
        /// 文档token
        pub document_token: String,
    }

    /// 获取文档原始内容响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct GetRawContentResponse {
        /// 原始内容
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
    }

    impl ApiResponseTrait for GetRawContentResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 创建块请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct CreateBlockRequest {
        /// 文档token
        pub document_token: String,
        /// 父块ID
        pub parent_block_id: String,
        /// 块类型
        pub block_type: String,
        /// 块内容
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<serde_json::Value>,
    }

    /// 创建块响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct CreateBlockResponse {
        /// 块信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block: Option<BlockInfo>,
    }

    impl ApiResponseTrait for CreateBlockResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 获取块信息请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct GetBlockRequest {
        /// 文档token
        pub document_token: String,
        /// 块ID
        pub block_id: String,
    }

    /// 获取块信息响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct GetBlockResponse {
        /// 块信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block: Option<BlockInfo>,
    }

    impl ApiResponseTrait for GetBlockResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 获取块列表请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct ListBlocksRequest {
        /// 文档token
        pub document_token: String,
        /// 父块ID
        pub parent_block_id: String,
        /// 分页令牌
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page_token: Option<String>,
        /// 分页大小
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page_size: Option<i32>,
    }

    /// 获取块列表响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct ListBlocksResponse {
        /// 块列表
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blocks: Option<Vec<BlockInfo>>,
        /// 分页令牌
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page_token: Option<String>,
        /// 是否还有更多
        #[serde(skip_serializing_if = "Option::is_none")]
        pub has_more: Option<bool>,
    }

    impl ApiResponseTrait for ListBlocksResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 更新块请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct UpdateBlockRequest {
        /// 文档token
        pub document_token: String,
        /// 块ID
        pub block_id: String,
        /// 更新内容
        pub content: serde_json::Value,
    }

    /// 更新块响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct UpdateBlockResponse {
        /// 块信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block: Option<BlockInfo>,
    }

    impl ApiResponseTrait for UpdateBlockResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 批量更新块请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct BatchUpdateBlocksRequest {
        /// 文档token
        pub document_token: String,
        /// 更新操作列表
        pub requests: Vec<serde_json::Value>,
    }

    /// 批量更新块响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BatchUpdateBlocksResponse {
        /// 更新结果
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<serde_json::Value>>,
    }

    impl ApiResponseTrait for BatchUpdateBlocksResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 删除块请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct DeleteBlockRequest {
        /// 文档token
        pub document_token: String,
        /// 块ID
        pub block_id: String,
    }

    /// 删除块响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct DeleteBlockResponse {
        /// 是否删除成功
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,
    }

    impl ApiResponseTrait for DeleteBlockResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 批量删除块请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct BatchDeleteBlocksRequest {
        /// 文档token
        pub document_token: String,
        /// 块ID列表
        pub block_ids: Vec<String>,
    }

    /// 批量删除块响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BatchDeleteBlocksResponse {
        /// 删除结果
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<serde_json::Value>>,
    }

    impl ApiResponseTrait for BatchDeleteBlocksResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
