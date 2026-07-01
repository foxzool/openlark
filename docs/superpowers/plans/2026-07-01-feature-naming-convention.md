---
change: feature-naming-convention
design-doc: docs/superpowers/specs/2026-07-01-feature-naming-convention-design.md
base-ref: 5d03e732563d1aba6772d328cda3d621bbed9176
---

# Feature 命名规范与死版本链清理 — 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** 建立飞书 SDK feature 命名规范文档，并移除 5 个混合 crate 中门控 0 行的死版本链 feature 及其下游引用。

**Architecture:** 纯 `Cargo.toml [features]` 段 + 文档清理，无 `src/` 业务逻辑变更、无可测试行为变化。死版本链 feature（`v1`/`v2`/`v3`/`v4`）在各 crate 的 `src/` 中 `cfg(feature=...)` 命中 0 行，移除后编译产物无差异。

**Tech Stack:** Rust + Cargo `[features]`、TOML、Markdown 文档。MSRV 1.88。

## Global Constraints

- **TDD 不适用**：本 change 是 Cargo.toml 配置 + 文档清理，无可测试的新行为或代码逻辑（design doc §6 验证策略以 fmt/lint/build/test 兜底，无单元测试可写）。所有任务采用 **direct 实现**，不写 failing test → 不适用红绿循环。
- **保留 `core = []` marker**：5 个混合 crate 的 `core = []` feature 字面看门控 0 行，但被模块 feature 引用（如 platform：`app-engine = ["app-engine-core", "core"]`），是合法 grouping marker。**严禁动 `core`**（design doc D1）。
- **保留 `openlark-user` 的 `v1`**：`v1` 门控 live 代码（`crates/openlark-user/src/` 中 `cfg(feature = "v1")` 命中 4 处：`preferences.rs`/`settings.rs`）。user 仅删 `v2`/`v3`/`v4`（design doc D3）。
- **保留 6 个单版本 crate**（ai/cardkit/application/helpdesk/mail/bot）的版本 feature 不动（合法例外）。
- **仅改 `[features]` 段**：不动依赖版本，不触发 `.github/msrv/Cargo.lock` 同步（design doc §6）。
- **BREAKING 直接删**：不走弃用别名周期。被移除的 feature 门控 0 行、零下游消费者（除 docs v2/v3 各 2 处引用），直接删安全（design doc D4）。
- **中文文档**：项目约定文档/注释中文优先（AGENTS.md CONVENTIONS），所有新增 Markdown 内容用中文。
- **验证必须双模式**：CI lint 跑 `--no-default-features` 与 `--all-features` 两种，漏改下游悬空引用会被 `--all-features` 捕获。验收前 `just lint` 必须两种模式都过。

## File Structure

