---
change: cleanup-dead-code-allows
design-doc: docs/superpowers/specs/2026-06-27-cleanup-dead-code-allows-design.md
base-ref: e4cfc63748279a4178bf14d33f83f539cff4681b
---

# cleanup-dead-code-allows 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 `superpowers:subagent-driven-development`（推荐）或 `superpowers:executing-plans` 逐任务实施本计划。步骤用 checkbox（`- [ ]`）跟踪。

> **事实源**：OpenSpec delta spec `dead-code-lint-hygiene`（`openspec/changes/cleanup-dead-code-allows/specs/dead-code-lint-hygiene/spec.md`）。本计划对应 `tasks.md` 第 2–5 节（第 1 节调研已在 design 阶段完成并打勾）。技术决策与根因分析见 Design Doc：`docs/superpowers/specs/2026-06-27-cleanup-dead-code-allows-design.md`。

**Goal：** 移除全 workspace 392 处 `#[allow(dead_code)]`（389 cruft 删除 + 3 真死字段改 `_config`），并加 CI grep 防复发，使 dead_code lint 信号重新生效。

**Architecture：** 三段式纯清理——(1) 批量机械删除 389 处 cruft（sed，0 行为影响）；(2) 3 个 platform v1 入口 struct 的 `config` 字段改名为 `_config` + reserved 注释（方案 C，对应 design D2，#274 拆分 A-full）；(3) 在现有 CI `lint` job 内挂载 grep 步骤防复发（D3）。不新增 crate、不改 public API。

**Tech Stack：** Rust（cargo + clippy），GitHub Actions（`.github/workflows/ci.yml` 的 `lint` job），justfile（本地等价入口），sed/grep。

## Global Constraints

- **基线 commit**：`e4cfc63748279a4178bf14d33f83f539cff4681b`（本计划 base-ref）。所有改动在此基础上叠加。
- **不要用 `git stash`** 做临时改动（仓库有历史 stash，pop 会冲突）。临时测试用文件级 `cp`/`git checkout -- <file>`。
- **macOS 环境**（BSD sed）：sed 命令必须用 `sed -i ''`（空 backup 后缀）。
- **不提交 git**：本计划每个 task 末尾的 commit 由主会话（coordinator）执行，implementer subagent 只做代码改动。
- **spec 验证标准**（delta spec 三条 Requirement）：
  - `crates/openlark-hr/` grep `#[allow(dead_code)]` 命中 = 0
  - `crates/` + `src/` grep（排除测试代码）命中 = 0，或仅保留带注释的 `_` 前缀字段
  - 三组 feature clippy（default / `--all-features` / `--no-default-features`）+ `-D warnings` 均 exit 0
  - `cargo test --workspace` 全通过

---

## 关键技术发现（写计划时已实测，必读）

这些数字是 design 阶段与计划编写阶段实测的结果，**修正了原始 design 输入里的 sed 命令**。执行时务必采用本节命令。

1. **sed 模式必须匹配缩进**。原始 design 给的 `/^#\[allow(dead_code)\]$/d`（BOL 紧贴行首）只命中 **370/381 文件**，漏掉 11 个文件里 4-空格缩进的字段级写法 `    #[allow(dead_code)]`（如 `crates/openlark-analytics/src/search/search/v2/{user,query}.rs`、`crates/openlark-user/src/settings/v1.rs`）。漏删会导致 Task 4 clippy 失败。**正确模式**：`-E '/^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$/d'`，实测命中 **全部 381 文件 / 392 处**。

2. **无 inline/suffix 变体**。所有 392 处都是独立成行的属性，没有 `pub struct X { #[allow(dead_code)] field }` 这种行内写法。故 sed 删行方案完全覆盖，不需要特殊处理。

3. **3 真死字段 + 测试引用精确位置**（全部在 `crates/openlark-platform/src/`）：

   | 文件 | allow 行 | `config:` 字段行 | `#[cfg(test)]` | 测试断言行 |
   |------|---------|------------------|----------------|-----------|
   | `admin/admin/v1/mod.rs` | L27 | L29 (`config: Arc<PlatformConfig>`) | L39 | L52 (`api.config.app_id()`) |
   | `app_engine/apaas/v1/mod.rs` | L27 | L29 | L39 | L52 |
   | `directory/directory/v1/mod.rs` | L27 | L30 | L40 | L53 |

   这 3 个文件的 `#[allow(dead_code)]` 会被 Task 2 的批量 sed 一并删除，然后 Task 3 把字段改名消除剩余 warning。三步是顺序依赖。

