# Comet Design Handoff

- Change: unify-platform-small-request-naming
- Phase: design
- Mode: compact
- Context hash: 519d3697427fed2cebf22909aadceeae02067dd12ff9495e468fda5fef4bdfe1

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-platform-small-request-naming/proposal.md

- Source: openspec/changes/unify-platform-small-request-naming/proposal.md
- Lines: 1-28
- SHA256: 78d0b4a8a644bd84b9e52f986db075b52d5cf3e50f8b7678404bda11b87ebcaf

```md
## Why

issue #271（命名统一）platform crate 的第 1 批（小批）。前序 auth/application/docs 3 批已归档，验证了模式：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。platform 是 #271 最后的大 crate（97 裸 Builder），按子系统分批；本批是 trust_party/mdm/tenant/spark 4 个子系统共 12 个。

## What Changes

- 将 platform 的 **12 个请求 builder `XxxBuilder` 重命名为规范 `XxxRequestBuilder`**（均有 `execute()`，无 trait impl、无 re-export → 最简模式）：
  - trust_party: `UserAuthDataRelationBindBuilder`、`UserAuthDataRelationUnbindBuilder`
  - mdm: `AssignInfoListQueryBuilder`（注：虽叫 Query，但有 execute，是请求类型）
  - tenant: `TenantQueryBuilder`、`CollaborationDepartmentGetBuilder`、`CollaborationTenantGetBuilder`、`CollaborationTenantListBuilder`、`CollaborationUserGetBuilder`、`VisibleOrganizationBuilder`（注：collaboration/visible 属 tenant 子系统目录）
  - spark: `CountryRegionBatchGetBuilder`、`CountryRegionListBuilder`、`DirectoryUserIdConvertBuilder`
- 旧名作 `#[deprecated(note="...")] pub type XxxBuilder = XxxRequestBuilder;`（v0.18→v1.0 软迁移，放 `#[cfg(test)]` 前）
- 无 re-export 链（12 个均无）→ 仅定义文件改
- **BREAKING**（软）：`#[deprecated]` alias 保证源码兼容

## Capabilities

### New Capabilities
- `platform-small-request-naming`: openlark-platform 的 trust_party/mdm/tenant/spark 子系统请求类型 builder SHALL 统一 `RequestBuilder` 后缀；旧 `Builder` 名作 `#[deprecated]` alias 保留至 v1.0。

### Modified Capabilities
<!-- 无 -->

## Impact

- **openlark-platform**：12 个 struct+impl 重命名 + 12 alias + 测试同步。无 re-export。
- **破坏性**：软 breaking（alias 源码兼容 + warning）。v1.0 移除 alias。
- **非目标**：不动 platform 其他子系统（app_engine 51/directory 21/admin 14，后续批次）；不动 body 模型；不动非请求 builder。
```

## openspec/changes/unify-platform-small-request-naming/design.md

- Source: openspec/changes/unify-platform-small-request-naming/design.md
- Lines: 1-34
- SHA256: 995b8b0f68243a6246729995a0cfbe569bb65918b8d97e94ec62369af083652f

```md
## Context

#271 platform 第 1 批（小批）。模式已在 auth/application/docs 3 批完全验证：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。本批 12 个 platform 类型（trust_party/mdm/tenant/spark 子系统），均无 trait impl、无 re-export → 最简实现。

## Goals / Non-Goals

**Goals:** platform 小批 12 个请求 builder 统一 `XxxRequestBuilder` + `#[deprecated]` alias 软迁移。

**Non-Goals:** 不动 platform 其他子系统（app_engine/directory/admin）；不动 body 模型；不在 v0.18 硬移除旧名。

## Decisions

### 决策 1：方向 Builder → RequestBuilder（沿用 #271 既定方向）
12 个统一 →RequestBuilder（与 auth/application/docs 一致；RequestBuilder 目标永不撞 body 模型名）。

### 决策 2：#[deprecated] type alias 软迁移（沿用前序）
`pub struct XxxRequestBuilder` + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

### 决策 3：无 re-export（本批最简）
12 个均无 re-export → 仅定义文件 struct+impl+测试改名 + alias，无 re-export 链同步。