| 文件 | 操作 | 责任 |
|------|------|------|
| `docs/FEATURE_NAMING_CONVENTION.md` | 新建 | feature 命名规范主文档（与 `docs/CLIENT_NAMING_CONVENTION.md` 同目录同风格） |
| `AGENTS.md` | 修改 | CONVENTIONS 段补一行指向规范文档 |
| `crates/openlark-platform/Cargo.toml` | 修改 | 移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"` |
| `crates/openlark-analytics/Cargo.toml` | 修改 | 移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"` |
| `crates/openlark-security/Cargo.toml` | 修改 | 移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"` |
| `crates/openlark-docs/Cargo.toml` | 修改 | 移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"` |
| `crates/openlark-user/Cargo.toml` | 修改 | 移除 `v2`/`v3`/`v4` 定义（**保留 `v1`**），`full` 去掉 `"v4"` |
| `crates/openlark-client/Cargo.toml` | 修改 | `docs` feature 数组移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` |
| `Cargo.toml`（根） | 修改 | `docs-ccm` 数组移除 v2/v3 两行 + 移除 `docs-sheets-v2`/`docs-sheets-v3` 整个定义 |

---

### Task 1: 落盘 feature 命名规范文档

**Files:**
- Create: `docs/FEATURE_NAMING_CONVENTION.md`

**Interfaces:**
- Produces: `docs/FEATURE_NAMING_CONVENTION.md` — 规范主文档，被 Task 2 的 AGENTS.md 引用、被 spec.md（已存在）的 Scenario "规范文档存在且内容完整" 验收。内容必须包含：模块 feature 为主原则、版本 feature 合法例外判定条件、三套方案（A/B/C）现状清单、6 个单版本 crate 例外清单、issue #272 对 bot v4 误判的显式纠正、死版本链判定准则。

- [x] **Step 1: 创建规范文档**

写入 `docs/FEATURE_NAMING_CONVENTION.md`，内容须包含以下章节（中文）：

```markdown
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
```

- [x] **Step 2: 验证文档已创建且内容完整**

Run: `test -f docs/FEATURE_NAMING_CONVENTION.md && grep -c 'openlark-bot' docs/FEATURE_NAMING_CONVENTION.md`
Expected: 文件存在且 `openlark-bot` 出现次数 ≥ 2（例外清单 + 纠正段落）。

- [x] **Step 3: Commit**

```bash
git add docs/FEATURE_NAMING_CONVENTION.md
git commit -m "docs(feature-naming): 落盘 feature 命名规范文档

- 模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外原则
- 三套方案（A 单版本/B 纯模块/C 混合）现状清单
- 6 个单版本 crate 合法例外清单（含门控真实代码证据）
- 显式纠正 issue #272 对 openlark-bot v4 的误判（v4 门控 live 代码）
- 死版本链判定准则，供 code-review/design-review skill 引用"
```

---

### Task 2: AGENTS.md CONVENTIONS 段补引用

**Files:**
- Modify: `AGENTS.md:81`（CONVENTIONS 段，"Client 类型" 行之后追加一行）

**Interfaces:**
- Consumes: Task 1 产出的 `docs/FEATURE_NAMING_CONVENTION.md`。
- Produces: AGENTS.md 多一行 `**Feature 命名**` 指向规范文档，与既有 `**Client 类型**` 行（`docs/CLIENT_NAMING_CONVENTION.md`）同风格。

- [x] **Step 1: 在 AGENTS.md CONVENTIONS 段追加一行**

在 `AGENTS.md` 第 81 行（`**Client 类型**: 所有业务 crate 导出 ...` 行）之后，追加：

```markdown
- **Feature 命名**: 模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外（详见 `docs/FEATURE_NAMING_CONVENTION.md`）
```

精确插入位置（old → new）：

```
- **Feature flags**: 短横线连接的小写（`core-services`, `cloud-docs`）
- **Client 类型**: 所有业务 crate 导出 `XxxClient` 类型（详见 `docs/CLIENT_NAMING_CONVENTION.md`）
- **Feature 命名**: 模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外（详见 `docs/FEATURE_NAMING_CONVENTION.md`）
```

- [x] **Step 2: 验证引用已添加**

Run: `grep -c 'FEATURE_NAMING_CONVENTION.md' AGENTS.md`
Expected: `1`

- [x] **Step 3: Commit**

```bash
git add AGENTS.md
git commit -m "docs(feature-naming): AGENTS.md CONVENTIONS 段引用 feature 命名规范"
```

---

### Task 3: 移除 openlark-platform 死版本链

**Files:**
- Modify: `crates/openlark-platform/Cargo.toml`（第 53-57 行 `# === API版本支持 ===` 段 + 第 61 行 `full` 数组）

**Interfaces:**
- Consumes: 无（独立 crate）。
- Produces: `crates/openlark-platform/Cargo.toml` 不再定义 `v1`/`v2`/`v3`/`v4`；`full` 不再引用 `"v4"`。`core = []` 与全部模块 feature（`app-engine`/`directory`/`admin`/`mdm`/`tenant`/`trust_party`/`spark`）保留。

