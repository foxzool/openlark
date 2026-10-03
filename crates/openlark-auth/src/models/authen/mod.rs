//! Authen相关数据模型
//!
//! 本模块包含用户认证相关的数据结构定义。

use chrono::{DateTime, Utc};
use openlark_core::api::responses::ApiResponseTrait;
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Deref;

mod token;

pub use token::{
    OidcRefreshUserAccessTokenRequest, OidcUserAccessTokenRequest, RefreshUserAccessTokenV1Request,
    UserAccessTokenResponse, UserAccessTokenV1Request,
};

/// 用户信息响应
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct UserInfoResponse {
    /// 用户信息
    pub data: UserInfo,
}

impl<'de> Deserialize<'de> for UserInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let data_value = value.get("data").cloned().unwrap_or(value);
        let data = UserInfo::deserialize(data_value).map_err(serde::de::Error::custom)?;
        Ok(Self { data })
    }
}

impl Deref for UserInfoResponse {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// 用户Open ID
    pub open_id: String,
    /// 用户Union ID
    pub union_id: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户名
    pub name: Option<String>,
    /// 用户英文名/昵称
    pub en_name: Option<String>,
    /// 用户个人邮箱地址
    pub email: Option<String>,
    /// 用户企业邮箱地址
    pub enterprise_email: Option<String>,
    /// 用户手机号码
    pub mobile: Option<String>,
    /// 用户头像URL
    pub avatar_url: Option<String>,
    /// 用户头像
    pub avatar: Option<UserAvatar>,
    /// 用户状态
    pub status: Option<UserStatus>,
    /// 所属部门信息
    pub department_ids: Option<Vec<String>>,
    /// 群组信息
    pub group_ids: Option<Vec<String>>,
    /// 职位信息
    pub positions: Option<Vec<String>>,
    /// 工号
    pub employee_no: Option<String>,
    /// 钉钉UserId
    pub dingtalk_user_id: Option<String>,
    /// 企业扩展属性
    pub enterprise_extension: Option<UserEnterpriseExtension>,
    /// 自定义属性
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    /// 租户key
    pub tenant_key: Option<String>,
}

/// 用户头像
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAvatar {
    /// 头像72x72
    pub avatar_72: Option<String>,
    /// 头像240x240
    pub avatar_240: Option<String>,
    /// 头像640x640
    pub avatar_640: Option<String>,
    /// 头像原图
    pub avatar_origin: Option<String>,
}

/// 用户状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatus {
    /// 是否激活
    pub is_activated: Option<bool>,
    /// 是否加入
    pub is_joined: Option<bool>,
    /// 是否预留
    pub is_reserved: Option<bool>,
    /// 是否离职
    pub is_exited: Option<bool>,
}

/// 企业扩展属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserEnterpriseExtension {
    /// 入职时间
    pub hire_date: Option<DateTime<Utc>>,
    /// 所属国家
    pub location: Option<String>,
    /// 工作地点
    pub work_station: Option<String>,
    /// 工作地点ID
    pub work_station_id: Option<String>,
    /// 通讯地址
    pub address: Option<String>,
    /// 邮政编码
    pub postcode: Option<String>,
    /// 订购电话
    pub mobile_phone: Option<String>,
    /// 分机号
    pub extension_number: Option<String>,
    /// 联系电话
    pub contact_phone: Option<String>,
    /// 主要电话
    pub primary_phone: Option<String>,
    /// 紧急联系人
    pub emergency_contact: Option<String>,
    /// 紧急联系电话
    pub emergency_phone: Option<String>,
    /// 家庭电话
    pub home_phone: Option<String>,
}

/// 用户自定义属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCustomAttr {
    /// 属性Key
    pub key: Option<String>,
    /// 属性值
    pub value: Option<serde_json::Value>,
}

/// 用户信息获取请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoRequest {
    /// 用户访问令牌
    pub user_access_token: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

// 为所有响应类型实现ApiResponseTrait
impl ApiResponseTrait for UserInfoResponse {}
impl ApiResponseTrait for UserAccessTokenResponse {}

