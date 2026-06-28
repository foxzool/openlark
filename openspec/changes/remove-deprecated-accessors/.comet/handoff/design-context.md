# Comet Design Handoff

- Change: remove-deprecated-accessors
- Phase: design
- Mode: compact
- Context hash: d11f6b67596b5bcfce89eae01e77f515bf9e1bcb9e3ba1661540595c8b7637bf

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/remove-deprecated-accessors/proposal.md

- Source: openspec/changes/remove-deprecated-accessors/proposal.md
- Lines: 1-31
- SHA256: 3af1c6c1d1bb9556cf5c937c839a465c64ab8b61e801c04a723ebdb8eaba31bc

```md
## Why

10 个 `#[deprecated]` 兼容访问器方法 clutter API 表面，v1.0 breaking 窗口应移除：

- **HR（8）**：`Hr` 结构体的 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` —— v0.15.0 起标记 deprecated，提示「用字段直接访问 `client.attendance`」。方法式访问与字段访问并存致 API 混乱。
- **analytics（2）**：`SearchV2` 的 `query()/user()` —— 未接线的 runtime stub（deprecation note：「not wired to a Feishu endpoint, prefer implemented surfaces」）。

实证：这 10 个方法**零内部调用**（examples/tests/其他 crate 均未用方法式调用）→ 可干净移除，无内部迁移。

来源：架构审计 issue [#268](https://github.com/foxzool/openlark/issues/268)（20 个 deprecated 拆分；本 change 做其中 A+E 共 10 个最干净的，其余 B/C/D/F/G 10 个异构项另开 issue）。

## What Changes

- **BREAKING**：移除 HR 8 个 service 访问器方法（`attendance()`/`corehr()`/`compensation()`/`payroll()`/`performance()`/`okr()`/`hire()`/`ehr()`）。用户改用字段访问（`client.attendance` 等，字段已存在且等价）。
- **BREAKING**：移除 analytics `SearchV2::query()`/`user()` 存根方法。用户改用已实现的 search surfaces（`doc_wiki`/`schema`/`app`/`message`）。`QueryApi`/`UserSearchApi` 类型与模块（`v2/query.rs`、`v2/user.rs`）保留为 `pub`（仍可经完整路径访问），仅移除便捷存根访问器。

## Capabilities

### New Capabilities
- `no-deprecated-compat-accessors`: openlark SHALL 不暴露已废弃的兼容访问器方法；HR 业务模块 SHALL 经字段访问（`client.attendance`），analytics SHALL 经已实现的 search surfaces 而非未接线存根。

### Modified Capabilities
<!-- 无现有 spec 需要修改 -->

## Impact

- **openlark-hr**：`src/lib.rs` 删除 8 个 `#[deprecated] pub fn` 方法（各 behind feature gate）。
- **openlark-analytics**：`src/search/search/v2.rs` 删除 2 个 `#[deprecated] pub fn`（query/user）。
- **破坏性**：移除公开方法（已 deprecated ≥ 一个次版本）。外部 `client.attendance()` → `client.attendance`；`search_v2.query()` → 用其他 surface 或完整路径。
- **CHANGELOG**：记入 `[Unreleased] > Breaking Changes`，附迁移指引。
- **非目标**：不动 B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder 方法）——另开 issue；不改字段访问本身；不新增 API；不移除 `QueryApi`/`UserSearchApi` 类型。
```

## openspec/changes/remove-deprecated-accessors/design.md

- Source: openspec/changes/remove-deprecated-accessors/design.md
- Lines: 1-47
- SHA256: bf2d816bb6f2289c58143e26e74d1f6e08238de3ae84979fa57b150003d413b3

```md
## Context

10 个 `#[deprecated]` 兼容访问器待移除（v1.0 breaking 窗口）：HR 8 个 service 访问器（v0.15.0 起 deprecated，字段访问替代）+ analytics 2 个未接线存根。零内部调用，移除干净。

deprecated 决策早已做出（since 0.15.0 / 0.5.0），本 change 只是**执行移除**，无新设计空间。关键点是迁移指引与 breaking 标注。

## Goals / Non-Goals

**Goals:**
- 移除 HR 8 + analytics 2 共 10 个 deprecated 访问器方法。
- CHANGELOG 提供清晰迁移指引（方法 → 字段 / 其他 surface）。

**Non-Goals:**
- 不动 B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）——异构，另开 issue。
- 不移除 `QueryApi`/`UserSearchApi` 类型与模块（仅移除便捷存根访问器）。
- 不改字段访问实现、不新增 API。

## Decisions

**D1（移除方式）—— 已确定**：直接删除 10 个 `#[deprecated] pub fn`。无替代方案（deprecation 早有 note 指引替代）。