- [x] **Step 1: 移除版本链定义段**

删除 `crates/openlark-platform/Cargo.toml` 中以下 5 行（含注释行）：

```toml
# === API版本支持 ===
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
v4 = ["v3"]
```

连同前面的空行一并删除，使 `core = []` 之后直接接 `# === 功能组合 ===` 段。

- [x] **Step 2: 从 `full` 数组去掉 `"v4"`**

修改 `crates/openlark-platform/Cargo.toml`（原第 61 行）：

old:
```toml
full = ["app-engine", "directory", "admin", "mdm", "tenant", "trust_party", "spark", "v4"]
```
new:
```toml
full = ["app-engine", "directory", "admin", "mdm", "tenant", "trust_party", "spark"]
```

- [x] **Step 3: 验证无悬空 v1-v4 定义**

Run: `grep -nE '^[[:space:]]*v[1-4][[:space:]]*=' crates/openlark-platform/Cargo.toml`
Expected: 无输出（所有 v1-v4 feature 定义已移除）。

Run: `grep -c '^core = \[\]' crates/openlark-platform/Cargo.toml`
Expected: `1`（core marker 保留）。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-platform/Cargo.toml
git commit -m "refactor(platform): 移除门控 0 行的死版本链 v1-v4 feature

BREAKING: 移除 openlark-platform 的 v1/v2/v3/v4 公开 Cargo feature。
这些 feature 在 src/ 中 cfg 命中 0 行，启用后编译产物无差异。
core marker 与全部模块 feature 保留不变。"
```

---

### Task 4: 移除 openlark-analytics 死版本链

**Files:**
- Modify: `crates/openlark-analytics/Cargo.toml`（第 33-37 行 `# === API版本支持 ===` 段 + 第 41 行 `full` 数组）

**Interfaces:**
- Consumes: 无。
- Produces: `crates/openlark-analytics/Cargo.toml` 不再定义 `v1`/`v2`/`v3`/`v4`；`full` 不再引用 `"v4"`。`core = []` 与模块 feature（`search`/`report`）保留。

- [x] **Step 1: 移除版本链定义段**

删除以下 5 行（含注释行与前置空行）：

```toml
# === API版本支持 ===
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
v4 = ["v3"]
```

- [x] **Step 2: 从 `full` 数组去掉 `"v4"`**

old:
```toml
full = ["search", "report", "v4"]
```
new:
```toml
full = ["search", "report"]
```

- [x] **Step 3: 验证**

Run: `grep -nE '^[[:space:]]*v[1-4][[:space:]]*=' crates/openlark-analytics/Cargo.toml`
Expected: 无输出。

Run: `grep -c '^core = \[\]' crates/openlark-analytics/Cargo.toml`
Expected: `1`。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-analytics/Cargo.toml
git commit -m "refactor(analytics): 移除门控 0 行的死版本链 v1-v4 feature

BREAKING: 移除 openlark-analytics 的 v1/v2/v3/v4 公开 Cargo feature。
src/ 中 cfg 命中 0 行，启用后无编译差异。core marker 保留。"
```

---

### Task 5: 移除 openlark-security 死版本链

**Files:**
- Modify: `crates/openlark-security/Cargo.toml`（第 40-43 行 `# === API版本支持 ===` 段 + 第 47 行 `full` 数组）

**Interfaces:**
- Consumes: 无。
- Produces: 不再定义 `v1`/`v2`/`v3`；`full` 不再引用 `"v3"`。`core = []` 与模块 feature（`auth`/`acs`/`audit`/`token`/`compliance`）保留。

- [x] **Step 1: 移除版本链定义段**

删除以下 4 行（含注释行与前置空行）：

```toml
# === API版本支持 ===
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
```

- [x] **Step 2: 从 `full` 数组去掉 `"v3"`**

