use crate::{Result, configuration_error, validation_error, with_context};
use openlark_core::config::{Config, ConfigSummary};
use openlark_core::error::ErrorTrait;
use std::env;

/// 环境变量校验结果
///
/// 由 `check_env_config` 返回，承载已校验的必填凭证，
/// 避免调用方重复读取环境变量。
pub struct EnvConfig {
    /// 飞书应用 App ID（已校验非空）
    pub app_id: String,
    /// 飞书应用 App Secret（已校验非空）
    pub app_secret: String,
}

/// 🔍 检查环境变量配置
///
/// 验证飞书应用所需的环境变量是否正确设置
///
/// # 返回
/// - `Ok(EnvConfig)`: 环境变量配置正确，返回已校验的凭证
/// - `Err(Error)`: 环境变量配置错误，包含详细的错误信息和恢复建议
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::{prelude::*, utils};
///
/// match utils::check_env_config() {
///     Ok(_) => println!("环境变量配置正确"),
///     Err(error) => {
///         eprintln!("❌ {}", error.user_message().unwrap_or("未知错误"));
///         for step in error.recovery_steps() {
///             eprintln!("• {}", step);
///         }
///     }
/// }
/// ```
pub fn check_env_config() -> Result<EnvConfig> {
    // 检查 OPENLARK_APP_ID
    let app_id = env::var("OPENLARK_APP_ID")
        .map_err(|_| configuration_error("环境变量检查失败 [variable: OPENLARK_APP_ID]"))?;

    if app_id.is_empty() {
        return with_context(
            Err(validation_error(
                "OPENLARK_APP_ID",
                "应用ID环境变量不能为空",
            )),
            "validation",
            "env_config",
        );
    }

    // 检查 OPENLARK_APP_SECRET
    let app_secret = env::var("OPENLARK_APP_SECRET")
        .map_err(|_| configuration_error("环境变量检查失败 [variable: OPENLARK_APP_SECRET]"))?;

    if app_secret.is_empty() {
        return with_context(
            Err(validation_error(
                "OPENLARK_APP_SECRET",
                "应用密钥环境变量不能为空",
            )),
            "validation",
            "env_config",
        );
    }

    // 检查可选的环境变量
    if let Ok(base_url) = env::var("OPENLARK_BASE_URL")
        && !base_url.starts_with("http://")
        && !base_url.starts_with("https://")
    {
        return with_context(
            Err(validation_error(
                "OPENLARK_BASE_URL",
                "基础URL必须以http://或https://开头",
            )),
            "validation",
            "env_config",
        );
    }

    // 检查超时设置
    if let Ok(timeout_str) = env::var("OPENLARK_TIMEOUT")
        && timeout_str.parse::<u64>().is_err()
    {
        return with_context(
            Err(validation_error(
                "OPENLARK_TIMEOUT",
                "超时设置必须是有效的数字（秒数）",
            )),
            "validation",
            "env_config",
        );
    }

    Ok(EnvConfig { app_id, app_secret })
}

/// 🔧 从环境变量创建配置
///
/// 自动读取环境变量并创建客户端配置
///
/// # 返回
/// - `Ok(Config)`: 成功创建配置
/// - `Err(Error)`: 配置创建失败，包含详细错误信息
pub fn create_config_from_env() -> Result<Config> {
    // 校验环境变量并获取已校验的凭证
    let env_cfg = check_env_config()?;

    let app_id = env_cfg.app_id;
    let app_secret = env_cfg.app_secret;

    let base_url =
        env::var("OPENLARK_BASE_URL").unwrap_or_else(|_| "https://open.feishu.cn".to_string());

    let timeout = env::var("OPENLARK_TIMEOUT")
        .ok()
        .and_then(|t| t.parse().ok())
        .map(std::time::Duration::from_secs);

    let enable_log = env::var("OPENLARK_ENABLE_LOG")
        .ok()
        .and_then(|l| l.parse().ok())
        .unwrap_or(false);

    let mut builder = Config::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .base_url(base_url)
        .enable_log(enable_log);

    if let Some(timeout_duration) = timeout {
        builder = builder.req_timeout(timeout_duration);
    }

    Ok(builder.build())
}

/// 📊 获取配置摘要
///
/// 返回当前配置的摘要信息，用于调试和监控
pub fn get_config_summary(config: &Config) -> ConfigSummary {
    config.summary()
}

/// 🏷️ 获取启用的功能列表
///
/// 返回当前编译时启用的功能标志列表
pub fn get_enabled_features() -> Vec<&'static str> {
    // 基础功能（始终启用）
    #[allow(unused_mut)]
    let mut features = vec!["auth"];

    // 可选功能（基于编译时标志）
    #[cfg(feature = "communication")]
    features.push("communication");

    #[cfg(feature = "docs")]
    features.push("docs");

    #[cfg(feature = "security")]
    features.push("security");

    #[cfg(feature = "hr")]
    features.push("hr");

    #[cfg(feature = "ai")]
    features.push("ai");

    features
}

