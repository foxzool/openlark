//! 编译期能力目录（compiled-capability catalog）
//!
//! 将 Cargo feature 与 Client 字段构造收敛到同一声明（见 issue #423 / #434–#437）。
//! 全部业务域均由本目录生成；不再维护 Client / registry 双声明。
//!
//! 统一声明入口：[`for_each_compiled_capability`]。

#[macro_use]
mod unique;

mod catalog;

pub(crate) use catalog::for_each_compiled_capability;
