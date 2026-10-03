//! 视图相关 API

pub mod views_get;

use openlark_core::config::Config;

/// workspace.view 资源服务（叶子级，持 owned Config + workspace_id + view_name）
#[derive(Debug, Clone)]
pub struct ViewService {
    config: Config,
    workspace_id: String,
    view_name: String,
}

impl ViewService {
    /// 创建新的 view 服务
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        view_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            view_name: view_name.into(),
        }
    }
    /// 获取视图详情
    pub fn views_get(&self) -> views_get::ViewsGetRequestBuilder {
        views_get::ViewsGetRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.view_name.clone(),
        )
    }
}
