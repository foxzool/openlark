---
change: remove-deprecated-im-alias
design-doc: docs/superpowers/specs/2026-06-28-remove-deprecated-im-alias-design.md
base-ref: 9aa40f87804961d92f81a63bc6edd9349dd34da0
---

# remove-deprecated-im-alias 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal**：迁移 `openlark-communication` 内 47 文件的 `im::im::` 旧嵌套路径到 canonical `im::`，并删除 `im/mod.rs` 的 deprecated `pub mod im` 别名块与依赖该别名的测试块。BREAKING。

**Architecture**：纯机械迁移 + 删除。无新逻辑、无新 API、无新测试。`sed` 把 47 文件里的 `im::im::` 缩短为 `im::`（解析目标不变，因 `im` 模块已有 `pub use project::{v1,v2}`）；随后整块删除 `im/mod.rs:12-20` 的 deprecated 别名与其唯一依赖测试块 `im/mod.rs:26-36`。

**Tech Stack**：Rust（无依赖变更），BSD `sed -i ''`（macOS），`grep -E` word-boundary。

## Global Constraints

- 作用域**仅限** `crates/openlark-communication/src/`。零跨 crate / examples / tests / 其它 crate 命中（design 实证）。
- **保留不动**：`pub mod card;`（`im/mod.rs:7`）/ `#[path="im/mod.rs"] mod project;`（line 8-9）/ `pub use project::{v1, v2};`（line 10-11）/ `pub mod im_ephemeral;`（line 21-22）/ `pub mod im_message;`（line 23-24）。
- 物理目录 `src/im/im/`（含 `mod.rs`/`v1/`/`v2/`）通过 `#[path]` 绑定到模块名 `project`——**不是**真实的 `im::im`。这是为何 `s/im::im::/im::/g` 不可能误伤：仓库里每处 `im::im::` 字面都指向 deprecated 别名。
- 删除前 `im/mod.rs` 共 37 行；删除别名块（line 12-20）+ 测试块（line 26-36）后剩约 24 行，仍为合法 Rust。
- 本计划不写源代码——所有源码改动是 `sed` 替换 + 整块删除（无手写新行）。
- Commit 风格沿用本仓库 conventional commits（`refactor:` / `chore:` / `docs(changelog):`）。

---

## Task 1: 迁移 47 文件导入路径

**Files:**
- Modify（机械替换，无手写）: `crates/openlark-communication/src/` 下 47 个 `.rs` 文件（含 `src/im/im/...` 即 `mod project` 物理目录的文件）。

**Interfaces:**
- Consumes: `im` 模块的 `pub use project::{v1, v2}`（line 10-11，已存在，本任务不动）
- Produces: 47 文件 `use crate::{ ..., im::v1::... / im::v2::... }`，删除别名后仍解析到同一目标

- [x] **Step 1: 迁移路径**

运行（design §3.1 定稿命令，BSD sed 语法）：

```bash
sed -i '' 's/im::im::/im::/g' $(grep -rln 'im::im::' crates/openlark-communication/src/)
```

预期：47 文件被改写，每文件 1 处（少数 2 处）。形如 `im::im::v1::message::models::UserIdType,` → `im::v1::message::models::UserIdType,`（仅分组 `use crate::{ ... }` 续行导入，零字符串字面量/文档/属性/类型位命中）。

- [x] **Step 2: 验证迁移完成**

```bash
grep -rn 'im::im::' crates/openlark-communication/src/
```

预期：无输出（exit 1，0 命中）。

- [x] **Step 3: 此时不提交（编译会失败）**

此时 `im/mod.rs` 的 deprecated 别名仍在，仓库能编译；但为保持变更原子性（迁移 + 删别名同一逻辑单元），不在中途 commit。继续 Task 2。

## Task 2: 删除 im 别名块 + 依赖测试块

**Files:**
- Modify（整块删除，无手写新行）: `crates/openlark-communication/src/im/mod.rs:12-20` 和 `:26-36`

**Interfaces:**
- Consumes: Task 1 的路径迁移（迁移后即使删别名，所有内部导入仍解析到 `im::v1`/`im::v2`）
- Produces: `im/mod.rs` 不再含 `pub mod im` 别名与依赖测试，满足 spec scenario「im 别名块移除」+「依赖别名测试块移除」

- [x] **Step 1: 删除 deprecated 别名块（line 12-20）**

删除 `crates/openlark-communication/src/im/mod.rs` 第 12-20 行整块：

```rust
#[allow(clippy::module_inception)]
#[deprecated(
    since = "0.15.0",
    note = "Use `openlark_communication::im::v1` or `::v2` directly; the nested `im::im` path is a legacy compatibility alias."
)]
/// 兼容历史嵌套路径的 IM 模块别名。
pub mod im {
    pub use super::project::{v1, v2};
}
```

> 即 line 11 `pub use project::{v1, v2};` 之后直接接 line 21 `/// Im Ephemeral 模块。`，中间空一行。保留 line 7-11（card / project / canonical re-export）与 line 21-24（im_ephemeral / im_message）不动。

