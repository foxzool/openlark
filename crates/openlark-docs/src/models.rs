/// 文档服务通用数据模型
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// use openlark_core::api::Response;

/// 文档资源基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBase {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub doc_type: DocumentType,
    /// 创建时间
    pub create_time: Timestamp,
    /// 修改时间
    pub modify_time: Timestamp,
    /// 创建者信息
    pub creator: UserInfo,
    /// 修改者信息
    pub modifier: Option<UserInfo>,
    /// 文档状态
    pub status: DocumentStatus,
}

/// 文档类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    /// 文本文档
    Doc,
    /// 表格
    Sheet,
    /// 幻灯片
    Slide,
    /// 思维笔记
    Mindnote,
    /// 流程图
    Flowchart,
    /// 多人实时文档
    Bitable,
    /// Wiki页面
    Wiki,
    /// 文件夹
    Folder,
    /// 其他
    Other(String),
}

/// 文档状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    /// 正常
    Normal,
    /// 回收站
    Recycle,
    /// 删除
    Deleted,
    /// 加密
    Encrypted,
    /// 归档
    Archived,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像URL
    pub avatar_url: Option<String>,
    /// 部门信息
    pub department: Option<DepartmentInfo>,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentInfo {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    /// 文件ID
    pub file_id: String,
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: u64,
    /// MIME类型
    pub mime_type: String,
    /// 文件URL
    pub url: String,
    /// 下载Token
    pub download_token: Option<String>,
    /// 缩略图URL
    pub thumbnail_url: Option<String>,
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Permission {
    /// 权限类型
    pub permission_type: PermissionType,
    /// 是否可读
    pub can_read: bool,
    /// 是否可写
    pub can_write: bool,
    /// 是否可删除
    pub can_delete: bool,
    /// 是否可分享
    pub can_share: bool,
    /// 权限到期时间
    pub expire_time: Option<Timestamp>,
}

/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PermissionType {
    /// 所有者
    Owner,
    /// 管理员
    Admin,
    /// 编辑者
    Editor,
    /// 评论者
    Commenter,
    /// 查看者
    Viewer,
}

/// 分享信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShareInfo {
    /// 分享链接
    pub share_url: String,
    /// 分享Token
    pub share_token: String,
    /// 权限类型
    pub permission_type: PermissionType,
    /// 是否需要密码
    pub need_password: bool,
    /// 过期时间
    pub expire_time: Option<Timestamp>,
    /// 访问次数限制
    pub visit_limit: Option<u32>,
    /// 当前访问次数
    pub visit_count: u32,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionInfo {
    /// 版本号
    pub version: i32,
    /// 版本名称
    pub version_name: String,
    /// 创建时间
    pub create_time: Timestamp,
    /// 创建者
    pub creator: UserInfo,
    /// 版本描述
    pub description: Option<String>,
    /// 是否为当前版本
    pub is_current: bool,
    /// 文件大小
    pub size: u64,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 文档信息
    pub document: DocumentBase,
    /// 匹配的片段
    pub snippets: Vec<String>,
    /// 相关度分数
    pub score: f64,
}

/// 文档统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentStats {
    /// 浏览次数
    pub view_count: u64,
    /// 编辑次数
    pub edit_count: u64,
    /// 评论数量
    pub comment_count: u64,
    /// 分享次数
    pub share_count: u64,
    /// 下载次数
    pub download_count: u64,
    /// 最后统计时间
    pub last_stats_time: Timestamp,
}

/// 文档操作记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    /// 记录ID
    pub record_id: String,
    /// 操作类型
    pub operation_type: OperationType,
    /// 操作时间
    pub operation_time: Timestamp,
    /// 操作者
    pub operator: UserInfo,
    /// 操作描述
    pub description: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 设备信息
    pub device_info: Option<String>,
}

/// 操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    /// 创建
    Create,
    /// 读取
    Read,
    /// 更新
    Update,
    /// 删除
    Delete,
    /// 分享
    Share,
    /// 下载
    Download,
    /// 复制
    Copy,
    /// 移动
    Move,
    /// 重命名
    Rename,
    /// 权限变更
    PermissionChange,
    /// 其他
    Other(String),
}

/// 文档导入/导出任务
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportExportTask {
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: TaskType,
    /// 任务状态
    pub status: TaskStatus,
    /// 进度百分比 (0-100)
    pub progress: u8,
    /// 开始时间
    pub start_time: Timestamp,
    /// 结束时间
    pub end_time: Option<Timestamp>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 结果URL
    pub result_url: Option<String>,
}

/// 任务类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    /// 导入
    Import,
    /// 导出
    Export,
    /// 转换
    Convert,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 等待中
    Pending,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 已失败
    Failed,
    /// 已取消
    Canceled,
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonResponse<T: PartialEq> {
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<T>,
    /// 错误信息
    pub error: Option<ErrorInfo>,
    /// 请求ID
    pub request_id: String,
}

/// 错误信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorInfo {
    /// 错误代码
    pub code: i32,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<HashMap<String, serde_json::Value>>,
}

/// 分页请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageRequest {
    /// 页码（从1开始）
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageResponse<T: PartialEq> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总条数
    pub total: u64,
    /// 当前页码
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
    /// 总页数
    pub total_page: u32,
    /// 是否有下一页
    pub has_next: bool,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            sort_field: None,
            sort_direction: Some(SortDirection::Desc),
        }
    }
}

