//! 安全服务配置（legacy，已在 v0.18 收缩）
//!
//! 仅用于向后兼容说明/迁移参考，请改用 core `openlark_core::config::Config` + `SecurityClient::new`。
#![allow(deprecated)]

/// 安全服务配置（已废弃）
///
/// v0.18 起请直接使用 `openlark_core::config::Config` + `SecurityClient::from_config`。
#[deprecated(
    since = "0.18.0",
    note = "使用 openlark_core::config::Config 代替，并通过 SecurityClient::new(config) 构造"
)]
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// 应用ID（仅供迁移参考）
    pub app_id: String,
    /// 应用密钥（仅供迁移参考）
    pub app_secret: String,
    /// 基础URL（仅供迁移参考）
    pub base_url: String,
}

impl SecurityConfig {
    /// 创建新的安全配置实例（仅供迁移参考，已废弃）。
    #[deprecated(
        since = "0.18.0",
        note = "直接使用 openlark_core::config::Config::builder() 后调用 SecurityClient::new"
    )]
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置基础URL（仅供迁移参考，已废弃）。
    #[deprecated(
        since = "0.18.0",
        note = "直接使用 openlark_core::config::Config::builder() 后调用 SecurityClient::new"
    )]
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    // get_app_access_token 等任何主动 field-by-field Config 转换 + 手工取 token 的 legacy 行为已完全移除（#447 收口）。
    // 旧代码请迁移到 canonical Config 路径。
}
