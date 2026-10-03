//! Snapshot tests for high-value communication helper outputs.
#![cfg(all(feature = "im", feature = "contact"))]

use insta::assert_json_snapshot;
use openlark_communication::{
    ChatLookupItem, MediaFileUpload, MediaImageUpload, MessageRecipient, PostMessage, ReplyTarget,
    UserLookupItem,
};
use serde_json::json;

#[test]
fn communication_helper_outputs_snapshot() {
    let recipient = MessageRecipient::open_id("ou_xxx");
    let post = PostMessage::zh_cn("项目播报", "今天完成发布");
    let reply_target = ReplyTarget::in_thread("om_xxx");
    let image_upload = MediaImageUpload::new(vec![1, 2, 3])
        .avatar()
        .file_name("avatar.png");
    let file_upload = MediaFileUpload::new("report.pdf", vec![9, 8, 7]).duration(15);
    let user = UserLookupItem {
        name: "zhangsan".to_string(),
        open_id: "ou_lookup".to_string(),
        user_id: Some("u_lookup".to_string()),
        department_ids: vec!["od_1".to_string(), "od_2".to_string()],
    };
    let chat = ChatLookupItem {
        chat_id: "oc_chat".to_string(),
        name: "项目群".to_string(),
        description: Some("核心项目协作群".to_string()),
        owner_id: Some("ou_owner".to_string()),
        owner_id_type: Some("open_id".to_string()),
        external: false,
        tenant_key: Some("tenant_key".to_string()),
        chat_status: Some("normal".to_string()),
    };

    assert_json_snapshot!(
        "communication_helper_outputs",
        json!({
            "message_recipient": {
                "receive_id": recipient.receive_id,
                "receive_id_type": recipient.receive_id_type.as_str(),
            },
            "post_message": {
                "locale": post.locale,
                "title": post.title,
                "text": post.text,
            },
            "reply_target": {
                "message_id": reply_target.message_id,
                "reply_in_thread": reply_target.reply_in_thread,
            },
            "media_image_upload": {
                "image_type": image_upload.image_type.as_str(),
                "file_name": image_upload.file_name,
                "bytes_len": image_upload.bytes.len(),
            },
            "media_file_upload": {
                "file_name": file_upload.file_name,
                "file_type": file_upload.file_type,
                "duration": file_upload.duration,
                "bytes_len": file_upload.bytes.len(),
            },
            "user_lookup_item": {
                "name": user.name,
                "open_id": user.open_id,
                "user_id": user.user_id,
                "department_ids": user.department_ids,
            },
            "chat_lookup_item": {
                "chat_id": chat.chat_id,
                "name": chat.name,
                "description": chat.description,
                "owner_id": chat.owner_id,
                "owner_id_type": chat.owner_id_type,
                "external": chat.external,
                "tenant_key": chat.tenant_key,
                "chat_status": chat.chat_status,
            },
        })
    );
}
