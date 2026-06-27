# Comet Design Handoff

- Change: cleanup-dead-code-allows
- Phase: design
- Mode: compact
- Context hash: aa98bd5aa5903a98e96205c1362d39ee3ce7bc1c1d4893fa35b2bbe9927411c0

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-dead-code-allows/proposal.md

- Source: openspec/changes/cleanup-dead-code-allows/proposal.md
- Lines: 1-31
- SHA256: dc64113ca8f2f538a0948988114217ba9b22bd5404d6c63ab868533a13a95a86

```md
## Why

全 workspace 有 **392 个 `#[allow(dead_code)]`**（分布在 381 个文件，92% 集中在 `openlark-hr`）。这种规模的 lint 抑制**淹没 dead_code 信号**——未来真正未读的字段将无法被编译器检出。

实证（移除全部 392 后跑 `cargo check --workspace`）：**仅 3 个真 warning**，其余 **389 个全是 cruft**（字段实际都在 `execute()` 中被读取——`config` 传入 Transport，参数序列化进 RequestBody）。3 个真死字段全在 platform v1 入口 struct（`admin/admin/v1/mod.rs`、`app_engine/apaas/v1/mod.rs`、`directory/directory/v1/mod.rs` 的 `config` 字段），是上一个 change（`fix-platform-v1-feature-gating` ungate v1）**新暴露**的——ungate 前藏在 cfg 门控后不编译，现在编译了才显现。

