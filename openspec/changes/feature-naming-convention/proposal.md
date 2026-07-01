## Why

issue #272 指出 feature 命名方案跨 crate 不一致、存在空版本 feature。核实代码后发现 issue 的诊断部分失准，真实问题更严重：

- **三套 feature 方案并存且无文档规范**：单版本门控（ai/cardkit/application/helpdesk/mail/bot）、纯模块门控（communication/hr/meeting/auth 等）、混合（docs/platform/security/analytics/user 同时有模块 feature 和死版本链）。用户需记忆不同 crate 的规则。
- **5 个混合 crate 存在门控 0 行的死版本链**（platform/analytics 的 v1-v4、security/docs 的 v1-v3、user 的 v2-v4）：`full` 拉入这些 feature 却不门控任何代码，启用 `v4` 期望版本控制实为 no-op，误导用户。
- **issue #272 误判 `openlark-bot` 的 v4 为空**：实际 v4 门控 `pub mod bot` + `BotService::bot()`，是 live 代码，必须保留并在规范中纠正记录。

现在做是因为死版本链属 cargo-cult 遗留，趁 v0.18 breaking 窗口期清理成本最低；同时补齐规范文档防止后续 crate 继续分化。

## What Changes

- **新增** feature 命名规范文档（`docs/FEATURE_NAMING_CONVENTION.md`）与 capability spec：确立「模块 feature 为主、版本 feature 为单 API 版本 crate 的合法例外」原则，记录三套方案现状与例外清单（含 bot v4 纠正说明）。
- **移除** 5 个混合 crate 中门控 0 行的死版本链 feature：
  - `openlark-platform`：移除 `v1`/`v2`/`v3`/`v4`，`full` 去掉 `v4`
  - `openlark-analytics`：移除 `v1`/`v2`/`v3`/`v4`，`full` 去掉 `v4`
  - `openlark-security`：移除 `v1`/`v2`/`v3`，`full` 去掉 `v3`
  - `openlark-docs`：移除 `v1`/`v2`/`v3`，`full` 去掉 `v3`
  - `openlark-user`：移除 `v2`/`v3`/`v4`（保留 live 的 `v1`），`full` 去掉 `v4`
- **同步下游引用**：`openlark-client` 与根 `Cargo.toml` 中引用 `openlark-docs/v2`、`openlark-docs/v3` 的 feature 定义需更新（`docs-sheets-v2`/`docs-sheets-v3` 等）。
- **保留** 6 个单版本 crate（ai/cardkit/application/helpdesk/mail/bot）的版本 feature，作为合法例外写入规范。
- **BREAKING**：移除公开 feature（如 `openlark-docs/v2`、`openlark-docs/v3`、`openlark-platform/v4` 等）。这些 feature 当前不门控任何代码、启用后行为无变化，但仍属公开 API；design 阶段决定直接删除还是走弃用别名周期。

## Capabilities

### New Capabilities

- `feature-naming-convention`: 飞书 SDK 各 crate 的 Cargo feature 命名规范——模块 feature 为主、版本 feature 为单 API 版本 crate 的合法例外；包含三套方案现状定义、合法例外清单、死版本链判定与清理准则。

### Modified Capabilities

（无现有 spec-level 行为变更——本次清理移除的是门控 0 行的死 feature，不改变任何 crate 的编译行为或 spec 要求。）

## Impact

- **代码**：5 个 crate 的 `Cargo.toml`（platform/analytics/security/docs/user）+ 2 个下游 `Cargo.toml`（openlark-client、根 crate）。无 `src/` 逻辑变更（user/v1 保留门控）。
- **公共 API**：移除多个公开 Cargo feature 属向后兼容性变更（BREAKING）。design 阶段评估外部用户依赖面，决定直接删 vs 弃用别名。
- **文档**：新增 `docs/FEATURE_NAMING_CONVENTION.md`；视情况更新 `FEATURE_MATRIX.md` 与 `AGENTS.md` SKILLS/CONVENTIONS 段。
- **CI**：`just lint` 含 `--no-default-features` 与 `--all-features` 两种构建，须确认移除 feature 后两种模式均通过。
- **依赖/性能**：无影响。
