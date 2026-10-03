/// CCM Drive Permission v1 API 模块
///
/// 提供云盘权限管理相关的API功能,包括权限查询、拥有者转移等。
pub mod v2;

use serde::{Deserialize, Serialize};

/// 判断协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermittedResponse {
    /// 是否有权限
    pub permitted: bool,
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferResponse {
    /// 是否成功
    pub success: bool,
}

/// 获取公开权限设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermissionResponse {
    /// 公开权限设置
    pub public_permission: Option<PublicPermission>,
}

/// 公开权限设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermission {
    /// 是否公开
    pub public: bool,
    /// 权限类型
    pub permission_type: String,
}
