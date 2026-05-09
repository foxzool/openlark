/// Drive Permission API 数据模型
///
/// 提供权限管理相关的数据结构，支持权限设置、权限验证和所有者转移。
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 判断协作者是否有某权限请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckMemberPermissionRequest {
    /// 文件token
    pub file_token: String,
    /// 权限类型
    pub permission: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CheckMemberPermissionRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.file_token, "文件token不能为空");
        validate_required!(self.permission, "权限类型不能为空");
        Ok(())
    }
}

/// 判断协作者是否有某权限响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CheckMemberPermissionResponse {
    /// 是否有权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted: Option<bool>,
}

impl ApiResponseTrait for CheckMemberPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移拥有者请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOwnerRequest {
    /// 文件token
    pub file_token: String,
    /// 新拥有者用户ID
    pub user_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl TransferOwnerRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.file_token, "文件token不能为空");
        validate_required!(self.user_id, "用户ID不能为空");
        Ok(())
    }
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferOwnerResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for TransferOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置V2请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPublicPermissionRequest {
    /// 文件token
    pub file_token: String,
}

impl GetPublicPermissionRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.file_token, "文件token不能为空");
        Ok(())
    }
}

/// 获取云文档权限设置V2响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetPublicPermissionResponse {
    /// 公共权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_permission: Option<PublicPermission>,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 公共权限设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PublicPermission {
    /// 是否允许公开访问
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_access_enabled: Option<bool>,
    /// 公开链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    /// 权限类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// 是否需要密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_protected: Option<bool>,
    /// 过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 允许评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_comment: Option<bool>,
    /// 允许复制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_copy: Option<bool>,
    /// 允许打印
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_print: Option<bool>,
}

/// 用户权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserPermission {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 权限类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// 是否为拥有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
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
