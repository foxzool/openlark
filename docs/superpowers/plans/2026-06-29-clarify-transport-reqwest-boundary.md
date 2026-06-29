---
change: clarify-transport-reqwest-boundary
design-doc: docs/superpowers/specs/2026-06-29-transport-reqwest-boundary-design.md
base-ref: ee2a7a8b8c3be85622b935df6546bbd7037283d4
archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

# clarify-transport-reqwest-boundary 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 闭环 issue #270——清理 12 个业务 crate 未用的 `reqwest` 依赖声明，把已存在且在工作的 `openlark_core::http::Transport<T>` HTTP 边界显式文档化，并新增防回归守卫脚本。

**Architecture:** 业务 crate 经 core 的 `Transport<T>::request()` 发起 HTTP 请求、源码 0 处直接使用 reqwest。本次只动 `Cargo.toml` 依赖声明、Markdown 文档、一行 webhook 文档注释、一个 shell 守卫脚本、justfile/CI 接线点——**不改任何 `.rs` 业务源码逻辑、不改任何公开 API**。守卫脚本匹配项目既有的 `tools/check_no_dead_code_allows.sh` hygiene 模式。

**Tech Stack:** Rust workspace（Cargo `[dependencies]`）、Bash 守卫脚本（`set -euo pipefail` + grep + 白名单）、just、GitHub Actions ci.yml、Markdown。

## Global Constraints

- **不改公开 API**：本次所有改动是依赖声明 + 文档 + 守卫脚本，不动 `.rs` 业务源码逻辑。唯一允许改的 `.rs` 是 webhook crate 的文档注释（doc comment，不改函数签名/行为）。
- **白名单精确为 3 个 crate**：`openlark-core`（抽象本体）、`openlark-client`（客户端装配 + websocket feature）、`openlark-webhook`（by-design 性能例外）。12 个业务 crate 不在白名单。
- **12 个业务 crate 名单（精确）**：`openlark-analytics`、`openlark-auth`、`openlark-bot`、`openlark-application`、`openlark-communication`、`openlark-mail`、`openlark-hr`、`openlark-docs`、`openlark-helpdesk`、`openlark-platform`、`openlark-user`、`openlark-workflow`。
- **特殊陷阱（auth）**：`openlark-auth` 的 reqwest 不仅是 `reqwest = { workspace = true, optional = true }` 一行，还出现在 `[features]` 的 `oauth = ["reqwest", "url"]`。删依赖行时**必须同时把 `"reqwest"` 从 oauth feature 列表里移除**，否则 `Cargo.toml` 会引用不存在的 optional dep 报错。
- **cargo-machete ignore 债务（全部 12 crate）**：12 个业务 crate 各有 `[package.metadata.cargo-machete] ignored = [...]` 列表，**全部含 `"reqwest"`**（这正是 `cargo machete` 报「无未用依赖」假阴性的根因——之前用 ignore 列表「承认债务」而非删除）。删 reqwest 依赖行时**必须同步把 `"reqwest"` 从各 crate 的 ignored 列表移除**，否则留下指向已删依赖的悬空条目（虽不破坏 build，但对 hygiene change 不一致）。**只清 `"reqwest"` 一项**——列表里的 `tokio`/`tracing`/`url`/`anyhow` 等是更广的未用依赖债务，**超出本次范围不动**。
- **守卫脚本风格对齐** `tools/check_no_dead_code_allows.sh`：`#!/usr/bin/env bash` + `set -euo pipefail` + grep + 白名单 + `✅/❌` 输出。
- **守卫白名单枚举完整写死**：`(openlark-core openlark-client openlark-webhook)`，不依赖运行时推断。
- **commit message 用中文 + Conventional Commits**（匹配项目既有风格，如 `chore(hygiene): ...`、`docs: ...`、`ci: ...`）。
- **base-ref**：`ee2a7a8b8c3be85622b935df6546bbd7037283d4`（执行前确认 working tree 干净）。

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## 文件结构

本计划涉及的全部文件（按类型分组）：

**1. 依赖清理（12 个 Cargo.toml，每文件删 1 行；auth 额外删 feature 项）**
- Modify: `crates/openlark-analytics/Cargo.toml` — 删 `reqwest = { workspace = true }`（line 52）
- Modify: `crates/openlark-auth/Cargo.toml` — 删 `reqwest = { workspace = true, optional = true }`（line 48）**+** 从 `oauth = ["reqwest", "url"]` 移除 `"reqwest"`
- Modify: `crates/openlark-bot/Cargo.toml` — 删（line 19）
- Modify: `crates/openlark-application/Cargo.toml` — 删（line 19）
- Modify: `crates/openlark-communication/Cargo.toml` — 删（line 22）
- Modify: `crates/openlark-mail/Cargo.toml` — 删（line 19）
- Modify: `crates/openlark-hr/Cargo.toml` — 删（line 39）
- Modify: `crates/openlark-docs/Cargo.toml` — 删（line 82）
- Modify: `crates/openlark-helpdesk/Cargo.toml` — 删（line 19）
- Modify: `crates/openlark-platform/Cargo.toml` — 删（line 72）
- Modify: `crates/openlark-user/Cargo.toml` — 删（line 52）
- Modify: `crates/openlark-workflow/Cargo.toml` — 删（line 20）

**2. 守卫脚本（新建）**
- Create: `tools/check_reqwest_boundary.sh` — 风格对齐 `check_no_dead_code_allows.sh`，检查业务 crate 的 `Cargo.toml` 不直接声明 reqwest

**3. 守卫接线（justfile + ci.yml）**
- Modify: `justfile` — 在 `no-dead-code-allows` recipe（line 17-19）后加平行 recipe `reqwest-boundary`
- Modify: `.github/workflows/ci.yml` — 在 lint job「Check no #[allow(dead_code)]」步（line 115-116）后加一步「Check Transport/reqwest boundary」

