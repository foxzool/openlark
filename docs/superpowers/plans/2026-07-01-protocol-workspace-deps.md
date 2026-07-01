---
change: protocol-workspace-deps
design-doc: docs/superpowers/specs/2026-07-01-protocol-workspace-deps-design.md
base-ref: 61faa8fe11651daf7378a422e17682df0e0f93b7
---

# protocol-workspace-deps 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 按任务逐个实施本计划。步骤用 checkbox（`- [ ]`）语法跟踪。

**目标：** 把 `openlark-protocol` 的 `bytes`/`prost` 两个依赖从 crate 级钉版本迁移到 workspace 统一声明，消除治理重复，并在根 `Cargo.toml` 落盘 `bytes` 的 workspace 版本。

**架构：** 2 处原子 `Cargo.toml` 编辑。`bytes` 当前仅 protocol 用、未进 workspace → 新增根声明；`prost` 已有 workspace 声明 → protocol 直接改 `{ workspace = true }`。`prost-build`（build-dep，vendored protobuf 工具链）保持不动。Cargo.lock resolved 版本不变（`bytes`=1.11.1、`prost`=0.13.5），故无需同步 MSRV lockfile。

**技术栈：** Rust workspace、Cargo `[workspace.dependencies]`、`just` 任务、`cargo tree -d`/`cargo deny`。

## 变更性质

这是一个 **small / 低风险** 变更：
- resolved 版本不变（Design Doc §2 已核实），无 lockfile/MSRV 影响
- 范围克制（只动 protocol 的 bytes/prost，不扩到其他 crate / prost-build）
- 范式成熟（`openlark-core` 已有 38 个依赖走 `{ workspace = true }`）
- 按 `openspec/changes/protocol-workspace-deps/tasks.md` 的 4 组 11 个子任务逐项执行即可

## 全局约束

- **范围边界：** 只改 `bytes`/`prost` 两个依赖；`prost-build = "0.12.6"`（build-dependency）**严禁改动**（vendored protobuf 工具链，Design Doc §5 明确划出范围外）
- **诚实限制：** 本 change **不消除** prost 0.12/0.13 既存多版本 split（由 `prost-build 0.12.6` 引入）；验证断言是「**不引入新多版本**」而非「单一版本」
- **版本对齐：** workspace `bytes = "1.6"`（非 `"1.6.0"`），与 protocol 现 caret `^1.6.0` 范围一致；`prost` 复用已有 workspace `0.13` 声明
- **Cargo.lock 不变：** 迁移后 resolved 版本不变，**不**手改 Cargo.lock、**不**同步 `.github/msrv/Cargo.lock`
- **提交规范：** 中文 commit message，每次完成一个可验证单元即提交（不积攒）
- **MSRV：** Rust 1.88+

---

## 任务依赖与结构

本计划对齐 `tasks.md` 的 4 组结构：

| Task | 对应 tasks.md 组 | 内容 |
|------|------------------|------|
| Task 0 | — | 迁移前 baseline（`cargo tree -d` 存档） |
| Task 1 | 组 1 | 根 `Cargo.toml` 新增 bytes workspace 声明 |
| Task 2 | 组 2 | protocol 改消费 workspace（bytes + prost，2 处） |
| Task 3 | 组 3 | Cargo.lock 与 MSRV 同步验证（预期无变化） |
| Task 4 | 组 4 | 完整验证（fmt / lint 双模式 / build / tree -d 对比 / MSRV --locked / deny） |

---

## Task 0: 捕获迁移前 `cargo tree -d` baseline

**Files:**
- Create（临时）: `/tmp/openlark-tree-d-baseline.txt`（仅作 diff 对比基准，不提交）

**说明：** 关键验证是「迁移前后 `cargo tree -d` 重复条目 diff 为空」。为此必须先有迁移前 baseline。本任务不产生 git 改动。

- [x] **Step 1: 跑 `cargo tree -d --workspace` 并存档**

```bash
cargo tree -d --workspace > /tmp/openlark-tree-d-baseline.txt 2>&1
```

预期输出包含（迁移前已存在的既存重复，**不计入新增**）：
- `bytes v1.11.1`（多处引用，单一版本）
- `prost v0.12.6` 与 `prost v0.13.5`（vendored prost-build 引入的既存 split）
- `prost-derive v0.12.6` 与 `prost-derive v0.13.5`（同上）

