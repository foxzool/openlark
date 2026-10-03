//! 自定义枚举相关 API

pub mod enum_get;
pub mod list;

use openlark_core::config::Config;

/// workspace.enum_mod 资源服务（叶子级，持 owned Config + workspace_id）
#[derive(Debug, Clone)]
pub struct EnumModService {
    config: Config,
    workspace_id: String,
}

impl EnumModService {
    /// 创建新的 enum_mod 服务
    pub fn new(config: Config, workspace_id: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
        }
    }
    /// 枚举列表
    pub fn list(&self) -> list::EnumListRequestBuilder {
        list::EnumListRequestBuilder::new(self.config.clone(), self.workspace_id.clone())
    }
    /// 获取枚举详情
    pub fn get(&self, enum_name: impl Into<String>) -> enum_get::EnumGetRequestBuilder {
        enum_get::EnumGetRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            enum_name,
        )
    }
}
