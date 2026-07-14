//! 编译期能力目录（compiled-capability catalog）
//!
//! 将 Cargo feature、Client 字段构造与 registry 诊断元数据收敛到同一声明
//!（见 issue #423 / #434–#436）。全部业务域均由本目录生成；不再维护
//! Client / registry 双声明。
//!
//! 统一声明入口：[`for_each_compiled_capability`]。
//! 宏面刻意保持最小（单列表 + 两投影 callback）。

#[macro_use]
mod unique;
#[macro_use]
mod macros;

mod catalog;

pub(crate) use catalog::for_each_compiled_capability;
pub(crate) use catalog::register_catalog_capabilities;

/// 独立于 catalog 生成的 feature oracle（#423：测试公共结果，不以宏内部为唯一期望源）。
///
/// 与 Cargo feature 门控对齐；新增业务域时此处与 `catalog.rs` 声明同步。
#[cfg(test)]
pub(crate) fn expected_capability_names_from_features() -> Vec<&'static str> {
    [
        None::<&str>,
        #[cfg(feature = "cardkit")]
        Some("cardkit"),
        #[cfg(feature = "auth")]
        Some("auth"),
        #[cfg(feature = "docs")]
        Some("docs"),
        #[cfg(feature = "communication")]
        Some("communication"),
        #[cfg(feature = "hr")]
        Some("hr"),
        #[cfg(feature = "meeting")]
        Some("meeting"),
        #[cfg(feature = "ai")]
        Some("ai"),
        #[cfg(feature = "workflow")]
        Some("workflow"),
        #[cfg(feature = "platform")]
        Some("platform"),
        #[cfg(feature = "application")]
        Some("application"),
        #[cfg(feature = "helpdesk")]
        Some("helpdesk"),
        #[cfg(feature = "mail")]
        Some("mail"),
        #[cfg(feature = "analytics")]
        Some("analytics"),
        #[cfg(feature = "user")]
        Some("user"),
        #[cfg(feature = "security")]
        Some("security"),
        #[cfg(feature = "bot")]
        Some("bot"),
    ]
    .into_iter()
    .flatten()
    .collect()
}