- [x] **Step 2: 确认 baseline 已记录 bytes/prost 现状**

```bash
grep -nE "^bytes|^prost" /tmp/openlark-tree-d-baseline.txt
```

预期：能看到 `bytes v1.11.1`、`prost v0.12.6`、`prost v0.13.5` 等条目。**记下这个输出**，Task 4 会与之 diff。

---

## Task 1: 根 `Cargo.toml` 新增 bytes workspace 声明

**Files:**
- Modify: `Cargo.toml:87`（`[workspace.dependencies]` 段，紧邻已有 `prost = { version = "0.13" }`）

**Interfaces:**
- Produces: 根 `[workspace.dependencies]` 中的 `bytes = "1.6"` 声明，供 Task 2 的 protocol `bytes = { workspace = true }` 引用。

**对应 tasks.md：** 组 1（1.1）

- [x] **Step 1: 在 `prost` 行后新增 `bytes` 声明**

把 `Cargo.toml` 第 87 行：

```toml
prost = { version = "0.13" }
```

改为：

```toml
prost = { version = "0.13" }
bytes = "1.6"
```

> 用 `"1.6"`（非 `"1.6.0"`）——对齐 protocol 现 caret `^1.6.0`，resolved 仍为 1.11.1，零 lockfile 影响（Design Doc D1）。

- [x] **Step 2: 验证 TOML 仍可解析**

```bash
cargo metadata --no-deps --format-version 1 > /dev/null && echo "TOML OK"
```

预期：输出 `TOML OK`（无解析错误）。

- [x] **Step 3: 提交**

```bash
git add Cargo.toml
git commit -m "chore(workspace): 新增 bytes = \"1.6\" workspace 依赖声明

为 protocol 迁移到 { workspace = true } 做准备（issue #273 Part B）。"
```

---

## Task 2: `openlark-protocol` 改消费 workspace（bytes + prost）

**Files:**
- Modify: `crates/openlark-protocol/Cargo.toml:18-19`

**Interfaces:**
- Consumes: Task 1 的 `bytes = "1.6"` workspace 声明；以及既存的 `prost = { version = "0.13" }` workspace 声明。
- Produces: protocol 不再自钉 `bytes`/`prost` 版本，统一走 workspace。

**对应 tasks.md：** 组 2（2.1 bytes + 2.2 prost，合并为一次原子编辑）

- [x] **Step 1: 修改 protocol Cargo.toml 的 `[dependencies]` 段**

把 `crates/openlark-protocol/Cargo.toml` 的第 18-19 行：

```toml
bytes = "1.6.0"
prost = "0.13.1"
```

改为：

```toml
bytes = { workspace = true }
prost = { workspace = true }
```

> **不要动** 第 23 行的 `prost-build = "0.12.6"`（build-dependency，范围外）。
> **不要动** 第 27 行的 `ignored = ["bytes"]`（cargo-machete 元数据，bytes 仍被 prost 间接使用，标注保留）。

- [x] **Step 2: 验证 TOML 仍可解析**

```bash
cargo metadata --no-deps --format-version 1 > /dev/null && echo "TOML OK"
```

预期：输出 `TOML OK`。

- [x] **Step 3: 验证 workspace 依赖被正确解析（无 "cannot find" 错误）**

```bash
cargo build -p openlark-protocol 2>&1 | tail -5
```

预期：构建成功（无 `failed to parse manifest` / `cannot find bytes in workspace.dependencies` 错误）。若有错误，回查 Task 1 是否已落盘 `bytes = "1.6"`。

- [x] **Step 4: 提交**

```bash
git add crates/openlark-protocol/Cargo.toml
git commit -m "refactor(protocol): bytes/prost 改用 { workspace = true }

消除 crate 级钉版本，统一走 [workspace.dependencies]（issue #273 Part B）。

- bytes: \"1.6.0\" → { workspace = true }（workspace 声明 \"1.6\"）
- prost: \"0.13.1\" → { workspace = true }（workspace 已声明 \"0.13\"）
- prost-build 0.12.6 保持不动（build-dep，范围外）

resolved 版本不变（bytes 1.11.1 / prost 0.13.5），Cargo.lock 无变化。"
```

---

## Task 3: Cargo.lock 与 MSRV lockfile 同步验证

**Files:**
- Read-only: `Cargo.lock`、`.github/msrv/Cargo.lock`

