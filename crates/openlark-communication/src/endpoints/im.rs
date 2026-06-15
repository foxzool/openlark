//! IM (即时通讯) 端点定义
//!
//! 即时通讯系统 - 消息发送、聊天管理、批量消息处理
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::im::*;
//!
//! let messages_endpoint = IM_V1_MESSAGES;
//! let chats_endpoint = IM_V1_CHATS;
//! let batch_messages_endpoint = IM_V1_BATCH_MESSAGES;
//! ```

// ==================== IM (即时通讯) v1/v2 ====================
// 即时通讯系统 - 消息发送、聊天管理、批量消息处理

/// IM消息管理 v1
/// 基础消息发送和管理
pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";

/// IM聊天管理 v1
/// 聊天会话的创建和管理
pub const IM_V1_CHATS: &str = "/open-apis/im/v1/chats";

/// IM批量消息 v1
/// 批量消息发送和处理
pub const IM_V1_BATCH_MESSAGES: &str = "/open-apis/im/v1/batch_messages";

/// IM话题（thread）v1
/// 话题消息相关接口
pub const IM_V1_THREADS: &str = "/open-apis/im/v1/threads";

/// IM Pin 消息 v1
/// 群内 Pin 消息管理
pub const IM_V1_PINS: &str = "/open-apis/im/v1/pins";

/// IM图片资源 v1
/// 上传/下载图片
pub const IM_V1_IMAGES: &str = "/open-apis/im/v1/images";

/// IM文件资源 v1
/// 上传/下载文件
pub const IM_V1_FILES: &str = "/open-apis/im/v1/files";

// ==================== IM v2 ====================

/// IM消息管理 v2
/// 增强版消息功能
pub const IM_V2_MESSAGES: &str = "/open-apis/im/v2/messages";

/// IM聊天管理 v2
/// 增强版聊天功能
pub const IM_V2_CHATS: &str = "/open-apis/im/v2/chats";

/// IM URL 预览 v2
pub const IM_V2_URL_PREVIEWS_BATCH_UPDATE: &str = "/open-apis/im/v2/url_previews/batch_update";

/// IM 应用消息流卡片 v2
pub const IM_V2_APP_FEED_CARD: &str = "/open-apis/im/v2/app_feed_card";
/// 端点路径常量。
pub const IM_V2_APP_FEED_CARD_BATCH: &str = "/open-apis/im/v2/app_feed_card/batch";

/// IM 消息流卡片 v2
pub const IM_V2_FEED_CARDS: &str = "/open-apis/im/v2/feed_cards";

/// IM 消息流卡片按钮 v2
pub const IM_V2_CHAT_BUTTON: &str = "/open-apis/im/v2/chat_button";

/// IM 租户标签与群绑定关系 v2
pub const IM_V2_BIZ_ENTITY_TAG_RELATION: &str = "/open-apis/im/v2/biz_entity_tag_relation";

/// IM 租户标签 v2
pub const IM_V2_TAGS: &str = "/open-apis/im/v2/tags";

// ==================== Ephemeral (临时消息) ====================

/// IM 临时消息 - 删除仅特定人可见的消息卡片
pub const EPHEMERAL_V1_DELETE: &str = "/open-apis/ephemeral/v1/delete";

/// IM 临时消息 - 发送仅特定人可见的消息卡片
pub const EPHEMERAL_V1_SEND: &str = "/open-apis/ephemeral/v1/send";

// ==================== Interactive (交互式卡片) ====================

/// IM 交互式卡片 - 延时更新消息卡片
pub const INTERACTIVE_V1_CARD_UPDATE: &str = "/open-apis/interactive/v1/card/update";

// ==================== IM v2 群组搜索 ====================

/// IM v2 群组搜索
pub const IM_V2_CHATS_SEARCH: &str = "/open-apis/im/v2/chats/search";

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_im_v1_endpoints() {
        assert!(IM_V1_MESSAGES.starts_with("/open-apis/im/v1/"));
        assert!(IM_V1_CHATS.contains("chats"));
        assert!(IM_V1_BATCH_MESSAGES.contains("batch_messages"));
        assert!(IM_V1_IMAGES.contains("images"));
        assert!(IM_V1_FILES.contains("files"));
    }

    #[test]
    fn test_im_v2_endpoints() {
        assert!(IM_V2_MESSAGES.starts_with("/open-apis/im/v2/"));
        assert!(IM_V2_CHATS.starts_with("/open-apis/im/v2/"));
        assert!(IM_V2_APP_FEED_CARD.contains("app_feed_card"));
        assert!(IM_V2_FEED_CARDS.contains("feed_cards"));
        assert!(IM_V2_TAGS.contains("tags"));
    }
}