old:
```toml
full = ["auth", "acs", "audit", "token", "compliance", "v3"]
```
new:
```toml
full = ["auth", "acs", "audit", "token", "compliance"]
```

- [x] **Step 3: 验证**

Run: `grep -nE '^[[:space:]]*v[1-4][[:space:]]*=' crates/openlark-security/Cargo.toml`
Expected: 无输出。

Run: `grep -c '^core = \[\]' crates/openlark-security/Cargo.toml`
Expected: `1`。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-security/Cargo.toml
git commit -m "refactor(security): 移除门控 0 行的死版本链 v1-v3 feature

BREAKING: 移除 openlark-security 的 v1/v2/v3 公开 Cargo feature。
src/ 中 cfg 命中 0 行。core marker 与模块 feature 保留。"
```

---

### Task 6: 移除 openlark-docs 死版本链

**Files:**
- Modify: `crates/openlark-docs/Cargo.toml`（第 64-67 行 `# === API版本支持 ===` 段 + 第 71 行 `full` 数组）

**Interfaces:**
- Consumes: 无。
- Produces: 不再定义 `v1`/`v2`/`v3`。`full` 不再引用 `"v3"`。`core = []` 与全部 ccm/bitable/base/baike/minutes 模块 feature 保留。**注意**：`v2`/`v3` 被下游引用（Task 8、Task 9 同步），本任务先移除 crate 本身的定义。

> **关键依赖**：本任务移除 `v2`/`v3` 后，下游 `crates/openlark-client/Cargo.toml` 与根 `Cargo.toml` 会出现悬空引用 `openlark-docs/v2`、`openlark-docs/v3`，必须在 Task 8/9 同步修复后 `cargo build --all-features` 才会通过。Task 3-5（platform/analytics/security）和 Task 7（user）无下游引用，独立可过。

- [x] **Step 1: 移除版本链定义段**

删除以下 4 行（含注释行与前置空行）：

```toml
# === API版本支持 ===
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
```

> 注意保留同文件第 30-31 行的 `ccm-sheets-v3` / `ccm-sheets` 模块 feature（这些是模块 feature，名称含 v3 但属合法模块命名，**不动**）。

- [x] **Step 2: 从 `full` 数组去掉 `"v3"`**

old:
```toml
full = ["ccm", "bitable", "base", "baike", "minutes", "v3"]
```
new:
```toml
full = ["ccm", "bitable", "base", "baike", "minutes"]
```

- [x] **Step 3: 验证 crate 本身**

Run: `grep -nE '^[[:space:]]*v[1-3][[:space:]]*=' crates/openlark-docs/Cargo.toml`
Expected: 无输出（`v1`/`v2`/`v3` 定义已移除）。

Run: `grep -c 'ccm-sheets-v3' crates/openlark-docs/Cargo.toml`
Expected: `1`（模块 feature 保留，未被误删）。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-docs/Cargo.toml
git commit -m "refactor(docs): 移除门控 0 行的死版本链 v1-v3 feature

BREAKING: 移除 openlark-docs 的 v1/v2/v3 公开 Cargo feature。
src/ 中 cfg 命中 0 行。下游 openlark-client 与根 Cargo.toml 的
v2/v3 引用将在后续 commit 同步移除。ccm-sheets-v3 模块 feature 保留。"
```

---

### Task 7: 移除 openlark-user 死版本链（保留 live v1）

**Files:**
- Modify: `crates/openlark-user/Cargo.toml`（第 35-37 行 `v2`/`v3`/`v4` 定义 + 第 41 行 `full` 数组）

**Interfaces:**
- Consumes: 无。
- Produces: 不再定义 `v2`/`v3`/`v4`；`full` 不再引用 `"v4"`。**`v1 = ["core"]` 保留**（门控 `src/preferences.rs`/`settings.rs` 等 4 处 live 代码）。`core = []` 与模块 feature（`settings`/`preferences`）保留。

- [x] **Step 1: 移除 v2/v3/v4 三行（保留 v1）**

在 `crates/openlark-user/Cargo.toml` 的 `# === API版本支持 ===` 段下，删除 `v2`/`v3`/`v4` 三行，**保留 `v1 = ["core"]`**：

