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

    /// 访问 alignment 资源。
    pub fn alignment(&self) -> AlignmentResource {
        AlignmentResource::new(self.config.clone())
    }

    /// 访问 category 资源。
    pub fn category(&self) -> CategoryResource {
        CategoryResource::new(self.config.clone())
    }

    /// 访问 cycle 资源。
    pub fn cycle(&self) -> CycleResource {
        CycleResource::new(self.config.clone())
    }

    /// 访问 indicator 资源。
    pub fn indicator(&self) -> IndicatorResource {
        IndicatorResource::new(self.config.clone())
    }

    /// 访问 key_result 资源。
    pub fn key_result(&self) -> KeyResultResource {
        KeyResultResource::new(self.config.clone())
    }

    /// 访问 objective 资源。
    pub fn objective(&self) -> ObjectiveResource {
        ObjectiveResource::new(self.config.clone())
    }
}

/// Alignment 资源
#[derive(Debug, Clone)]
pub struct AlignmentResource {
    config: Config,
}

impl AlignmentResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Category 资源
#[derive(Debug, Clone)]
pub struct CategoryResource {
    config: Config,
}

impl CategoryResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Cycle 资源
#[derive(Debug, Clone)]
pub struct CycleResource {
    config: Config,
}

impl CycleResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Indicator 资源
#[derive(Debug, Clone)]
pub struct IndicatorResource {
    config: Config,
}

impl IndicatorResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// KeyResult 资源
#[derive(Debug, Clone)]
pub struct KeyResultResource {
    config: Config,
}

impl KeyResultResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Objective 资源
#[derive(Debug, Clone)]
pub struct ObjectiveResource {
    config: Config,
}

impl ObjectiveResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
