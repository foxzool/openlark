//! 邮件 API 端点定义（类型安全枚举系统）

/// 邮件 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MailApiV1 {
    // MailGroup APIs
    /// Mail Group Create 端点。
    MailGroupCreate,
    /// Mail Group Get 端点。
    MailGroupGet(String),
    /// Mail Group Update 端点。
    MailGroupUpdate(String),
    /// Mail Group Delete 端点。
    MailGroupDelete(String),
    /// Mail Group List 端点。
    MailGroupList,
    /// Mail Group Patch 端点。
    MailGroupPatch(String),
    // MailGroup Alias APIs
    /// Mail Group Alias Create 端点。
    MailGroupAliasCreate(String),
    /// Mail Group Alias Delete 端点。
    MailGroupAliasDelete(String, String),
    /// Mail Group Alias List 端点。
    MailGroupAliasList(String),
    // MailGroup Manager APIs
    /// Mail Group Manager Batch Create 端点。
    MailGroupManagerBatchCreate(String),
    /// Mail Group Manager Batch Delete 端点。
    MailGroupManagerBatchDelete(String),
    /// Mail Group Manager List 端点。
    MailGroupManagerList(String),
    // MailGroup Member APIs
    /// Mail Group Member Create 端点。
    MailGroupMemberCreate(String),
    /// Mail Group Member Delete 端点。
    MailGroupMemberDelete(String, String),
    /// Mail Group Member Get 端点。
    MailGroupMemberGet(String, String),
    /// Mail Group Member List 端点。
    MailGroupMemberList(String),
    /// Mail Group Member Batch Create 端点。
    MailGroupMemberBatchCreate(String),
    /// Mail Group Member Batch Delete 端点。
    MailGroupMemberBatchDelete(String),
    // MailGroup Permission Member APIs
    /// Mail Group Permission Member Create 端点。
    MailGroupPermissionMemberCreate(String),
    /// Mail Group Permission Member Delete 端点。
    MailGroupPermissionMemberDelete(String, String),
    /// Mail Group Permission Member Get 端点。
    MailGroupPermissionMemberGet(String, String),
    /// Mail Group Permission Member List 端点。
    MailGroupPermissionMemberList(String),
    /// Mail Group Permission Member Batch Create 端点。
    MailGroupPermissionMemberBatchCreate(String),
    /// Mail Group Permission Member Batch Delete 端点。
    MailGroupPermissionMemberBatchDelete(String),
    // Public Mailbox APIs
    /// Public Mailbox Create 端点。
    PublicMailboxCreate,
    /// Public Mailbox Get 端点。
    PublicMailboxGet(String),
    /// Public Mailbox Update 端点。
    PublicMailboxUpdate(String),
    /// Public Mailbox Delete 端点。
    PublicMailboxDelete(String),
    /// Public Mailbox List 端点。
    PublicMailboxList,
    /// Public Mailbox Patch 端点。
    PublicMailboxPatch(String),
    /// Public Mailbox Remove To Recycle Bin 端点。
    PublicMailboxRemoveToRecycleBin(String),
    // Public Mailbox Alias APIs
    /// Public Mailbox Alias Create 端点。
    PublicMailboxAliasCreate(String),
    /// Public Mailbox Alias Delete 端点。
    PublicMailboxAliasDelete(String, String),
    /// Public Mailbox Alias List 端点。
    PublicMailboxAliasList(String),
    // Public Mailbox Member APIs
    /// Public Mailbox Member Create 端点。
    PublicMailboxMemberCreate(String),
    /// Public Mailbox Member Delete 端点。
    PublicMailboxMemberDelete(String, String),
    /// Public Mailbox Member Get 端点。
    PublicMailboxMemberGet(String, String),
    /// Public Mailbox Member List 端点。
    PublicMailboxMemberList(String),
    /// Public Mailbox Member Batch Create 端点。
    PublicMailboxMemberBatchCreate(String),
    /// Public Mailbox Member Batch Delete 端点。
    PublicMailboxMemberBatchDelete(String),
    /// Public Mailbox Member Clear 端点。
    PublicMailboxMemberClear(String),
    // User APIs
    /// User Query 端点。
    UserQuery,
    // User Mailbox APIs
    /// User Mailbox Delete 端点。
    UserMailboxDelete(String),
    // User Mailbox Alias APIs
    /// User Mailbox Alias Create 端点。
    UserMailboxAliasCreate(String),
    /// User Mailbox Alias Delete 端点。
    UserMailboxAliasDelete(String, String),
    /// User Mailbox Alias List 端点。
    UserMailboxAliasList(String),
    // User Mailbox Event APIs
    /// User Mailbox Event Subscribe 端点。
    UserMailboxEventSubscribe(String),
    /// User Mailbox Event Unsubscribe 端点。
    UserMailboxEventUnsubscribe(String),
    /// User Mailbox Event Subscription 端点。
    UserMailboxEventSubscription(String),
    // User Mailbox Folder APIs
    /// User Mailbox Folder Create 端点。
    UserMailboxFolderCreate(String),
    /// User Mailbox Folder Delete 端点。
    UserMailboxFolderDelete(String),
    /// User Mailbox Folder List 端点。
    UserMailboxFolderList(String),
    /// User Mailbox Folder Patch 端点。
    UserMailboxFolderPatch(String),
    // User Mailbox Mail Contact APIs
    /// User Mailbox Mail Contact Create 端点。
    UserMailboxMailContactCreate(String),
    /// User Mailbox Mail Contact Delete 端点。
    UserMailboxMailContactDelete(String),
    /// User Mailbox Mail Contact List 端点。
    UserMailboxMailContactList(String),
    /// User Mailbox Mail Contact Patch 端点。
    UserMailboxMailContactPatch(String),
    // User Mailbox Message APIs
    /// User Mailbox Message Get 端点。
    UserMailboxMessageGet(String),
    /// User Mailbox Message Get By Card 端点。
    UserMailboxMessageGetByCard(String),
    /// User Mailbox Message List 端点。
    UserMailboxMessageList(String),
    /// User Mailbox Message Send 端点。
    UserMailboxMessageSend(String),
    /// User Mailbox Message Attachment Download Url 端点。
    UserMailboxMessageAttachmentDownloadUrl(String),
    // User Mailbox Rule APIs
    /// User Mailbox Rule Create 端点。
    UserMailboxRuleCreate(String),
    /// User Mailbox Rule Delete 端点。
    UserMailboxRuleDelete(String),
    /// User Mailbox Rule List 端点。
    UserMailboxRuleList(String),
    /// User Mailbox Rule Reorder 端点。
    UserMailboxRuleReorder(String),
    /// User Mailbox Rule Update 端点。
    UserMailboxRuleUpdate(String),
}

