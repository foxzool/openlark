//! 编译期能力目录（compiled-capability catalog）
//!
//! 将 Cargo feature、Client 字段构造与 registry 诊断元数据收敛到同一声明
//!（见 issue #423 / #434）。当前仅 `bot` 作为 tracer bullet 走此路径；
//! 其余业务域仍由 `declare_client!` 与 `registry/catalog.rs` 分别维护。
//!
//! 统一声明入口：[`for_each_compiled_capability`]。

#[macro_use]
mod macros;

mod catalog;

pub(crate) use catalog::for_each_compiled_capability;
pub(crate) use catalog::register_catalog_capabilities;
