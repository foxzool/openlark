# Tasks — remove-deprecated-im-alias

> 已确认拆分项 F（#268 剩余）。47 文件 `im::im::` → `im::` 路径缩短 + 删除 `pub mod im` 别名。BREAKING。

## 1. 迁移 47 文件导入路径

- [x] 1.1 `sed -i '' 's/im::im::/im::/g'` 替换 `crates/openlark-communication/src/` 下所有命中文件（47 个）
- [x] 1.2 验证 `grep -rn 'im::im::' crates/openlark-communication/src/` = 0

## 2. 删除 im 别名块

- [x] 2.1 删除 `crates/openlark-communication/src/im/mod.rs:12-20` 的 `#[allow(clippy::module_inception)] #[deprecated(...)] /// 兼容历史... pub mod im { pub use super::project::{v1, v2}; }` 整块 + 连同删除 `:26-36` 依赖测试块（保留 `mod project`/`pub use project::{v1,v2}`/`pub mod im_ephemeral`/`pub mod im_message`）

## 3. 验证

- [x] 3.1 `grep -E 'pub mod im\b' crates/openlark-communication/src/im/mod.rs` = 0（word-boundary，排除 `im_ephemeral`/`im_message`）；`grep -rn 'im::im::' crates/openlark-communication/src/` = 0
- [x] 3.2 三组 feature clippy（default/all-features/no-default）`-- -Dwarnings -A missing_docs` 全 exit 0
- [x] 3.3 `cargo test --workspace` 通过（0 failed）

## 4. CHANGELOG

- [x] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射（`im::im::` → `im::`）

## 5. 代码审查（review_mode: standard）

- [x] 5.1 requesting-code-review subagent 审查 `9aa40f878..35f3101bb`：**Ready to merge: Yes**。Critical 0 / Important 0 / Minor 1。
  - Minor（接受，不修）：被删测试 `nested_im_path_remains_a_compatibility_alias` 本为 deprecated 期保航，随别名退役正确；canonical re-export `pub use project::{v1,v2}` 已被 47 个内部 import 间接覆盖，审查者明确不建议新增替代测试。接受理由：无功能回归，新增测试价值低。