old:
```toml
# === API版本支持 ===
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
v4 = ["v3"]
```
new:
```toml
# === API版本支持（v1 门控 live preferences/settings，保留） ===
v1 = ["core"]
```

- [x] **Step 2: 从 `full` 数组去掉 `"v4"`**

old:
```toml
full = ["settings", "preferences", "v4"]
```
new:
```toml
full = ["settings", "preferences"]
```

- [x] **Step 3: 验证 v1 保留、v2-v4 移除**

Run: `grep -nE '^[[:space:]]*v[2-4][[:space:]]*=' crates/openlark-user/Cargo.toml`
Expected: 无输出（v2/v3/v4 已移除）。

Run: `grep -c '^v1 = \["core"\]' crates/openlark-user/Cargo.toml`
Expected: `1`（v1 保留）。

Run: `grep -rn 'cfg(feature = "v1")' crates/openlark-user/src/ | wc -l`
Expected: `4`（live 门控证据，证明 v1 必须保留）。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-user/Cargo.toml
git commit -m "refactor(user): 移除死版本链 v2/v3/v4，保留门控 live 代码的 v1

BREAKING: 移除 openlark-user 的 v2/v3/v4 公开 Cargo feature。
v1 保留——门控 src/preferences.rs 与 settings.rs 共 4 处 live 代码，
符合「版本 feature 必须门控真实代码」的合法例外条件。"
```

---

### Task 8: 同步 openlark-client docs feature 引用

**Files:**
- Modify: `crates/openlark-client/Cargo.toml:66-72`（`docs` feature 数组）

**Interfaces:**
- Consumes: Task 6 已移除 `openlark-docs` 的 `v2`/`v3` 定义。
- Produces: `openlark-client` 的 `docs` feature 不再引用悬空的 `openlark-docs/v2`、`openlark-docs/v3`。

- [x] **Step 1: 从 docs 数组移除 v2/v3 两行**

修改 `crates/openlark-client/Cargo.toml` 第 66-72 行：

old:
```toml
docs = [
    "auth",
    "dep:openlark-docs",
    "openlark-docs/ccm-core",
    "openlark-docs/v2",
    "openlark-docs/v3",
]
```
new:
```toml
docs = [
    "auth",
    "dep:openlark-docs",
    "openlark-docs/ccm-core",
]
```

- [x] **Step 2: 验证无悬空引用**

Run: `grep -c 'openlark-docs/v[23]' crates/openlark-client/Cargo.toml`
Expected: `0`。

- [x] **Step 3: Commit**

```bash
git add crates/openlark-client/Cargo.toml
git commit -m "refactor(client): docs feature 移除已删除的 openlark-docs/v2、v3 引用

