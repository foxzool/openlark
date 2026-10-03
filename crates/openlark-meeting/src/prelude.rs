//! OpenLark Meeting Crate 预导出模块
//!
//! 集中导出最常用的类型和特征，简化导入。

// 重新导出核心类型
pub use openlark_core::{SDKResult, config::Config};

// 重新导出通用工具
pub use crate::common::chain::MeetingClient;

// 重新导出端点常量
pub use crate::endpoints::*;

// 重新导出请求选项
pub use openlark_core::req_option::RequestOption;
