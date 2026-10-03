//! OpenLark CardKit Module
//!
//! 飞书 CardKit（卡片能力）服务模块，提供卡片实体与组件相关 API。
//!
//! ## 目录组织（strict）
//!
//! 实现文件严格按 `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs` 组织，
//! 数据源为仓库根目录 `api_list_export.csv`。

pub mod common;
pub mod endpoints;

// 业务模块（按 bizTag 组织）
pub mod cardkit;

pub use common::chain::CardkitClient;
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{SDKResult, config::Config};
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_cardkit_client_creation() {
        let config = create_test_config();
        let client = CardkitClient::new(config);
        assert!(client.config().app_id() == "test_app");
    }

    #[test]
    fn test_cardkit_client_clone() {
        let config = create_test_config();
        let client = CardkitClient::new(config);
        let cloned = client.clone();
        assert!(cloned.config().app_id() == "test_app");
    }

    #[test]
    fn test_cardkit_client_v1() {
        let config = create_test_config();
        let client = CardkitClient::new(config);
        let _v1 = &client.v1;
    }
}
