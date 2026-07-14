//! trybuild harness：capability catalog 生成期唯一性（#423 / #455）
//!
//! 本 crate **`publish = false`**，且 **不是** `openlark-client` 的依赖。
//! UI 用例通过 `#[path]` 直接引入生产 `capability/unique.rs`，
//! 无需复制宏体，也不会在可发布 crate 上暴露测试宏或内部 feature。
