//! OKR API v2 模块
//!
//! OKR v2 版本的 API 实现，提供目标、关键结果、对齐、分类、周期等管理接口。

pub mod alignment;
pub mod category;
pub mod cycle;
pub mod indicator;
pub mod key_result;
pub mod objective;

use openlark_core::config::Config;

/// OKR v2 服务入口。
#[derive(Debug, Clone)]
pub struct OkrV2 {
    config: Config,
}

impl OkrV2 {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