**D2（QueryApi/UserSearchApi 模块去留）—— 保留**：移除 `SearchV2::query()`/`user()` 存根访问器，但 `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型保留为 `pub`（仍可经完整路径访问；其补全见 #276）。仅移除便捷存根方法，避免连带删除尚有补全计划的类型。

## 迁移映射（写入 CHANGELOG）

| 旧（移除） | 新 |
|------------|-----|
| `hr.attendance()` | `hr.attendance`（字段） |
| `hr.corehr()` / `compensation()` / ... | `hr.corehr` / `hr.compensation` / ...（字段） |
| `search_v2.query()` | 用 `doc_wiki`/`schema`/`app`/`message` surface，或 `SearchV2` 不再暴露 query 存根 |
| `search_v2.user()` | 同上（user-search 未实现，无替代 surface） |

## Risks / Trade-offs

- **[Breaking]** 移除公开方法 → 用户代码 `client.attendance()` 编译失败。缓解：CHANGELOG 迁移指引 + 字段访问是 drop-in 替代（`.attendance()` → `.attendance`，1 字符之差）。
- **[analytics user() 无替代]** user-search 未实现，移除 `user()` 后无 surface。缓解：保留 `UserSearchApi` 类型（完整路径可达），#276/#275 跟踪补全。

## Migration Plan

1. 删除 HR 8 + analytics 2 共 10 个 `#[deprecated] pub fn`。
2. `cargo clippy` 三组 feature + `cargo test --workspace`。
3. CHANGELOG `[Unreleased] > Breaking Changes` + 迁移表。
4. 回滚：git revert。

## Open Questions

- 无（移除是 pre-decided；仅执行）。
```

## openspec/changes/remove-deprecated-accessors/tasks.md

- Source: openspec/changes/remove-deprecated-accessors/tasks.md
- Lines: 1-28
- SHA256: c7deae096468ecca4bb47864e080792fd0db6f833bdc7cd954a576995fb3a4c8

```md
# Tasks — remove-deprecated-accessors

> 范围：移除 10 个 deprecated 兼容访问器（HR 8 + analytics 2），均 0 内部调用、干净移除。BREAKING。关联 #268（A+E 子集）；B/C/D/F/G 另开 issue。

## 1. 移除 HR 8 个 deprecated 访问器

- [ ] 1.1 删除 `crates/openlark-hr/src/lib.rs` 的 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` 8 个 `#[deprecated] pub fn`（含其 `#[cfg(feature)]` 门控与文档注释）
- [ ] 1.2 确认 `Hr` 的对应字段（`attendance`/`corehr`/...）保留且 `pub`（字段访问替代）

## 2. 移除 analytics 2 个 deprecated 存根

- [ ] 2.1 删除 `crates/openlark-analytics/src/search/search/v2.rs` 的 `query()`/`user()` 2 个 `#[deprecated] pub fn`
- [ ] 2.2 确认 `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型保留 `pub`（仅移除便捷存根方法）

## 3. 验证

- [ ] 3.1 `grep '#\[deprecated' crates/openlark-hr/ crates/openlark-analytics/` 确认目标 10 个已移除（剩余的 B/C/D/F/G 不在本范围）
- [ ] 3.2 examples 不引用已移除方法：`grep '\.attendance()|\.corehr()|\.query()' examples/` = 0
- [ ] 3.3 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [ ] 3.4 `--all-features` exit 0
- [ ] 3.5 `--no-default-features` exit 0
- [ ] 3.6 `cargo test --workspace` 通过

## 4. 文档与收尾

- [ ] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加迁移条目 + 迁移映射表（方法→字段 / 其他 surface）
- [ ] 4.2 开 follow-up issue：B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）共 10 个 deprecated 异构项的迁移清理
- [ ] 4.3 评论 #268：本 change 处理 A+E（10），B/C/D/F/G 拆至 follow-up；#268 待 follow-up 完成后关闭
```

## openspec/changes/remove-deprecated-accessors/specs/no-deprecated-compat-accessors/spec.md

- Source: openspec/changes/remove-deprecated-accessors/specs/no-deprecated-compat-accessors/spec.md
- Lines: 1-34
- SHA256: fe1d644cc0b19ba9c87be796df79fa4bbf244cacbbf326501840555f3a6ba896

```md
## ADDED Requirements

### Requirement: HR 不暴露 deprecated service 访问器方法
openlark-hr 的 `Hr` 结构体 SHALL 不再提供 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` 兼容访问器方法。用户 SHALL 经字段访问（`client.attendance` 等）获取各业务模块客户端。

#### Scenario: HR crate 无 deprecated 访问器
- **WHEN** 在 `crates/openlark-hr/src/lib.rs` 中 grep `pub fn attendance|pub fn corehr|...|pub fn ehr`
- **THEN** 命中数为 0（8 个兼容访问器方法全部移除）

#### Scenario: 字段访问等价可用
- **WHEN** 用户以 default feature 构建，访问 `hr.attendance`（字段）
- **THEN** 返回 `Attendance` 客户端（与原 `hr.attendance()` 方法等价），编译通过

### Requirement: analytics SearchV2 不暴露 deprecated 未接线存根
openlark-analytics 的 `SearchV2` SHALL 不再提供 `query()`/`user()` deprecated 存根方法。

#### Scenario: SearchV2 无 query/user 存根
- **WHEN** 在 `crates/openlark-analytics/src/search/search/v2.rs` 中 grep `pub fn query|pub fn user`
- **THEN** 命中数为 0（2 个 deprecated 存根访问器移除）

#### Scenario: QueryApi/UserSearchApi 类型保留
- **WHEN** 移除存根访问器后构建 openlark-analytics
- **THEN** `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型仍 `pub` 可用（经完整路径），仅便捷存根方法被移除

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default 任一 feature 组合的 clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: examples 不引用已移除方法
- **WHEN** 在 `examples/` 中 grep `\.attendance()|\.corehr()|\.query()` 等
- **THEN** 命中数为 0（examples 不使用已移除的访问器方法）
```