// Docs和Docx项目的特定模型
// 注意：这些模型的定义在各自的模块中，这里不重复导入

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_document_type_serialization() {
        test_roundtrip(&DocumentType::Doc);
        test_roundtrip(&DocumentType::Sheet);
        test_roundtrip(&DocumentType::Other("custom".to_string()));
    }

    #[test]
    fn test_document_status_serialization() {
        test_roundtrip(&DocumentStatus::Normal);
        test_roundtrip(&DocumentStatus::Archived);
    }

    #[test]
    fn test_permission_type_serialization() {
        test_roundtrip(&PermissionType::Owner);
        test_roundtrip(&PermissionType::Viewer);
    }

    #[test]
    fn test_operation_type_serialization() {
        test_roundtrip(&OperationType::Create);
        test_roundtrip(&OperationType::Other("custom_op".to_string()));
    }

    #[test]
    fn test_task_type_serialization() {
        test_roundtrip(&TaskType::Import);
        test_roundtrip(&TaskType::Export);
    }

    #[test]
    fn test_task_status_serialization() {
        test_roundtrip(&TaskStatus::Pending);
        test_roundtrip(&TaskStatus::Completed);
    }

    #[test]
    fn test_sort_direction_serialization() {
        test_roundtrip(&SortDirection::Asc);
        test_roundtrip(&SortDirection::Desc);
    }

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            user_id: "user123".to_string(),
            name: "测试用户".to_string(),
            email: Some("test@example.com".to_string()),
            avatar_url: None,
            department: Some(DepartmentInfo {
                department_id: "dept456".to_string(),
                name: "技术部".to_string(),
            }),
        };
        test_roundtrip(&user);
    }

    #[test]
    fn test_department_info_serialization() {
        let dept = DepartmentInfo {
            department_id: "dept789".to_string(),
            name: "产品部".to_string(),
        };
        test_roundtrip(&dept);
    }

    #[test]
    fn test_file_info_serialization() {
        let file = FileInfo {
            file_id: "file123".to_string(),
            name: "test.pdf".to_string(),
            size: 1024000,
            mime_type: "application/pdf".to_string(),
            url: "https://example.com/file".to_string(),
            download_token: Some("token123".to_string()),
            thumbnail_url: None,
        };
        test_roundtrip(&file);
    }

    #[test]
    fn test_permission_serialization() {
        let perm = Permission {
            permission_type: PermissionType::Editor,
            can_read: true,
            can_write: true,
            can_delete: false,
            can_share: true,
            expire_time: None,
        };
        test_roundtrip(&perm);
    }

    #[test]
    fn test_share_info_serialization() {
        let share = ShareInfo {
            share_url: "https://share.example.com/doc".to_string(),
            share_token: "token456".to_string(),
            permission_type: PermissionType::Viewer,
            need_password: false,
            expire_time: None,
            visit_limit: Some(100),
            visit_count: 10,
        };
        test_roundtrip(&share);
    }

    #[test]
    fn test_version_info_serialization() {
        let version = VersionInfo {
            version: 1,
            version_name: "v1.0".to_string(),
            create_time: Timestamp::now(),
            creator: UserInfo {
                user_id: "user001".to_string(),
                name: "张三".to_string(),
                email: None,
                avatar_url: None,
                department: None,
            },
            description: Some("初始版本".to_string()),
            is_current: true,
            size: 2048,
        };
        test_roundtrip(&version);
    }

    #[test]
    fn test_document_stats_serialization() {
        let stats = DocumentStats {
            view_count: 100,
            edit_count: 20,
            comment_count: 5,
            share_count: 10,
            download_count: 3,
            last_stats_time: Timestamp::now(),
        };
        test_roundtrip(&stats);
    }

    #[test]
    fn test_import_export_task_serialization() {
        let task = ImportExportTask {
            task_id: "task789".to_string(),
            task_type: TaskType::Convert,
            status: TaskStatus::Processing,
            progress: 50,
            start_time: Timestamp::now(),
            end_time: None,
            error_message: None,
            result_url: Some("https://result.example.com".to_string()),
        };
        test_roundtrip(&task);
    }

    #[test]
    fn test_error_info_serialization() {
        let mut details = HashMap::new();
        details.insert("field".to_string(), serde_json::json!("value"));

        let error = ErrorInfo {
            code: 404,
            message: "Not Found".to_string(),
            details: Some(details),
        };
        test_roundtrip(&error);
    }

    #[test]
    fn test_page_request_serialization() {
        let req = PageRequest {
            page: 1,
            page_size: 20,
            sort_field: Some("create_time".to_string()),
            sort_direction: Some(SortDirection::Desc),
        };
        test_roundtrip(&req);
    }

    #[test]
    fn test_page_request_default() {
        let default_req = PageRequest::default();
        assert_eq!(default_req.page, 1);
        assert_eq!(default_req.page_size, 20);
    }

    #[test]
    fn test_common_response_serialization() {
        let response: CommonResponse<String> = CommonResponse {
            success: true,
            data: Some("test data".to_string()),
            error: None,
            request_id: "req123".to_string(),
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_page_response_serialization() {
        let response: PageResponse<String> = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            total: 100,
            page: 1,
            page_size: 20,
            total_page: 5,
            has_next: true,
        };
        test_roundtrip(&response);
    }
}