impl MailApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // MailGroup APIs
            MailApiV1::MailGroupCreate => "/open-apis/mail/v1/mailgroups".to_string(),
            MailApiV1::MailGroupGet(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}")
            }
            MailApiV1::MailGroupUpdate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}")
            }
            MailApiV1::MailGroupDelete(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}")
            }
            MailApiV1::MailGroupList => "/open-apis/mail/v1/mailgroups".to_string(),
            MailApiV1::MailGroupPatch(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}")
            }
            // MailGroup Alias APIs
            MailApiV1::MailGroupAliasCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/aliases")
            }
            MailApiV1::MailGroupAliasDelete(group_id, alias_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/aliases/{alias_id}")
            }
            MailApiV1::MailGroupAliasList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/aliases")
            }
            // MailGroup Manager APIs
            MailApiV1::MailGroupManagerBatchCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/managers/batch_create")
            }
            MailApiV1::MailGroupManagerBatchDelete(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/managers/batch_delete")
            }
            MailApiV1::MailGroupManagerList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/managers")
            }
            // MailGroup Member APIs
            MailApiV1::MailGroupMemberCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members")
            }
            MailApiV1::MailGroupMemberDelete(group_id, member_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members/{member_id}")
            }
            MailApiV1::MailGroupMemberGet(group_id, member_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members/{member_id}")
            }
            MailApiV1::MailGroupMemberList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members")
            }
            MailApiV1::MailGroupMemberBatchCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members/batch_create")
            }
            MailApiV1::MailGroupMemberBatchDelete(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/members/batch_delete")
            }
            // MailGroup Permission Member APIs
            MailApiV1::MailGroupPermissionMemberCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members")
            }
            MailApiV1::MailGroupPermissionMemberDelete(group_id, perm_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members/{perm_id}")
            }
            MailApiV1::MailGroupPermissionMemberGet(group_id, perm_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members/{perm_id}")
            }
            MailApiV1::MailGroupPermissionMemberList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members")
            }
            MailApiV1::MailGroupPermissionMemberBatchCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members/batch_create")
            }
            MailApiV1::MailGroupPermissionMemberBatchDelete(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{group_id}/permission_members/batch_delete")
            }
            // Public Mailbox APIs
            MailApiV1::PublicMailboxCreate => "/open-apis/mail/v1/public_mailboxes".to_string(),
            MailApiV1::PublicMailboxGet(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}")
            }
            MailApiV1::PublicMailboxUpdate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}")
            }
            MailApiV1::PublicMailboxDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}")
            }
            MailApiV1::PublicMailboxList => "/open-apis/mail/v1/public_mailboxes".to_string(),
            MailApiV1::PublicMailboxPatch(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}")
            }
            MailApiV1::PublicMailboxRemoveToRecycleBin(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/remove_to_recycle_bin")
            }
            // Public Mailbox Alias APIs
            MailApiV1::PublicMailboxAliasCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/aliases")
            }
            MailApiV1::PublicMailboxAliasDelete(mailbox_id, alias_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/aliases/{alias_id}")
            }
            MailApiV1::PublicMailboxAliasList(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/aliases")
            }
            // Public Mailbox Member APIs
            MailApiV1::PublicMailboxMemberCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members")
            }
            MailApiV1::PublicMailboxMemberDelete(mailbox_id, member_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members/{member_id}")
            }
            MailApiV1::PublicMailboxMemberGet(mailbox_id, member_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members/{member_id}")
            }
            MailApiV1::PublicMailboxMemberList(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members")
            }
            MailApiV1::PublicMailboxMemberBatchCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members/batch_create")
            }
            MailApiV1::PublicMailboxMemberBatchDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members/batch_delete")
            }
            MailApiV1::PublicMailboxMemberClear(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{mailbox_id}/members/clear")
            }
            // User APIs
            MailApiV1::UserQuery => "/open-apis/mail/v1/user".to_string(),
            // User Mailbox APIs
            MailApiV1::UserMailboxDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}")
            }
            // User Mailbox Alias APIs
            MailApiV1::UserMailboxAliasCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/aliases")
            }
            MailApiV1::UserMailboxAliasDelete(mailbox_id, alias_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/aliases/{alias_id}")
            }
            MailApiV1::UserMailboxAliasList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/aliases")
            }
            // User Mailbox Event APIs
            MailApiV1::UserMailboxEventSubscribe(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/event/subscribe")
            }
            MailApiV1::UserMailboxEventUnsubscribe(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/event/unsubscribe")
            }
            MailApiV1::UserMailboxEventSubscription(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/event/subscription")
            }
            // User Mailbox Folder APIs
            MailApiV1::UserMailboxFolderCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/folder")
            }
            MailApiV1::UserMailboxFolderDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/folder")
            }
            MailApiV1::UserMailboxFolderList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/folder")
            }
            MailApiV1::UserMailboxFolderPatch(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/folder")
            }
            // User Mailbox Mail Contact APIs
            MailApiV1::UserMailboxMailContactCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/mail_contact")
            }
            MailApiV1::UserMailboxMailContactDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/mail_contact")
            }
            MailApiV1::UserMailboxMailContactList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/mail_contact")
            }
            MailApiV1::UserMailboxMailContactPatch(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/mail_contact")
            }
            // User Mailbox Message APIs
            MailApiV1::UserMailboxMessageGet(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/message/get")
            }
            MailApiV1::UserMailboxMessageGetByCard(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/message/get_by_card")
            }
            MailApiV1::UserMailboxMessageList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/message/list")
            }
            MailApiV1::UserMailboxMessageSend(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/message/send")
            }
            MailApiV1::UserMailboxMessageAttachmentDownloadUrl(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{mailbox_id}/message/attachment/download_url"
                )
            }
            // User Mailbox Rule APIs
            MailApiV1::UserMailboxRuleCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/rule")
            }
            MailApiV1::UserMailboxRuleDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/rule")
            }
            MailApiV1::UserMailboxRuleList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/rule")
            }
            MailApiV1::UserMailboxRuleReorder(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/rule/reorder")
            }
            MailApiV1::UserMailboxRuleUpdate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{mailbox_id}/rule")
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_mailapiv1_to_url_coverage() {
        let urls = vec![
            MailApiV1::MailGroupCreate.to_url(),
            MailApiV1::MailGroupGet("id1".to_string()).to_url(),
            MailApiV1::MailGroupUpdate("id1".to_string()).to_url(),
            MailApiV1::MailGroupDelete("id1".to_string()).to_url(),
            MailApiV1::MailGroupList.to_url(),
            MailApiV1::MailGroupPatch("id1".to_string()).to_url(),
            MailApiV1::MailGroupAliasCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupAliasDelete("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::MailGroupAliasList("id1".to_string()).to_url(),
            MailApiV1::MailGroupManagerBatchCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupManagerBatchDelete("id1".to_string()).to_url(),
            MailApiV1::MailGroupManagerList("id1".to_string()).to_url(),
            MailApiV1::MailGroupMemberCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupMemberDelete("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::MailGroupMemberGet("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::MailGroupMemberList("id1".to_string()).to_url(),
            MailApiV1::MailGroupMemberBatchCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupMemberBatchDelete("id1".to_string()).to_url(),
            MailApiV1::MailGroupPermissionMemberCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupPermissionMemberDelete("id1".to_string(), "id2".to_string())
                .to_url(),
            MailApiV1::MailGroupPermissionMemberGet("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::MailGroupPermissionMemberList("id1".to_string()).to_url(),
            MailApiV1::MailGroupPermissionMemberBatchCreate("id1".to_string()).to_url(),
            MailApiV1::MailGroupPermissionMemberBatchDelete("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxCreate.to_url(),
            MailApiV1::PublicMailboxGet("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxUpdate("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxDelete("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxList.to_url(),
            MailApiV1::PublicMailboxPatch("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxRemoveToRecycleBin("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxAliasCreate("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxAliasDelete("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::PublicMailboxAliasList("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberCreate("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberDelete("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberGet("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberList("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberBatchCreate("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberBatchDelete("id1".to_string()).to_url(),
            MailApiV1::PublicMailboxMemberClear("id1".to_string()).to_url(),
            MailApiV1::UserQuery.to_url(),
            MailApiV1::UserMailboxDelete("id1".to_string()).to_url(),
            MailApiV1::UserMailboxAliasCreate("id1".to_string()).to_url(),
            MailApiV1::UserMailboxAliasDelete("id1".to_string(), "id2".to_string()).to_url(),
            MailApiV1::UserMailboxAliasList("id1".to_string()).to_url(),
            MailApiV1::UserMailboxEventSubscribe("id1".to_string()).to_url(),
            MailApiV1::UserMailboxEventUnsubscribe("id1".to_string()).to_url(),
            MailApiV1::UserMailboxEventSubscription("id1".to_string()).to_url(),
            MailApiV1::UserMailboxFolderCreate("id1".to_string()).to_url(),
            MailApiV1::UserMailboxFolderDelete("id1".to_string()).to_url(),
            MailApiV1::UserMailboxFolderList("id1".to_string()).to_url(),
            MailApiV1::UserMailboxFolderPatch("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMailContactCreate("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMailContactDelete("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMailContactList("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMailContactPatch("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMessageGet("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMessageGetByCard("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMessageList("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMessageSend("id1".to_string()).to_url(),
            MailApiV1::UserMailboxMessageAttachmentDownloadUrl("id1".to_string()).to_url(),
            MailApiV1::UserMailboxRuleCreate("id1".to_string()).to_url(),
            MailApiV1::UserMailboxRuleDelete("id1".to_string()).to_url(),
            MailApiV1::UserMailboxRuleList("id1".to_string()).to_url(),
            MailApiV1::UserMailboxRuleReorder("id1".to_string()).to_url(),
            MailApiV1::UserMailboxRuleUpdate("id1".to_string()).to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }
}