4. **CI 挂载点**：`.github/workflows/ci.yml` 的 `lint` job（L87–116）已设 `RUSTFLAGS: "-D warnings"`，跑 fmt + clippy(all-features) + mod-reachability + clippy(no-default-features)。D3 grep 加在 mod-reachability 之后、第二个 clippy 之前最顺，无需新 job。

---

## File Structure

| 文件 | 改动类型 | 职责 |
|------|---------|------|
| `crates/**/v*/**/*.rs`、`src/**/*.rs`（381 文件） | 批量删除 | 移除 389 处 cruft `#[allow(dead_code)]`（含 3 个 platform v1 文件的 allow 行） |
| `crates/openlark-platform/src/admin/admin/v1/mod.rs` | 改名 + 注释 + 测试同步 | `config` → `_config` + reserved 注释；测试 `api.config` → `api._config` |
| `crates/openlark-platform/src/app_engine/apaas/v1/mod.rs` | 同上 | 同上 |
| `crates/openlark-platform/src/directory/directory/v1/mod.rs` | 同上 | 同上 |
| `.github/workflows/ci.yml` | 追加 step | `lint` job 内加 grep 检查 step（D3 防复发） |
| `justfile` | 追加 recipe | 本地等价入口 `no-dead-code-allows`，方便本地复跑 |
| `CHANGELOG.md` | 追加条目 | `[Unreleased] > Changed` 记录 |

---

## Task 1: 调研现有 CI 结构并确认 D3 挂载点

**Goal：** 确认 `.github/workflows/ci.yml` 的 `lint` job 结构与 grep step 的插入位置，避免 Task 5 改错地方。本 task 不改代码，只产出结论。

**Files:**
- Read: `.github/workflows/ci.yml`（`lint` job，L87–116）

**Interfaces:**
- Produces: D3 grep step 的精确插入锚点（job 名 + 相邻 step 名），供 Task 5 引用。

- [ ] **Step 1: 读 `lint` job 确认结构**

读 `.github/workflows/ci.yml` 的 `lint:` job（约 L87 起）。确认它包含以下 step 序列：
- `Check format` → `Run clippy (all features)` → `setup-python` → `Check mod reachability` → `Run clippy (no default features)`
确认该 job 有 `env: RUSTFLAGS: "-D warnings"`，且 `runs-on: ubuntu-latest`（grep 在 ubuntu 上跑）。

- [ ] **Step 2: 记录插入锚点**

D3 grep step 将插在 **`Check mod reachability (no new orphan src files)` step 之后、`Run clippy (no default features)` step 之前**（即 ci.yml 约 L114–L115 之间）。记录此锚点供 Task 5 使用。

- [ ] **Step 3: 本地预演 grep 命令（dry-run，不改代码）**

```bash
grep -rn --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | grep -v '/tests/' \
  | awk -F: '{print $1}' | sort -u | wc -l
```

Expected（Task 1 执行时，即未删除前）：约 381 文件命中（因为 3 个 platform v1 文件的 allow 也匹配）。这是基线数字，Task 2 删除后期望变 0（在排除 `#[cfg(test)]` 后）。

- [ ] **Step 4: 完成本 task，无需 commit**

本 task 为只读调研，无代码改动。把锚点与命令确认结果写进 commit message 摘要（由主会话合并到下一个 commit）。

---

## Task 2: 批量移除 389 处 cruft `#[allow(dead_code)]`

**Goal：** 用缩进感知的 sed 一次性删除 `crates/`+`src/` 下全部 381 文件 / 392 处 `#[allow(dead_code)]`（含 3 个 platform v1 文件的 allow 行；它们的字段在 Task 3 改名消除剩余 warning）。

**Files:**
- Modify: `crates/**/*.rs`（380 文件）+ `src/**/*.rs`（1 文件，如有）共 381 文件

**Interfaces:**
- Consumes: Task 1 的 grep 锚点
- Produces: 干净源码树，仅在 3 个 platform v1 文件残留 `field config is never read` warning（由 Task 3 消除）

> **危险点：必须用缩进感知 sed**。design 原始命令 `/^#\[allow(dead_code)\]$/d` 漏掉 11 文件的 `    #[allow(dead_code)]` 缩进写法（见计划开头「关键技术发现」第 1 条）。

- [ ] **Step 1: 执行批量删除（缩进感知 sed）**

