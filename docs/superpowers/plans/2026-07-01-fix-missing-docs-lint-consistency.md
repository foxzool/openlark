---
change: fix-missing-docs-lint-consistency
design-doc: docs/superpowers/specs/2026-07-01-fix-missing-docs-lint-consistency-design.md
base-ref: ba071decc3ce472220c876b2bd7d3112a1dc2c72
archived-with: 2026-07-01-fix-missing-docs-lint-consistency
---

# fix-missing-docs-lint-consistency 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 消除「同一份代码 just 绿、CI 红」的 missing_docs lint 执行不一致，收编源码 deny 死代码。

**Architecture:** 3 处零风险编辑（justfile:14 移除 `-A missing_docs`、security lib.rs:88 删 deny、client lib.rs:238 删死注释）+ 1 处确认保留（protocol item 级 allow）。所有改动通过单一 lint 治理点 `[workspace.lints.rust] missing_docs = "warn"` 收敛，对齐 CI `RUSTFLAGS="-D warnings"` 事实标准。

**Tech Stack:** Rust workspace lints、Cargo clippy、just

## Global Constraints

- **范围严格限定**：只动 4 个文件，不做 doc 回补、不升 workspace 到 deny、不动 analytics（另案）
- **零行为变化**：移除 `-A missing_docs` 后 `just lint` 必须仍通过（missing_docs 现状 0 警告）
- **单一治理点**：所有 crate 统一回落到 `[workspace.lints.rust] missing_docs = "warn"`，不留 crate 级 deny 残留
- **MSRV**：Rust 1.88（`--locked` 用 `.github/msrv/Cargo.lock`）
- **不新增依赖**：纯配置与注释编辑

> 这是 small / 零风险变更。改动局限于 1 行 justfile recipe + 1 行 deny 删除 + 1 行死注释删除。每个 Task 独立可测、可提交。

archived-with: 2026-07-01-fix-missing-docs-lint-consistency
---

### Task 1: just lint 对齐 CI（D1）

**Files:**
- Modify: `justfile:14`

**Interfaces:**
- Consumes: CI `.github/workflows/ci.yml:91` 的 `RUSTFLAGS="-D warnings"` 事实标准
- Produces: `just lint` recipe 与 CI 行为一致（无 `-A missing_docs` 放过）

- [x] **Step 1: 编辑 justfile:14 移除 `-A missing_docs`**

将 `justfile` 第 14 行：

```just
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs
```

改为：

```just
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings
```

完整 recipe（修改后）：

```just
# Lint code
lint:
  @echo "🔍 Linting code (exclude benches/dev-tests)..."
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings
```

- [x] **Step 2: 跑 just lint 确认仍通过**

Run: `just lint`
Expected: `Finished` + exit 0（missing_docs 现状 0 警告，移除 `-A` 不破坏）

> 若失败：说明 workspace 存在未文档化项（不在本 change 范围），停止并回报失败项，不要回补 doc。

- [x] **Step 3: 提交**

```bash
git add justfile
git commit -m "fix(lint): just lint 移除 -A missing_docs 对齐 CI（issue #273 Part A1 D1）

CI ci.yml:91 RUSTFLAGS=\"-D warnings\" 是事实标准，just recipe 之前带
-A missing_docs 放过导致本地绿 CI 红。移除后统一到单一治理点。"
```

archived-with: 2026-07-01-fix-missing-docs-lint-consistency
---

### Task 2: 源码 outlier 清理（D2 + D3，D4 确认保留）

**Files:**
- Modify: `crates/openlark-security/src/lib.rs:88`
- Modify: `crates/openlark-client/src/lib.rs:238`
- Confirm: `crates/openlark-protocol/src/lib.rs:9`（不动）

**Interfaces:**
- Consumes: `[workspace.lints.rust] missing_docs = "warn"` 基线（回落后生效）
- Produces: workspace 内无 crate 级 `deny(missing_docs)` 残留；`grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client` 命中为空

- [x] **Step 1: 删除 security lib.rs:88 的 `#![deny(missing_docs)]`**

将 `crates/openlark-security/src/lib.rs:88`：

```rust
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
```

改为（删第 88 行，保留后续 3 行）：

```rust
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
```

> 依据：security 当前以 deny 编译通过 = 全文档化，回落 workspace warn 后 missing_docs 仍 0 警告。

- [x] **Step 2: 删除 client lib.rs:238 的死注释**

删除 `crates/openlark-client/src/lib.rs:238` 整行：

```rust
//#![deny(missing_docs)]  // 暂时禁用以完成基本编译
```

删除后该位置上下文（`//! ``` 闭合 → 空行 → `// 核心模块`）：

```rust
//! ```
```

（紧接原本的第 239 空行与第 240 行 `// 核心模块`，不留多余空行）

- [x] **Step 3: 确认 protocol lib.rs:9 的 item 级 allow 保留不动**

读取确认 `crates/openlark-protocol/src/lib.rs:9` 内容（**不修改**）：

```rust
#[allow(missing_docs)]
pub mod pbbp2 {
    include!(concat!(env!("OUT_DIR"), "/pbbp2.rs"));
}
```

