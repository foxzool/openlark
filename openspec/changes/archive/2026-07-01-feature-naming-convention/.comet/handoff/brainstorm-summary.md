# Brainstorm Summary

- Change: feature-naming-convention
- Date: 2026-07-01

## 确认的技术方案

**执行策略 A：本 change 一次性直接删除 5 crate 死版本链 + 同步 docs 下游，不走弃用别名。**

改动清单（原子、单 PR）：
1. 新增 `docs/FEATURE_NAMING_CONVENTION.md`（模块为主 + 版本为例外、三套现状、6 单版本 crate 例外、bot v4 纠正）；`AGENTS.md` CONVENTIONS 段加引用
2. 5 crate Cargo.toml 死链移除：
   - platform/analytics：删 v1/v2/v3/v4，`full` 去 v4
   - security：删 v1/v2/v3，`full` 去 v3
   - docs：删 v1/v2/v3，`full` 去 v3
   - user：删 v2/v3/v4（**保留 live v1**），`full` 去 v4
3. 下游同步：`openlark-client/Cargo.toml` `docs` feature 去 `openlark-docs/v2`、`/v3`；根 `Cargo.toml` workspace `docs` 去这两引用 + **移除 `docs-sheets-v2`/`docs-sheets-v3`**
4. Spec Patch：收窄 spec Requirement 3 范围（见下）

## 关键取舍与风险

- **BREAKING 移除公开 feature**：已核实这些 feature 门控 0 行代码、零下游消费者（仅 docs/v2、docs/v3 各 2 处下游引用）、v0.18 本就是 breaking 窗口 → 直接删安全，弃用别名只增噪音
- **引用面核实**：platform/analytics/security/user 的版本 feature 完全孤立（无下游）；docs 仅根 Cargo + openlark-client 两文件；examples/tests/.github/FEATURE_MATRIX/CI matrix 均无引用
- **`core` marker 误伤**：5 crate 的 `core` feature 在 src 门控 0 行，但被模块 feature（如 `app-engine=["app-engine-core","core"]`）引用，是合法 grouping marker → 由 Spec Patch 排除
- **下游悬空引用**：由 CI `--all-features`/`--no-default-features` 双模式兜底捕获

## 测试策略

- `just fmt` + `cargo fmt --check`
- `just lint`（CI 双模式 `--no-default-features` + `--all-features`，验证无悬空 feature 引用 —— 捕获漏改下游的关键防线）
- `just build` + `just test`

## Spec Patch

**回写 `specs/feature-naming-convention/spec.md` Requirement 3**：把「任何 Cargo feature MUST 门控实际代码」收窄为「**版本链 feature（v1/v2/v3/v4）** MUST 门控实际代码」，并显式排除 `core`/`full`/`default`/`all-*` 等合法 grouping marker feature。

**理由**：brainstorming 核实发现 `core` 在 5 crate 均 src 门控 0 行，但作为 grouping marker 被模块 feature 引用，属合法。原 spec 字面会误判这些 marker 为违规。

## 开放问题决议

- Q1（BREAKING 处理）= 直接删，不走弃用别名
- Q2（docs-sheets-v2/v3）= 直接移除（全仓无消费者）
- Q3（FEATURE_MATRIX.md 同步）= 无需同步（不列这些 feature）
