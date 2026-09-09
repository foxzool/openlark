//! 妙搭产品使用权限。

use serde::{Deserialize, Serialize};

/// 获取妙搭产品使用权限。
pub mod get;
/// 修改妙搭产品使用权限。
pub mod update;

pub use get::{GetMiaodaAvailableScopeRequest, GetMiaodaAvailableScopeResponse};
pub use update::{
    UpdateMiaodaAvailableScopeBody, UpdateMiaodaAvailableScopeRequest,
    UpdateMiaodaAvailableScopeResponse,
};

/// 妙搭产品使用权限模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvailableScopeMode {
    /// 全部允许。
    AllowAll,
    /// 全部禁止。
    DenyAll,
    /// 部分允许。
    AllowPart,
    /// 部分禁止。
    DenyPart,
}

#[cfg(test)]
mod tests {
    use super::AvailableScopeMode;
    use serde_json::json;

    #[test]
    fn available_scope_mode_screaming_snake() {
        assert_eq!(
            serde_json::to_value(AvailableScopeMode::AllowAll).unwrap(),
            json!("ALLOW_ALL")
        );
        assert_eq!(
            serde_json::to_value(AvailableScopeMode::DenyAll).unwrap(),
            json!("DENY_ALL")
        );
        assert_eq!(
            serde_json::to_value(AvailableScopeMode::AllowPart).unwrap(),
            json!("ALLOW_PART")
        );
        assert_eq!(
            serde_json::to_value(AvailableScopeMode::DenyPart).unwrap(),
            json!("DENY_PART")
        );
    }
}
