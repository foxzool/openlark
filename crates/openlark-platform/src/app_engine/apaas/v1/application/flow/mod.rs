//! 流程相关 API

pub mod execute;

use openlark_core::config::Config;

/// application.flow 服务（叶子级，持 namespace + flow_id）
#[derive(Debug, Clone)]
pub struct FlowService {
    config: Config,
    namespace: String,
    flow_id: String,
}

impl FlowService {
    /// 创建新的 flow 服务
    pub fn new(config: Config, namespace: impl Into<String>, flow_id: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            flow_id: flow_id.into(),
        }
    }
    /// 执行流程
    pub fn execute(&self) -> execute::FlowExecuteRequestBuilder {
        execute::FlowExecuteRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.flow_id.clone(),
        )
    }
}
