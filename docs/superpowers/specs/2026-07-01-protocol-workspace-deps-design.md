---
comet_change: protocol-workspace-deps
role: technical-design
canonical_spec: openspec
---

# protocol-workspace-deps 技术设计

> 关联 OpenSpec change：`openspec/changes/protocol-workspace-deps/`（canonical spec 以 OpenSpec 为准）。
> 范围：issue #273 **Part B**——仅 `openlark-protocol` 的 `bytes`/`prost` 依赖迁移到 workspace。
> Part A（workspace lint 策略统一）已拆分，另开 change。

## 1. 背景与现状

OpenLark 是 18 crate workspace。依赖治理存在两种并存形态：

| 形态 | 代表 | 特征 |
|------|------|------|
| workspace 统一 | core（38 依赖走 `{ workspace = true }`） | 版本在根 `[workspace.dependencies]` 声明 |
| crate 级钉版本 | protocol（`bytes = "1.6.0"`、`prost = "0.13.1"`） | 版本钉在 crate Cargo.toml，绕过 workspace |

`prost` 已在 `[workspace.dependencies]` 声明为 `{ version = "0.13" }`，但 protocol 仍自钉 `0.13.1`（治理重复）；`bytes` 完全未进 workspace，仅 protocol 一处用。

issue #273 Part B 要求 protocol 依赖走 workspace，消除未来版本演化时的多版本共存风险，并落盘一条 `workspace-dependency-policy` capability 防复发。

## 2. 关键技术事实（brainstorming 已核实）

通过 `grep` + `Cargo.lock` 复核（2026-07-01）：

| 依赖 | 当前声明位置 | Cargo.lock resolved | 迁移后是否变化 |
|------|-------------|---------------------|----------------|
| `bytes` | protocol `1.6.0`（caret `^1.6.0`） | **1.11.1** | 否（workspace `"1.6"` 同为 `^1.6.0`） |
| `prost` | protocol `0.13.1` / workspace `0.13` | **0.13.5** | 否（两者都解析到 0.13.5） |
| `prost-build` | protocol `0.12.6`（build-dep） | 0.12.6 | **不动**（见 §5 范围外） |

**结论**：`bytes`/`prost` resolved 版本迁移后**不变** → `Cargo.lock` 不动 → **无需同步 `.github/msrv/Cargo.lock`**。

## 3. 实现方案（Approach A，2 处原子改动）

### 改动 1：根 `Cargo.toml` 新增 bytes workspace 声明

`[workspace.dependencies]` 段新增（紧邻已有 `prost = { version = "0.13" }`）：

```toml
bytes = "1.6"
```

> 用 `"1.6"`（非 `"1.6.0"`）对齐 protocol 现有 caret 范围，resolved 仍为 1.11.1，零 lockfile 影响。

### 改动 2：`crates/openlark-protocol/Cargo.toml` 改消费 workspace

```toml
# before
bytes = "1.6.0"
prost = "0.13.1"

# after
bytes = { workspace = true }
prost = { workspace = true }
```

`prost-build = "0.12.6"`（build-dependency）**保持不动**，理由见 §5。

## 4. 决策

- **D1（bytes workspace 版本 = `1.6`）**：对齐 protocol 现有 caret `^1.6.0`，保证 resolved 不变，最小化 lockfile/MSRV 影响。
- **D2（prost 复用已有 workspace 声明）**：`[workspace.dependencies]` 已有 `prost = { version = "0.13" }`，protocol 直接改 `{ workspace = true }`，同 minor 无破坏。
- **D3（MSRV lockfile 同步）**：resolved 不变 → `Cargo.lock` 不动 → **无需同步** `.github/msrv/Cargo.lock`（Q1 决议：无需做）。迁移后用 pinned lockfile 跑 `--locked` 验证应直接过。
- **D4（落盘 `workspace-dependency-policy` capability）**：对齐 `feature-naming-convention` 范式，记录「跨 crate 共享依赖 MUST 走 workspace」准则 + 判定命令（`cargo tree -d` 查多版本、grep crate 级钉版本），供后续 code-review/design-review 引用。

## 5. 范围外（诚实限制）

**prost 0.12/0.13 多版本共存本 change 消不掉**。来源：

```
crates/openlark-protocol/Cargo.toml:
  prost-build = "0.12.6"   # build-dependency，编译 vendored lark-websocket-protobuf
```

`prost-build 0.12.6` 在 build script 中拉入 `prost 0.12.6`（`Cargo.lock` 中 prost 同时存在 `0.12.6` 与 `0.13.5`）。这是 vendored protobuf crate 的 build 工具链问题，与 runtime 依赖治理是独立问题，属另一个 change 的范围。本 change **不宣称消除 prost 多版本**，只承诺**不引入新多版本**。

此限制已通过 Spec Patch 回写 delta spec（见 §7），spec 不再断言「prost 单一 resolved 版本」。

## 6. 测试策略

| 验证项 | 命令 | 预期 |
|--------|------|------|
| 格式 | `cargo fmt --check` | 过 |
| Lint 双模式 | `just lint`（`--all-features` + `--no-default-features`，`-Dwarnings`） | 双过 |
| 构建 | `cargo build --workspace --all-features` | 过 |
| **不引入新多版本** | `cargo tree -d --workspace`（迁移前后对比） | bytes 仍单版本 1.11.1；prost 0.12.6/0.13.5 split **维持原状、不新增** |
| MSRV `--locked` | 用 `.github/msrv/Cargo.lock` 跑 docker rust:1.88 | 过（Cargo.lock 不变，应直接通过） |
| 依赖图一致性 | `cargo deny check` | 无新冲突 |

> 关键断言是「**不引入新多版本**」而非「单一版本」——`cargo tree -d` 前后 diff 应为空（prost 0.12/0.13 既存 split 不计入）。

## 7. Spec Patch（已回写 delta spec）

收窄 `specs/workspace-dependency-policy/spec.md` 的「无多版本共存」Scenario——原表述过强（prost 0.12/0.13 split 本 change 消不掉）。改为：

- 「protocol 消费的 `bytes`/`prost` MUST 为 `{ workspace = true }` 形态」（既有 scenario，保留）
- 「本迁移 MUST NOT 引入 `bytes`/`prost` 的**新**多版本」（`cargo tree -d` 对比前后无新增重复条目）
- 移除「prost 单一 resolved 版本」断言（与 vendored prost-build 0.12.6 冲突，另案）

## 8. 风险与取舍

- **范围克制**：只做 protocol bytes/prost，不扩到 prost-build / 其他 crate，避免范围蔓延。
- **风险低**：resolved 版本不变，无 lockfile/MSRV 影响；范式成熟（core 38 deps 已用 workspace）。
- **诚实性**：spec 不 over-claim——迁移收益是「治理一致性 + 防未来漂移」，而非「消除现存 prost 多版本」。
