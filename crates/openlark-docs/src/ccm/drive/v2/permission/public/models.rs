use serde::{Deserialize, Serialize};

/// 云文档权限设置（permission_public）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPublic {
    /// 是否允许内容被分享到组织外
    pub external_access_entity: Option<String>,
    /// 谁可以复制内容、创建副本、打印、下载
    pub security_entity: Option<String>,
    /// 谁可以评论
    pub comment_entity: Option<String>,
    /// 从组织维度，设置谁可以查看、添加、移除协作者
    pub share_entity: Option<String>,
    /// 从协作者维度，设置谁可以查看、添加、移除协作者
    pub manage_collaborator_entity: Option<String>,
    /// 链接分享设置
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容
    pub copy_entity: Option<String>,
    /// 节点是否已加锁（加锁后不再继承父级页面权限）
    pub lock_switch: Option<bool>,
}