## Risks / Trade-offs

- alias 放 `#[cfg(test)]` 前（clippy items_after_test_module 教训）。
- push 前跑 `cargo fmt --check`（CI lint 教训）。
- 软 breaking，回滚 = revert。

## Migration Plan

v0.18：重命名 + alias。v1.0：移除 alias。CHANGELOG v0.18 breaking 段记录。

## Open Questions

无（12 个均无 trait impl/re-export，已核实）。
```

## openspec/changes/unify-platform-small-request-naming/tasks.md

- Source: openspec/changes/unify-platform-small-request-naming/tasks.md
- Lines: 1-19
- SHA256: 5f6a42d3891ab84a68ec9924b630ece6962693db61f01a8cad4718db3cf2fcfc

```md
# Tasks — unify-platform-small-request-naming（#271 platform 第 1 批小批）

> platform trust_party/mdm/tenant/spark 12 个请求 builder → XxxRequestBuilder + #[deprecated] alias。#271 既定模式，最简实现（无 trait impl、无 re-export）。

## 1. 重命名 + alias

- [ ] 1.1 12 个定义文件：struct+impl+测试 `XxxBuilder` → `XxxRequestBuilder`；在 `#[cfg(test)]` 前加 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`。12 类型：AssignInfoListQuery/CollaborationDepartmentGet/CollaborationTenantGet/CollaborationTenantList/CollaborationUserGet/CountryRegionBatchGet/CountryRegionList/DirectoryUserIdConvert/TenantQuery/UserAuthDataRelationBind/UserAuthDataRelationUnbind/VisibleOrganization

## 2. 验证

- [ ] 2.1 `cargo build --workspace --all-features` exit 0
- [ ] 2.2 三组 clippy（default/all/no-default + `-D warnings`）均 exit 0
- [ ] 2.3 `cargo test -p openlark-platform` 0 failed
- [ ] 2.4 **`cargo fmt --all -- --check` exit 0**（CI lint 教训）
- [ ] 2.5 grep 确认 12 RequestBuilder struct + 12 deprecated alias + 0 旧 struct 残留

## 3. CHANGELOG

- [ ] 3.1 CHANGELOG v0.18 breaking 段记录 12 个重命名（platform 小批）
```

## openspec/changes/unify-platform-small-request-naming/specs/platform-small-request-naming/spec.md

- Source: openspec/changes/unify-platform-small-request-naming/specs/platform-small-request-naming/spec.md
- Lines: 1-38
- SHA256: b3386f22bf61d36a0efa6b4848e83b9e9323ec95fb8b67124521dc7a9928f100

```md
## ADDED Requirements

### Requirement: platform 小批请求 builder 统一 RequestBuilder 后缀
openlark-platform 的 trust_party/mdm/tenant/spark 子系统请求类型 builder SHALL 统一 `RequestBuilder` 后缀。12 个原 `XxxBuilder` SHALL 重命名为 `XxxRequestBuilder`。

#### Scenario: 12 个请求类型重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（AssignInfoListQuery/CollaborationDepartmentGet/CollaborationTenantGet/CollaborationTenantList/CollaborationUserGet/CountryRegionBatchGet/CountryRegionList/DirectoryUserIdConvert/TenantQuery/UserAuthDataRelationBind/UserAuthDataRelationUnbind/VisibleOrganization）
- **THEN** 12 个新 `XxxRequestBuilder` struct 存在

#### Scenario: 旧裸 Builder struct 不再存在
- **WHEN** grep 12 个旧 `XxxBuilder` 作 struct 定义
- **THEN** 0 命中（全退化为 type alias）

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 12 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: 旧名 alias 存在且标 deprecated + 在 test 前
- **WHEN** grep 12 个 `pub type XxxBuilder` + 检查与 `#[cfg(test)]` 顺序
- **THEN** 12 个 alias 带 `#[deprecated]`，均在 `#[cfg(test)]` 前

### Requirement: 重命名不破坏构建、lint、测试、格式
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** clippy default/all/no-default + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: platform 测试通过
- **WHEN** `cargo test -p openlark-platform`
- **THEN** 0 failed

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0
```