**4. ARCHITECTURE.md 边界文档化**
- Modify: `ARCHITECTURE.md` — 在 `## 模块详细设计`（line 106）下、`### 核心模块` 之前，新增 `### Transport HTTP 边界` 小节

**5. webhook 文档注释强化**
- Modify: `crates/openlark-webhook/src/robot/v1/send.rs` — `shared_client()` doc 注释补一句指向 ARCHITECTURE.md 边界约定的交叉引用
- Modify: `crates/openlark-webhook/src/robot/v1/client.rs` — `WebhookClient` doc 注释同上（已有 `见 send 模块说明 + issue #214`，加 ARCHITECTURE.md 指向）

**6. CHANGELOG**
- Modify: `CHANGELOG.md` — `[Unreleased]` 段（即 v0.18 待发段）`### Changed` 下补 hygiene 条目

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 1：移除 12 个业务 crate 未用的 reqwest 依赖

**Files:**
- Modify: 12 个 `crates/openlark-*/Cargo.toml`（见文件结构第 1 组）
- Test: `cargo build --workspace --all-features`（删依赖后全量构建必须通过——证明源码 0 处引用 reqwest）

**Interfaces:**
- Consumes: 无（首个任务）
- Produces: 12 个业务 crate 的 `Cargo.toml` 不再含 reqwest 行；`auth` 的 oauth feature 不再含 `"reqwest"`。后续 Task 2 的守卫脚本依赖这个状态（清理后守卫应通过）。

**陷阱说明（实现者必读）：** `openlark-auth` 是 12 个里唯一带 `optional = true` + feature 引用的。其余 11 个都是普通 `reqwest = { workspace = true }` 单行，直接删。auth 必须两处同改（依赖行 + oauth feature 列表），缺一即报错（feature 引用不存在的 optional dep → `cargo` 解析失败）。

- [x] **Step 1.1：删 11 个普通业务 crate 的 reqwest 行**

对以下 11 个 crate，各删除其 `Cargo.toml` 中 `[dependencies]` section 下的 `reqwest = { workspace = true }` 一行（连同上方 `# HTTP客户端` 之类的注释行若仅服务于该 reqwest 行，一并删除；若注释同时描述其他依赖则保留）：

```
crates/openlark-analytics/Cargo.toml    (line 52)
crates/openlark-bot/Cargo.toml          (line 19)
crates/openlark-application/Cargo.toml  (line 19)
crates/openlark-communication/Cargo.toml(line 22)
crates/openlark-mail/Cargo.toml         (line 19)
crates/openlark-hr/Cargo.toml           (line 39)
crates/openlark-docs/Cargo.toml         (line 82)
crates/openlark-helpdesk/Cargo.toml     (line 19)
crates/openlark-platform/Cargo.toml     (line 72)
crates/openlark-user/Cargo.toml         (line 52)
crates/openlark-workflow/Cargo.toml     (line 20)
```

每行确切文本形如：
```toml
reqwest = { workspace = true }
```
（部分 crate 可能在上方有 `# HTTP客户端` / `# reqwest 客户端` 注释——若该注释只描述 reqwest，删除整行注释；若同时描述上下相邻的其他依赖则保留。）

- [x] **Step 1.1b：移除 12 个 crate 的 cargo-machete ignored 列表中的 `"reqwest"` 项**

对**全部 12 个业务 crate**（含 auth），各在其 `[package.metadata.cargo-machete] ignored = [...]` 列表中仅移除 `"reqwest"` 这一项，其余项（tokio/tracing/url/anyhow 等）原样保留。各 crate 当前 ignored 行（核实过）：

```
analytics  line 78:  ignored = [..., "reqwest", ...]
auth       line 84:  ignored = [..., "reqwest", ...]    ← 与 Step 1.2 同 crate，一并改
bot        line 37:  ignored = ["reqwest", "tokio", "tracing"]            → ["tokio", "tracing"]
applicationline 37:  ignored = ["reqwest", "tokio", "tracing"]            → ["tokio", "tracing"]
communication line 39: ignored = ["reqwest", "tracing"]                   → ["tracing"]
mail       line 37:  ignored = ["reqwest", "tokio", "tracing"]            → ["tokio", "tracing"]
hr         line 52:  ignored = [..., "reqwest", ...]
docs       line 117: ignored = [..., "reqwest", ...]
helpdesk   line 37:  ignored = ["reqwest", "tokio", "tracing"]            → ["tokio", "tracing"]
platform   line 99:  ignored = [..., "reqwest", ...]
user       line 78:  ignored = [..., "reqwest", ...]
workflow   line 43:  ignored = ["reqwest", "tokio", "tracing"]            → ["tokio", "tracing"]
```

操作：把每个 ignored 列表里的 `"reqwest", `（或 `"reqwest"` 若是末项）删掉，保持 TOML 数组合法（逗号/引号不残留）。auth 的 line 84 与 Step 1.2 的两处改动同属一个 commit。

- [x] **Step 1.2：删 auth crate 的 reqwest（依赖行 + oauth feature 项）**

`crates/openlark-auth/Cargo.toml` 需要两处改动：

**改动 A**——删除 line 48 依赖行（含其上方紧邻的注释 line 47）：

删除前（line 47-48）：
```toml
# HTTP客户端 (OAuth需要)
reqwest = { workspace = true, optional = true }
```
删除后：这两行整段移除（保留 line 49 的 `url = { workspace = true, optional = true }` 不动——url 仍被 oauth feature 使用）。

**改动 B**——从 oauth feature 列表移除 `"reqwest"`（line 76）：

删除前：
```toml
oauth = ["reqwest", "url"]
```
删除后：
```toml
oauth = ["url"]
```