来源：架构审计 issue [#267](https://github.com/foxzool/openlark/issues/267)（原假设"codegen 过度生成"，实证后修正为"389 cruft + 3 真死字段"）。

## What Changes

- **移除 389 个不必要的 `#[allow(dead_code)]`**（cruft，已验证移除后 0 warning）——机械、安全。
- **修正 3 个真死字段**（platform v1 入口 struct 的 `config`）：design 阶段决定修法（读取并传给子 API，或移除/重命名 `_config`）。
- 恢复 dead_code lint 信号，使未来真死字段可被编译器检出。
- 可选：加 workspace 级约束防止复发（如 clippy 规则或约定，design 阶段评估）。

## Capabilities

### New Capabilities
- `dead-code-lint-hygiene`: openlark 公开代码 SHALL 不用 `#[allow(dead_code)]` 掩盖可修复的死字段；真死字段必须修正（读取/移除）或显式 `_` 前缀 + 注释说明。dead_code lint 信号 SHALL 保持有效。

### Modified Capabilities
<!-- 无现有 spec 需要修改 -->

## Impact

- **openlark-hr**：移除 361 处 cruft（92%）——纯 lint 抑制删除，零行为/API 变化。
- **openlark-platform**：移除 2 处 cruft + 修正 3 处真死字段（v1 入口 struct `config`）。
- **其他 crate**（ai/user/docs/analytics/workflow/mail/helpdesk/bot/application）：移除 ~28 处 cruft。
- **API/行为**：389 处删除无任何影响；3 处真死字段修正属内部清理（`config` 字段若本就未用，修正不改变对外行为；若应被读取则修复潜在功能缺陷）。
- **质量**：dead_code lint 信号恢复；issue #267 闭环。
- **非目标**：不改 codegen 工具（`codegen.py` 本就不 emit dead_code）；不重构 request builder 结构；不引入新 API。
```

## openspec/changes/cleanup-dead-code-allows/design.md

- Source: openspec/changes/cleanup-dead-code-allows/design.md
- Lines: 1-61
- SHA256: efc5e4052c544ce6f1d2b8f2616dbe1c9746f09aa95c85d11bc63f5743b9784e

```md
## Context

392 个 `#[allow(dead_code)]` 散布全 workspace，淹没 dead_code lint 信号。实证：389 cruft（字段已读）+ 3 真死字段（platform v1 入口 struct `config`）。

```
392 #[allow(dead_code)]
 ├─ 389 cruft（字段在 execute() 中已读）→ 直接删除，0 风险
 └─ 3 真死字段（platform v1 入口 struct 的 config）
     ├─ admin/admin/v1/mod.rs:28      AdminV1.config
     ├─ app_engine/apaas/v1/mod.rs:28 ApaasV1.config
     └─ directory/directory/v1/mod.rs:29 DirectoryV1.config
     （fix-platform-v1-feature-gating ungate 后新暴露）
```

389 个删除是机械操作（已验证移除后 0 warning）。真正的设计点是 3 个 platform v1 `config` 字段如何处理。

## Goals / Non-Goals

**Goals:**
- 移除全部 392 个 `#[allow(dead_code)]`（389 删除 + 3 修正）。
- 恢复 dead_code lint 信号。
- 修正 3 个真死字段（platform v1 `config`），不破坏 v1 API 行为。

**Non-Goals:**
- 不改 codegen 工具（`codegen.py` 不 emit dead_code；HR 的 allows 是历史脚手架遗留，无活跃 codegen 复发）。
- 不重构 request builder / v1 入口 struct 结构。
- 不引入新 API。

## Decisions

> 核心选型留 design 阶段 brainstorming 确认。

**D1（389 cruft 删除）—— 已确定**：机械移除，已验证安全（0 warning）。

**D2（3 个 platform v1 `config` 真死字段，待 brainstorm）**——需先读 `v1/mod.rs` 确认 `config` 为何未被读：
- 候选 A：入口 struct 应把 `config` 传给子 API 访问器（如 `AdminV1::badge()` 应 `BadgeApi::new(self.config.clone())`）——若子 API 当前没收到 config，这是**潜在功能缺陷**，修法是真正读取 config 并传递。
- 候选 B：子 API 已通过其他途径获得 config，入口 struct 的 `config` 是冗余 → 移除字段（连同 `new(config)` 签名调整，但 `AdminService::v1()` 调用 `AdminV1::new(config)`，需同步）。
- 候选 C：保留字段但 `_config` 前缀 + 注释说明（最小改动，但未真正修复）。

倾向先读代码判明 A vs B（是缺陷还是冗余），再定修法。

**D3（防复发，待 brainstorm）**：是否加 workspace 级约束（clippy custom / 约定文档 / CI grep 检查）防止 `#[allow(dead_code)]` 再度泛滥。

## Risks / Trade-offs

- **[3 字段修正可能触及 v1 公共签名]** 若 D2 选 B（移除 config），`AdminV1::new` 签名变化，但 `AdminV1` 本身刚 ungate（上个 change），外部依赖极少 → 低风险。
- **[mass-deletion 误删]** 389 删除已验证 0 warning，但需 final clippy 复核 default+full+no-default 三组 feature。
- **[其他 crate 31 处]** 已纳入范围（实证 0 真死字段），一并清理。

## Migration Plan

1. 移除 389 cruft（机械）。
2. 按 D2 修法处理 3 个 platform v1 config 字段。
3. `cargo clippy --workspace` 三组 feature（default/full/no-default）`-D warnings` 全过。
4. 测试通过。
5. 回滚：纯删除/小改，git revert 即可。

## Open Questions

- D2：3 个 `config` 字段是缺陷（应传递）还是冗余（应移除）？
- D3：是否加防复发约束？
```

## openspec/changes/cleanup-dead-code-allows/tasks.md

- Source: openspec/changes/cleanup-dead-code-allows/tasks.md
- Lines: 1-32
- SHA256: d7cc68daeec8ee7650edc3ffa953cfab2355f79129ec904466eb87bc516b0525

```md
# Tasks — cleanup-dead-code-allows

> 范围：移除全 392 处 `#[allow(dead_code)]` = 389 cruft 删除 + 3 真死字段最小修（`_config`）。关联 issue #267；导航补全拆至 #274。D2/D3 已 design 确认。

## 1. 调研（build 前）

- [x] 1.1 读 3 个 platform v1 `mod.rs` 判明 config 字段无访问器（子模块无 service 类型）→ D2 定方案 C
- [x] 1.2 确认移除 389 cruft 后三组 feature均 0 warning（workspace 实证：仅 3 真死字段）

## 2. 移除 389 处 cruft（机械）

- [ ] 2.1 批量移除 `crates/`+`src/` 下全部 `#[allow(dead_code)]` 行（381 文件，389 处）
- [ ] 2.2 保留 3 个 platform v1 字段的处理给 Task 3（它们的 allow 删除后由 `_config` 改名消除 warning）

## 3. 修正 3 个真死字段（方案 C：`_config` + 注释）

- [ ] 3.1 `admin/admin/v1/mod.rs`：`config` → `_config` + 注释「reserved：待装访问器（见 #274）」；测试 `api.config` → `api._config`
- [ ] 3.2 `app_engine/apaas/v1/mod.rs`：同上
- [ ] 3.3 `directory/directory/v1/mod.rs`：同上

## 4. 验证

- [ ] 4.1 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [ ] 4.3 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [ ] 4.4 `cargo test --workspace` 通过

## 5. 防复发与收尾

- [ ] 5.1 D3：加 CI grep 检查（`.github/workflows` 或 justfile）——禁止非测试代码出现 `#[allow(dead_code)]`
- [ ] 5.2 CHANGELOG `[Unreleased] > Changed/Fixed` 记录
- [ ] 5.3 关闭 issue #267（归档后）；#274（导航补全）保持 open 待后续 change
```

## openspec/changes/cleanup-dead-code-allows/specs/dead-code-lint-hygiene/spec.md

- Source: openspec/changes/cleanup-dead-code-allows/specs/dead-code-lint-hygiene/spec.md
- Lines: 1-30
- SHA256: 66958e5fa75ac3179a3310d98ba81e18674406ffa394b9c34b1a54c61c1f8521

```md
## ADDED Requirements

### Requirement: 不用 #[allow(dead_code)] 掩盖可修复的死字段
openlark 公开源代码 SHALL 不使用 `#[allow(dead_code)]` 抑制可修复的字段级 dead_code 警告。本次变更 SHALL 移除全部 392 处既有 `#[allow(dead_code)]`（其中 389 处经实证为不必要 cruft，3 处为需修正的真死字段）。

#### Scenario: HR crate 移除后无残留
- **WHEN** 在 `crates/openlark-hr/` 中 grep `#[allow(dead_code)]`
- **THEN** 命中数为 0（361 处 cruft 全部移除）

#### Scenario: 全 workspace 移除后无 cruft 残留
- **WHEN** 在 `crates/` + `src/` 中 grep `#[allow(dead_code)]`（排除 `#[cfg(test)]` 测试代码）
- **THEN** 命中数为 0，或仅保留带显式注释说明的 `_` 前缀字段

### Requirement: 真死字段必须修正或显式处理
3 个 platform v1 入口 struct 的 `config` 真死字段（`admin/admin/v1`、`app_engine/apaas/v1`、`directory/directory/v1`）SHALL 被修正（读取并传递给子 API）或显式处理（移除/`_` 前缀 + 注释），不得用 `#[allow(dead_code)]` 掩盖。

#### Scenario: platform v1 入口 struct config 字段不再触发 dead_code
- **WHEN** 移除 `#[allow(dead_code)]` 后运行 `cargo check -p openlark-platform`
- **THEN** 不再出现 `field config is never read` 警告（3 处全部解决）

### Requirement: dead_code lint 信号保持有效
本次变更后，dead_code lint SHALL 能检出未来引入的真死字段（信号未被 mass-suppression 淹没）。

#### Scenario: 三组 feature clippy 零 warning
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 `（default）`、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: 测试不回归
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部测试通过（389 删除无行为影响；3 字段修正不破坏 v1 API）
```

