//! 审批 v4 共享嵌套模型
//!
//! 多个创建类 API（创建审批定义 / 创建三方审批定义 / 创建审批实例 / 同步三方审批实例）
//! 复用的官方嵌套结构：国际化文案 `i18n_resource`、可见人 `approval_create_viewers`。

use serde::{Deserialize, Serialize};

/// 国际化文案（i18n_resource）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct I18nResourceV4 {
    /// 语言（zh-CN / en-US / ja-JP 等）
    pub locale: String,
    /// 文案的 Key:Value 列表
    pub texts: Vec<I18nResourceTextV4>,
    /// 是否为默认语言
    pub is_default: bool,
}

/// 国际化文案条目（i18n_resource_text）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct I18nResourceTextV4 {
    /// 文案 Key（以 `@i18n@` 开头）
    pub key: String,
    /// 文案内容
    pub value: String,
}

/// 审批可见人（approval_create_viewers）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalCreateViewerV4 {
    /// 可见人类型（TENANT / DEPARTMENT / USER / NONE）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_type: Option<String>,
    /// viewer_type 为 USER 时的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_user_id: Option<String>,
    /// viewer_type 为 DEPARTMENT 时的部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_department_id: Option<String>,
}