（验证：`url` 保留是因为 oauth 仍在用它——`use crate::...` 中 url crate 被引用；reqwest 0 处引用故移除。）

- [x] **Step 1.3：构建验证（删依赖后必须通过）**

Run: `cargo build --workspace --all-features`
Expected: exit 0，无 "unresolved dependency" / "unused feature" 报错。

若失败，最可能是 auth 的 oauth feature 项未同步移除 `"reqwest"`（回到 Step 1.2 改动 B 检查），或某 crate 的源码实际引用了 reqwest（违背 design 实证——立即停止并加载 systematic-debugging）。

- [x] **Step 1.4：grep 双重确认清理彻底**

Run（确认 12 业务 crate 全清，3 例外保留）：
```bash
echo "=== 12 业务 crate Cargo.toml reqwest 命中（应全 0）==="
for c in analytics auth bot application communication mail hr docs helpdesk platform user workflow; do
  n=$(grep -cE '^\s*reqwest\s*=' "crates/openlark-$c/Cargo.toml")
  echo "openlark-$c: $n"
done
echo "=== 3 例外 crate（应保留 reqwest）==="
for c in core client webhook; do
  grep -nE '^\s*reqwest\s*=' "crates/openlark-$c/Cargo.toml"
done
echo "=== auth oauth feature 不再含 reqwest ==="
grep -nE 'oauth\s*=' crates/openlark-auth/Cargo.toml
echo "=== 12 crate 的 cargo-machete ignored 列表不再含 reqwest ==="
for c in analytics auth bot application communication mail hr docs helpdesk platform user workflow; do
  n=$(grep -A1 "cargo-machete" "crates/openlark-$c/Cargo.toml" | grep -c "reqwest")
  [ "$n" = "0" ] || echo "RESIDUAL openlark-$c: ignored 列表仍含 reqwest"
done
echo "(无 RESIDUAL 输出 = ignore 债务已清)"
```
Expected:
- 12 业务 crate 全部 `0`
- core/client/webhook 各打印一行 `reqwest = ...`
- auth 的 oauth 行为 `oauth = ["url"]`（不含 `"reqwest"`）
- 无 `RESIDUAL` 输出（cargo-machete ignored 列表已无 reqwest）

- [x] **Step 1.5：commit**

```bash
git add crates/openlark-*/Cargo.toml
git commit -m "chore(hygiene): 移除 12 个业务 crate 未用的 reqwest 依赖声明

业务 crate 经 core Transport<T> 发请求，源码 0 处直接使用 reqwest（#270 实证）。
11 个为普通 reqwest = { workspace = true } 单行；auth 额外从 oauth feature
列表移除 \"reqwest\"（optional dep 仅被 feature 引用、源码无引用）。
同步清除 12 crate 的 [package.metadata.cargo-machete] ignored 列表里的
\"reqwest\" 项（这是 cargo-machete 假阴性的根因——此前用 ignore 列表承认债务
而非删除；其余 tokio/tracing/url 等未用债务出范围不动）。
保留例外：core（抽象本体）/ client（装配 + websocket feature）/ webhook
（by-design 连接池复用例外，见 ARCHITECTURE.md）。非 breaking：不改公开 API。

Refs: #270"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 2：新增 Transport/reqwest 边界守卫脚本

**Files:**
- Create: `tools/check_reqwest_boundary.sh`
- Test: 本地 `bash tools/check_reqwest_boundary.sh`（清理后必须 exit 0）；构造违规场景验证能抓回归（exit 1）

**Interfaces:**
- Consumes: Task 1 的清理结果（12 业务 crate 已无 reqwest；core/client/webhook 保留）
- Produces: `tools/check_reqwest_boundary.sh` 可执行脚本，供 Task 3 的 justfile/CI 接线调用。脚本契约：清理后 `exit 0` + 打印 `✅ ...`；任一业务 crate 重新声明 reqwest 则 `exit 1` + 打印 `❌ ...` + 列违规文件路径。

**风格基准：** 严格对齐 `tools/check_no_dead_code_allows.sh`——`#!/usr/bin/env bash` + `set -euo pipefail` + 头部注释说明用途与调用方 + grep + 白名单 + `✅/❌` 输出到 stderr（错误时）。

- [x] **Step 2.1：创建守卫脚本**

Create `tools/check_reqwest_boundary.sh`，完整内容：

```bash
#!/usr/bin/env bash
# 检查业务 crate 的 Cargo.toml 不直接声明 reqwest 依赖（issue #270 边界防复发）。
#
# 架构约定：业务 crate 须经 core 的 openlark_core::http::Transport<T>::request()
# 发起 HTTP 请求，不在各自 Cargo.toml 中声明 reqwest、不在源码使用 reqwest 类型。
# 唯一例外白名单（见 ARCHITECTURE.md「Transport HTTP 边界」）：
#   - openlark-core    抽象本体（Transport 定义在此）
#   - openlark-client  客户端装配 + websocket feature
#   - openlark-webhook by-design 性能例外（无鉴权推送器，进程级共享 reqwest::Client 复用连接池，#214）
#
# 被 justfile (just reqwest-boundary) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail

# 例外白名单（精确枚举，不依赖运行时推断）
ALLOW=(openlark-core openlark-client openlark-webhook)

hits=""
for toml in crates/*/Cargo.toml; do
  crate=$(basename "$(dirname "$toml")")
  # 跳过白名单 crate
  for a in "${ALLOW[@]}"; do
    [ "$crate" = "$a" ] && continue 2
  done
  # 业务 crate 的 Cargo.toml 任何 section 出现 reqwest = ... 即违规
  # （依赖声明在 [dependencies] 即属边界泄漏，无需区分 dev/build section）
  if grep -qE '^[[:space:]]*reqwest[[:space:]]*=' "$toml"; then
    hits="${hits}${toml}"$'\n'
  fi
done

if [ -n "$hits" ]; then
  echo "❌ 业务 crate 直接声明了 reqwest 依赖（须经 core Transport 发请求，见 ARCHITECTURE.md）：" >&2
  printf '%s' "$hits" >&2
  exit 1
fi

echo "✅ 业务 crate Cargo.toml 无 reqwest 直接依赖（HTTP 边界由 core Transport 收口）"
```

