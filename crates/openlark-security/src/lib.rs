//! OpenLark 安全服务模块
//!
//! 提供飞书开放平台的完整安全服务，包括访问控制(ACS)和安全合规管理。
//!
//! ## 架构设计
//!
//! 采用 Project-Version-Resource (PVR) 三层架构，使用 canonical `openlark_core::config::Config`（#444–#447）：
//!
//! ```text
//! openlark-security/src/
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
//!     // 使用 canonical core Config（v0.18 推荐唯一路径，完整保留 token provider / headers / timeout 等）
//!     let config = Config::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .build();
//!     let security = SecurityClient::new(config);
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

/// 安全服务客户端（唯一公开入口）。
///
/// 直接持有 canonical `openlark_core::config::Config` + 项目（真实实现深度）。
/// 遵循 CLIENT_NAMING_CONVENTION：提供 `new(config: Config)`。
///
/// 用法：`client.security.acs...` 或 `client.security.config()`
///
/// 不再公开重复的 SecurityServices（#447 收口，消除 middle-man / duplicated shell）。
#[derive(Debug, Clone)]
pub struct SecurityClient {
    config: openlark_core::config::Config,
    /// ACS 项目
    pub acs: AcsProject,
    /// 安全合规项目
    pub security_and_compliance: SecurityAndComplianceProject,
}

impl SecurityClient {
    /// 使用 canonical `openlark_core::config::Config` 构造（v0.18 推荐/唯一路径）。
    ///
    /// 完整保留 token_provider、自定义 headers、timeout、retry 等配置。
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            acs: AcsProject::new(config.clone()),
            security_and_compliance: SecurityAndComplianceProject::new(config.clone()),
            config,
        }
    }

    /// 从 canonical core Config 构造（与 new 等价，兼容旧调用站点）。
    pub fn from_config(config: openlark_core::config::Config) -> Self {
        Self::new(config)
    }

    /// 返回当前 canonical 配置（不建议直接读取字段用于“验证”，请用行为测试证明传播）。
    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }
}

impl Default for SecurityClient {
    fn default() -> Self {
        Self::new(openlark_core::config::Config::default())
    }
}

/// 结果类型别名
pub type SecurityResult<T> = Result<T, crate::error::SecurityError>;

/// 预导出模块
pub mod prelude {
    pub use super::{
        AcsProject, SecurityAndComplianceProject, SecurityClient, SecurityResult,
    };

    // 避免v1命名空间冲突，明确导出需要的类型
    pub use super::acs::acs::{AcsProject as Acs, AcsV1Service};
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

    /// 直接用 SecurityClient::from_config 构造，证明 ACS 使用 retained canonical Config：
    /// - 自定义 base_url、headers、token_provider 生效
    /// - timeout、response-size 配置保持
    /// - 代表性 ACS leaf (users.list) wiremock 测试
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
            .req_timeout(std::time::Duration::from_secs(30))
            .max_response_size(8 * 1024 * 1024)
            .build();

        let config_with_provider = base.with_token_provider(TestTokenProvider("test_tok_from_provider"));

        let client = SecurityClient::from_config(config_with_provider);

        // 构造 sanity（可选）：实际验收通过 mock 的精确 header 匹配 + 下面的行为错误测试证明配置传播。
        // 不再把“读 config 字段”作为主要 #425 验收依据。
        let _ = (client.config().base_url(), client.acs.config().base_url()); // touch only