- [x] **Step 2: 删除依赖测试块（line 26-36，删除 line 26-36 原 37 行文件中现已是新行号）**

删除整个 `#[cfg(test)] mod tests { ... }` 块：

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[allow(deprecated)]
    fn nested_im_path_remains_a_compatibility_alias() {
        let canonical = std::any::type_name::<super::v1::message::create::CreateMessageBody>();
        let legacy = std::any::type_name::<super::im::v1::message::create::CreateMessageBody>();

        assert_eq!(canonical, legacy);
    }
}
```

> 该测试 line 32 用 `super::im::v1::...` 引用别名——删别名后必编译失败，必须同删。它是仓库中**唯一**引用别名的测试。

- [x] **Step 3: 验证两块均已移除**

```bash
grep -E 'pub mod im\b' crates/openlark-communication/src/im/mod.rs
grep 'nested_im_path_remains_a_compatibility_alias' crates/openlark-communication/src/im/mod.rs
```

预期：两条命令均无输出（exit 1，0 命中）。

> 注：必须用 `grep -E 'pub mod im\b'`（word-boundary）。裸 `grep 'pub mod im'` 会匹配保留的 `pub mod im_ephemeral` / `pub mod im_message` 前缀，永远 >0，是 false-negative 陷阱（spec scenario「im 别名块移除」明示）。

- [x] **Step 4: 提交 Task 1 + Task 2（原子单元）**

```bash
git add crates/openlark-communication/src/
git commit -m "refactor(communication)!: remove deprecated im::im nested alias

47 files migrated im::im:: -> im:: (crates/openlark-communication/src/).
Removed pub mod im alias block (im/mod.rs:12-20) and its sole dependent
test nested_im_path_remains_a_compatibility_alias (im/mod.rs:26-36).

BREAKING CHANGE: openlark_communication::im::im::v1/v2 removed (deprecated
since 0.15.0). Migrate to im::v1 / im::v2. See CHANGELOG.

Refs #278 (F)"
```

## Task 3: 验证构建与测试

**Files:**
- 无文件改动（仅运行验证命令，对应 spec scenario「三组 feature clippy 通过」+「tests 通过」）

- [x] **Step 1: 三组 feature clippy（default）**

```bash
cargo clippy --workspace --all-targets -D warnings
```

预期：exit 0，无 warning。default features = `auth`。

- [x] **Step 2: 三组 feature clippy（--all-features）**

```bash
cargo clippy --workspace --all-targets --all-features -D warnings
```

预期：exit 0。

- [x] **Step 3: 三组 feature clippy（--no-default-features）**

```bash
cargo clippy --workspace --all-targets --no-default-features -D warnings
```

预期：exit 0。

- [x] **Step 4: 测试**

```bash
cargo test --workspace
```

预期：全部通过。

- [x] **Step 5: 若任一 clippy/test 失败，加载 systematic-debugging skill**

失败时不要手改源码补丁——根因大概率是漏迁移的某文件（grep 应已为 0，但若外部 examples/tests 命中需补查）。加载 superpowers:systematic-debugging 定位根因后修复，再回 Step 1。

## Task 4: CHANGELOG

**Files:**
- Modify: `CHANGELOG.md`（追加于 `## [Unreleased]` > `### Breaking Changes`，wiki Params 条目之后、docs deprecated 条目之前）

**Interfaces:**
- 镜像既有 v0.18 breaking house style（中文、整标签加粗、反引号、全角括号、内联 `→`、`关联 #<issue>（子项）` 尾注）

- [x] **Step 1: 插入条目**

在 `CHANGELOG.md` 第 24 行（wiki Params 块结束：`...一并删除）。关联 #268（B）。`）之后、第 26 行（`- **Removed docs deprecated 方法**...`）之前，插入空行 + 新条目。最终段落为：

```markdown

- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移
  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。
```

- [x] **Step 2: 提交 CHANGELOG**

```bash
git add CHANGELOG.md
git commit -m "docs(changelog): note im::im nested alias removal (#278 F)"
```

## Self-Review 校验

完成所有 Task 后回看 spec 每条 scenario：

| spec scenario | 覆盖 task | 验证命令 |
|---|---|---|
| im 别名块移除 | Task 2 Step 1+3 | `grep -E 'pub mod im\b' im/mod.rs` = 0 |
| 依赖别名测试块移除 | Task 2 Step 2+3 | `grep 'nested_im_path_remains_a_compatibility_alias' im/mod.rs` = 0 |
| 内部导入路径迁移 | Task 1 Step 1+2 | `grep -rn 'im::im::' crates/openlark-communication/src/` = 0 |
| 三组 feature clippy 通过 | Task 3 Step 1-3 | 三组 exit 0 |
| tests 通过 | Task 3 Step 4 | 全通过 |

无 spec 项缺 task；无 placeholder；类型/路径一致（`im::v1`/`im::v2` 全文统一）。