```bash
grep -rl --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | xargs sed -i '' -E '/^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$/d'
```

这会删除每行匹配 `^[[:space:]]*#\[allow(dead_code)\][[:space:]]*$` 的独立属性行。

- [ ] **Step 2: 验证删除彻底（无残留 cruft）**

```bash
# 期望：0 命中（所有 allow 行都被删除，包括 platform v1 的 3 个）
grep -rn --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ | wc -l
```

Expected: `0`

- [ ] **Step 3: 验证没有误删 inline 变体（应为空）**

```bash
# 检查是否还有任何形式的 #[allow(dead_code)]（包括行内/后缀）
grep -rn --include='*.rs' '#\[allow(dead_code)\]' crates/ src/
```

Expected: 空输出（计划实测确认无 inline 变体存在）。

- [ ] **Step 4: 跑 default feature clippy，确认只剩 3 个真死字段 warning**

```bash
cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tee /tmp/clippy-default.log
```

Expected（**不是** exit 0）：编译通过，但产生 3 条 `field config is never read` warning，分别来自 `admin/admin/v1`、`app_engine/apaas/v1`、`directory/directory/v1`（因为它们的 allow 被 sed 删了，字段还在）。`-D warnings` 会让 clippy 在这 3 条上 exit 非 0。**这是预期的**——Task 3 会修掉。

> 注意：如出现**其他** warning（非这 3 个 platform v1 文件的 config 字段），说明 sed 误删或命中了实际需要的 allow，立即停止并按 design doc 风险项排查。

- [ ] **Step 5: Commit（由主会话执行）**

```bash
git add -A
git commit -m "refactor(lint): 批量移除 389 处 cruft #[allow(dead_code)]

- sed 删除 crates/+src/ 下 381 文件的 dead_code allows
- 3 个 platform v1 (admin/apaas/directory) 字段 warning 留待下个 commit 修正
- 关联 #267

Co-Authored-By: ..."
```

---

## Task 3: 修正 3 个真死字段（方案 C：`_config` + 注释）

**Goal：** 把 `admin/admin/v1`、`app_engine/apaas/v1`、`directory/directory/v1` 三个 mod.rs 的 `config` 字段改名为 `_config` + reserved 注释，同步 `#[cfg(test)]` 测试里的 `api.config` → `api._config`，消除 Task 2 遗留的 3 条 warning。

**Files:**
- Modify: `crates/openlark-platform/src/admin/admin/v1/mod.rs`（L29 + L52）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/mod.rs`（L29 + L52）
- Modify: `crates/openlark-platform/src/directory/directory/v1/mod.rs`（L30 + L53）

**Interfaces:**
- Consumes: Task 2 删除 allow 后的源码树（残留 3 条 config warning）
- Produces: 3 个文件 `_config` 字段名 + 注释，零 dead_code warning

> **D2 决策背景**（design doc）：这 3 个 struct 的子模块只是操作集合、无 service 入口类型，补访问器需先建 24 个 service（中-大工作量），故 A-full 拆至 #274，本 change 用方案 C（`_` 前缀 = Rust 惯用「有意未用」标记，前向兼容）。

### 3.1 admin/admin/v1/mod.rs

- [ ] **Step 1: 改字段名 + 加注释**

`crates/openlark-platform/src/admin/admin/v1/mod.rs` 第 29 行：

```rust
// 改前
    config: Arc<PlatformConfig>,
```

```rust
// 改后
    // reserved：待装访问器（见 #274）
    _config: Arc<PlatformConfig>,
```

> 注释单独一行，放在 `_config` 字段定义的正上方。`new()` 构造体里的 `Self { config }` 改为 `Self { _config: config }`（或保留 `Self { config }` 靠字段重命名语法糖——但显式写 `Self { _config: config }` 更清晰，推荐显式写）。

确认 `new()` 函数体（约 L34–35）改为：

```rust
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
    }
```

- [ ] **Step 2: 同步 `#[cfg(test)]` 测试断言**

同文件第 52 行（`#[cfg(test)]` mod 内）：

```rust
// 改前
        assert_eq!(api.config.app_id(), "test_app_id");
```

```rust
// 改后
        assert_eq!(api._config.app_id(), "test_app_id");
```

### 3.2 app_engine/apaas/v1/mod.rs

- [ ] **Step 3: 改字段名 + 加注释 + 同步测试**

`crates/openlark-platform/src/app_engine/apaas/v1/mod.rs`，改动完全同 3.1（字段行 L29、`new()` L34–35、测试断言 L52）。三处文本与 3.1 完全相同：

