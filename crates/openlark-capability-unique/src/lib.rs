//! trybuild harness：capability catalog 生成期唯一性（#423 / #455）
//!
//! 本 crate **`publish = false`**，且 **不是** `openlark-client` 的依赖。
//! 生产路径使用 `openlark-client` 内的 crate 私有宏；此处仅供 compile-fail
//! 固定期望，避免在可发布 crate 上导出测试宏或内部 Cargo feature。
//!
//! Internal test harness (not published).
//! The ui tests directly invoke `openlark_client::assert_capability_catalog_unique!`
//! (the production macro, enabled only via the _test-catalog-unique feature on the dep).
//! This ensures the tests cover the actual production uniqueness logic.