/// 🔍 验证功能依赖关系
///
/// 检查启用的功能是否满足依赖关系要求
pub fn validate_feature_dependencies() -> Result<Vec<String>> {
    let enabled_features = get_enabled_features();
    let mut issues = Vec::new();

    // 检查核心依赖
    if enabled_features.contains(&"communication") && !enabled_features.contains(&"auth") {
        issues.push("通讯服务 (communication) 需要启用认证服务 (auth)".to_string());
    }

    if enabled_features.contains(&"docs") && !enabled_features.contains(&"auth") {
        issues.push("文档服务 (docs) 需要启用认证服务 (auth)".to_string());
    }

    if enabled_features.contains(&"hr") && !enabled_features.contains(&"auth") {
        issues.push("人力资源服务 (hr) 需要启用认证服务 (auth)".to_string());
    }

    if enabled_features.contains(&"ai") && !enabled_features.contains(&"auth") {
        issues.push("AI服务 (ai) 需要启用认证服务 (auth)".to_string());
    }

    if enabled_features.contains(&"calendar") && !enabled_features.contains(&"auth") {
        issues.push("日历服务 (calendar) 需要启用认证服务 (auth)".to_string());
    }

    if enabled_features.contains(&"admin") && !enabled_features.contains(&"hr") {
        issues.push("管理服务 (admin) 建议启用人力资源服务 (hr) 以获得完整功能".to_string());
    }

    if enabled_features.contains(&"approval") && !enabled_features.contains(&"auth") {
        issues.push("审批服务 (approval) 需要启用认证服务 (auth)".to_string());
    }

    if issues.is_empty() {
        Ok(issues)
    } else {
        with_context(
            Err(configuration_error(format!(
                "发现 {} 个功能依赖问题: {}",
                issues.len(),
                issues.join("; ")
            ))),
            "validation",
            "feature_dependencies",
        )
    }
}

/// 🏥 诊断系统配置
///
/// 执行全面的系统配置检查，包括环境变量、功能依赖等
pub fn diagnose_system() -> SystemDiagnostics {
    let mut diagnostics = SystemDiagnostics::new();

    // 检查环境变量
    match check_env_config() {
        Ok(_) => {
            diagnostics.env_config_status = "✅ 正常".to_string();
        }
        Err(error) => {
            diagnostics.env_config_status =
                format!("❌ {}", error.user_message().unwrap_or("未知错误"));
            diagnostics.add_issue("环境变量", error.user_message().unwrap_or("未知错误"));
        }
    }

    // 检查功能依赖
    match validate_feature_dependencies() {
        Ok(_) => {
            diagnostics.feature_deps_status = "✅ 正常".to_string();
        }
        Err(error) => {
            diagnostics.feature_deps_status =
                format!("❌ {}", error.user_message().unwrap_or("未知错误"));
            diagnostics.add_issue("功能依赖", error.user_message().unwrap_or("未知错误"));
        }
    }

    // 列出启用的功能
    diagnostics.enabled_features = get_enabled_features()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    diagnostics
}

/// 🏥 系统诊断结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemDiagnostics {
    /// 🌍 环境变量配置状态
    pub env_config_status: String,
    /// 🔗 功能依赖状态
    pub feature_deps_status: String,
    /// 🏷️ 启用的功能列表
    pub enabled_features: Vec<String>,
    /// ⚠️ 发现的问题列表
    pub issues: Vec<DiagnosticIssue>,
}

impl SystemDiagnostics {
    /// 创建新的诊断结果
    pub fn new() -> Self {
        Self {
            env_config_status: "未检查".to_string(),
            feature_deps_status: "未检查".to_string(),
            enabled_features: Vec::new(),
            issues: Vec::new(),
        }
    }

    /// 添加问题到诊断结果
    pub fn add_issue(&mut self, category: &str, description: &str) {
        self.issues.push(DiagnosticIssue {
            category: category.to_string(),
            description: description.to_string(),
        });
    }

    /// 获取健康状态摘要
    pub fn health_summary(&self) -> String {
        let healthy_count = self.issues.len();
        if healthy_count == 0 {
            "🟢 系统配置健康".to_string()
        } else {
            format!("🟡 发现 {healthy_count} 个配置问题")
        }
    }

    /// 检查是否有严重问题
    pub fn has_critical_issues(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.category.contains("环境变量") || issue.category.contains("功能依赖"))
    }
}

impl Default for SystemDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔍 诊断问题条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticIssue {
    /// 🏷️ 问题类别
    pub category: String,
    /// 📝 问题描述
    pub description: String,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
