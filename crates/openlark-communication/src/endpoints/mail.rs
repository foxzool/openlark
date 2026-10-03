//! Mail (邮件服务) 端点定义
//!
//! 邮件系统 - 邮件组管理、邮箱事件、文件夹和消息
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::mail::*;
//!
//! let mailgroups_endpoint = MAIL_V1_MAILGROUPS;
//! let mailboxes_endpoint = MAIL_V1_USER_MAILBOXES;
//! ```

// ==================== Mail (邮件服务) v1 ====================
// 邮件系统 - 邮件组管理、邮箱事件、文件夹和消息

/// Mail邮件组管理
/// 企业邮件组的创建和管理
pub const MAIL_V1_MAILGROUPS: &str = "/open-apis/mail/v1/mailgroups";
/// 端点路径常量。
pub const MAIL_V1_MAILGROUP: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";

/// Mail邮件组管理员
/// 邮件组管理员权限管理
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";
/// 端点路径常量。
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";
/// 端点路径常量。
pub const MAIL_V1_MAILGROUP_MANAGERS: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";

/// Mail用户邮箱
/// 用户邮箱的基础管理
pub const MAIL_V1_USER_MAILBOXES: &str = "/open-apis/mail/v1/user_mailboxes";

/// Mail用户邮箱事件
/// 邮箱事件的订阅和管理
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";

/// Mail用户邮箱文件夹
/// 邮件文件夹组织管理
pub const MAIL_V1_USER_MAILBOX_FOLDERS: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_FOLDER: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";

/// Mail用户邮箱消息
/// 邮件消息的处理和管理
pub const MAIL_V1_USER_MAILBOX_MESSAGES: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_MESSAGE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";

/// Mail用户邮箱规则
/// 邮件处理规则和过滤设置
pub const MAIL_V1_USER_MAILBOX_RULES: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_RULE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";

/// Mail用户邮箱联系人
/// 邮件联系人管理
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";
/// 端点路径常量。
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";

/// Mail用户邮箱附件
/// 邮件附件下载和管理
pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_mail_endpoints() {
        assert!(MAIL_V1_MAILGROUPS.starts_with("/open-apis/mail/v1/"));
        assert!(MAIL_V1_MAILGROUPS.contains("mailgroups"));
        assert!(MAIL_V1_USER_MAILBOXES.contains("user_mailboxes"));
        assert!(MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE.contains("events"));
        assert!(MAIL_V1_USER_MAILBOX_FOLDERS.contains("folders"));
        assert!(MAIL_V1_USER_MAILBOX_MESSAGES.contains("messages"));
        assert!(MAIL_V1_USER_MAILBOX_RULES.contains("rules"));
    }
}
