//! 编译期能力目录（compiled-capability catalog）
//!
//! 将 Cargo feature、Client 字段构造与 registry 诊断元数据收敛到同一声明
//!（见 issue #423 / #434–#436）。全部业务域均由本目录生成；不再维护
//! Client / registry 双声明。
//!
//! 统一声明入口：[`for_each_compiled_capability`]。
//! 宏面刻意保持最小（单列表 + 两投影 callback）。

#[macro_use]
mod macros;

mod catalog;

pub(crate) use catalog::for_each_compiled_capability;
pub(crate) use catalog::register_catalog_capabilities;

#[cfg(test)]
pub(crate) use catalog::catalog_capability_names;
