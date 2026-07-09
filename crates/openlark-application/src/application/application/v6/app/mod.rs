pub mod get;
/// 应用详情相关数据模型。
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// 应用资源服务。
#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
}

impl App {
    /// 创建新的应用资源服务。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 返回获取应用请求构建器。
    pub fn get(&self) -> get::GetAppRequest {
        get::GetAppRequest::new(self.config.clone())
    }
}

pub use get::GetAppRequest;
// models 模块显式导出
pub use models::GetAppResponse;
pub mod create;
pub mod delete;
pub mod list;
pub mod patch;
