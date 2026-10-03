/// CCM Drive Permission V2 数据模型
use serde::{Deserialize, Serialize};

/// 判断协作者是否有某权限请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemberPermissionParams {
    /// 文件token
    #[serde(rename = "obj_token")]
    pub obj_token: String,
    /// 权限类型
    pub permission: String,
    /// 用户ID（可选，默认为当前用户）
    #[serde(rename = "member_id")]
    pub member_id: Option<String>,
    /// 用户ID类型
    #[serde(rename = "member_id_type")]
    pub member_id_type: Option<String>,
}

/// 判断协作者是否有某权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemberPermissionResponse {
    /// 权限检查结果
    pub data: Option<PermissionCheckResult>,
}

/// 权限检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResult {
    /// 是否有权限
    pub permitted: bool,
    /// 权限详情
    pub permission: Option<String>,
}

/// 转移拥有者请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnerParams {
    /// 文件token
    #[serde(rename = "obj_token")]
    pub obj_token: String,
    /// 新拥有者用户ID
    #[serde(rename = "member_id")]
    pub member_id: String,
    /// 用户ID类型
    #[serde(rename = "member_id_type")]
    pub member_id_type: String,
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnerResponse {
    /// 转移结果
    pub data: Option<TransferResult>,
}

/// 转移结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

/// 获取云文档权限设置V2请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionParams {
    /// 文件token
    #[serde(rename = "obj_token")]
    pub obj_token: String,
}

/// 获取云文档权限设置V2响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionResponse {
    /// 权限设置信息
    pub data: Option<PublicPermission>,
}

/// 公开权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPermission {
    /// 是否公开
    #[serde(rename = "is_public")]
    pub is_public: bool,
    /// 公开链接
    #[serde(rename = "public_link")]
    pub public_link: Option<String>,
    /// 权限类型
    #[serde(rename = "link_type")]
    pub link_type: Option<String>,
    /// 是否需要密码
    #[serde(rename = "need_password")]
    pub need_password: bool,
    /// 过期时间
    #[serde(rename = "expire_time")]
    pub expire_time: Option<i64>,
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
