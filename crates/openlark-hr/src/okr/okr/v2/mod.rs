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
use std::sync::Arc;

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

    /// 创建获取对齐请求。
    pub fn get(&self, alignment_id: impl Into<String>) -> alignment::get::Request {
        alignment::get::Request::new(Arc::new(self.config.clone())).alignment_id(alignment_id)
    }

    /// 创建删除对齐请求。
    pub fn delete(&self, alignment_id: impl Into<String>) -> alignment::delete::Request {
        alignment::delete::Request::new(Arc::new(self.config.clone())).alignment_id(alignment_id)
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

    /// 创建获取所有 OKR 分类请求。
    pub fn list(&self) -> category::list::Request {
        category::list::Request::new(Arc::new(self.config.clone()))
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

    /// 创建获取用户 OKR 周期列表请求。
    pub fn list(&self, user_id: impl Into<String>) -> cycle::list::Request {
        cycle::list::Request::new(Arc::new(self.config.clone())).user_id(user_id)
    }

    /// 创建修改 OKR 目标位置请求。
    pub fn objectives_position(
        &self,
        cycle_id: impl Into<String>,
    ) -> cycle::objectives_position::Request {
        cycle::objectives_position::Request::new(Arc::new(self.config.clone())).cycle_id(cycle_id)
    }

    /// 创建修改 OKR 目标权重请求。
    pub fn objectives_weight(
        &self,
        cycle_id: impl Into<String>,
    ) -> cycle::objectives_weight::Request {
        cycle::objectives_weight::Request::new(Arc::new(self.config.clone())).cycle_id(cycle_id)
    }

    /// 创建 OKR 目标请求。
    pub fn create_objective(
        &self,
        cycle_id: impl Into<String>,
    ) -> cycle::objective::create::Request {
        cycle::objective::create::Request::new(Arc::new(self.config.clone())).cycle_id(cycle_id)
    }

    /// 创建获取用户 OKR 周期内的目标请求。
    pub fn list_objectives(&self, cycle_id: impl Into<String>) -> cycle::objective::list::Request {
        cycle::objective::list::Request::new(Arc::new(self.config.clone())).cycle_id(cycle_id)
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

    /// 创建更新量化指标请求。
    pub fn patch(&self, indicator_id: impl Into<String>) -> indicator::patch::Request {
        indicator::patch::Request::new(Arc::new(self.config.clone())).indicator_id(indicator_id)
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

    /// 创建获取关键结果请求。
    pub fn get(&self, key_result_id: impl Into<String>) -> key_result::get::Request {
        key_result::get::Request::new(Arc::new(self.config.clone())).key_result_id(key_result_id)
    }

    /// 创建删除关键结果请求。
    pub fn delete(&self, key_result_id: impl Into<String>) -> key_result::delete::Request {
        key_result::delete::Request::new(Arc::new(self.config.clone())).key_result_id(key_result_id)
    }

    /// 创建编辑关键结果请求。
    pub fn patch(&self, key_result_id: impl Into<String>) -> key_result::patch::Request {
        key_result::patch::Request::new(Arc::new(self.config.clone())).key_result_id(key_result_id)
    }

    /// 创建获取关键结果的量化指标请求。
    pub fn list_indicators(
        &self,
        key_result_id: impl Into<String>,
    ) -> key_result::indicator::list::Request {
        key_result::indicator::list::Request::new(Arc::new(self.config.clone()))
            .key_result_id(key_result_id)
    }

    /// 创建获取关键结果下的进展记录请求。
    pub fn list_progresses(
        &self,
        key_result_id: impl Into<String>,
    ) -> key_result::progress::list::Request {
        key_result::progress::list::Request::new(Arc::new(self.config.clone()))
            .key_result_id(key_result_id)
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

    /// 创建获取目标详细信息请求。
    pub fn get(&self, objective_id: impl Into<String>) -> objective::get::Request {
        objective::get::Request::new(Arc::new(self.config.clone())).objective_id(objective_id)
    }

    /// 创建删除 OKR 目标请求。
    pub fn delete(&self, objective_id: impl Into<String>) -> objective::delete::Request {
        objective::delete::Request::new(Arc::new(self.config.clone())).objective_id(objective_id)
    }

    /// 创建编辑 OKR 目标请求。
    pub fn patch(&self, objective_id: impl Into<String>) -> objective::patch::Request {
        objective::patch::Request::new(Arc::new(self.config.clone())).objective_id(objective_id)
    }

    /// 创建修改关键结果位置请求。
    pub fn key_results_position(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::key_results_position::Request {
        objective::key_results_position::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建修改关键结果权重请求。
    pub fn key_results_weight(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::key_results_weight::Request {
        objective::key_results_weight::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建创建目标对齐关系请求。
    pub fn create_alignment(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::alignment::create::Request {
        objective::alignment::create::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建获取目标的对齐信息请求。
    pub fn list_alignments(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::alignment::list::Request {
        objective::alignment::list::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建获取目标的量化指标请求。
    pub fn list_indicators(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::indicator::list::Request {
        objective::indicator::list::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建在目标下创建关键结果请求。
    pub fn create_key_result(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::key_result::create::Request {
        objective::key_result::create::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建获取目标下的所有关键结果请求。
    pub fn list_key_results(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::key_result::list::Request {
        objective::key_result::list::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }

    /// 创建获取目标下的进展记录请求。
    pub fn list_progresses(
        &self,
        objective_id: impl Into<String>,
    ) -> objective::progress::list::Request {
        objective::progress::list::Request::new(Arc::new(self.config.clone()))
            .objective_id(objective_id)
    }
}
