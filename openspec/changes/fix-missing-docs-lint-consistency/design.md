## Context

OpenLark 的 missing_docs lint 跨 5 层治理，但执行层不一致：

| 层 | 现状 | 一致性 |
|----|------|--------|
| `[workspace.lints.rust]`（Cargo.toml:63） | `missing_docs = "warn"` | 基线 |
| `just lint`（justfile:14） | `... -Dwarnings -A missing_docs` | ❌ 放过（与 CI 相反） |
| CI lint job（ci.yml:91） | `RUSTFLAGS="-D warnings"`，无 `-A` | ✅ 强制 |
| 源码 `#![deny]` | security(lib.rs:88)、client 死注释(lib.rs:238) | ❌ 冗余/残留 |
| 源码 `#![allow]` | analytics(lib.rs:35) | ❌ 隐藏（另案） |

净效果：「同一份代码，just 绿、CI 红」——开发者本地无法复现 CI 的 missing_docs 失败。

调研另发现：17 个 missing_docs Python 测试（`tools/tests/test_*_missing_docs.py`）**不在 CI 跑**（CI ci.yml:113 只跑 `test_check_mod_reachability`），是死测试；1057 行占位 doc（`/// 待补充文档。`）撑着 0 警告。两者均**不在本 change 范围**。

约束：MSRV 1.88；security crate 当前以 `#![deny(missing_docs)]` 编译通过（=全文档化）。

## Goals / Non-Goals

**Goals:**

- `just lint` 与 CI lint job 行为一致（移除 justfile 的 `-A missing_docs`）
- 移除 security 的冗余 `#![deny]`（回落 workspace warn）
- 移除 client 的死注释残留
- missing_docs 仍 0 警告（迁移不暴露新问题）

**Non-Goals（均另案，记为已知债务）：**

- analytics `#![allow(missing_docs)]`（lib.rs:35，隐藏未文档化项，移除须回补 doc——独立 change）
- 17 个 missing_docs Python 测试不在 CI（死测试，加入 CI 或删除——独立决策）
- codegen `tools/codegen.py:185` 的 `-A missing_docs` 闭环（codegen 范围）
- 1057 行占位 doc 治理（独立 change）
- 不升 workspace `missing_docs = "deny"`（保持 warn）

## Decisions

### D1: just lint 移除 `-A missing_docs`，对齐 CI

`justfile:14` 从 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs` 改为 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`。CI ci.yml:91 是事实标准（`RUSTFLAGS="-D warnings"` 无 `-A`），just 应对齐而非反向。移除后 `just lint` 会强制 missing_docs，但因现状 0 警告（其他 crate 全文档化 + analytics 自身 allow 压制），仍通过。

### D2: 移除 security 的 `#![deny(missing_docs)]`（回落 warn）

security lib.rs:88 的 `#![deny]` 是对 workspace `warn` 的冗余升级。security 当前以 deny 编译通过 → 全文档化 → 移除 deny 后回落 workspace warn，missing_docs 仍 0。统一到 `[workspace.lints.rust]` 单一来源。

### D3: 移除 client 死注释

client lib.rs:238 的 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译` 是历史未完成迁移的注释残留（被注释掉，无实际作用）。直接删除，清理技术债。

### D4: 保留 protocol 的 item 级例外

protocol lib.rs:9 的 `#[allow(missing_docs)]` 是 item 级（vendored pbbp2 生成模块），已登记例外（`test_workspace_missing_docs.py:37` 的 item 级 allowlist 唯一条目）。保留。

## Risks / Trade-offs

- **[移除 security deny 暴露 missing_docs]** → Mitigation：security 当前 deny 编译通过 = 全文档化，移除后 0 警告；build 阶段 `cargo doc -p openlark-security` + `cargo clippy -p openlark-security` 验证。
- **[just lint 移除 -A 后本地失败]** → Mitigation：现状 missing_docs = 0（workspace 全文档化除 analytics 自带 allow），`just lint` 移除 -A 仍通过；build 阶段验证。
- **[范围克制]**：analytics / Python 测试 / codegen / 占位 doc 全 Non-Goals，避免范围蔓延；本 change 仅做零风险执行层对齐。

## Open Questions

- 无（3 处改动均零风险，调研已确认事实）。analytics 处理方式属下一个 change 的 design 决策。