**实现者注意：**
- `continue 2` 跳出内层 for 继续外层 for（跳过白名单 crate），不要写成 `continue`（只跳内层循环会误判白名单 crate）。
- `hits` 用 `$'\n'` 分隔（可移植的换行），最后用 `printf '%s'` 输出（避免 echo -e 的可移植性问题）。
- grep 模式 `^[[:space:]]*reqwest[[:space:]]*=` 匹配 `reqwest = ...` 与带缩进的 `    reqwest = ...`，用 POSIX 字符类而非 `\s`（macOS/BSD grep 兼容）。

- [x] **Step 2.2：设置可执行权限**

Run: `chmod +x tools/check_reqwest_boundary.sh`
Expected: 无输出；`ls -l tools/check_reqwest_boundary.sh` 显示 `-rwxr-xr-x`（或带 `x`）。

- [x] **Step 2.3：验证脚本——清理后通过（happy path）**

前置：Task 1 已完成（12 业务 crate 已清理）。

Run: `bash tools/check_reqwest_boundary.sh`
Expected: exit 0，stdout 打印：
```
✅ 业务 crate Cargo.toml 无 reqwest 直接依赖（HTTP 边界由 core Transport 收口）
```

- [x] **Step 2.4：验证脚本——能抓回归（违规 path）**

临时在一个业务 crate（如 `openlark-mail`）的 `[dependencies]` 末尾追加一行 `reqwest = { workspace = true }`，验证脚本能抓到：

```bash
# 临时注入违规
printf '\nreqwest = { workspace = true }\n' >> crates/openlark-mail/Cargo.toml
bash tools/check_reqwest_boundary.sh
echo "exit code: $?"   # 期望非 0
# 还原
git checkout -- crates/openlark-mail/Cargo.toml
```
Expected: 脚本 exit 1，stderr 打印 `❌ 业务 crate 直接声明了 reqwest 依赖...` 并列出 `crates/openlark-mail/Cargo.toml`。

**关键：** 还原后再次运行 Step 2.3 确认回到 exit 0，证明注入/还原干净。

- [x] **Step 2.5：commit**

```bash
git add tools/check_reqwest_boundary.sh
git commit -m "chore(hygiene): 新增 Transport/reqwest 边界守卫脚本

tools/check_reqwest_boundary.sh 风格对齐 check_no_dead_code_allows.sh：
业务 crate Cargo.toml 不得直接声明 reqwest（白名单 core/client/webhook）。
防止 #270 清理被未来 PR 回归。下个 task 接入 justfile 与 CI lint job。"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 3：守卫接入 justfile 与 CI

**Files:**
- Modify: `justfile`（line 17-19 的 `no-dead-code-allows` recipe 后）
- Modify: `.github/workflows/ci.yml`（line 115-116 的 dead_code 检查步后）
- Test: `just reqwest-boundary` 本地通过；ci.yml YAML 语法正确（`python3 -c 'import yaml; yaml.safe_load(open(".github/workflows/ci.yml"))'`）

**Interfaces:**
- Consumes: Task 2 的 `tools/check_reqwest_boundary.sh`
- Produces: `just reqwest-boundary` recipe + ci.yml lint job 新增一步。下个 Task 起为纯文档改动。

- [x] **Step 3.1：justfile 加平行 recipe**

Modify `justfile`：在现有 `no-dead-code-allows` recipe（line 16-19）之后、`test` recipe（line 21）之前，插入平行 recipe。

现有（line 16-19）：
```just
# Check no #[allow(dead_code)] in non-test code (issue #267 防复发)
no-dead-code-allows:
  @echo "🛡️ Checking no #[allow(dead_code)] in non-test code..."
  @bash tools/check_no_dead_code_allows.sh
```

在其后（line 20 空行之后、line 21 `# Run tests` 之前）插入：
```just

# Check Transport/reqwest boundary (issue #270 防复发)
reqwest-boundary:
  @echo "🛡️ Checking Transport/reqwest boundary (no reqwest in business crates)..."
  @bash tools/check_reqwest_boundary.sh
```

（两 recipe 之间保留一个空行；新 recipe 与下方 `# Run tests` 注释之间也保留一个空行——匹配 justfile 既有两-recipe 间单空行分隔的格式。）

- [x] **Step 3.2：验证 just recipe**

Run: `just reqwest-boundary`
Expected: exit 0，stdout：
```
🛡️ Checking Transport/reqwest boundary (no reqwest in business crates)...
✅ 业务 crate Cargo.toml 无 reqwest 直接依赖（HTTP 边界由 core Transport 收口）
```

- [x] **Step 3.3：ci.yml lint job 加一步**

Modify `.github/workflows/ci.yml`：在现有「Check no #[allow(dead_code)]」步（line 115-116）之后插入平行步。

现有（line 115-116）：
```yaml
      - name: Check no #[allow(dead_code)] in non-test code
        run: bash tools/check_no_dead_code_allows.sh
```

在其后（line 116 之后、line 117 的 `- name: Run clippy (no default features)` 之前）插入：
```yaml
      - name: Check Transport/reqwest boundary
        run: bash tools/check_reqwest_boundary.sh
```

（缩进与相邻 step 完全一致：`      - name:` 6 空格 + `- `，`        run:` 8 空格。）

- [x] **Step 3.4：验证 ci.yml YAML 语法**

