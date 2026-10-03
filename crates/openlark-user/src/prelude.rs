//! 用户设置预导入模块
//!
//! 导入用户设置常用的类型和特征。

// 重新导出核心类型
/// 用户服务配置别名。
pub use crate::UserConfig;
/// 用户服务统一入口。
pub use crate::UserService;

// 重新导出 openlark-core 核心类型
/// 核心配置、错误与结果类型。
pub use openlark_core::{CoreError, SDKResult, config::Config};

// 重新导出常用特征
/// OpenLark API 请求特征。
pub use openlark_core::api::ApiRequest;
