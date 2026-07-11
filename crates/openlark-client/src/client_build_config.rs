//! Client 构造期校验 helper（#415 migrate 后保留，供 `Client::with_core_config` 与后续 contraction 切片使用）。
//!
//! 配置**状态**已迁至 core [`openlark_core::config::ConfigBuilder`]；本文件不再镜像字段。

use crate::Result;
use openlark_core::config::Config as CoreConfig;

/// 对已有 core `Config` 做 Client 入口侧校验（弱于 `Config::validate`：不含域名白名单）。
///
/// `ClientBuilder::build` 已改用 `Config::validate` + 零超时检查；本函数仍被
/// [`crate::Client::with_core_config`] 使用，待下一切片统一构造 seam 后删除。
pub(crate) fn validate_core_config(config: &CoreConfig) -> Result<()> {
    if config.app_id().is_empty() {
        return Err(crate::error::validation_error("app_id", "app_id不能为空"));
    }

    if config.app_secret().is_empty() {
        return Err(crate::error::validation_error(
            "app_secret",
            "app_secret不能为空",
        ));
    }

    validate_base_url(config.base_url())?;

    if config
        .req_timeout()
        .is_some_and(|timeout| timeout.is_zero())
    {
        return Err(crate::error::validation_error(
            "timeout",
            "timeout必须大于0",
        ));
    }

    if config.retry_count() > 10 {
        return Err(crate::error::validation_error(
            "retry_count",
            "retry_count不能超过10",
        ));
    }

    Ok(())
}

fn validate_base_url(base_url: &str) -> Result<()> {
    if base_url.is_empty() {
        return Err(crate::error::validation_error(
            "base_url",
            "base_url不能为空",
        ));
    }

    if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
        return Err(crate::error::validation_error(
            "base_url",
            "base_url必须以http://或https://开头",
        ));
    }

    Ok(())
}
