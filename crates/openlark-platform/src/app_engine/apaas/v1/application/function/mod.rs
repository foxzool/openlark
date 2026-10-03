//! 函数相关 API

pub mod invoke;

use openlark_core::config::Config;

/// application.function 服务（叶子级，持 namespace + function_api_name）
#[derive(Debug, Clone)]
pub struct FunctionService {
    config: Config,
    namespace: String,
    function_api_name: String,
}

impl FunctionService {
    /// 创建新的 function 服务
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        function_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            function_api_name: function_api_name.into(),
        }
    }
    /// 调用函数
    pub fn invoke(&self) -> invoke::FunctionInvokeRequestBuilder {
        invoke::FunctionInvokeRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.function_api_name.clone(),
        )
    }
}
