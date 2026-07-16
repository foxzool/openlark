//! Drive、Explorer 与 Permission API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};

/// CCM Drive Explorer API Old V2 端点枚举
/// 对应 meta.project = ccm_drive_explorer, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDriveExplorerApiOld {
    /// 获取我的空间（根文件夹）元数据
    RootFolderMeta,
    /// 获取文件夹元数据
    FolderMeta(String), // folder_token
    /// 新建文件
    File(String), // folder_token
    /// 删除Sheet
    FileSpreadsheets(String), // spreadsheet_token
    /// 复制文档
    FileCopy(String), // file_token
    /// 删除Doc
    FileDocs(String), // doc_token
    /// 获取文件夹下的文档清单
    FolderChildren(String), // folder_token
    /// 新建文件夹
    Folder(String), // folder_token
}

impl CcmDriveExplorerApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDriveExplorerApiOld::RootFolderMeta => {
                "/open-apis/drive/explorer/v2/root_folder/meta".to_string()
            }
            CcmDriveExplorerApiOld::FolderMeta(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{folder_token}/meta")
            }
            CcmDriveExplorerApiOld::File(folder_token) => {
                format!("/open-apis/drive/explorer/v2/file/{folder_token}")
            }
            CcmDriveExplorerApiOld::FileSpreadsheets(spreadsheet_token) => {
                format!("/open-apis/drive/explorer/v2/file/spreadsheets/{spreadsheet_token}")
            }
            CcmDriveExplorerApiOld::FileCopy(file_token) => {
                format!("/open-apis/drive/explorer/v2/file/copy/files/{file_token}")
            }
            CcmDriveExplorerApiOld::FileDocs(doc_token) => {
                format!("/open-apis/drive/explorer/v2/file/docs/{doc_token}")
            }
            CcmDriveExplorerApiOld::FolderChildren(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{folder_token}/children")
            }
            CcmDriveExplorerApiOld::Folder(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{folder_token}")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for CcmDriveExplorerApiOld {
    fn to_url(&self) -> String {
        CcmDriveExplorerApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::RootFolderMeta | Self::FolderMeta(_) | Self::FolderChildren(_) => HttpMethod::Get,
            Self::File(_) | Self::FileCopy(_) | Self::Folder(_) => HttpMethod::Post,
            Self::FileSpreadsheets(_) | Self::FileDocs(_) => HttpMethod::Delete,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// CCM Drive Explorer API V1 端点枚举
/// 对应 meta.project = ccm_drive_explorer, meta.version = v1
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDriveExplorerApi {
    /// 获取根目录元数据
    RootFolderMeta,
    /// 获取文件夹元数据
    FolderMeta(String), // folder_token
    /// 获取文件元数据
    File(String), // file_token
    /// 复制文件
    FileCopy(String), // file_token
    /// 获取文档文件信息
    FileDocs(String), // file_token
    /// 获取表格文件信息
    FileSpreadsheets(String), // file_token
    /// 获取文件夹子内容
    FolderChildren(String), // folder_token
    /// 创建文件夹
    Folder,
}

impl CcmDriveExplorerApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDriveExplorerApi::RootFolderMeta => {
                "/open-apis/drive/v1/explorer/root_folder/meta".to_string()
            }
            CcmDriveExplorerApi::FolderMeta(folder_token) => {
                format!("/open-apis/drive/v1/explorer/folder/{folder_token}/meta")
            }
            CcmDriveExplorerApi::File(file_token) => {
                format!("/open-apis/drive/v1/explorer/file/{file_token}")
            }
            CcmDriveExplorerApi::FileCopy(file_token) => {
                format!("/open-apis/drive/v1/explorer/file/copy/files/{file_token}")
            }
            CcmDriveExplorerApi::FileDocs(file_token) => {
                format!("/open-apis/drive/v1/explorer/file/docs/{file_token}")
            }
            CcmDriveExplorerApi::FileSpreadsheets(file_token) => {
                format!("/open-apis/drive/v1/explorer/file/spreadsheets/{file_token}")
            }
            CcmDriveExplorerApi::FolderChildren(folder_token) => {
                format!("/open-apis/drive/v1/explorer/folder/{folder_token}/children")
            }
            CcmDriveExplorerApi::Folder => "/open-apis/drive/v1/explorer/folder".to_string(),
        }
    }

    /// 生成带参数的 URL
    pub fn to_url_with_params(&self, params: &[(&str, String)]) -> String {
        let base_url = self.to_url();
        if params.is_empty() {
            return base_url;
        }

        let query_string = params
            .iter()
            .map(|(key, value)| format!("{}={}", key, simple_url_encode(value)))
            .collect::<Vec<_>>()
            .join("&");

        format!("{base_url}?{query_string}")
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for CcmDriveExplorerApi {
    fn to_url(&self) -> String {
        CcmDriveExplorerApi::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::RootFolderMeta
            | Self::FolderMeta(_)
            | Self::File(_)
            | Self::FileDocs(_)
            | Self::FileSpreadsheets(_)
            | Self::FolderChildren(_) => HttpMethod::Get,
            Self::FileCopy(_) | Self::Folder => HttpMethod::Post,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// 简单的URL编码函数，用于查询参数编码
fn simple_url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

/// CCM Drive Permission API V1 端点枚举
/// 对应 meta.project = permission, meta.version = v1
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionApi {
    /// 判断协作者是否有某权限
    MemberPermitted,
    /// 转移拥有者
    MemberTransfer,
    /// 获取云文档权限设置V2
    Public,
}

impl PermissionApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            PermissionApi::MemberPermitted => {
                "/open-apis/drive/v1/permission/member/permitted".to_string()
            }
            PermissionApi::MemberTransfer => {
                "/open-apis/drive/v1/permission/member/transfer".to_string()
            }
            PermissionApi::Public => "/open-apis/drive/v1/permission/v2/public/".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for PermissionApi {
    fn to_url(&self) -> String {
        PermissionApi::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// CCM Drive Permission API Old V2 端点枚举
/// 对应 meta.project = permission, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionApiOld {
    /// 判断协作者是否有某权限
    MemberPermitted,
    /// 转移拥有者
    MemberTransfer,
    /// 获取云文档权限设置V2
    Public,
}

impl PermissionApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            PermissionApiOld::MemberPermitted => {
                "/open-apis/drive/v1/permission/member/permitted".to_string()
            }
            PermissionApiOld::MemberTransfer => {
                "/open-apis/drive/v1/permission/member/transfer".to_string()
            }
            PermissionApiOld::Public => "/open-apis/drive/v1/permission/v2/public/".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for PermissionApiOld {
    fn to_url(&self) -> String {
        PermissionApiOld::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// Drive API 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DriveApi {
    // V1 APIs - 文件操作
    /// 获取文件夹中的文件清单
    ListFiles,
    /// 新建文件夹
    CreateFolder,
    /// 查询异步任务状态
    TaskCheck,
    /// 获取文件元数据（批量查询）
    BatchQueryMetas,
    /// 获取文件统计信息
    GetFileStatistics(String), // file_token
    /// 获取文件访问记录
    ListFileViewRecords(String), // file_token
    /// 复制文件
    CopyFile(String), // file_token
    /// 移动文件或文件夹
    MoveFile(String), // file_token
    /// 删除文件或文件夹
    DeleteFile(String), // file_token
    /// 创建文件快捷方式
    CreateShortcut,
    /// 上传文件
    UploadFile,
    /// 分片上传文件-预上传
    UploadPrepare,
    /// 分片上传文件-上传分片
    UploadPart,
    /// 分片上传文件-完成上传
    UploadFinish,
    /// 下载文件
    DownloadFile(String), // file_token
    /// 创建导入任务
    CreateImportTask,
    /// 查询导入任务结果
    GetImportTask(String), // ticket
    /// 创建导出任务
    CreateExportTask,
    /// 查询导出任务结果
    GetExportTask(String), // ticket
    /// 下载导出文件
    DownloadExportFile(String), // file_token
    /// 上传素材
    UploadMedia,
    /// 分片上传素材-预上传
    UploadMediaPrepare,
    /// 分片上传素材-上传分片
    UploadMediaPart,
    /// 分片上传素材-完成上传
    UploadMediaFinish,
    /// 下载素材
    DownloadMedia(String), // file_token
    /// 获取素材临时下载链接
    GetMediaTempDownloadUrls,
    /// 创建文档版本
    CreateFileVersion(String), // file_token
    /// 获取文档版本列表
    ListFileVersions(String), // file_token
    /// 获取文档版本信息
    GetFileVersion(String, String), // file_token, version_id
    /// 删除文档版本
    DeleteFileVersion(String, String), // file_token, version_id
    /// 订阅云文档事件
    SubscribeFile(String), // file_token
    /// 查询云文档事件订阅状态
    GetFileSubscribe(String), // file_token
    /// 取消云文档事件订阅
    DeleteFileSubscribe(String), // file_token
    /// 增加协作者权限
    CreatePermissionMember(String), // token
    /// 批量增加协作者权限
    BatchCreatePermissionMember(String), // token
    /// 更新协作者权限
    UpdatePermissionMember(String, String), // token, member_id
    /// 获取云文档协作者
    ListPermissionMembers(String), // token
    /// 移除云文档协作者权限
    DeletePermissionMember(String, String), // token, member_id
    /// 转移云文档所有者
    TransferOwner(String), // token
    /// 判断用户云文档权限
    AuthPermissionMember(String), // token
    /// 更新云文档权限设置
    UpdatePublicPermission(String), // token
    /// 获取云文档权限设置
    GetPublicPermission(String), // token
    /// 启用云文档密码
    CreatePublicPassword(String), // token
    /// 刷新云文档密码
    UpdatePublicPassword(String), // token
    /// 停用云文档密码
    DeletePublicPassword(String), // token
    /// 获取云文档所有评论
    ListFileComments(String), // file_token
    /// 批量获取评论
    BatchQueryComments(String), // file_token
    /// 解决/恢复评论
    PatchComment(String, String), // file_token, comment_id
    /// 添加全文评论
    CreateComment(String), // file_token
    /// 获取全文评论
    GetComment(String, String), // file_token, comment_id
    /// 获取回复信息
    ListCommentReplies(String, String), // file_token, comment_id
    /// 添加回复
    CreateCommentReply(String, String), // file_token, comment_id
    /// 更新回复的内容
    UpdateCommentReply(String, String, String), // file_token, comment_id, reply_id
    /// 删除回复
    DeleteCommentReply(String, String, String), // file_token, comment_id, reply_id
    /// 订阅用户云文档事件
    UserSubscription,
    /// 取消用户云文档事件订阅
    UserRemoveSubscription,
    /// 查询用户云文档事件订阅状态
    UserSubscriptionStatus,
    /// 获取订阅状态
    GetFileSubscription(String, String), // file_token, subscription_id
    /// 创建订阅
    CreateFileSubscription(String), // file_token
    /// 更新订阅状态
    UpdateFileSubscription(String, String), // file_token, subscription_id

    // V2 APIs
    /// 获取云文档的点赞者列表
    ListFileLikes(String), // file_token
    /// 获取云文档权限设置（v2）
    GetPublicPermissionV2(String), // token
    /// 更新云文档权限设置（v2）
    UpdatePublicPermissionV2(String), // token
    /// 添加或取消评论表情回应
    UpdateCommentReaction(String), // file_token

    // Media Upload Task APIs
    /// 创建媒体上传任务
    MediaUploadTasks,
    /// 获取媒体上传任务
    MediaUploadTask(String), // task_id
    /// 创建媒体分享链接
    CreateMediaShareLink(String), // file_token
    /// 获取公开密码
    GetPublicPassword(String), // file_token
}

impl DriveApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // V1 File APIs
            DriveApi::ListFiles => "/open-apis/drive/v1/files".to_string(),
            DriveApi::CreateFolder => "/open-apis/drive/v1/files/create_folder".to_string(),
            DriveApi::TaskCheck => "/open-apis/drive/v1/files/task_check".to_string(),
            DriveApi::BatchQueryMetas => "/open-apis/drive/v1/metas/batch_query".to_string(),
            DriveApi::GetFileStatistics(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/statistics")
            }
            DriveApi::ListFileViewRecords(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/view_records")
            }
            DriveApi::CopyFile(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/copy")
            }
            DriveApi::MoveFile(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/move")
            }
            DriveApi::DeleteFile(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}")
            }
            DriveApi::CreateShortcut => "/open-apis/drive/v1/files/create_shortcut".to_string(),
            DriveApi::UploadFile => "/open-apis/drive/v1/files/upload_all".to_string(),
            DriveApi::UploadPrepare => "/open-apis/drive/v1/files/upload_prepare".to_string(),
            DriveApi::UploadPart => "/open-apis/drive/v1/files/upload_part".to_string(),
            DriveApi::UploadFinish => "/open-apis/drive/v1/files/upload_finish".to_string(),
            DriveApi::DownloadFile(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/download")
            }

            // Import/Export Task APIs
            DriveApi::CreateImportTask => "/open-apis/drive/v1/import_tasks".to_string(),
            DriveApi::GetImportTask(ticket) => {
                format!("/open-apis/drive/v1/import_tasks/{ticket}")
            }
            DriveApi::CreateExportTask => "/open-apis/drive/v1/export_tasks".to_string(),
            DriveApi::GetExportTask(ticket) => {
                format!("/open-apis/drive/v1/export_tasks/{ticket}")
            }
            DriveApi::DownloadExportFile(file_token) => {
                format!("/open-apis/drive/v1/export_tasks/file/{file_token}/download")
            }

            // Media APIs
            DriveApi::UploadMedia => "/open-apis/drive/v1/medias/upload_all".to_string(),
            DriveApi::UploadMediaPrepare => "/open-apis/drive/v1/medias/upload_prepare".to_string(),
            DriveApi::UploadMediaPart => "/open-apis/drive/v1/medias/upload_part".to_string(),
            DriveApi::UploadMediaFinish => "/open-apis/drive/v1/medias/upload_finish".to_string(),
            DriveApi::DownloadMedia(file_token) => {
                format!("/open-apis/drive/v1/medias/{file_token}/download")
            }
            DriveApi::GetMediaTempDownloadUrls => {
                "/open-apis/drive/v1/medias/batch_get_tmp_download_url".to_string()
            }

            // File Version APIs
            DriveApi::CreateFileVersion(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/versions")
            }
            DriveApi::ListFileVersions(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/versions")
            }
            DriveApi::GetFileVersion(file_token, version_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/versions/{version_id}")
            }
            DriveApi::DeleteFileVersion(file_token, version_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/versions/{version_id}")
            }

            // Subscription APIs
            DriveApi::SubscribeFile(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/subscribe")
            }
            DriveApi::GetFileSubscribe(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/get_subscribe")
            }
            DriveApi::DeleteFileSubscribe(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/delete_subscribe")
            }

            // Permission Member APIs
            DriveApi::CreatePermissionMember(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/members")
            }
            DriveApi::BatchCreatePermissionMember(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/members/batch_create")
            }
            DriveApi::UpdatePermissionMember(token, member_id) => {
                format!("/open-apis/drive/v1/permissions/{token}/members/{member_id}")
            }
            DriveApi::ListPermissionMembers(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/members")
            }
            DriveApi::DeletePermissionMember(token, member_id) => {
                format!("/open-apis/drive/v1/permissions/{token}/members/{member_id}")
            }
            DriveApi::TransferOwner(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/members/transfer_owner")
            }
            DriveApi::AuthPermissionMember(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/members/auth")
            }

            // Permission Public APIs
            DriveApi::UpdatePublicPermission(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/public")
            }
            DriveApi::GetPublicPermission(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/public")
            }
            DriveApi::CreatePublicPassword(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/public/password")
            }
            DriveApi::UpdatePublicPassword(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/public/password")
            }
            DriveApi::DeletePublicPassword(token) => {
                format!("/open-apis/drive/v1/permissions/{token}/public/password")
            }

            // Comment APIs
            DriveApi::ListFileComments(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments")
            }
            DriveApi::BatchQueryComments(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments/batch_query")
            }
            DriveApi::PatchComment(file_token, comment_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments/{comment_id}")
            }
            DriveApi::CreateComment(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments")
            }
            DriveApi::GetComment(file_token, comment_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments/{comment_id}")
            }
            DriveApi::ListCommentReplies(file_token, comment_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies")
            }
            DriveApi::CreateCommentReply(file_token, comment_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies")
            }
            DriveApi::UpdateCommentReply(file_token, comment_id, reply_id) => {
                format!(
                    "/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies/{reply_id}"
                )
            }
            DriveApi::DeleteCommentReply(file_token, comment_id, reply_id) => {
                format!(
                    "/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies/{reply_id}"
                )
            }
            DriveApi::UserSubscription => "/open-apis/drive/v1/user/subscription".to_string(),
            DriveApi::UserRemoveSubscription => {
                "/open-apis/drive/v1/user/remove_subscription".to_string()
            }
            DriveApi::UserSubscriptionStatus => {
                "/open-apis/drive/v1/user/subscription_status".to_string()
            }

            // File Subscription APIs
            DriveApi::GetFileSubscription(file_token, subscription_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/subscriptions/{subscription_id}")
            }
            DriveApi::CreateFileSubscription(file_token) => {
                format!("/open-apis/drive/v1/files/{file_token}/subscriptions")
            }
            DriveApi::UpdateFileSubscription(file_token, subscription_id) => {
                format!("/open-apis/drive/v1/files/{file_token}/subscriptions/{subscription_id}")
            }

            // V2 APIs
            DriveApi::ListFileLikes(file_token) => {
                format!("/open-apis/drive/v2/files/{file_token}/likes")
            }
            DriveApi::GetPublicPermissionV2(token) => {
                format!("/open-apis/drive/v2/permissions/{token}/public")
            }
            DriveApi::UpdatePublicPermissionV2(token) => {
                format!("/open-apis/drive/v2/permissions/{token}/public")
            }
            DriveApi::UpdateCommentReaction(file_token) => {
                format!("/open-apis/drive/v2/files/{file_token}/comments/reaction")
            }

            // Media Upload Task APIs
            DriveApi::MediaUploadTasks => "/open-apis/drive/v1/medias/upload_tasks".to_string(),
            DriveApi::MediaUploadTask(task_id) => {
                format!("/open-apis/drive/v1/medias/upload_tasks/{task_id}")
            }
            DriveApi::CreateMediaShareLink(file_token) => {
                format!("/open-apis/drive/v1/medias/{file_token}/share_link")
            }
            DriveApi::GetPublicPassword(file_token) => {
                format!("/open-apis/drive/v1/publics/{file_token}/password")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }

    /// 使用补充了动态 query 的 URL 构建请求。
    pub fn to_request_with_url<R>(&self, url: impl Into<String>) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request_with_url(self, url)
    }
}

impl CatalogEndpoint for DriveApi {
    fn to_url(&self) -> String {
        DriveApi::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::ListFiles
            | Self::TaskCheck
            | Self::GetFileStatistics(_)
            | Self::ListFileViewRecords(_)
            | Self::GetImportTask(_)
            | Self::GetExportTask(_)
            | Self::DownloadFile(_)
            | Self::DownloadExportFile(_)
            | Self::DownloadMedia(_)
            | Self::GetMediaTempDownloadUrls
            | Self::ListFileVersions(_)
            | Self::GetFileVersion(_, _)
            | Self::GetFileSubscribe(_)
            | Self::ListPermissionMembers(_)
            | Self::AuthPermissionMember(_)
            | Self::GetPublicPermission(_)
            | Self::ListFileComments(_)
            | Self::GetComment(_, _)
            | Self::ListCommentReplies(_, _)
            | Self::GetFileSubscription(_, _)
            | Self::ListFileLikes(_)
            | Self::GetPublicPermissionV2(_)
            | Self::MediaUploadTask(_)
            | Self::GetPublicPassword(_) => HttpMethod::Get,
            Self::UserSubscriptionStatus => HttpMethod::Get,
            Self::CreateFolder
            | Self::CopyFile(_)
            | Self::MoveFile(_)
            | Self::CreateShortcut
            | Self::UploadFile
            | Self::UploadPrepare
            | Self::UploadPart
            | Self::UploadFinish
            | Self::CreateImportTask
            | Self::CreateExportTask
            | Self::UploadMedia
            | Self::UploadMediaPrepare
            | Self::UploadMediaPart
            | Self::UploadMediaFinish
            | Self::CreateFileVersion(_)
            | Self::SubscribeFile(_)
            | Self::CreatePermissionMember(_)
            | Self::BatchCreatePermissionMember(_)
            | Self::TransferOwner(_)
            | Self::CreatePublicPassword(_)
            | Self::CreateComment(_)
            | Self::CreateCommentReply(_, _)
            | Self::UserSubscription
            | Self::UpdateCommentReaction(_)
            | Self::CreateFileSubscription(_)
            | Self::MediaUploadTasks
            | Self::CreateMediaShareLink(_)
            | Self::BatchQueryMetas
            | Self::BatchQueryComments(_) => HttpMethod::Post,
            Self::UpdatePermissionMember(_, _)
            | Self::UpdatePublicPassword(_)
            | Self::UpdateCommentReply(_, _, _) => HttpMethod::Put,
            Self::UpdatePublicPermission(_)
            | Self::PatchComment(_, _)
            | Self::UpdateFileSubscription(_, _)
            | Self::UpdatePublicPermissionV2(_) => HttpMethod::Patch,
            Self::DeleteFile(_)
            | Self::DeleteFileVersion(_, _)
            | Self::DeleteFileSubscribe(_)
            | Self::DeletePermissionMember(_, _)
            | Self::DeletePublicPassword(_)
            | Self::DeleteCommentReply(_, _, _) => HttpMethod::Delete,
            Self::UserRemoveSubscription => HttpMethod::Delete,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::test_support::assert_endpoint_semantics;

    #[test]
    fn drive_catalog_covers_every_family_and_http_method_class() {
        assert_endpoint_semantics(
            CcmDriveExplorerApiOld::RootFolderMeta,
            HttpMethod::Get,
            "/open-apis/drive/explorer/v2/root_folder/meta",
        );
        assert_endpoint_semantics(
            CcmDriveExplorerApiOld::File("folder".into()),
            HttpMethod::Post,
            "/open-apis/drive/explorer/v2/file/folder",
        );
        assert_endpoint_semantics(
            CcmDriveExplorerApiOld::FileDocs("doc".into()),
            HttpMethod::Delete,
            "/open-apis/drive/explorer/v2/file/docs/doc",
        );

        assert_endpoint_semantics(
            CcmDriveExplorerApi::RootFolderMeta,
            HttpMethod::Get,
            "/open-apis/drive/v1/explorer/root_folder/meta",
        );
        assert_endpoint_semantics(
            CcmDriveExplorerApi::Folder,
            HttpMethod::Post,
            "/open-apis/drive/v1/explorer/folder",
        );
        assert_endpoint_semantics(
            PermissionApi::MemberPermitted,
            HttpMethod::Post,
            "/open-apis/drive/v1/permission/member/permitted",
        );
        assert_endpoint_semantics(
            PermissionApiOld::MemberPermitted,
            HttpMethod::Post,
            "/open-apis/drive/v1/permission/member/permitted",
        );

        assert_endpoint_semantics(
            DriveApi::ListFiles,
            HttpMethod::Get,
            "/open-apis/drive/v1/files",
        );
        assert_endpoint_semantics(
            DriveApi::CreateFolder,
            HttpMethod::Post,
            "/open-apis/drive/v1/files/create_folder",
        );
        assert_endpoint_semantics(
            DriveApi::UpdatePermissionMember("token".into(), "member".into()),
            HttpMethod::Put,
            "/open-apis/drive/v1/permissions/token/members/member",
        );
        assert_endpoint_semantics(
            DriveApi::UpdatePublicPermission("token".into()),
            HttpMethod::Patch,
            "/open-apis/drive/v1/permissions/token/public",
        );
        assert_endpoint_semantics(
            DriveApi::DeleteFile("token".into()),
            HttpMethod::Delete,
            "/open-apis/drive/v1/files/token",
        );
    }
}