同步 openlark-docs 死版本链移除，消除悬空 feature 引用。"
```

---

### Task 9: 同步根 Cargo.toml docs feature 引用

**Files:**
- Modify: `Cargo.toml:307-321`（`docs-ccm` 数组 + `docs-sheets-v2`/`docs-sheets-v3` 定义）

**Interfaces:**
- Consumes: Task 6 已移除 `openlark-docs` 的 `v2`/`v3` 定义。
- Produces: 根 workspace `docs-ccm` 不再引用悬空的 `openlark-docs/v2`、`openlark-docs/v3`；`docs-sheets-v2`/`docs-sheets-v3` 两个根 feature 整个定义被移除（design doc D2：除引用 v2/v3 外无其他消费者，退化为别名只留噪音）。

- [x] **Step 1: 从 docs-ccm 数组移除 v2/v3 两行**

修改 `Cargo.toml` 第 307-314 行的 `docs-ccm` 数组：

old:
```toml
docs-ccm = [
    "auth",
    "dep:openlark-docs",
    "openlark-client/docs",
    "openlark-docs/ccm-core",
    "openlark-docs/v2",
    "openlark-docs/v3",
]
```
new:
```toml
docs-ccm = [
    "auth",
    "dep:openlark-docs",
    "openlark-client/docs",
    "openlark-docs/ccm-core",
]
```

- [x] **Step 2: 移除 docs-sheets-v2 与 docs-sheets-v3 整个定义**

删除 `Cargo.toml` 第 320-321 行（design doc D2 决策：直接移除，不退化别名）：

```toml
docs-sheets-v2 = ["docs-ccm", "openlark-docs/v2"]
docs-sheets-v3 = ["docs-ccm", "openlark-docs/v3"]
```

保留同段的 `docs-sheets = ["docs-ccm"]`（第 319 行，无版本引用，合法模块 feature）。

- [x] **Step 3: 验证无悬空引用、无 sheets-v2/v3 残留**

Run: `grep -c 'openlark-docs/v[23]' Cargo.toml`
Expected: `0`。

Run: `grep -c 'docs-sheets-v[23]' Cargo.toml`
Expected: `0`。

Run: `grep -c '^docs-sheets =' Cargo.toml`
Expected: `1`（无版本的 `docs-sheets` 保留）。

- [x] **Step 4: Commit**

```bash
git add Cargo.toml
git commit -m "refactor(workspace): docs-ccm 移除 v2/v3 引用，删除 docs-sheets-v2/v3