Run:
```bash
python3 -c 'import yaml; yaml.safe_load(open(".github/workflows/ci.yml")); print("ci.yml YAML OK")'
```
Expected: 打印 `ci.yml YAML OK`（无异常）。

若系统无 PyYAML，用 actionlint 或 GitHub Actions schema 校验替代；最起码目视确认新 step 的缩进与相邻 step 一致（6 空格起 `- name:`）。

- [x] **Step 3.5：commit**

```bash
git add justfile .github/workflows/ci.yml
git commit -m "ci: 接入 Transport/reqwest 边界守卫到 just 与 CI lint job

justfile 新增 reqwest-boundary recipe（平行于 no-dead-code-allows）；
ci.yml lint job 新增 'Check Transport/reqwest boundary' 步。
防止 #270 清理被未来 PR 回归。"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 4：ARCHITECTURE.md 新增 Transport HTTP 边界小节

**Files:**
- Modify: `ARCHITECTURE.md`（`## 模块详细设计` line 106 下、`### 核心模块 (Core Modules)` line 108 之前插入新小节）
- Test: grep ARCHITECTURE.md 能定位「Transport HTTP 边界」「webhook」「by-design」「future」「规划中」等关键词

**Interfaces:**
- Consumes: 无（文档任务）
- Produces: ARCHITECTURE.md 含「Transport HTTP 边界」小节，覆盖 delta spec 三个文档化 Requirement（边界约定 / webhook 例外 / 中间件层 future 标注）。Task 5 的 webhook 注释与 Task 6 的 CHANGELOG 会反向链接到此小节。

**上下文（实现者必读）：**
- ARCHITECTURE.md 已有顶部「免责声明」明确分级 `✅ 已实现` / `🚧 规划中`（line 1-13），且 line 47-49 已注明 `🔄 RetryMiddleware → 实际为 RetryPolicy（策略配置模式）`、line 52-58 列 `🚧 规划中` 含 CircuitBreaker。本任务**新增独立小节集中陈述边界**，不重写既有分级，只在边界小节里再次点明中间件层属 future 并指回既有分级。

- [x] **Step 4.1：插入「Transport HTTP 边界」小节**

Modify `ARCHITECTURE.md`：在 `## 模块详细设计`（line 106）与 `### 核心模块 (Core Modules)`（line 108）之间插入新小节（保留它们之间的空行结构）。

删除前（line 105-108）：
```markdown
```

## 模块详细设计

### 核心模块 (Core Modules)
```

替换为：
```markdown
```

## 模块详细设计

### Transport HTTP 边界

> 架构约定：`openlark_core::http::Transport<T>` 是 OpenLark 的**唯一 HTTP 出口**。
> 业务 crate 经 `Transport` 抽象发请求，**不在各自 `Cargo.toml` 声明 reqwest 依赖、不在源码使用 reqwest 类型**。
> 此边界由 `tools/check_reqwest_boundary.sh` 机器检验（`just reqwest-boundary` / CI lint job），防止分层泄漏复发（见 issue #270）。

**调用路径**（仅 core 碰 reqwest）：

```
*Request::execute()
  └─> openlark_core::http::Transport::request(req, &config, option)
        └─> ReqTranslator / UnifiedRequestBuilder
              └─> reqwest::RequestBuilder  ← 仅 openlark-core 这一跳
```

**Cargo 依赖边界**：

| crate | 可否声明 reqwest | 原因 |
|-------|----------------|------|
| `openlark-core` | ✅ 声明 | Transport 抽象本体，reqwest 实现细节收敛于此 |
| `openlark-client` | ✅ 声明（optional，websocket feature 引用） | 客户端装配 + WebSocket 升级握手 |
| `openlark-webhook` | ✅ 声明 | by-design 性能例外（见下） |
| 其余业务 crate（hr/communication/docs/workflow/...） | ❌ 禁止 | 须经 `Transport::request()` 发请求 |

**webhook by-design 例外**：

`openlark-webhook` 直接使用 `reqwest::Client`（`crates/openlark-webhook/src/robot/v1/send.rs::shared_client()`，进程级 `OnceLock` 共享单个 `Client` 复用连接池），**不经 core `Transport`**。原因：webhook 自定义机器人**不是飞书开放平台 API**——目标 URL 是用户配置的绝对地址、鉴权用 URL 携带的签名密钥（非 Bearer token）、响应体是 `{code,msg}` 非标准包装，与 `Transport` 固定的 `/open-apis/` 基址、强制 token 注入、`ApiResponse<R>` 解析三者均不兼容。这是**有意保留的独立 reqwest 路径**（调研见 GitHub issue #214），**不视为分层泄漏**。详见 `send` 模块文档注释。

**Transport 中间件 / 熔断 / 智能重试中间件 — 规划中（future change）**

本文档部分章节（如服务层重构草案、CircuitBreaker、AsyncMiddlewareChain）描述了 Transport 中间件链 / 熔断器 / 智能重试中间件的**目标形态**。这些设计**当前未实现**——实际重试能力是 `openlark_core::error::RetryPolicy` 的配置模式（无中间件链、无熔断器）。本文档顶部「文档内容分级」已将这些标注为 `🚧 规划中`。中间件/熔断/重试链的落地属**独立的 future change**，不属本次 #270 边界澄清范围。

### 核心模块 (Core Modules)
```

- [x] **Step 4.2：验证 grep 能定位关键词（覆盖 3 个文档化 Requirement）**