这是 item 级（vendored 生成模块）例外，是 `test_workspace_missing_docs.py:37` allowlist 唯一条目，保留。本步骤只做"确认未误改"的读取，无编辑动作。

- [x] **Step 4: 核心断言——security 移除 deny 后 0 警告**

Run: `cargo clippy -p openlark-security --all-features -- -Dwarnings`
Expected: `Finished` + exit 0（核心断言：全文档化，回落 warn 后仍过）

> 若失败：说明 security 存在未文档化项（与 design D2 假设矛盾），停止并回报失败项，**不要回补 doc**（回补是 analytics 另案的范畴，security 应已全文档化）。

- [x] **Step 5: outlier 清理结果校验**

Run: `grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client`
Expected: 无任何输出（命中为空）

- [x] **Step 6: 提交**

```bash
git add crates/openlark-security/src/lib.rs crates/openlark-client/src/lib.rs
git commit -m "fix(lint): 移除 security deny/client 死注释，统一回落 workspace warn（issue #273 Part A1 D2+D3）

- security lib.rs:88 #![deny(missing_docs)] 冗余（workspace 已 warn），删除
- client lib.rs:238 死注释 //#![deny(missing_docs)] 删除（被注释无作用，技术债）
- protocol lib.rs:9 item 级 #[allow] 保留（vendored 例外，allowlist 登记）"
```

archived-with: 2026-07-01-fix-missing-docs-lint-consistency
---

### Task 3: 完整验证（对齐 design 第 5 节测试矩阵）

**Files:**
- 无编辑，仅运行验证命令

**Interfaces:**
- Consumes: Task 1 + Task 2 的全部改动
- Produces: change 可进入 verify 阶段的证据（所有验证项 PASS）

- [x] **Step 1: cargo fmt --check**

Run: `cargo fmt --check`
Expected: exit 0（无格式差异）

> 依据 MEMORY：CI lint 第一步是 `cargo fmt --check`，clippy 通过 ≠ fmt 通过。

- [x] **Step 2: cargo doc —— missing_docs warning = 0**

Run: `cargo doc --workspace --all-features 2>&1 | grep -i "missing_docs" || echo "NO missing_docs WARNING"`
Expected: 输出 `NO missing_docs WARNING`（deny/warn 不变，0 警告）

- [x] **Step 3: cargo clippy -p openlark-security —— deny 移除后仍过**

Run: `cargo clippy -p openlark-security --all-features -- -Dwarnings`
Expected: `Finished` + exit 0

- [x] **Step 4: just lint —— 移除 -A 后与 CI 一致**

Run: `just lint`
Expected: `Finished` + exit 0

- [x] **Step 5: cargo clippy ndf（CI 同款）**

Run: `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
Expected: `Finished` + exit 0

> 依据 MEMORY：CI lint 跑 `--no-default-features`，cfg(feature) 门控方法的测试也要 cfg(feature)，本地 `--all-features` 会掩盖。

- [x] **Step 6: cargo build --workspace --all-features**

Run: `cargo build --workspace --all-features`
Expected: `Finished` + exit 0

- [x] **Step 7: MSRV --locked 验证（pinned lockfile）**

Run: `cargo +1.88 check --locked`（先用 `cp .github/msrv/Cargo.lock Cargo.lock` 覆盖仓库 lock，验完恢复）
Expected: `Finished` + exit 0（无回归）

> 依据 MEMORY：CI msrv job 用 `.github/msrv/Cargo.lock`（pinned）。本 change 纯编辑无依赖变化，msrv 应无回归；但按 design 第 5 节矩阵仍需显式验证。

- [x] **Step 8: 最终 outlier 复核**

Run: `grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client`
Expected: 无输出（空）

- [x] **Step 9: 提交验证记录（可选）**

本 Task 无源码改动，无需 commit。如需记录验证通过，更新 `openspec/changes/fix-missing-docs-lint-consistency/tasks.md` 勾选状态并提交：

```bash
git add openspec/changes/fix-missing-docs-lint-consistency/tasks.md
git commit -m "chore(verify): fix-missing-docs-lint-consistency 全验证通过"
```

archived-with: 2026-07-01-fix-missing-docs-lint-consistency
---

## Self-Review

**1. Spec coverage**：design 4 项决策（D1 just 对齐 / D2 security deny 删除 / D3 client 死注释删除 / D4 protocol allow 保留）→ Task 1 覆盖 D1，Task 2 覆盖 D2+D3+D4，Task 3 覆盖 design 第 5 节全部 8 项测试矩阵。OpenSpec tasks.md 三组（1.1-1.2 / 2.1-2.3 / 3.1-3.8）逐条对应 Step。无遗漏。

**2. Placeholder scan**：所有 Step 含具体命令、expected、从源文件实际读取的 before/after 代码块。无 TBD/TODO/"适当处理"。

**3. Type consistency**：本 change 无类型/签名变更（纯 lint 配置 + 注释），不适用。

**4. 风险复核**：
- Task 1 Step 2 失败路径已标注（停止回报，不回补 doc）
- Task 2 Step 4 核心断言失败路径已标注（与 D2 假设矛盾时停止）
- Task 3 全部为只读验证命令，零风险
