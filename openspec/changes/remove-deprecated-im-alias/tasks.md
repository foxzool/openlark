# Tasks — remove-deprecated-im-alias

> 已确认拆分项 F（#268 剩余）。47 文件 `im::im::` → `im::` 路径缩短 + 删除 `pub mod im` 别名。BREAKING。

## 1. 迁移 47 文件导入路径

- [ ] 1.1 `sed -i '' 's/im::im::/im::/g'` 替换 `crates/openlark-communication/src/` 下所有命中文件（47 个）
- [ ] 1.2 验证 `grep -rn 'im::im::' crates/openlark-communication/src/` = 0

## 2. 删除 im 别名块

- [ ] 2.1 删除 `crates/openlark-communication/src/im/mod.rs` 的 `#[allow(clippy::module_inception)] #[deprecated(...)] /// 兼容历史... pub mod im { pub use super::project::{v1, v2}; }` 整块（保留 `mod project`/`pub use project::{v1,v2}`/其它 pub mod）

## 3. 验证

- [ ] 3.1 `pub mod im` grep im/mod.rs = 0；`im::im::` 全 crate = 0
- [ ] 3.2 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] 3.3 `cargo test --workspace` 通过

## 4. CHANGELOG

- [ ] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射（`im::im::` → `im::`）
