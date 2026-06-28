---
comet_change: remove-deprecated-im-alias
role: technical-design
canonical_spec: openspec
---

# Design — remove-deprecated-im-alias

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/remove-deprecated-im-alias/specs/no-deprecated-im-alias/spec.md` 为 canonical。

## 1. 背景与目标

`openlark-communication` 的 `im/mod.rs` 自 0.15.0 起保留一个 deprecated 嵌套别名：

```rust
#[allow(clippy::module_inception)]
#[deprecated(since = "0.15.0", note = "Use `openlark_communication::im::v1` or `::v2` directly; ...")]
/// 兼容历史嵌套路径的 IM 模块别名。
pub mod im {
    pub use super::project::{v1, v2};
}
```

它造成冗余路径 `im::im::v1`（canonical 是 `im::v1`）。本 change 在 v0.18 breaking 窗口移除该别名，并把 crate 内部 47 个仍用旧路径的 `.rs` 文件迁移到 canonical 路径。

**目标**：47 文件 `im::im::` → `im::`；删除 deprecated 别名块及其依赖测试块。
**非目标**：不动 im v1/v2 实现；不重命名模块；不动 `mod project`（实际模块）；不处理其它 deprecated 项（wiki Params 已分项完成）。

## 2. 模块体系（为什么 sed 安全）

`crates/openlark-communication/src/im/mod.rs` 的关键声明：

```rust
pub mod card;
#[path = "im/mod.rs"]
mod project;                         // 物理 src/im/im/ 绑定到模块名 `project`
pub use project::{v1, v2};           // canonical: im::v1 / im::v2
...
pub mod im {                         // 【deprecated 别名】唯一产生 im::im 路径的地方
    pub use super::project::{v1, v2};
}
```

- 物理 `src/im/im/` 目录（含 `mod.rs` / `v1/` / `v2/`）通过 `#[path = "im/mod.rs"]` 绑定到模块名 **`project`**，加载 `src/im/im/mod.rs`（内含 `pub mod v1; pub mod v2;`）。
- 因此**唯一**能产生 `im::im` 路径序列的就是 deprecated `pub mod im` 别名。不存在任何真实的嵌套 `im::im` 模块。
- 结论：对源码做 `s/im::im::/im::/g` 不可能误伤——每处 `im::im::` 都指向 deprecated 别名，缩短后经 `im` 模块的 `pub use project::{v1,v2}` 解析到完全相同的目标。

## 3. 实现步骤

### 3.1 迁移 47 个 `.rs` 文件

```bash
sed -i '' 's/im::im::/im::/g' $(grep -rln 'im::im::' crates/openlark-communication/src/)
```

- 作用域**仅限** `crates/openlark-communication/src/`（实证：跨 crate / examples / tests / src 根 / 其它 crate 的 `im::im::` 及所有路径形态命中均为 0）。
- 47 文件 / 50 行命中，全部是分组 `use crate::{ ... }` 块的续行导入，形如：
  ```rust
  use crate::{
      common::api_utils::{extract_response_data, serialize_params},
      endpoints::IM_V1_CHATS,
      im::im::v1::message::models::UserIdType,   //  →  im::v1::message::models::UserIdType
  };
  ```
- 零字符串字面量 / 文档注释 / 属性 / 类型位 / 宏命中。
- 注：该模式也出现在本 change 自身的 4 个 openspec 文档（proposal/design/tasks/spec）里作为**描述性引用**，不在 sed 作用域，无需处理。

### 3.2 删除 deprecated 别名块（im/mod.rs:12-20）

整块删除（含属性与文档注释）：

| 行 | 内容 |
|----|------|
| 12 | `#[allow(clippy::module_inception)]` |
| 13-16 | `#[deprecated(since = "0.15.0", note = "...")]` |
| 17 | `/// 兼容历史嵌套路径的 IM 模块别名。` |
| 18-20 | `pub mod im { pub use super::project::{v1, v2}; }` |

> design.md 原写 `17-20`，实证修正为 **12-20**（漏计 `#[allow]` 与 `#[deprecated]`）。

### 3.3 删除依赖测试块（im/mod.rs:26-36）

整个 `#[cfg(test)] mod tests { ... }` 删除。其唯一测试 `nested_im_path_remains_a_compatibility_alias`（line 30）在 line 32 用 `super::im::v1::message::create::CreateMessageBody` 引用别名——删别名后该测试必然编译失败，必须连同删除。该测试是仓库中**唯一**引用别名的测试。

### 3.4 保留不动

`pub mod card;` / `#[path="im/mod.rs"] mod project;` / `pub use project::{v1, v2};` / `pub mod im_ephemeral;` / `pub mod im_message;`。

删除后 `im/mod.rs` 剩余：模块 doc 注释 + 上述保留声明，为合法 Rust。

## 4. Spec Patch（回写 delta spec）

回写 `specs/no-deprecated-im-alias/spec.md`，两处变更：

1. **修正 "im 别名块移除" 场景**：验证命令由 `grep 'pub mod im'` 改为 `grep -E 'pub mod im\b'`。原命令是 false-negative 陷阱——`pub mod im_ephemeral` / `pub mod im_message` 匹配 `pub mod im` 前缀，移除别名后仍命中 2 行，场景永不满足。word-boundary（`\b` 或 POSIX `-w`）精确收敛到唯别别名行。

2. **新增场景 "依赖别名测试块移除"**：`WHEN` grep `nested_im_path_remains_a_compatibility_alias` in `im/mod.rs` `THEN` 命中 0。覆盖任务缺口（测试块必须与别名同删）。

## 5. CHANGELOG

追加于 `## [Unreleased]` > `### Breaking Changes`（wiki Params 条目之后），镜像既有 v0.18 breaking house style（中文、整标签加粗、反引号、全角括号、内联 `→`、`关联 #<issue>（子项）` 尾注）：

```
- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移
  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。
```

## 6. 测试策略

| 验证 | 命令 | 期望 |
|------|------|------|
| 路径迁移完成 | `grep -rn 'im::im::' crates/openlark-communication/src/` | 0 |
| 别名块移除 | `grep -E 'pub mod im\b' crates/openlark-communication/src/im/mod.rs` | 0 |
| 测试块移除 | `grep 'nested_im_path_remains_a_compatibility_alias' crates/openlark-communication/src/im/mod.rs` | 0 |
| 三组 clippy | `cargo clippy --workspace --all-targets -D warnings`（default / `--all-features` / `--no-default-features`） | exit 0 |
| 测试 | `cargo test --workspace` | 通过 |

## 7. 风险与回滚

- **[Breaking，外部]** `openlark_communication::im::im::v1` 外部路径移除 → CHANGELOG 迁移指引（drop-in）。内部与 examples/tests 已确认零命中，影响纯外部用户。
- **回滚**：`git revert`。
