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
#[derive(Debug, Clone)]
pub struct WorkspaceService {
    config: Arc<PlatformConfig>,
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

    /// workspace.table 子资源
    pub fn table(&self, table_name: impl Into<String>) -> table::TableService {
        table::TableService::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            table_name,
        )
    }

    /// workspace.view 子资源
    pub fn view(&self, view_name: impl Into<String>) -> view::ViewService {
        view::ViewService::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            view_name,
        )
    }

    /// workspace.enum_mod 子资源
    pub fn enum_mod(&self) -> enum_mod::EnumModService {
        enum_mod::EnumModService::new(self.config.as_ref().clone(), self.workspace_id.clone())
    }

    /// 执行 SQL 命令
    pub fn sql_commands(&self, sql: impl Into<String>) -> sql_commands::SqlCommandsRequestBuilder {
        sql_commands::SqlCommandsRequestBuilder::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            sql,
        )
    }
}
