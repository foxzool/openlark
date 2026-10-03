//! OpenLark Communication Crate 预导出模块
//!
//! 集中导出最常用的类型和特征，简化导入。

// 重新导出核心类型
pub use openlark_core::{SDKResult, config::Config};

// 重新导出通用工具
pub use crate::common::chain::CommunicationClient;
#[cfg(feature = "contact")]
pub use crate::common::chain::UserLookupItem;
#[cfg(feature = "im")]
pub use crate::common::chain::{
    ChatLookupItem, MediaFileUpload, MediaImageUpload, MessageRecipient, PostMessage, ReplyTarget,
};

// 重新导出端点常量
pub use crate::endpoints::{aily::*, contact::*, event::*, im::*, mail::*, moments::*, vc::*};

// 重新导出请求选项
pub use openlark_core::req_option::RequestOption;

// 重新导出常用的 API 类型（按 feature gate）
#[cfg(feature = "im")]
pub use crate::im::v1::message::models::ReceiveIdType;

#[cfg(feature = "contact")]
pub use crate::contact::contact::v3::user::models::UserIdType;

#[cfg(feature = "contact")]
pub use crate::contact::contact::v3::user::models::DepartmentIdType;
