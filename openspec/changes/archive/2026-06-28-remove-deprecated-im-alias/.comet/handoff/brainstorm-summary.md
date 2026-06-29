# Brainstorm Summary

- Change: remove-deprecated-im-alias
- Date: 2026-06-28

## 确认的技术方案

方案 A（一次性 sed），用户已确认。

- **迁移 47 个 `.rs` 文件**：`sed -i '' 's/im::im::/im::/g'` 仅作用于 `crates/openlark-communication/src/`。命中均为分组 `use crate::{ ... }` 块的续行导入（`im::im::v1::...` → `im::v1::...`，经 `im` 模块的 `pub use project::{v1,v2}` 解析到同一目标）。
- **删除别名块 `im/mod.rs:12-20`**：`#[allow(clippy::module_inception)]` + `#[deprecated(...)]` + `/// 兼容历史...` + `pub mod im { pub use super::project::{v1, v2}; }`（整块含属性）。
- **删除依赖测试块 `im/mod.rs:26-36`**：整个 `#[cfg(test)] mod tests { ... }`（其唯一测试 `nested_im_path_remains_a_compatibility_alias` 在 line 32 用 `super::im::v1::...` 引用别名，删别名后必编译失败）。
- **保留**：`pub mod card` / `mod project` / `pub use project::{v1,v2}` / `pub mod im_ephemeral` / `pub mod im_message`。
- **CHANGELOG breaking 条目**：镜像 wiki Params house style，追加于 `[Unreleased] > ### Breaking Changes`（line 20 区块，wiki 条目之后）。

## 关键取舍与风险

- **sed 安全性（已对抗验证）**：物理 `src/im/im/` 实为 `mod project`（`#[path="im/mod.rs"]`），唯一真实 `im::im` 路径就是 deprecated 别名 → sed 无误伤。47 `.rs` 文件 / 50 行命中全为导入续行，零字符串/文档/属性/类型位命中。
- **scope 收紧（验证修正）**：47/50 计数**仅限 `.rs` 源文件**。该模式还出现在本 change 自己的 4 个 openspec 文档（proposal/design/tasks/spec，19 处，描述性引用），不在 sed 作用域。
- **[Breaking]** 外部 `openlark_communication::im::im::v1` 编译失败 → CHANGELOG 迁移指引（drop-in，`im::im::` → `im::`）。跨 crate/examples/tests/其它 crate 命中 = 0，breaking 影响纯外部用户。
- **行号精确性（修正 design.md）**：design.md 写 `17-20`，实际别名块 = `12-20`（含 `#[allow]` 12 + `#[deprecated]` 13-16 + doc 17 + `pub mod im{}` 18-20）。
- 回滚：`git revert`。

## 测试策略

- `grep -rn 'im::im::' crates/openlark-communication/src/` = 0
- `grep -E 'pub mod im\b' crates/openlark-communication/src/im/mod.rs` = 0（word-boundary 排除 `im_ephemeral`/`im_message`；`grep -w` POSIX 等价）
- `grep 'nested_im_path_remains_a_compatibility_alias' crates/openlark-communication/src/im/mod.rs` = 0
- 三组 feature clippy `-D warnings` exit 0：default / `--all-features` / `--no-default-features`
- `cargo test --workspace` 通过

## Spec Patch

回写 `specs/no-deprecated-im-alias/spec.md`：

1. **修正 "im 别名块移除" 场景**的验证命令：`grep 'pub mod im'` → `grep -E 'pub mod im\b'`（= 0）。原命令是 false-negative 陷阱（`pub mod im_ephemeral`/`im_message` 匹配前缀，移除别名后仍命中 2 行，场景永不满足）。
2. **新增场景 "依赖别名测试块移除"**：WHEN 在 `im/mod.rs` 中 grep `nested_im_path_remains_a_compatibility_alias` THEN 命中 0（覆盖任务缺口：测试块必须连同别名删除）。

## CHANGELOG 条目（定稿）

```
- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移 `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。
```