**Interfaces:**
- Consumes: Task 1+2 的 Cargo.toml 改动。

**对应 tasks.md：** 组 3（3.1 + 3.2）

**预期结论：** resolved 版本不变 → Cargo.lock 无变化 → 无需同步 MSRV lockfile。本任务用 `git diff` 证实这一点。

- [x] **Step 1: 跑 `cargo update -p bytes -p prost --workspace` 触发解析器重新求解**

```bash
cargo update -p bytes -p prost --workspace 2>&1 | tail -10
```

预期：输出 `Updating`/`Locking` 行数极少或为空（resolved 版本应保持 `bytes 1.11.1` / `prost 0.13.5`）。若输出显示版本变化，**停下来查 Design Doc §2**——可能是 workspace 版本写错（应为 `"1.6"` 而非 `"1.6.0"` / `"0.13"` 而非 `"0.13.1"`）。

> **执行结果订正（build 阶段发现）：** `cargo update` 是**错误**的验证手段——它本身会"更新到最新兼容版"（实测把 bytes 1.11.1 → 1.12.0、还移除 itertools 0.14.0），与迁移无关（原 `bytes = "1.6.0"` 同样允许 1.12.0）。已 `git checkout Cargo.lock` 回滚。**正确验证 = `cargo build --locked`**（证明既有 lockfile 仍满足新 Cargo.toml），实测 `Finished` 成功、bytes 仍 1.11.1、prost 仍 0.12.6/0.13.5。结论：**迁移本身不改变 Cargo.lock**。

- [x] **Step 2: 确认 Cargo.lock 无变化**

```bash
git diff --stat Cargo.lock
```

预期：**空输出**（Cargo.lock 未改动）。若 `git diff` 显示了变化，记录是哪些 package 变了版本——这违反了 Design Doc §2 的预期，需要回到 Task 1/2 检查 workspace 版本字符串。

- [x] **Step 3: 确认 `.github/msrv/Cargo.lock` 无需同步**

由于 Step 2 已证明根 `Cargo.lock` 不变，`.github/msrv/Cargo.lock`（pin 的副本）也**无需同步**。明确跳过 tasks.md 3.2 的"若变化"分支（Q1 决议：无需做）。

```bash
# 仅作 sanity check：MSRV lockfile 存在且未被本 change 触及
ls -la .github/msrv/Cargo.lock
git status .github/msrv/Cargo.lock
```

预期：文件存在；`git status` 无输出（未改动）。

> **本 Task 不产生 commit**（无文件改动）。若 Step 1/2 显示 Cargo.lock 有变化，则属于意外，需回 Task 1/2 修复后重跑。

---

## Task 4: 完整验证

**Files:**
- Read-only: 全 workspace

**Interfaces:**
- Consumes: Task 1+2 的所有改动。

**对应 tasks.md：** 组 4（4.1 ~ 4.5）

- [x] **Step 1: `cargo fmt --check`（tasks 4.1）**

```bash
cargo fmt --check
```

预期：无输出、退出码 0（本 change 只改 Cargo.toml，正常应直接过）。

> **注意：** CI lint job 第一步就是 `cargo fmt --check`，clippy 通过 ≠ fmt 通过，必须显式跑。

- [x] **Step 2: `just lint` 双模式（tasks 4.2）**

```bash
just lint
```

预期：CI 双模式（`--all-features` + `--no-default-features`，`-Dwarnings`）均过。本 change 不改 `.rs` 代码，应直接通过。

- [x] **Step 3: `cargo build --workspace --all-features`（tasks 4.3）**

```bash
cargo build --workspace --all-features 2>&1 | tail -5
```

预期：`Finished` 成功，无 error。

- [x] **Step 4: `cargo tree -d` 对比 baseline —— 关键断言「不引入新多版本」（tasks 4.5 的一半）**

```bash
cargo tree -d --workspace > /tmp/openlark-tree-d-after.txt 2>&1
diff /tmp/openlark-tree-d-baseline.txt /tmp/openlark-tree-d-after.txt
```

**预期：diff 为空**（迁移前后重复条目完全一致）。

**断言细则：**
- `bytes` 仍单版本 `1.11.1`（无新增）
- `prost v0.12.6` / `prost v0.13.5` split **维持原状**（这是 vendored prost-build 引入的既存 split，Design Doc §5 明确不计入新增）
- `prost-derive v0.12.6` / `prost v0.13.5` 同上