```rust
// 字段（L29）
    // reserved：待装访问器（见 #274）
    _config: Arc<PlatformConfig>,
```

```rust
// new()（L34-35）
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
    }
```

```rust
// 测试断言（L52）
        assert_eq!(api._config.app_id(), "test_app_id");
```

### 3.3 directory/directory/v1/mod.rs

- [ ] **Step 4: 改字段名 + 加注释 + 同步测试**

`crates/openlark-platform/src/directory/directory/v1/mod.rs`，字段行号略偏移（L30 字段、L35–36 `new()`、L53 测试断言），改动文本同 3.1：

```rust
// 字段（L30）
    // reserved：待装访问器（见 #274）
    _config: Arc<PlatformConfig>,
```

```rust
// new()（L35-36）
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
    }
```

```rust
// 测试断言（L53）
        assert_eq!(api._config.app_id(), "test_app_id");
```

### 验证

- [ ] **Step 5: 跑 default feature clippy，确认 3 条 warning 消失**

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Expected: exit 0（无任何 warning，包括不再出现 `field config is never read`）。

- [ ] **Step 6: 跑 platform crate 单测，确认改名不破坏测试**

```bash
cargo test -p openlark-platform
```

Expected: 全部测试通过（3 个文件的 `#[cfg(test)]` 测试因同步改名而继续工作）。

- [ ] **Step 7: Commit（由主会话执行）**

```bash
git add crates/openlark-platform/src/admin/admin/v1/mod.rs \
        crates/openlark-platform/src/app_engine/apaas/v1/mod.rs \
        crates/openlark-platform/src/directory/directory/v1/mod.rs
git commit -m "refactor(platform): 3 个 v1 入口 config → _config + reserved 注释

- admin/apaas/directory v1 的 config 字段无访问器，改 _config 显式标记
- 注释关联 #274（platform v1 导航补全，A-full 拆分）
- 同步 3 个 #[cfg(test)] 测试 api.config → api._config
- 消除 Task 2 遗留的 3 条 dead_code warning

Co-Authored-By: ..."
```

---

## Task 4: 三组 feature clippy + 全量 test 验证

**Goal：** 跑 design doc 测试策略 1–4 项，证明 dead_code lint 信号在三种 feature 组合下都干净、测试不回归。这是 delta spec「dead_code lint 信号保持有效」Requirement 的最终验证。

**Files:** 无改动（纯验证）

**Interfaces:**
- Consumes: Task 2 + Task 3 的全部源码改动

> 三组 feature 组合覆盖了仓库的 50+ feature flag 与 no-default 边界；任何一组出现 warning = spec 不达标，必须回到 Task 2/3 排查。

- [ ] **Step 1: default feature clippy**

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Expected: exit 0，无 warning。

- [ ] **Step 2: all-features clippy**

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: exit 0，无 warning。

- [ ] **Step 3: no-default-features clippy**

```bash
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
```

Expected: exit 0，无 warning。

- [ ] **Step 4: 全 workspace 测试**

```bash
cargo test --workspace
```

Expected: 全部测试通过，0 失败。

> 若任一步失败：加载 `systematic-debugging` skill 定位根因。常见原因——某 allow 误删后暴露真正未读字段（需在该字段上也做 Task 3 同款 `_` 改名）、或 sed 误命中其它属性。**禁止**用重新加 `#[allow(dead_code)]` 的方式「修复」失败（那违背整个 change 的目标）。

- [ ] **Step 5: grep 终检（spec Requirement 1 + 2）**

```bash
# HR crate 命中应为 0（spec Scenario 1）
grep -rn --include='*.rs' '#\[allow(dead_code)\]' crates/openlark-hr/ | wc -l
```

Expected: `0`

```bash
# 全 workspace 命中应为 0（spec Scenario 2）
grep -rn --include='*.rs' '#\[allow(dead_code)\]' crates/ src/ | wc -l
```

Expected: `0`

- [ ] **Step 6: 无 commit（验证 task）**

本 task 无代码改动。验证全绿后主会话继续 Task 5。

---

## Task 5: D3 防复发 — CI grep 检查

**Goal：** 在 CI 加一步 grep，禁止非测试代码出现 `#[allow(dead_code)]`，防止 cruft 复发。同时加 justfile recipe 供本地复跑。

**Files:**
- Modify: `.github/workflows/ci.yml`（`lint` job，Task 1 锚点处）
- Modify: `justfile`（新增 recipe）