/// 用户令牌信息
#[derive(Debug, Clone)]
pub struct UserTokenInfo {
    /// 用户访问令牌
    pub user_access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
    /// 令牌类型
    pub token_type: String,
    /// 授权范围
    pub scope: Option<String>,
    /// 获取时间
    pub created_at: DateTime<Utc>,
}

impl UserTokenInfo {
    /// 创建新的用户令牌信息
    pub fn new(
        user_access_token: String,
        expires_in: u64,
        token_type: String,
        refresh_token: Option<String>,
        scope: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(expires_in as i64);

        Self {
            user_access_token,
            refresh_token,
            expires_at,
            token_type,
            scope,
            created_at: now,
        }
    }

    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// 检查令牌是否即将过期（提前5分钟）
    pub fn is_expiring_soon(&self) -> bool {
        let soon = Utc::now() + chrono::Duration::minutes(5);
        soon >= self.expires_at
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }

    /// 检查是否有刷新令牌
    pub fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_serialization() {
        let user_info = UserInfo {
            open_id: "test_open_id".to_string(),
            union_id: Some("test_union_id".to_string()),
            user_id: Some("test_user_id".to_string()),
            name: Some("测试用户".to_string()),
            en_name: Some("Test User".to_string()),
            email: Some("test@example.com".to_string()),
            enterprise_email: Some("test@company.com".to_string()),
            mobile: Some("13800138000".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            avatar: None,
            status: None,
            department_ids: None,
            group_ids: None,
            positions: None,
            employee_no: None,
            dingtalk_user_id: None,
            enterprise_extension: None,
            custom_attrs: None,
            tenant_key: None,
        };

        let json = serde_json::to_string(&user_info).unwrap();
        assert!(json.contains("test_open_id"));
        assert!(json.contains("测试用户"));
    }

    #[test]
    fn test_user_token_info() {
        let token = UserTokenInfo::new(
            "test_token".to_string(),
            3600,
            "Bearer".to_string(),
            Some("refresh_token".to_string()),
            Some("user_info".to_string()),
        );

        assert_eq!(token.user_access_token, "test_token");
        assert_eq!(token.token_type, "Bearer");
        assert!(token.has_refresh_token());
        assert!(!token.is_expired());
        assert!(token.remaining_seconds() > 0);
    }

    #[test]
    fn test_user_avatar() {
        let avatar = UserAvatar {
            avatar_72: Some("https://example.com/72.jpg".to_string()),
            avatar_240: Some("https://example.com/240.jpg".to_string()),
            avatar_640: Some("https://example.com/640.jpg".to_string()),
            avatar_origin: Some("https://example.com/original.jpg".to_string()),
        };

        let json = serde_json::to_string(&avatar).unwrap();
        assert!(json.contains("72.jpg"));
        assert!(json.contains("240.jpg"));
    }

    #[test]
    fn test_user_info_with_enterprise_email() {
        // 测试包含企业邮箱的用户信息
        let user_info = UserInfo {
            open_id: "test_open_id".to_string(),
            union_id: Some("test_union_id".to_string()),
            user_id: Some("test_user_id".to_string()),
            name: Some("测试用户".to_string()),
            en_name: Some("Test User".to_string()),
            email: Some("test@example.com".to_string()),
            enterprise_email: Some("test@company.com".to_string()),
            mobile: Some("13800138000".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            avatar: None,
            status: None,
            department_ids: None,
            group_ids: None,
            positions: None,
            employee_no: None,
            dingtalk_user_id: None,
            enterprise_extension: None,
            custom_attrs: None,
            tenant_key: None,
        };

        // 序列化
        let json = serde_json::to_string(&user_info).unwrap();
        assert!(json.contains("enterprise_email"));
        assert!(json.contains("test@company.com"));

        // 反序列化
        let deserialized: UserInfo = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(
            deserialized.enterprise_email,
            Some("test@company.com".to_string())
        );

        // 测试没有企业邮箱的情况
        let user_info_no_enterprise_email = UserInfo {
            enterprise_email: None,
            ..user_info
        };

        let json_no_enterprise = serde_json::to_string(&user_info_no_enterprise_email).unwrap();
        let deserialized_no_enterprise: UserInfo =
            serde_json::from_str(&json_no_enterprise).expect("JSON 反序列化失败");
        assert!(deserialized_no_enterprise.enterprise_email.is_none());
    }
}
