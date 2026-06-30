//! 环境变量相关 API

pub mod get;
pub mod query;

use openlark_core::config::Config;

/// application.environment_variable 服务（叶子级，持 namespace）
#[derive(Debug, Clone)]
pub struct EnvironmentVariableService {
    config: Config,
    namespace: String,
}

impl EnvironmentVariableService {
    /// 创建新的 environment_variable 服务
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
        }
    }
    /// 查询环境变量列表
    pub fn query(&self) -> query::EnvironmentVariableQueryRequestBuilder {
        query::EnvironmentVariableQueryRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
        )
    }
    /// 获取环境变量详情
    pub fn get(
        &self,
        env_var_api_name: impl Into<String>,
    ) -> get::EnvironmentVariableGetRequestBuilder {
        get::EnvironmentVariableGetRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            env_var_api_name,
        )
    }
}