        // 执行 ACS leaf 调用（代表性）
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
            "请求路径应指向 ACS leaf，实际: {}",
            req.url
        );
    }

    /// 代表性 compliance (security_and_compliance v2) leaf 也应收到 retained canonical Config。
    /// 证明与 ACS 相同：base_url、headers、token_provider 完整保留。
    #[tokio::test]
    async fn security_client_from_config_propagates_to_compliance_v2_leaf() {
        let server = MockServer::start().await;

        // 精确匹配 header，证明 token provider 和自定义 header 传播
        Mock::given(method("GET"))
            .and(path("/open-apis/security_and_compliance/v2/device_records/mine"))
            .and(header("Authorization", "Bearer test_tok_from_provider"))
            .and(header("X-Compliance-Test", "propagated"))
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
            .add_header("X-Compliance-Test", "propagated")
            .req_timeout(std::time::Duration::from_secs(30))
            .max_response_size(8 * 1024 * 1024)
            .build();

        let config_with_provider = base.with_token_provider(TestTokenProvider("test_tok_from_provider"));

        let client = SecurityClient::from_config(config_with_provider);

        // 传播证明主要靠 mock header 匹配 + execute 成功；config 读仅作构造触达。
        let _ = client.config().base_url();

        let _ = client
            .security_and_compliance
            .v2()
            .device_records()
            .mine()
            .execute()
            .await
            .expect("compliance v2 leaf 应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1, "应只发一次 compliance leaf 请求");
        assert!(
            received[0].url.path().contains("/device_records/mine"),
            "请求路径应指向 compliance v2 leaf"
        );
    }

    /// 代表性 compliance v1 leaf（审计日志 list_data）也应收到 retained canonical Config。
    #[tokio::test]
    async fn security_client_from_config_propagates_to_compliance_v1_leaf() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/open-apis/security_and_compliance/v1/openapi_logs/list_data"))
            .and(header("Authorization", "Bearer test_tok_from_provider"))
            .and(header("X-Compliance-V1", "propagated"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "code": 0,
                "msg": "success",
                "data": { "items": [{ "request_id": "r1" }], "has_more": false }
            })))
            .mount(&server)
            .await;

        let base = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .allow_custom_base_url(true)
            .add_header("X-Compliance-V1", "propagated")
            .build();

        let config_with_provider = base.with_token_provider(TestTokenProvider("test_tok_from_provider"));

        let client = SecurityClient::from_config(config_with_provider);

        // 结构 sanity：header 已通过 provider + 传播生效（见 mock 精确匹配）
        // 不再依赖“读取存储字段”作为传播验收；下面补充真实错误路径触发测试。

        use serde_json::json;
        let _ = client
            .security_and_compliance
            .v1()
            .openapi_logs()
            .list_data()
            .body(json!({ "start_time": 1700000000, "end_time": 1700003600 }))
            .execute()
            .await
            .expect("compliance v1 leaf 应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(received[0].url.path().contains("/openapi_logs/list_data"));
    }

    /// 行为验证：通过极小 timeout 触发超时错误，证明 req_timeout 配置已传播到执行路径。
    /// 不依赖 client.config() 读取做验收。
    #[tokio::test]
    async fn security_config_timeout_propagates_and_triggers_timeout_error() {
        let server = MockServer::start().await;

        // 模拟慢响应（> timeout）
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/users"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"code":0,"msg":"ok","data":{"has_more":false,"items":[]}}))
                    .set_delay(std::time::Duration::from_millis(800)),
            )
            .mount(&server)
            .await;

        let base = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .allow_custom_base_url(true)
            .req_timeout(std::time::Duration::from_millis(50)) // 极短，必超时
            .build();

        let config = base.with_token_provider(TestTokenProvider("test_tok_for_timeout_test"));

        let client = SecurityClient::new(config);

        let result = client
            .acs
            .v1()
            .users()
            .list()
            .execute()
            .await;

        // 必须是错误，且与超时相关（reqwest timeout 或上层包装）
        assert!(result.is_err(), "应因 timeout 配置触发错误");
        let err = result.unwrap_err().to_string().to_lowercase();
        // 短 timeout 常表现为网络/请求错误（reqwest 超时包装），接受宽松匹配
        assert!(
            err.contains("timeout") || err.contains("time") || err.contains("deadline") || err.contains("network") || err.contains("send"),
            "错误应体现超时或网络失败，实际: {}",
            err
        );
    }

    /// 行为验证：小 max_response_size + 大响应体 → response_too_large 错误。
    #[tokio::test]
    async fn security_config_max_response_size_propagates_and_triggers_size_error() {
        let server = MockServer::start().await;

        // 构造一个 > limit 的响应（用大 body）
        let big_body = serde_json::json!({
            "code": 0,
            "msg": "ok",
            "data": { "has_more": false, "items": [ {"x": "y".repeat(1024)} ] }
        });
        let big_json = serde_json::to_vec(&big_body).unwrap();

        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/users"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(big_json, "application/json"),
            )
            .mount(&server)
            .await;

        let base = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .allow_custom_base_url(true)
            .max_response_size(512) // 故意很小
            .build();

        let config = base.with_token_provider(TestTokenProvider("test_tok_for_size_test"));

        let client = SecurityClient::new(config);

        let result = client
            .acs
            .v1()
            .users()
            .list()
            .execute()
            .await;

        assert!(result.is_err(), "应因 response size 超限触发错误");
        let err = result.unwrap_err().to_string();
        // 精确错误来自 CoreError::response_too_large
        assert!(
            err.contains("响应体过大") || err.to_lowercase().contains("large") || err.to_lowercase().contains("size") || err.contains("超过限制"),
            "错误应体现响应过大，实际: {}",
            err
        );
    }
}