BREAKING: 移除根 Cargo.toml 的 docs-sheets-v2 / docs-sheets-v3 feature
（除引用已删除的 openlark-docs/v2、v3 外无其他消费者，design D2 直接
移除不退化别名）。docs-ccm 数组同步移除 v2/v3 两行。docs-sheets 保留。"
```

---

### Task 10: 全量验证（fmt + lint 双模式 + build + test）

**Files:**
- 无文件改动（纯验证任务，对应 tasks.md 4.1-4.4）

**Interfaces:**
- Consumes: Task 1-9 全部完成。
- Produces: 全部验证命令通过的证据，作为 change 进入 verify 阶段的前提。

> 这是本计划的关键闸口——所有 Cargo.toml 改动是否完整、有无漏改悬空引用，全靠这一步的双模式 lint 暴露。**不要跳过 `cargo fmt --check`**（CI lint 首步，clippy 通过 ≠ fmt 通过，项目有两次 CI lint fail 教训）。

- [x] **Step 1: cargo fmt --check（CI lint 首步）**

Run: `cargo fmt --all -- --check`
Expected: 退出码 0，无 diff 输出。

> Cargo.toml 改动通常不触发 fmt diff，但本任务改动较多，必须显式确认。

- [x] **Step 2: just lint（双模式 clippy）**

Run: `just lint`
Expected: 退出码 0。CI 等价命令为 `cargo clippy --workspace --all-targets --no-default-features` 与 `cargo clippy --workspace --all-targets --all-features` 两种模式均过。

> **关键防线**：`--all-features` 模式会暴露移除 `openlark-docs/v2`、`v3` 后遗留的悬空引用（Task 8/9 漏改即在此失败）；`--no-default-features` 模式会暴露 cfg 门控可达性问题。

- [x] **Step 3: just build**

Run: `just build`
Expected: 退出码 0，全 workspace 编译通过。

- [x] **Step 4: just test**

Run: `just test`
Expected: 全部测试通过。预期无行为回归（死版本链本就不门控代码，user/v1 保留 live 门控不变）。

- [x] **Step 5: 核实 docs/FEATURE_MATRIX.md 是否需同步（tasks.md 4.4 / Q3）**

Run: `grep -l 'docs-sheets-v2\|docs-sheets-v3\|openlark-docs/v2\|openlark-docs/v3\|openlark-platform/v4\|openlark-user/v[234]' docs/FEATURE_MATRIX.md 2>/dev/null; echo "exit=$?"`
Expected: 若 `docs/FEATURE_MATRIX.md` 存在且命中被移除的 feature，则需同步更新该文档移除对应行；若文件不存在或无命中，记录「无需同步」结论即可。

> Q3 开放问题在此步落实：核实后若有命中则在 FEATURE_MATRIX.md 中移除对应条目并追加一个 commit；无命中则不动。

- [x] **Step 6: 最终确认无悬空引用（兜底 grep）**

Run: `grep -rn 'openlark-docs/v[23]\|docs-sheets-v[23]' --include=Cargo.toml .`
Expected: 无输出（全仓无任何对已移除 feature 的悬空引用）。

Run: `grep -rn 'cfg(feature = "v[2-4]")' crates/openlark-user/src/`
Expected: 无输出（确认 user 的 v2-v4 确实门控 0 行，移除安全）。

- [x] **Step 7: 无 commit（验证任务）**

本任务无文件改动（除非 Step 5 触发 FEATURE_MATRIX.md 同步），不产生 commit。验证通过后即可由协调者推进到 comet verify 阶段。

---

## Self-Review 结果

**1. Spec 覆盖核对**（逐条 spec Requirement / Scenario）：
- "模块 feature 为主要门控方案" → Task 1 文档原则章节 ✓
- "版本 feature 仅作单 API 版本 crate 合法例外" → Task 1 例外清单 + 6 个单版本 crate 表 ✓
- "禁止门控 0 行的死版本链 feature" → Task 3-7 移除 5 crate 死链 ✓
- "混合 crate 的死版本链被移除" Scenario（platform/analytics v1-v4、security/docs v1-v3、user v2-v4）→ Task 3/4/5/6/7 逐一覆盖 ✓
- "live 版本门控保留不动"（user v1、bot v4）→ Task 7 显式保留 v1 + Step 3 验证；bot v4 本 change 不动 ✓
- "死版本链移除须同步下游 feature 引用" → Task 8（client）+ Task 9（根 Cargo）✓
- "docs 版本 feature 的下游引用同步" Scenario → Task 8/9 ✓
- "命名规范文档落盘并维护例外清单" → Task 1 + Task 2 ✓
- "规范文档存在且内容完整" Scenario（含 bot v4 纠正）→ Task 1 Step 1 内容清单含「issue #272 纠正」段落 ✓

**2. tasks.md 4.x 验证项映射**：
- 4.1 fmt → Task 10 Step 1 ✓
- 4.2 lint 双模式 → Task 10 Step 2 ✓
- 4.3 build + test → Task 10 Step 3/4 ✓
- 4.4 FEATURE_MATRIX.md 核实 → Task 10 Step 5 ✓

**3. 占位符扫描**：无 TBD/TODO/"实现细节"/"类似 Task N" 等占位；每个代码 step 都给出了精确 old/new TOML 片段。

**4. 类型一致性**：TOML 改动均为纯文本替换，无跨任务符号依赖。Task 6 → Task 8/9 的依赖关系（docs v2/v3 移除后下游悬空）已在 Task 6 的 Interfaces 块显式标注，Task 10 Step 2/6 用 `--all-features` 与兜底 grep 验证。

**5. 关键约束复核**：
- `core = []` 保留 → Task 3-7 每个 Step 3 都验证 `grep -c '^core = \[\]' == 1` ✓
- user v1 保留 → Task 7 Step 1 注释「保留 v1」+ Step 3 三重验证 ✓
- 下游两处同步 → Task 8（client）+ Task 9（根 Cargo）✓
- fmt --check → Task 10 Step 1 ✓
- lint 双模式 → Task 10 Step 2 ✓
