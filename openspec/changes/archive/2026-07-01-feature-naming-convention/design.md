## Context

飞书 SDK 各 crate 的 Cargo feature 长期无统一规范，演化出三套并存方案：

| 方案 | 代表 crate | 特征 |
|------|-----------|------|
| A. 单版本门控 | ai/cardkit/application/helpdesk/mail/bot | 唯一版本 feature（`v1` 或 `v4`）门控真代码，无 v2 可选 |
| B. 纯模块门控 | communication/hr/meeting/auth/core/client | 按业务域命名，无版本 feature |
| C. 混合（模块 + 死版本链） | docs/platform/security/analytics/user | 模块 feature 之外还挂了门控 0 行的 `v1→v2→v3→v4` 链 |

issue #272 诊断部分失准：把 live 的 `openlark-bot` v4 误判为空，且只点了 platform/bot 两个空 feature，实际 5 个混合 crate 都有死版本链。本 change 据实纠正并清理。

约束：MSRV 1.88；CI lint 跑 `--no-default-features` 与 `--all-features` 两种模式；AGENTS.md 要求不破坏向后兼容性（公开 feature 亦是公开 API）。

## Goals / Non-Goals

**Goals:**

- 建立 feature 命名规范（模块为主、版本为例外），落盘 `docs/FEATURE_NAMING_CONVENTION.md` 与 capability spec
- 移除 5 个混合 crate 门控 0 行的死版本链，同步所有下游 `Cargo.toml` 引用
- 纠正 issue #272 对 `bot` v4 的误判记录

**Non-Goals:**

- 不动 6 个单版本 crate 的版本 feature（保留为合法例外）
- 不把三套方案统一/重命名为单一方案
- 不改变任何 crate 的实际编译行为（死链本就不门控代码）
- 不新增 API、不重命名 feature（除移除死链外）

## Decisions

### D1: 规范力度 = 文档 + 清理死链

**选择**：既写规范文档，也清理违规死链。**理由**：仅写文档不清理，死版本链继续误导用户（启用 `v4` 期望版本控制实为 no-op）；文档化现状才有判定准则。**备选**：仅文档（零风险但留垃圾）/ 全面标准化（重命名统一，breaking 面太大且非必要）。

### D2: 死链处理 = 全部移除（含 docs，同步下游）

**选择**：5 个混合 crate 的死版本链 feature 一律移除，docs 同步更新 `openlark-client` 与根 `Cargo.toml`。**理由**：死 feature 门控 0 行代码，移除是纯清理无行为变化；保留只会持续误导。**备选**：改 `reserved-vN` 占位（保留扩展位但仍是门控 0 行的噪音）/ 仅文档标注（不清理）。

### D3: 单版本 crate = 保留现状

**选择**：ai/cardkit/application/helpdesk/mail/bot 的版本 feature 保留，作为合法例外写入规范。**理由**：这些 feature 门控真实代码且反映服务端 API 版本（bot=v4 搜索、其余=v1），符合「版本 feature 门控真代码」的合法条件。**备选**：移除冗余版本门控让代码常开（改 src + Cargo，范围大且丢失版本语义）。

### D4: 规范文档位置 = `docs/FEATURE_NAMING_CONVENTION.md`

**选择**：与现有 `docs/CLIENT_NAMING_CONVENTION.md` 同目录、同命名风格。**理由**：保持文档发现一致性，便于在 AGENTS.md CONVENTIONS 段统一引用。

## Risks / Trade-offs

- **[移除公开 feature 属 BREAKING]** → 这些 feature（`openlark-docs/v2`、`openlark-platform/v4` 等）门控 0 行代码，启用后行为无变化，外部用户实际无法依赖其产生任何编译差异；但仍属公开 API。Mitigation：design 阶段评估是否需弃用别名周期（见开放问题 Q1）。
- **[docs 下游引用遗漏]** → 移除 `openlark-docs/v2`/`v3` 后若漏改下游 feature 定义，`cargo build` 会因悬空 feature 引用直接失败。Mitigation：tasks 中显式列出下游同步项 + CI `--all-features`/`--no-default-features` 双模式验证。
- **[规范与后续 crate 漂移]** → 新 crate 可能再次引入死版本链。Mitigation：spec 落盘为持久约束 + 文档给出判定准则，后续 code-review/design-review skill 可引用。

## Open Questions

- **Q1（BREAKING 处理）**：被移除的版本 feature 是否需要先走一个弃用别名周期（如保留 `openlark-docs/v2` 为空别名 + `#[deprecated]` 等价物）再删？还是鉴于 v0.18 本就是 breaking 窗口、且这些 feature 无实际编译效果，直接删？→ `/comet-design` 阶段 brainstorming 决定。
- **Q2（docs-sheets-v2/v3 去留）**：根 `Cargo.toml` 的 `docs-sheets-v2`/`docs-sheets-v3` feature 移除 `openlark-docs/v2`/`v3` 引用后，是退化为 `docs-ccm` 别名保留，还是直接移除这两个根 feature？→ design 阶段决定。
- **Q3（FEATURE_MATRIX.md 同步）**：`docs/FEATURE_MATRIX.md` 是否需要同步更新以反映移除的 feature？→ build 阶段核实。
