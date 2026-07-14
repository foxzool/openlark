//! trybuild harness：capability catalog 生成期唯一性（#423 / #455）
//!
//! 本 crate **`publish = false`**，且 **不是** `openlark-client` 的依赖。
//! 生产路径使用 `openlark-client` 内的 crate 私有宏；此处仅供 compile-fail
//! 固定期望，避免在可发布 crate 上导出测试宏或内部 Cargo feature。
//!
//! 宏实现臂通过 include! 共享自 openlark-client 的 unique-macro.inc.rs。
//! 生产 client 在 _test-catalog-unique feature 下导出，harness reexport 以保持测试接口不变。
//! 正常构建的生产 client 不导出内部实现。

pub use openlark_client::assert_capability_catalog_unique;