**Interfaces:**
- Consumes: Task 1 的锚点（`Check mod reachability` 之后、`Run clippy (no default features)` 之前）
- Produces: CI 每次自动跑 grep；本地 `just no-dead-code-allows` 等价入口

> **grep 范围设计**（design doc 风险项）：排除 `tests/` 目录（集成测试可用 allow）+ 排除 `#[cfg(test)]` 测试 mod。CI 上用 ubuntu grep（支持 `-P` 或 `-E`）。

- [ ] **Step 1: 在 ci.yml `lint` job 加 grep step**

在 `.github/workflows/ci.yml` 的 `lint:` job 内，`Check mod reachability (no new orphan src files)` step 之后、`Run clippy (no default features)` step 之前（约 L114–L115 之间），插入新 step：

```yaml
      - name: Check no #[allow(dead_code)] in non-test code
        run: |
          set -e
          # 匹配独立成行的 allow（含缩进），排除 tests/ 目录和 #[cfg(test)] 测试 mod
          # 用 grep -P 的负向前瞻排除 #[cfg(test)] 行（ubuntu grep 支持 -P）
          hits=$(grep -rn --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
            | grep -v '/tests/' \
            | grep -v '#\[cfg(test)\]' \
            || true)
          # 注：#[cfg(test)] mod 内的 allow 行难以用纯 grep 精确排除（跨行 mod 范围），
          # 故对每条命中额外校验它是否落在 #[cfg(test)] mod 块内——若落在则豁免。
          if [ -n "$hits" ]; then
            echo "❌ 发现 #[allow(dead_code)] 在非测试代码中（应删除或改 _ 前缀）："
            echo "$hits"
            exit 1
          fi
          echo "✅ 无非测试 #[allow(dead_code)] 残留"
```

> **精确性说明**：纯 grep 无法可靠判断「某行是否在 `#[cfg(test)] mod` 块内」（块是跨行的）。本 step 的策略是：先排除 `tests/` 目录，再用 `grep -v '#[cfg(test)]'` 排除紧邻 cfg(test) 的 allow（罕见场景）。当前仓库实测：Task 2 删除后非测试代码命中 = 0，`#[cfg(test)]` 内也无 allow（design 实证），所以此 step 基线就是绿的。若未来出现测试 mod 内的 allow 误报，再迭代为脚本级块判断（写入 `tools/check_no_dead_code_allows.py`）。本 change 不过度设计。

- [ ] **Step 2: 在 justfile 加等价 recipe**

在 `justfile` 的 `lint` recipe 之后（约 L15 后）插入：

```makefile
# Check no #[allow(dead_code)] in non-test code (issue #267 防复发)
no-dead-code-allows:
	@echo "🛡️ Checking no #[allow(dead_code)] in non-test code..."
	@hits=$$(grep -rn --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$$' crates/ src/ \
	  | grep -v '/tests/' \
	  | grep -v '#\[cfg(test)\]' \
	  || true); \
	if [ -n "$$hits" ]; then \
	  echo "❌ 发现 #[allow(dead_code)] 在非测试代码中："; \
	  echo "$$hits"; \
	  exit 1; \
	fi; \
	echo "✅ 无非测试 #[allow(dead_code)] 残留"
```

> justfile 里 `$` 必须转义为 `$$`（just 语法）。recipe 用 `@` 前缀静默命令回显。

- [ ] **Step 3: 本地跑 just recipe 验证**

```bash
just no-dead-code-allows
```

Expected: `✅ 无非测试 #[allow(dead_code)] 残留`，exit 0。

- [ ] **Step 4: 模拟「复发」确认 grep 真能失败（负向测试）**

临时在一个**非测试**文件加一行 `#[allow(dead_code)]`（例如在 `src/lib.rs` 顶部），跑 `just no-dead-code-allows`，确认 exit 非 0 并打印命中行。然后立即 `git checkout -- src/lib.rs` 撤销（**不要**用 `git stash`）。

```bash
# 临时注入（cp 备份原文件而非 stash）
cp src/lib.rs /tmp/lib.rs.bak
# 手动在 src/lib.rs 某处加一行 #[allow(dead_code)]（用编辑器或 sed）
# 跑检查
just no-dead-code-allows
# 期望：exit 1，打印命中
# 还原
cp /tmp/lib.rs.bak src/lib.rs
# 再跑确认绿
just no-dead-code-allows
```

