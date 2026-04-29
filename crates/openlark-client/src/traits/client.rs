//! OpenLark Client 核心特征
//!
//! 定义客户端的统一接口和行为

use crate::Config;
use std::time::Duration;

/// 🚀 OpenLark客户端核心特征
///
/// 所有OpenLark客户端实现都应该实现此特征
///
/// # 特性要求
/// - 异步支持：所有操作都是异步的
/// - 线程安全：客户端可以跨线程安全使用
/// - 配置访问：可以访问客户端配置
/// - 错误处理：统一的错误处理机制
pub trait LarkClient: Send + Sync {
    /// 🔧 获取客户端配置
    fn config(&self) -> &Config;

    /// ✅ 检查客户端是否已正确配置
    ///
    /// # 返回值
    /// 返回true如果app_id和app_secret都不为空
    fn is_configured(&self) -> bool {
        !self.config().app_id.is_empty() && !self.config().app_secret.is_empty()
    }

    /// 🔍 获取应用ID
    fn app_id(&self) -> &str {
        &self.config().app_id
    }

    /// 🔑 获取应用密钥
    fn app_secret(&self) -> &str {
        &self.config().app_secret
    }

    /// 🌐 获取API基础URL
    fn base_url(&self) -> &str {
        &self.config().base_url
    }

    /// ⏱️ 获取请求超时时间
    fn timeout(&self) -> Duration {
        self.config().timeout
    }

    /// 🔄 获取重试次数
    fn retry_count(&self) -> u32 {
        self.config().retry_count
    }

    /// 📝 检查是否启用了日志
    fn is_log_enabled(&self) -> bool {
        self.config().enable_log
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct TestClient {
        config: Config,
    }

    impl LarkClient for TestClient {
        fn config(&self) -> &Config {
            &self.config
        }
    }

    #[test]
    fn test_lark_client_basic_methods() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: openlark_core::constants::AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://test.feishu.cn".to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
            core_config: None,
        };

        let client = TestClient { config };

        assert_eq!(client.app_id(), "test_app_id");
        assert_eq!(client.app_secret(), "test_app_secret");
        assert_eq!(client.base_url(), "https://test.feishu.cn");
        assert_eq!(client.timeout(), Duration::from_secs(30));
        assert_eq!(client.retry_count(), 3);
        assert!(client.is_log_enabled());
        assert!(client.is_configured());
    }

    #[test]
    fn test_not_configured_client() {
        let config = Config {
            app_id: String::new(),
            app_secret: String::new(),
            ..Default::default()
        };

        let client = TestClient { config };
        assert!(!client.is_configured());
    }
}