Run:
```bash
echo "=== 边界约定（Requirement: Transport 边界显式文档化）==="
grep -nE 'Transport.*边界|Transport HTTP 边界|经.*Transport.*发请求|不.*直接.*依赖.*reqwest' ARCHITECTURE.md | head
echo "=== webhook 例外（Requirement: webhook by-design 例外）==="
grep -nE 'webhook.*by-design|by-design 例外|webhook 自定义机器人' ARCHITECTURE.md | head
echo "=== 中间件 future（Requirement: ARCHITECTURE.md 中间件层标注为 future）==="
grep -nE '规划中.*future|future change|当前未实现.*中间件|RetryPolicy.*配置模式' ARCHITECTURE.md | head
```
Expected: 每组至少打印 1 行命中（在新插入的小节内）。

- [x] **Step 4.3：commit**

```bash
git add ARCHITECTURE.md
git commit -m "docs(arch): 新增 Transport HTTP 边界小节（#270 边界显式化）

集中陈述 openlark_core::http::Transport<T> 作为唯一 HTTP 出口的约定：
- 调用路径（仅 core 碰 reqwest）
- Cargo 依赖边界表（core/client/webhook 允许，业务 crate 禁止）
- webhook by-design 例外（进程级共享 Client 复用连接池，#214）
- Transport 中间件/熔断/重试链标注为规划中 future（实际为 RetryPolicy 配置模式）

不改既有分级章节，仅在 '## 模块详细设计' 下集中陈述边界。
Refs: #270 #214"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 5：webhook crate 文档注释补 ARCHITECTURE.md 交叉引用

**Files:**
- Modify: `crates/openlark-webhook/src/robot/v1/send.rs`（`shared_client()` doc 注释，line 13-22）
- Modify: `crates/openlark-webhook/src/robot/v1/client.rs`（`WebhookClient` doc 注释，line 9-12）
- Test: `cargo build -p openlark-webhook --all-features`（doc 注释改动不影响构建）+ grep 确认引用 ARCHITECTURE.md

**Interfaces:**
- Consumes: Task 4 的 ARCHITECTURE.md「Transport HTTP 边界」小节
- Produces: webhook crate 两个核心 doc 注释显式指向 ARCHITECTURE.md 边界约定。覆盖 delta spec「webhook crate 文档注释说明例外」Scenario。

**上下文（实现者必读，避免过度改动）：**
- `send.rs` line 13-22 已有**详尽**的「为什么不走 Transport」说明（issue #214、签名密钥、响应体格式等）。**不要重写这段**，只在末尾追加一行指向 ARCHITECTURE.md 边界小节的交叉引用即可。
- `client.rs` line 9-12 的注释较短（`见 send 模块说明 + issue #214`），同样只追加 ARCHITECTURE.md 指向。
- 这是本计划**唯一允许改 `.rs` 的任务**，且仅改 doc comment 文本（不改任何 `fn`/`struct`/类型签名/逻辑）。

- [x] **Step 5.1：send.rs `shared_client()` 注释追加交叉引用**

Modify `crates/openlark-webhook/src/robot/v1/send.rs`：在 `shared_client()` 的 doc 注释末尾（line 22 的 `避免每个请求 ... 开销。` 之后、line 23 `pub(super) fn shared_client()` 之前）追加一行。

删除前（line 22-23）：
```rust
/// 避免每个请求 `reqwest::Client::new()` 新建连接池的开销。
pub(super) fn shared_client() -> &'static reqwest::Client {
```

替换为：
```rust
/// 避免每个请求 `reqwest::Client::new()` 新建连接池的开销。
///
/// 这是 `Transport` 边界的 **by-design 例外**——架构约定与白名单见
/// `ARCHITECTURE.md`「Transport HTTP 边界」小节，并由
/// `tools/check_reqwest_boundary.sh` 守卫（#270）。
pub(super) fn shared_client() -> &'static reqwest::Client {
```

- [x] **Step 5.2：client.rs `WebhookClient` 注释追加交叉引用**

Modify `crates/openlark-webhook/src/robot/v1/client.rs`：在 `WebhookClient` doc 注释末尾（line 12 `——webhook 是...（见 send 模块说明 + issue #214）。` 之后、line 13 `#[derive(Debug, Clone)]` 之前）追加一行。

删除前（line 12-13）：
```rust
/// ——webhook 是出站自定义机器人 URL，非飞书开放平台 API（见 `send` 模块说明 + issue #214）。
#[derive(Debug, Clone)]
```

替换为：
```rust
/// ——webhook 是出站自定义机器人 URL，非飞书开放平台 API（见 `send` 模块说明 + issue #214）。
///
/// 这是 `Transport` 边界的 **by-design 例外**（白名单见 `ARCHITECTURE.md`
/// 「Transport HTTP 边界」小节，#270）。
#[derive(Debug, Clone)]
```

- [x] **Step 5.3：构建 + grep 验证**

Run:
```bash
cargo build -p openlark-webhook --all-features
echo "exit: $?"
echo "=== webhook 注释引用 ARCHITECTURE.md ==="
grep -rnE 'ARCHITECTURE\.md.*Transport HTTP 边界|by-design 例外' crates/openlark-webhook/src/robot/v1/
```
Expected: build exit 0；grep 至少命中 2 行（send.rs + client.rs 各一处）。

- [x] **Step 5.4：commit**

```bash
git add crates/openlark-webhook/src/robot/v1/send.rs crates/openlark-webhook/src/robot/v1/client.rs
git commit -m "docs(webhook): 注释补 Transport 边界 by-design 例外的 ARCHITECTURE.md 交叉引用

shared_client() 与 WebhookClient 的 doc 注释已在 send 模块详述为何不走
Transport（#214）；本次仅追加一行指向 ARCHITECTURE.md「Transport HTTP 边界」
小节，让边界约定集中可查。仅改 doc comment，不改源码逻辑/签名。

Refs: #270"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 6：CHANGELOG 补 v0.18 hygiene 条目

**Files:**
- Modify: `CHANGELOG.md`（`## [Unreleased]` 段 `### Changed` 下，line 10-18 区域）
- Test: grep CHANGELOG.md 命中新条目