若 diff 非空（出现新的重复条目），**停下来排查**——可能是 workspace 版本声明写错导致解析到新版本。

- [x] **Step 5: `cargo deny check`（tasks 4.5 的另一半）**

```bash
cargo deny check 2>&1 | tail -15
```

预期：无新冲突（`finished` on all checks）。本 change resolved 版本不变，不应引入新 license/ban/advisory 告警。

- [x] **Step 6: MSRV `--locked` 验证（tasks 4.4）**

用 pin 的 `.github/msrv/Cargo.lock` 跑（docker rust:1.88 或本地 rustc 1.88）：

**选项 A：docker（推荐，CI 同款）**

```bash
# 拷贝 msrv lockfile 覆盖根 Cargo.lock（CI 同款做法）
cp .github/msrv/Cargo.lock Cargo.lock.msrv-snapshot

docker run --rm -v "$PWD":/workspace -w /workspace rust:1.88 \
  sh -c 'cp Cargo.lock.msrv-snapshot Cargo.lock && \
         cargo build --workspace --all-features --locked 2>&1 | tail -10'

# 清理临时文件
rm Cargo.lock.msrv-snapshot
```

**选项 B：本地 rustc 1.88**

```bash
cp .github/msrv/Cargo.lock Cargo.lock.msrv-snapshot
cp Cargo.lock.msrv-snapshot Cargo.lock
cargo build --workspace --all-features --locked 2>&1 | tail -10
rm Cargo.lock.msrv-snapshot
# 还原原 Cargo.lock（git 已跟踪，直接 checkout）
git checkout Cargo.lock
```

预期：`Finished` 成功（`--locked` 不报 `Cargo.lock needs update`）。由于 Cargo.lock 未变，应直接通过。

> 若 `--locked` 报 lockfile 需更新，说明 Cargo.lock 实际有变化——回 Task 3 Step 2 复查。

- [x] **Step 7: 全部验证通过后，本 change 无额外 commit**

本 Task 全是验证命令，不产生代码改动。所有验证通过即代表 change 实施完成，可进入 comet verify 阶段。

---

## 完成判据

全部满足即视为实施完成：

1. ✅ Task 1：根 `Cargo.toml` 有 `bytes = "1.6"` workspace 声明（紧邻 `prost`）
2. ✅ Task 2：`crates/openlark-protocol/Cargo.toml` 的 `bytes`/`prost` 均为 `{ workspace = true }`，`prost-build` 未动
3. ✅ Task 3：`git diff Cargo.lock` 为空（resolved 版本不变），`.github/msrv/Cargo.lock` 未被同步
4. ✅ Task 4：fmt / lint 双模式 / build / cargo tree -d diff（空）/ cargo deny / MSRV `--locked` 全过
5. ✅ `cargo tree -d` 前后 diff 为空（不引入新多版本）—— 本 change 的核心断言

---

## 自检（Self-Review）

**Spec coverage：**
- Design Doc §3 改动 1（根 Cargo.toml bytes）→ Task 1 ✓
- Design Doc §3 改动 2（protocol bytes/prost workspace）→ Task 2 ✓
- Design Doc §5（prost-build 不动）→ Task 2 Step 1 备注 + 全局约束 ✓
- Design Doc §6 测试矩阵全部命令 → Task 4 Step 1-6 ✓（fmt/lint/build/tree -d/deny/MSRV --locked）
- Design Doc D3（无需同步 MSRV lockfile）→ Task 3 Step 3 ✓
- tasks.md 组 1（1.1）→ Task 1 ✓
- tasks.md 组 2（2.1, 2.2）→ Task 2 ✓
- tasks.md 组 3（3.1, 3.2）→ Task 3 ✓（3.2 的"若变化"分支因 resolved 不变被跳过，已说明）
- tasks.md 组 4（4.1-4.5）→ Task 4 ✓

**Placeholder scan：** 无 TBD/TODO/「适当处理」，所有步骤都有具体命令与预期输出。

**Type/命名一致性：** `bytes`、`prost`、`prost-build` 在所有 Task 中大小写与 TOML key 一致；版本字符串（`"1.6"` / `"0.13"` / `"1.6.0"` / `"0.13.1"` / `"0.12.6"`）与源文件实际值一致。
