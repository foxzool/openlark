/// CCM Drive Explorer V2 数据模型
use serde::{Deserialize, Serialize};

/// 获取文件夹元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMetaResponse {
    /// 文件夹元信息
    pub data: Option<FolderMeta>,
}

/// 文件夹元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMeta {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹标题
    pub title: String,
    /// 文件夹类型
    #[serde(rename = "folder_type")]
    pub folder_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 所有者信息
    pub owner: Option<UserInfo>,
}

/// 新建文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileParams {
    /// 文件标题
    pub title: String,
    /// 文件类型
    #[serde(rename = "parent_type")]
    pub parent_type: String,
    /// 文件格式（可选）
    #[serde(rename = "file_extension")]
    pub file_extension: Option<String>,
}

/// 新建文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// 文件信息
    pub data: Option<FileInfo>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 文件标题
    pub title: String,
    /// 文件类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
}

/// 复制文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileParams {
    /// 目标文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 新文件名（可选）
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// 复制文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// 复制结果
    pub data: Option<CopyResult>,
}

/// 复制结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyResult {
    /// 复制的文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 文件标题
    pub title: String,
}

/// 获取文件夹下的文档清单请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenParams {
    /// 每页数量，默认20，最大100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 文件类型过滤
    #[serde(rename = "doc_type")]
    pub doc_type: Option<String>,
}

/// 获取文件夹下的文档清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenResponse {
    /// 文档列表
    pub data: Option<FolderChildrenData>,
}

/// 文件夹子项数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderChildrenData {
    /// 文件/文件夹列表
    pub items: Vec<FileItem>,
    /// 是否有更多数据
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页面标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 文件/文件夹项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 文件标题
    pub title: String,
    /// 文件类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 是否为文件夹
    #[serde(rename = "is_folder")]
    pub is_folder: bool,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
}

/// 新建文件夹请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderParams {
    /// 文件夹标题
    pub title: String,
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 文件夹信息
    pub data: Option<NewFolderInfo>,
}

/// 新建文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFolderInfo {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹标题
    pub title: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名称
    pub name: String,
}

/// 删除操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// 删除结果
    pub data: Option<DeleteResult>,
}

/// 删除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}