**Interfaces:**
- Consumes: Task 1-5 全部完成
- Produces: CHANGELOG `[Unreleased]`（v0.18 待发段）记录本次 hygiene 清理，为下游用户提供迁移提示。

- [x] **Step 6.1：在 `### Changed` 末尾追加条目**

Modify `CHANGELOG.md`：在 `## [Unreleased]` → `### Changed` section 的末尾（line 18 `#[expect(dead_code)]...` 那条之后、`### Breaking Changes` line 20 之前）追加一条。

定位锚点（line 16-18 现有条目）：
```markdown
- **CI 防复发**：`lint` job 新增 `tools/check_no_dead_code_allows.sh` 检查，禁止非测试代码
  引入 `#[allow(dead_code)]`（本地 `just no-dead-code-allows`）；`#[expect(dead_code)]` 为
  受控的预期死代码豁免。闭环 #267。
```

在其后（`### Breaking Changes` 之前）插入：
```markdown
- **Transport 边界 hygiene**：移除 12 个业务 crate（analytics/auth/bot/application/
  communication/mail/hr/docs/helpdesk/platform/user/workflow）未用的 `reqwest` 依赖声明，
  并清除这些 crate 的 `[package.metadata.cargo-machete] ignored` 列表里对应的 `"reqwest"`
  项（此前用 ignore 列表承认债务而非删除，是 `cargo machete` 假阴性根因）。
  业务 crate 经 core `Transport<T>` 发请求、源码 0 处直接使用 reqwest（#270 实证）。
  保留例外：`openlark-core`（抽象本体）/ `openlark-client`（装配 + websocket）/ `openlark-webhook`
  （by-design 连接池复用例外，见 ARCHITECTURE.md「Transport HTTP 边界」）。新增
  `tools/check_reqwest_boundary.sh` 守卫并接入 just 与 CI lint job 防回归。
  **非 breaking**：不改公开 API（业务 crate 无 re-export reqwest）。
```

- [x] **Step 6.2：验证**

Run: `grep -nE 'Transport 边界 hygiene|check_reqwest_boundary.sh' CHANGELOG.md`
Expected: 至少 2 行命中（条目标题 + 守卫脚本名）。

- [x] **Step 6.3：commit**

```bash
git add CHANGELOG.md
git commit -m "docs(changelog): v0.18 补 Transport 边界 hygiene 条目（#270）

记录移除 12 业务 crate 未用 reqwest 依赖 + 新增 check_reqwest_boundary.sh 守卫。
非 breaking，不改公开 API。"
```

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## Task 7：全量验证（build / clippy / test / grep / 守卫）

**Files:**
- 无文件改动；纯验证任务。

**Interfaces:**
- Consumes: Task 1-6 全部完成并 commit。
- Produces: 全部 delta spec Scenario 的验证证据；计划完成，进入 verify 阶段。

**说明：** delta spec 有 5 个 Requirement，其中「清理不破坏构建/lint/测试」Requirement 含 3 个 Scenario（全 feature 构建 / 三组 clippy / 测试）。本任务一次性跑齐 design 第 4 节的全部验证命令。

- [x] **Step 7.1：全 feature 构建**

Run: `cargo build --workspace --all-features`
Expected: exit 0（覆盖 Scenario「全 feature 构建通过」）。

- [x] **Step 7.2：三组 feature clippy（均 -D warnings）**

Run（三组逐条，每组必须 exit 0）：
```bash
echo "=== (1) default features ==="
cargo clippy --workspace --all-targets -- -D warnings && echo "OK: default"
echo "=== (2) --all-features ==="
cargo clippy --workspace --all-targets --all-features -- -D warnings && echo "OK: all-features"
echo "=== (3) --no-default-features ==="
cargo clippy --workspace --all-targets --no-default-features -- -D warnings && echo "OK: no-default"
```
Expected: 三组均 exit 0（覆盖 Scenario「三组 feature clippy 通过」）。

注意：CI 现有 lint job 用 `-D warnings -A missing_docs`（justfile line 14），但 design 验证要求纯 `-D warnings`。若 default 组因既有 missing-docs 噪音失败，按 design 第 4 节精神——目标是「本次清理不引入新 warning」，可与 lint job 配置（`-A missing_docs`）对齐复跑以分离既有噪音；但**不得**用 `-A` 抑制本次可能引入的新 warning。

- [x] **Step 7.3：workspace 测试**

Run: `cargo test --workspace`
Expected: 全部通过，0 failed（覆盖 Scenario「测试通过」）。

注：若个别测试因环境（如需 `.env` 凭证的网络测试）跳过/ignored 属正常；关注的是 0 failed 且无因删依赖导致的编译/链接错误。

- [x] **Step 7.4：守卫脚本（清理后通过）**

Run: `bash tools/check_reqwest_boundary.sh`
Expected: exit 0，打印 `✅ ...`（覆盖 Scenario「清理后守卫通过」）。

- [x] **Step 7.5：grep 双重确认（边界完整性）**

