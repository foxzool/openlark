//! 工作空间相关 API
//!
//! 包含数据表、视图、枚举、SQL 执行等功能

pub mod enum_mod;
pub mod sql_commands;
pub mod table;
pub mod view;

pub use enum_mod::enum_get;
pub use enum_mod::list as enum_list;
pub use sql_commands::SqlCommandsRequestBuilder;
pub use table::list as table_list;
pub use table::table_get;
pub use view::views_get;

use crate::PlatformConfig;
use std::sync::Arc;

/// workspace 资源服务（中间级，绑定 workspace_id）
///
/// 深嵌套子级访问器（table/view/enum_mod/sql_commands）见 Task 5。
#[derive(Debug, Clone)]
pub struct WorkspaceService {
    // Task 5 将消费（深嵌套 table/view/enum_mod/sql_commands 访问器）
    #[allow(dead_code)]
    config: Arc<PlatformConfig>,
    #[allow(dead_code)]
    workspace_id: String,
}

impl WorkspaceService {
    /// 创建新的 workspace 服务
    pub fn new(config: Arc<PlatformConfig>, workspace_id: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
        }
    }
}
