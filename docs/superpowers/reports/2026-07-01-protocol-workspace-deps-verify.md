# 验证报告：protocol-workspace-deps

- Change: protocol-workspace-deps（issue #273 Part B）
- 分支: feature/20260701/protocol-workspace-deps
- base-ref: 61faa8fe11651daf7378a422e17682df0e0f93b7
- HEAD: 9691b1014
- verify_mode: full（10 tasks / 13 文件 / 1 capability，均超阈值）
- 日期: 2026-07-01

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | 10/10 tasks `[x]`，2 Requirements 全覆盖 |
| Correctness | 4/4 spec scenario 通过；D1-D4 决策全部落地 |
| Coherence | 实现与 Design Doc / design.md / proposal 一致；无矛盾 |

**Final Assessment：全检查通过，ready for archive。**

## 1. Completeness

### Task completion
- tasks.md：`grep -c '\- \[ \]'` = **0**（全部勾选），10/10 `[x]`
- plan：19 个真实 step 复选框全 `[x]`（plan 中唯一剩余 `- [ ]` 在 line 9 是 agentic-workers 说明文字，非任务）

### Spec coverage（2 ADDED Requirements，4 Scenario）
| Requirement | Scenario | 验证 |
|---|---|---|
| Req1 跨 crate 共享依赖 MUST 经 workspace | 多 crate 共享依赖走 workspace | ✅ prost 被 client/core/protocol 消费，全 `{workspace=true}`；全 workspace 无 crate 级 bytes/prost 钉版本（`grep -rnE '^(bytes\|prost)\b' crates/*/Cargo.toml`） |
| Req1 同上 | 单 crate 专用依赖鼓励走 workspace | ✅ bytes（原仅 protocol 用）已进 `[workspace.dependencies]` |
| Req2 依赖声明一致性 | openlark-protocol bytes/prost 走 workspace | ✅ protocol Cargo.toml:18-19 均为 `{ workspace = true }`，无钉版本；bytes 已在根 Cargo.toml:88 声明 |
| Req2 同上 | 迁移不引入新多版本 | ✅ `cargo tree -d` 迁移前后 diff **为空**；bytes 单版本 1.11.1；prost 0.12/0.13 既存 split 维持原状（vendored prost-build 引入，范围外） |

## 2. Correctness

### Design 决策落地（design.md D1-D4）
| 决策 | 预期 | 实测 |
|---|---|---|
| D1 bytes workspace = "1.6" | 根 Cargo.toml 声明 `"1.6"`（caret `^1.6.0`，对齐 protocol 原 `"1.6.0"`） | ✅ Cargo.toml:88 `bytes = "1.6"` |
| D2 prost 复用 workspace | protocol `{workspace=true}`，版本由 workspace `0.13` 统一 | ✅ Cargo.toml:19 `prost = { workspace = true }`；workspace `prost = { version = "0.13" }` |
| D3 MSRV lockfile 同步 | resolved 不变 → Cargo.lock 不动 → 无需同步 | ✅ `git diff Cargo.lock` 为空；`cargo +1.88 check --locked`（pinned msrv lockfile）Finished |
| D4 policy capability | 落盘 workspace-dependency-policy spec | ✅ specs/workspace-dependency-policy/spec.md（2 Requirements） |

### Design Doc 一致性
- Design Doc §3 改动 1（根 bytes）→ Cargo.toml:88 ✅
- Design Doc §3 改动 2（protocol bytes/prost）→ Cargo.toml:18-19 ✅
- Design Doc §5（prost-build 不动 / prost 0.12/0.13 既存 split 范围外）→ Cargo.toml:23 `prost-build = "0.12.6"` 保留 ✅

### proposal.md 目标
- protocol bytes/prost 走 workspace ✅
- bytes 加入 workspace.dependencies ✅
- 不升级大版本（bytes 1.x / prost 0.13.x）✅
- 公共 API 无变化（纯声明位置迁移）✅
- CI 全过（fmt/lint 双模式/build/deny/msrv --locked）✅

## 3. Coherence

### delta spec 与 design doc 一致性（漂移检查）
- 本 change 在 design 阶段做了 Spec Patch：收窄 spec「无多版本共存」→「迁移不引入新多版本」（prost 0.12/0.13 既存 split 由 vendored prost-build 引入，范围外）。
- Design Doc §5 + §7 与该 Spec Patch 完全对齐，**无矛盾**。
- build 阶段 tasks.md 4.5 文案已同步订正为「不引入新多版本」（与 spec 一致）。

### 诚实限制确认
- 本 change **不消除** prost 0.12/0.13 既存多版本（prost-build 0.12.6 → prost 0.12.6 是 vendored lark-websocket-protobuf 的 build 工具链，runtime 用 0.13.5）。
- spec scenario 已用边界条款明确此限制，不 over-claim。

## 4. 验证命令实测证据（fresh run，build 阶段 + verify 复核）

| 命令 | 结果 |
|---|---|
| `cargo fmt --check` | exit 0 |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings`（ci.yml:107） | Finished，无 warning |
| `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`（ci.yml:120） | Finished，无 warning |
| `cargo build --workspace --all-features` | Finished |
| `cargo tree -d --workspace`（前后 diff） | **diff 为空**（核心断言通过） |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| `cargo +1.88 check --workspace --all-features --locked`（pinned .github/msrv/Cargo.lock，ci.yml:244） | Finished |

## 5. 安全检查
- 无硬编码密钥/凭证 ✅
- 无新增 `unsafe` ✅
- 依赖图无新冲突（cargo deny 全过）✅
- 无 `.env` 或敏感文件被提交 ✅

## 6. code review（review_mode=standard，build 阶段已执行）
- 审查范围：2 处 Cargo.toml 实现 diff
- 结论：**Ready to merge**，0 Critical / 0 Important
- 2 个 Minor（M1 cargo-machete 注释 / P1 tasks.md 3.1 订正同步）+ 1 个流程观察（M2 勾选批量化）
- M1 + P1 已采纳并提交（9691b1014）；M2 记录于此（单会话连续完成 4 task，勾选批量化提交，非阻塞，本 change 无回滚需求）

## 7. build 阶段执行发现（供复盘）
- **Plan Task 3 Step 1 的 `cargo update` 是错误验证手段**——它自身会"更新到最新兼容版"（实测 bytes 1.11.1→1.12.0、移除 itertools 0.14.0），与迁移无关（原 `bytes="1.6.0"` 同样允许 1.12.0）。已 `git checkout Cargo.lock` 回滚，改用 `cargo build --locked` 作为正确验证。此发现已写入 plan Task 3 Step 1 订正段 + tasks.md 3.1 订正说明。
- **`just lint` 仅覆盖 `--all-features` 模式**，CI ci.yml:120 的 `--no-default-features` clippy 需手动补跑（已补跑通过）。这是 justfile recipe 与 CI 的覆盖差异，非本 change 范围，记于此供后续改进。

## Final Assessment
全检查通过（Completeness / Correctness / Coherence 三维度 + 7 项 full 验证检查项 + 安全 + review）。**Ready for archive。**