Run:
```bash
echo "=== (A) 12 业务 crate Cargo.toml reqwest 命中（应全 0）==="
for c in analytics auth bot application communication mail hr docs helpdesk platform user workflow; do
  n=$(grep -cE '^[[:space:]]*reqwest[[:space:]]*=' "crates/openlark-$c/Cargo.toml")
  [ "$n" = "0" ] || echo "VIOLATION openlark-$c: $n"
done
echo "(无 VIOLATION 输出 = 全清)"

echo "=== (B) 12 业务 crate src/ reqwest 命中（应全 0）==="
for c in analytics auth bot application communication mail hr docs helpdesk platform user workflow; do
  n=$(grep -rE 'reqwest' "crates/openlark-$c/src/" 2>/dev/null | wc -l | tr -d ' ')
  [ "$n" = "0" ] || echo "USAGE openlark-$c: $n"
done
echo "(无 USAGE 输出 = 源码无引用)"

echo "=== (C) 3 例外 crate Cargo.toml 仍保留 reqwest ==="
for c in core client webhook; do
  grep -nE '^[[:space:]]*reqwest[[:space:]]*=' "crates/openlark-$c/Cargo.toml"
done

echo "=== (D) justfile + ci.yml 接线存在 ==="
grep -n 'reqwest-boundary' justfile
grep -n 'check_reqwest_boundary.sh' .github/workflows/ci.yml
```
Expected:
- (A) 无 `VIOLATION` 输出（覆盖 Scenario「12 个业务 crate 的 Cargo.toml 不含 reqwest」）
- (B) 无 `USAGE` 输出（覆盖 Scenario「业务 crate 源码不出现 reqwest 类型」）
- (C) core/client/webhook 各打印一行（覆盖 Scenario「允许的三个例外 crate 仍可声明 reqwest」）
- (D) justfile 与 ci.yml 各打印至少一行（覆盖 Scenario「守卫接入 CI 与 justfile」）

- [x] **Step 7.6：守卫脚本存在性与可执行（覆盖 Scenario「守卫脚本存在」）**

Run:
```bash
ls -l tools/check_reqwest_boundary.sh
echo "=== set -euo pipefail ==="
grep -n 'set -euo pipefail' tools/check_reqwest_boundary.sh
echo "=== 白名单 core/client/webhook ==="
grep -nE 'ALLOW=\([^)]*\)' tools/check_reqwest_boundary.sh
```
Expected: 文件存在且带 `x` 权限；含 `set -euo pipefail`；`ALLOW=(...)` 含 core/client/webhook。

- [x] **Step 7.7：（可选）提交验证证据到 change 目录**

本步骤无代码改动。若团队约定在 `openspec/changes/clarify-transport-reqwest-boundary/` 下记录验证证据（如 `.comet/` 或 verify 产物），按 comet verify 流程处理；否则本计划至此完成，进入 comet verify 阶段。

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## 自检（Self-Review）

**1. Spec 覆盖：** 逐条核对 delta spec 的 Requirement 与 Scenario：

| Requirement / Scenario | 覆盖任务 | 状态 |
|---|---|---|
| 业务 crate 不直接依赖 reqwest — 「12 crate Cargo.toml 不含 reqwest」 | Task 1 + Step 7.5(A) | ✅ |
| 同上 — 「业务 crate 源码不出现 reqwest 类型」 | Step 7.5(B)（源码本就 0 引用，本次不改源码故保持 0） | ✅ |
| 同上 — 「三个例外 crate 仍可声明 reqwest」 | Task 1（保留 core/client/webhook）+ Step 7.5(C) | ✅ |
| Transport 边界显式文档化 — ARCHITECTURE.md 含边界约定 | Task 4 + Step 4.2 | ✅ |
| webhook by-design 例外文档化 — ARCHITECTURE.md 记录 | Task 4 + Step 4.2 | ✅ |
| 同上 — webhook crate 文档注释说明例外 | Task 5 + Step 5.3 | ✅ |
| Transport 边界由守卫脚本机器检验 — 脚本存在 | Task 2 + Step 7.6 | ✅ |
| 同上 — 清理后守卫通过 | Step 2.3 + Step 7.4 | ✅ |
| 同上 — 守卫能抓回归 | Step 2.4 | ✅ |
| 同上 — 守卫接入 CI 与 justfile | Task 3 + Step 7.5(D) | ✅ |
| ARCHITECTURE.md 中间件层标注为 future | Task 4（边界小节 future 段）+ Step 4.2 | ✅ |
| 清理不破坏构建 — 全 feature 构建通过 | Step 7.1 | ✅ |
| 同上 — 三组 feature clippy 通过 | Step 7.2 | ✅ |
| 同上 — 测试通过 | Step 7.3 | ✅ |

**2. 占位符扫描：** 全文无 TBD/TODO/"实现细节后填"/"类似 Task N"。所有代码块（Cargo.toml 改动、shell 脚本、doc 注释、CHANGELOG 条目、grep 命令）均给出确切文本。

**3. 类型/命名一致性：** 守卫脚本名 `check_reqwest_boundary.sh`、recipe 名 `reqwest-boundary`、CI step 名「Check Transport/reqwest boundary」、ARCHITECTURE.md 小节名「Transport HTTP 边界」—— 全文四处引用一致。白名单三 crate（core/client/webhook）在脚本白名单、Task 1 保留列表、ARCHITECTURE.md 边界表、CHANGELOG 条目中完全一致。12 业务 crate 名单在 Task 1、CHANGELOG、Step 7.5 grep 循环中完全一致。

**4. 与 design 的偏差（已记录）：** design 第 3.1 节表述「各删 `[dependencies]` 下的 `reqwest = { workspace = true }` 一行」对 11 个 crate 成立，但 **auth 例外**——它是 `optional = true` 且被 `oauth` feature 引用，删依赖行须同步从 `oauth = ["reqwest", "url"]` 移除 `"reqwest"`（Task 1 Step 1.2 已专门处理，并在 Global Constraints 标为陷阱）。这是 design 未明说但实现必需的步骤，本计划已覆盖。

archived-with: 2026-06-29-clarify-transport-reqwest-boundary
---

## 执行交接

计划已完成并保存至 `docs/superpowers/plans/2026-06-29-clarify-transport-reqwest-boundary.md`。两种执行方式：

**1. Subagent-Driven（推荐）** — 每个 Task 派发独立 subagent，任务间审查，快速迭代

**2. Inline Execution** — 在当前会话用 executing-plans 批量执行，带检查点审查

选哪种？
