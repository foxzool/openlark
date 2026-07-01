# Brainstorm Summary

- Change: protocol-workspace-deps
- Date: 2026-07-01

## 确认的技术方案（待用户确认）

**Approach A：单 PR，protocol bytes/prost 走 workspace。**

改动（原子）：
1. 根 `Cargo.toml [workspace.dependencies]` 新增 `bytes = "1.6"`（prost 已有 `0.13`，不动）
2. `openlark-protocol/Cargo.toml`：`bytes = "1.6.0"` → `bytes = { workspace = true }`；`prost = "0.13.1"` → `prost = { workspace = true }`
3. **不动 lockfile**（resolved 版本不变，见下）→ 无需同步 `.github/msrv/Cargo.lock`
4. 不动 `prost-build`（0.12.6，属 vendored lark-websocket-protobuf 的 0.12 多版本问题，另案）

## 关键技术事实（brainstorm 核实）

- **bytes**：Cargo.lock resolved = **1.11.1**（protocol 钉 "1.6.0" 是 caret `^1.6.0`，实际 1.11.1）。workspace "1.6" 同为 `^1.6.0` → resolved 不变。
- **prost**：Cargo.lock resolved = **0.13.5**（protocol "0.13.1" 与 workspace "0.13" 都解析到 0.13.5）。→ resolved 不变。
- → **Cargo.lock 不变** → Q1（msrv lockfile 同步）**无需做**。
- **prost 0.12.6 多版本**：由 vendored `lark-websocket-protobuf` 的 `prost-build 0.12.6` build-dep 引入（runtime 用 0.13.5、build 用 0.12.6）。本 change 消不掉，属独立问题，出范围。

## 关键取舍与风险

- **范围克制**：只做 issue #273 Part B 的 protocol bytes/prost，不扩到 prost-build / 其他 crate（避免范围蔓延）。
- **诚实限制**：spec 不得宣称「消除 prost 多版本」——0.12/0.13 split 由 vendored crate 引起，另案。
- **风险低**：resolved 版本不变，无 lockfile/MSRV 影响；范式成熟（core 38 deps 已用 workspace）。

## 测试策略

- `cargo fmt --check`
- `just lint`（CI 双模式）
- `cargo build --workspace --all-features`
- `cargo tree -d`：确认本迁移**不引入新多版本**（bytes 仍单版本 1.11.1；prost 0.12.6/0.13.5 split 维持原状，不新增）
- msrv `--locked`：Cargo.lock 不变，应直接过
- `cargo deny check`

## Spec Patch（将回写 delta spec）

收窄 `specs/workspace-dependency-policy/spec.md` 的「无多版本共存」Scenario——原表述过强（prost 0.12/0.13 split 本 change 消不掉）。改为：
- 「protocol 消费的 bytes/prost MUST 为 `{ workspace = true }` 形态」
- 「本迁移 MUST NOT 引入 bytes/prost 的新多版本」（`cargo tree -d` 对比前后无新增重复条目）
- 移除「prost 单一 resolved 版本」断言（与 vendored lark-websocket-protobuf 的 prost-build 0.12.6 冲突，另案处理）

## 开放问题决议

- Q1（.github/msrv/Cargo.lock 同步方式）= **无需同步**（resolved 版本不变，lockfile 不动）
