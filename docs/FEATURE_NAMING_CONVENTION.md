# Feature 命名规范

飞书 SDK 各 crate 的 Cargo feature 命名遵循「**模块 feature 为主、版本 feature 为单 API 版本 crate 的合法例外**」原则。本规范与 [`CLIENT_NAMING_CONVENTION.md`](./CLIENT_NAMING_CONVENTION.md) 同级，供 code-review / design-review skill 与贡献者参照。

## 原则

1. **模块 feature 为主要门控手段**：业务 crate 按子域命名 feature（如 `attendance`、`im`、`search`、`app-engine`），通过 `cfg(feature = "...")` 门控对应子模块代码。
2. **版本 feature 仅作单 API 版本 crate 的合法例外**：当一个 crate 全量围绕单一 API 版本构建，且该版本 feature **门控真实代码**时，可保留唯一版本 feature。
3. **禁止门控 0 行的死版本链**：定义了但在整个 crate `src/` 中 `cfg` 命中 0 行的版本链 feature（`v1 = [...]` → `v2 = ["v1"]` → `v3 = ["v2"]` → `v4 = ["v3"]` 这类链式占位）必须移除。

> **例外（不在死版本链约束内）**：模块分组 marker feature（`core`、`full`、`default`、`all-*` 别名）虽不直接 `cfg` 门控代码，但作为合法的 grouping/aggregation 节点被其他 feature 引用，不属死版本链。

## 当前三套方案现状

| 方案 | 代表 crate | 特征 |
|------|-----------|------|
| A. 单版本门控 | `ai` / `cardkit` / `application` / `helpdesk` / `mail` / `bot` | 唯一版本 feature（`v1` 或 `v4`）门控真实代码 |
| B. 纯模块门控 | `communication` / `hr` / `meeting` / `auth` / `core` / `client` | 按业务域命名，无版本 feature |
| C. 混合（模块 + 死版本链） | ~~`docs` / `platform` / `security` / `analytics` / `user`~~ | 模块 feature 之外曾挂门控 0 行的 `v1→v2→v3→v4` 链（**已于本 change 清理**） |

## 6 个单版本 crate 合法例外清单

以下 crate 的版本 feature **门控真实代码**，符合「版本 feature 必须门控真实代码」的合法条件，保留：

| Crate | 版本 feature | 门控的真实代码 |
|-------|-------------|---------------|
| `openlark-ai` | `v1` | ai 服务域 v1 API |
| `openlark-cardkit` | `v1` | cardkit v1 |
| `openlark-application` | `v1` | application v1 |
| `openlark-helpdesk` | `v1` | helpdesk v1 |
| `openlark-mail` | `v1` | mail v1 |
| `openlark-bot` | `v4` | `pub mod bot` + `BotService::bot()`（见下方纠正说明） |

### 关于 issue #272 对 `openlark-bot` v4 的纠正

issue #272 在诊断 feature 命名问题时，将 `openlark-bot` 的 `v4` feature 误判为「空 feature」。实际证据：

- `crates/openlark-bot/src/lib.rs:14` — `#[cfg(feature = "v4")]` 门控 `pub mod bot`
- `crates/openlark-bot/src/service.rs:20` — `#[cfg(feature = "v4")]` 门控 `BotService::bot()`

→ `openlark-bot` 的 `v4` 门控 live 代码，**非空 feature**，是合法的单版本门控例外，必须保留。

## 死版本链判定准则（供 review skill 引用）

判定一个版本 feature 是否为「死版本链」需移除：

1. 名称匹配 `vN` 形态（`v1`/`v2`/`v3`/`v4` ...）。
2. 在该 crate 的 `src/` 树中 `grep -rn 'cfg(feature = "vN")'` 命中 **0 行**。
3. 不属于单版本 crate 的合法例外（即该 crate 同时存在模块 feature）。

满足以上三条的版本 feature 必须移除，并同步下游 `Cargo.toml` 中所有引用。

判定命令（以 `openlark-platform` 为例）：

```bash
grep -rn 'cfg(feature = "v[1-4]")' crates/openlark-platform/src/
# 命中 0 行 → v1/v2/v3/v4 均为死版本链
```

对照（`openlark-user` 的 `v1` 为 live）：

```bash
grep -rn 'cfg(feature = "v1")' crates/openlark-user/src/
# 命中 4 处（preferences.rs / settings.rs）→ v1 保留，仅删 v2/v3/v4
```
