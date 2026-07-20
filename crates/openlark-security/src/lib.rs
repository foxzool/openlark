//! OpenLark 安全服务模块
//!
//! 提供飞书开放平台的完整安全服务，包括访问控制(ACS)和安全合规管理。
//!
//! ## 架构设计
//!
//! 采用 Project-Version-Resource (PVR) 三层架构：
//!
//! ```text
//! openlark-security/src/
//! ├── config.rs         # 安全服务配置（SecurityConfig）
//! ├── acs/              # 访问控制系统 (Project)
//! │   └── v1/          # API版本v1 (Version)
//! └── security_and_compliance/  # 安全合规管理 (Project)
//!     ├── v1/          # API版本v1 (Version) - 审计日志
//!     └── v2/          # API版本v2 (Version) - 设备记录管理
//! ```
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use openlark_security::prelude::*;
//! use openlark_core::config::Config;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 新推荐路径：使用 canonical core Config（完整保留 token provider / headers 等）
//!     let config = Config::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .build();
//!     let security = SecurityClient::from_config(config);
//!
//!     // 获取门禁用户列表（响应 data 透传为 ListUsersResponse）
//!     let users = security.acs.v1().users().list()
//!         .page_size(20)
//!         .execute()
//!         .await?;
//!
//!     println!("是否有更多: {}", users.has_more);
//!     Ok(())
//! }
//! ```
//!
//! ## API覆盖
//!
//! ### acs (v1) - 访问控制系统
//!
//! 所有端点走 `openlark_core::Transport` + App access token，返回响应 `data` 字段内容。
//! 各 Service 是门面，方法返回 `*Request` 构建器（`.execute()` 发请求）。
//!
//! #### 用户管理 (users)
//! - `users.get(user_id)` / `users.list()` / `users.create()` / `users.patch(user_id)` / `users.delete(user_id)`
//!
//! #### 用户人脸 (user_faces，`/users/{user_id}/face`)
//! - `user_faces.get(user_id)` - 下载用户人脸
//! - `user_faces.update(user_id)` - 上传用户人脸
//!
//! #### 人脸资源 (face，独立资源 `/faces`)
//! - `face().get(face_id)` / `face().create()` / `face().delete(face_id)`
//!
//! #### 设备管理 (devices)
//! - `devices.get/list/create/update/delete/approve/query`
//! - `client_device(device_id)` - 客户端设备认证（便捷方法）
//!
//! #### 权限规则 (rule_external)
//! - `rule_external.create(rule_id)` - 创建或更新权限组（body `{"rule":...}`）
//! - `rule_external.get(device_id)` - 获取权限组
//! - `rule_external.delete(rule_id)` - 删除权限组（无 body）
//! - `rule_external.device_bind()` - 设备绑定（`{device_id, rule_ids[]}`）
//!
//! #### 访客管理 (visitors)
//! - `visitors.create()` / `visitors.delete(visitor_id)`
//!
//! #### 访问记录 (access_records)
//! - `access_records.list()` / `access_records.get_access_photo(access_record_id)`
//!
//! ### security_and_compliance (v2/v1) - 安全合规管理
//! #### 设备记录管理 (device_record - v2)
//! - `device_records.mine()` - 获取客户端设备认证信息
//! - `device_records.create()` - 新增设备
//! - `device_records.list()` - 查询设备信息
//! - `device_records.get()` - 获取设备信息
//! - `device_records.update()` - 更新设备
//! - `device_records.delete()` - 删除设备
//!
//! #### 设备申报审批 (device_apply_record - v2)
//! - `device_apply_records.approve()` - 审批设备申报
//!
//! #### 审计日志管理 (openapi_log - v1)
//! - `openapi_logs.list_data()` - 获取OpenAPI审计日志数据

#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// 错误处理模块
pub mod error;

// 安全服务配置
pub mod config;

// Project: acs - 访问控制系统
pub mod acs;
pub mod security;

// 重新导出主要类型
pub use acs::acs::{AcsProject, AcsV1Service};
pub use security::security_and_compliance::{
    SecurityAndComplianceProject, SecurityAndComplianceV1Service, SecurityAndComplianceV2Service,
};

// 重新导出错误类型
pub use crate::error::SecurityError;

// 重新导出 canonical core Config，便于独立使用 security crate
pub use openlark_core::config::Config;

/// 安全服务统一入口
#[derive(Debug)]
pub struct SecurityServices {
    /// 安全配置
    pub config: std::sync::Arc<crate::config::SecurityConfig>,
    /// ACS门禁控制项目
    pub acs: AcsProject,
    /// 安全合规项目
    pub security_and_compliance: SecurityAndComplianceProject,
}

impl SecurityServices {
    /// 创建新的安全服务实例
    ///
    /// 内部把 `SecurityConfig` 转换为一份 `openlark_core::Config`，acs 与
    /// security_and_compliance 都用它走 SDK 标准的 Transport 路径。
    pub fn new(config: crate::config::SecurityConfig) -> Self {
        let config = std::sync::Arc::new(config);

        // SecurityConfig → openlark_core::Config（owned）
        let core_config = openlark_core::config::Config::builder()
            .app_id(&config.app_id)
            .app_secret(&config.app_secret)
            .base_url(&config.base_url)
            .build();

        Self {
            acs: AcsProject::new(core_config.clone()),
            security_and_compliance: SecurityAndComplianceProject::new(core_config),
            config,
        }
    }

    /// 获取配置信息
    pub fn config(&self) -> &crate::config::SecurityConfig {
        &self.config
    }