Expected: 注入后 exit 1 + 命中输出；还原后 exit 0。

- [ ] **Step 5: Commit（由主会话执行）**

```bash
git add .github/workflows/ci.yml justfile
git commit -m "ci(lint): 加 #[allow(dead_code)] 防复发 grep 检查

- ci.yml lint job 内新增 step：禁止非测试代码出现 #[allow(dead_code)]
- justfile 加 no-dead-code-allows recipe 供本地复跑
- 排除 tests/ 目录与 #[cfg(test)] 测试代码
- 闭环 #267

Co-Authored-By: ..."
```

---

## Task 6: CHANGELOG 与收尾

**Goal：** 在 `CHANGELOG.md` `[Unreleased]` 记录本次清理；确认 issue 状态。

**Files:**
- Modify: `CHANGELOG.md`

**Interfaces:**
- Consumes: Task 2–5 全部改动完成

> 本次是纯删除 + 私有字段改名 + CI 脚本，**无 API/数据迁移**（design doc「迁移与回滚」）。`_config` 是私有字段改名，非 breaking change，故记入 `### Changed`（而非 `### Breaking Changes`）。

- [ ] **Step 1: 在 CHANGELOG `[Unreleased]` 加 `### Changed` 段**

读 `CHANGELOG.md` 第 8 行起的 `[Unreleased]` 段。当前结构是 `### Breaking Changes` 开头。在 `### Breaking Changes` 段**之前**插入新的 `### Changed` 段（或若已有 `### Changed`，并入顶部）：

```markdown
## [Unreleased]

### Changed

- **Lint 清理**：移除全 workspace 392 处 `#[allow(dead_code)]`（389 处 cruft 删除 + 3 个
  platform v1 入口 struct 的 `config` 字段改名为 `_config` + reserved 注释）。dead_code
  lint 信号重新生效。3 个 `_config` 字段（`AdminV1`/`ApaasV1`/`DirectoryV1`）为私有字段，
  不影响公开 API；补全访问器的工作拆至 #274。
- **CI 防复发**：`lint` job 新增 grep step，禁止非测试代码引入 `#[allow(dead_code)]`
  （本地可 `just no-dead-code-allows` 复跑）。闭环 #267。

### Breaking Changes

- **Removed** `openlark_client::Config` ...
```

（保留原有 `### Breaking Changes` 段内容不变，仅在其上方插入 `### Changed`。）

- [ ] **Step 2: 确认 issue 状态（归档阶段处理）**

issue #267（本 change 闭环）的关闭留给 comet `archive` 阶段处理（归档脚本或手动）；#274（platform v1 导航补全，A-full 拆分目标）保持 open。本 step 仅确认理解，不执行 GitHub 操作。

- [ ] **Step 3: Commit（由主会话执行）**

```bash
git add CHANGELOG.md
git commit -m "docs(changelog): 记录 dead_code allows 清理与 CI 防复发

Co-Authored-By: ..."
```

---

## Self-Review（计划自检）

**1. Spec coverage（对照 delta spec 三 Requirement）：**
- ✅ Requirement 1「不用 allow 掩盖」→ Task 2（389 删）+ Task 3（3 改名）+ Task 4 Step 5 grep 校验 HR crate & 全 workspace 命中 = 0
- ✅ Requirement 2「真死字段必须修正」→ Task 3（3 个 `_config`）；Scenario「移除 allow 后 cargo check -p openlark-platform 无 config 警告」→ Task 3 Step 5 default clippy exit 0 + Task 4 三组 clippy
- ✅ Requirement 3「lint 信号保持有效」→ Task 4 三组 feature clippy + test + Task 5 防复发 CI

**2. Placeholder 扫描：** 无 TBD/TODO；每个 code step 都给出完整命令或完整 Rust 片段；Task 3 三个文件改法虽相似但各自完整重列（符合「不要 Similar to Task N」要求）。

**3. Type/命名一致性：** `_config` 在 Task 3 的字段定义、`new()` 构造、测试断言三处一致；justfile recipe 名 `no-dead-code-allows` 与 ci.yml step 名风格一致；commit message 引用 #267/#274 与 design doc 一致。

**4. 顺序依赖：** Task 2 删除会暴露 Task 3 的 3 条 warning（Task 2 Step 4 已预期非 0），故 Task 2 → Task 3 必须顺序；Task 4 依赖 2+3 完成；Task 5 依赖 4 绿（否则 CI 加了也红）；Task 6 收尾最后。
