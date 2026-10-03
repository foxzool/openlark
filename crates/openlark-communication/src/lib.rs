//! Open-Lark Communication Module
//!
//! 飞书通讯协作服务模块，提供完整的沟通和协作功能。
//! Communication / Workflow helper 分层边界见 `docs/communication-workflow-helper-boundaries.md`。
//!
//! ## 主要功能
//!
//! - **即时通讯**: 消息发送、聊天管理、批量消息处理
//! - **邮件服务**: 邮件组管理、邮箱事件订阅、文件夹操作
//! - **视频会议**: 会议室管理、会议创建、参会控制
//! - **动态分享**: 朋友圈动态、内容分享、社交互动
//! - **事件系统**: 事件订阅、处理、分发机制
//! - **AILY (AI学习平台)**: 数据知识管理、AI 会话和技能调用
//! - **通讯录**: 用户、部门、用户组、角色等管理
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use openlark_communication::prelude::*;
//!
//! // 使用端点常量
//! let messages_endpoint = IM_V1_MESSAGES;
//!
//! // 使用 helper 发送常见消息
//! let communication = CommunicationClient::new(config);
//! let response = communication
//!     .im
//!     .send_text(MessageRecipient::open_id("ou_xxx"), "Hello")
//!     .await?;
//! let _user = communication.contact.find_user_by_name("zhangsan").await?;
//! let _chat = communication.im.find_chat_by_name("项目群").await?;
//! let _reply = communication
//!     .im
//!     .reply_text(ReplyTarget::direct("om_xxx"), "收到")
//!     .await?;
//! let _uploaded = communication
//!     .im
//!     .upload_image(MediaImageUpload::new(vec![1, 2, 3]))
//!     .await?;
//! ```
//!
//! ## 子模块
//!
//! 端点模块按服务分类组织，便于维护和查找：
//! - `endpoints/aily.rs` - AILY 相关端点
//! - `endpoints/im.rs` - IM 相关端点
//! - `endpoints/mail.rs` - Mail 相关端点
//! - `endpoints/vc.rs` - VC 相关端点
//! - `endpoints/event.rs` - Event 相关端点
//! - `endpoints/moments.rs` - Moments 相关端点
//! - `endpoints/contact.rs` - Contact 相关端点
//!
//! ## 功能模块
//!
//! 这些模块包含完整的 API 端点常量定义和测试用例。

// Include macros first
#[macro_use]
mod macros;

// 通用工具与类型
pub mod common;

// API 端点常量定义
pub mod endpoints;

// Event（事件系统）模块 - 事件订阅、出口 IP、长连接在线数量等
#[cfg(feature = "event")]
pub mod event;

// AILY 模块
#[cfg(feature = "aily")]
pub mod aily;

// IM 模块
#[cfg(feature = "im")]
pub mod im;

// Contact 模块
#[cfg(feature = "contact")]
pub mod contact;

// Moments 模块
#[cfg(feature = "moments")]
pub mod moments;

// Prelude 模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use common::chain::CommunicationClient;
#[cfg(feature = "contact")]
pub use common::chain::UserLookupItem;
#[cfg(feature = "im")]
pub use common::chain::{
    ChatLookupItem, MediaFileUpload, MediaImageUpload, MessageRecipient, PostMessage, ReplyTarget,
};