    /// 使用 canonical `openlark_core::config::Config` 构造（新推荐路径）。
    ///
    /// 该路径可完整保留 token_provider、自定义 headers、timeout、retry_count、
    /// max_response_size 等配置信息，不会因 SecurityConfig 镜像而丢失。
    ///
    /// 旧的 `SecurityConfig` 构造路径暂时保留，用于 expand-contract 迁移。
    pub fn from_config(config: openlark_core::config::Config) -> Self {
        // 过渡期：为 pub config 字段合成最小 SecurityConfig，保持字段访问兼容。
        // 真实业务逻辑全部走传入的 canonical Config（传给 Projects）。
        let legacy = std::sync::Arc::new(crate::config::SecurityConfig::new(
            config.app_id(),
            config.app_secret(),
        ).with_base_url(config.base_url()));

        Self {
            acs: AcsProject::new(config.clone()),
            security_and_compliance: SecurityAndComplianceProject::new(config),
            config: legacy,
        }
    }

    /// 返回底层的 canonical core 配置（推荐用于诊断与直接传递给函数式 API）。
    pub fn core_config(&self) -> &openlark_core::config::Config {
        // Projects 内部持有相同 Config，这里借用 acs 的即可。
        self.acs.config()
    }
}

/// 安全服务客户端 — Arc 包装的 [`SecurityServices`]，支持零成本克隆。
///
/// 用法：`client.security.acs...`
#[derive(Debug, Clone)]
pub struct SecurityClient {
    inner: std::sync::Arc<SecurityServices>,
}

impl SecurityClient {
    /// 从安全配置创建客户端实例（旧路径，保留用于 expand-contract 迁移）。
    pub fn new(config: crate::config::SecurityConfig) -> Self {
        Self {
            inner: std::sync::Arc::new(SecurityServices::new(config)),
        }
    }

    /// 从 canonical core Config 构造 SecurityClient（新推荐路径）。
    ///
    /// 根 Client 内部与直接构造均应使用此路径，以确保配置不丢失。
    pub fn from_config(config: openlark_core::config::Config) -> Self {
        Self {
            inner: std::sync::Arc::new(SecurityServices::from_config(config)),
        }
    }
}

impl std::ops::Deref for SecurityClient {
    type Target = SecurityServices;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for SecurityServices {
    fn default() -> Self {
        Self::new(crate::config::SecurityConfig::default())
    }
}

/// 结果类型别名
pub type SecurityResult<T> = Result<T, crate::error::SecurityError>;

/// 预导出模块
pub mod prelude {
    pub use super::{
        AcsProject, SecurityAndComplianceProject, SecurityClient, SecurityResult, SecurityServices,
    };

    // 避免v1命名空间冲突，明确导出需要的类型
    pub use super::acs::acs::{AcsProject as Acs, AcsV1Service};
    pub use super::config::SecurityConfig;
    pub use super::security::security_and_compliance::{
        SecurityAndComplianceV1Service, SecurityAndComplianceV2Service,
    };
}

#[cfg(test)]
mod construction_tests {
    use super::*;
    use openlark_core::auth::{TokenProvider, TokenRequest};
    use std::future::Future;
    use std::pin::Pin;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// 测试用 TokenProvider：总是返回固定 token，用于验证 provider 传播。
    #[derive(Debug, Clone)]
    struct TestTokenProvider(&'static str);

    impl TokenProvider for TestTokenProvider {
        fn get_token(
            &self,
            _request: TokenRequest,
        ) -> Pin<Box<dyn Future<Output = openlark_core::SDKResult<String>> + Send + '_>> {
            let token = self.0.to_string();
            Box::pin(async move { Ok(token) })
        }
    }

    /// 直接用 SecurityClient::from_config 构造，证明：
    /// - 自定义 base_url 生效
    /// - 自定义 header 传播
    /// - token_provider 提供的 token 作为 Authorization
    #[tokio::test]
    async fn security_client_from_canonical_config_propagates_base_headers_and_token_provider() {
        let server = MockServer::start().await;

        // 精确匹配 header，证明三者都传播到了外发请求
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/users"))
            .and(header("Authorization", "Bearer test_tok_from_provider"))
            .and(header("X-Custom-Prop", "yes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "code": 0,
                "msg": "success",
                "data": { "has_more": false, "items": [] }
            })))
            .mount(&server)
            .await;

        let base = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .allow_custom_base_url(true)
            .add_header("X-Custom-Prop", "yes")
            .build();

        let config_with_provider = base.with_token_provider(TestTokenProvider("test_tok_from_provider"));

        let client = SecurityClient::from_config(config_with_provider);

        // 执行 leaf 调用
        let _resp = client
            .acs
            .v1()
            .users()
            .list()
            .execute()
            .await
            .expect("wiremock 应返回成功响应");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1, "应只发一次 security leaf 请求");

        // 路径匹配器 + header 匹配器已证明 base_url、自定义 header、token provider 生效。
        // 这里再做一次宽松确认（url 里包含我们 mock 的路径即可）。
        let req = &received[0];
        assert!(
            req.url.path() == "/open-apis/acs/v1/users" || req.url.as_str().contains("/acs/v1/users"),
            "请求路径应指向 security leaf，实际: {}",
            req.url
        );
    }

    /// 验证旧路径仍然可用（expand-contract 期间）
    #[test]
    fn legacy_security_config_path_still_works() {
        let sec_cfg = crate::config::SecurityConfig::new("a", "b").with_base_url("https://example.com");
        let _client = SecurityClient::new(sec_cfg);
        // 仅验证可构造，不发起网络
    }
}
