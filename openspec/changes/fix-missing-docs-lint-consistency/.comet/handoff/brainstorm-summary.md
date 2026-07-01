# Brainstorm Summary

- Change: fix-missing-docs-lint-consistency
- Date: 2026-07-01

## 确认的技术方案（最小版，待用户确认）

**3 处零风险改动，统一 missing_docs lint 执行层：**

1. **D1 just 对齐 CI**：`justfile:14` 移除 `-A missing_docs` → `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`（对齐 ci.yml:91）
2. **D2 security outlier 收编**：删 `crates/openlark-security/src/lib.rs:88` 的 `#![deny(missing_docs)]` → 回落 workspace `warn`（security 已全文档化，deny 冗余）
3. **D3 client 死注释清理**：删 `crates/openlark-client/src/lib.rs:238` 的 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译`
4. **D4 protocol 例外保留**：`crates/openlark-protocol/src/lib.rs:9` 的 `#[allow(missing_docs)]`（item 级 vendored pbbp2 生成模块，已登记例外）不动

## 关键技术事实（open 阶段核实）

- **just/CI 分裂**：`just lint`（justfile:14）`-A missing_docs` 放过；CI lint job（ci.yml:91 `RUSTFLAGS="-D warnings"`）强制 → 本地绿 CI 红
- **security deny 冗余**：security lib.rs:88 `#![deny]` 是对 workspace warn 的升级；security 以 deny 编译通过 = 全文档化 → 移除后回落 warn 仍 0 警告
- **client 死注释**：lib.rs:238 `//#![deny]` 被注释掉无作用，纯技术债
- **protocol 例外**：lib.rs:9 `#[allow]` 是 item 级（非 crate 级），`test_workspace_missing_docs.py:37` allowlist 唯一条目，保留

## 关键取舍与风险

- **范围克制（最小版）**：仅做零风险执行层对齐。analytics `#![allow]`（隐藏未文档化，移除须回补 doc）、17 个 Python 死测试（不在 CI）、codegen `-A missing_docs`、1057 占位 doc 全 Non-Goals 另案
- **不升 workspace deny**：保持 warn（已有等效强制 + 升 deny 会固化占位 doc）
- **风险极低**：3 处改动，security 移除 deny 后 missing_docs 仍 0（build 验证）

## 测试策略

- `just lint` 移除 -A 后仍过（missing_docs 现状 0）
- `cargo clippy -p openlark-security --all-features -- -Dwarnings` 过（移除 deny 后 security 0 警告）
- `cargo doc --workspace --all-features` missing_docs = 0
- clippy `--no-default-features`（CI 同款）/ build / msrv --locked 无回归
- `grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client` 无命中

## Spec Patch

`specs/lint-execution-consistency/spec.md` 的 2 Requirements（just MUST 对齐 CI / outlier 收归 workspace 单一来源）已与设计完全一致。analytics 范围边界已用边界条款诚实声明（不在本 requirement 范围）。无需 Spec Patch。

## 开放问题决议

- 无（design.md 明示：3 处改动均零风险，调研已确认事实）
