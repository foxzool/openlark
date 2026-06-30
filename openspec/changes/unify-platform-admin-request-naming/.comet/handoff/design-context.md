# Comet Design Handoff

- Change: unify-platform-admin-request-naming
- Phase: design
- Mode: compact
- Context hash: 5557f8cfc6818789774601bacea9bbe4aaab5d26f569f9998d1141f487024a76

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-platform-admin-request-naming/proposal.md

- Source: openspec/changes/unify-platform-admin-request-naming/proposal.md
- Lines: 1-14
- SHA256: c12732346bdb1fab6889a606fd015c272cf9187f141897cb65efd11e0fdb4b80

```md
## Why
issue #271 platform 第 2 批（admin）。前 4 批已归档（auth 12 + application/docs 4 + platform 小批 12），模式验证。admin 14 个裸 Builder，全最简（无 trait impl/re-export/service）。

## What Changes
- 14 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 类型：CreateBadge/CreateBadgeGrant/CreateBadgeImage/DeleteBadgeGrant/GetBadge/GetBadgeGrant/ListAdminDeptStat/ListAdminUserStat/ListAuditInfo/ListBadge/ListBadgeGrant/ResetPassword/UpdateBadge/UpdateBadgeGrant
- **BREAKING**（软）：alias 源码兼容 + warning，v1.0 移除

## Capabilities
### New Capabilities
- `platform-admin-request-naming`: platform admin 子系统请求 builder SHALL 统一 RequestBuilder 后缀。

## Impact
- openlark-platform admin 子系统 14 个定义文件改名 + alias。无 re-export/service/trait impl。
```

## openspec/changes/unify-platform-admin-request-naming/design.md

- Source: openspec/changes/unify-platform-admin-request-naming/design.md
- Lines: 1-10
- SHA256: a30021a26602dba0e36992cbda4a1f15b36c31266571552a57fbe28eb058874f

```md
## Context
#271 platform admin 批。模式 4 批验证。14 个全无 trait impl/re-export/service → 最简。

## Decisions
1. Builder→RequestBuilder（#271 既定方向）
2. #[deprecated] alias（放 #[cfg(test)] 前）
3. 无 re-export/service → 仅定义文件改

## Risks
alias 放 #[cfg(test)] 前；push 前 cargo fmt --check。
```

## openspec/changes/unify-platform-admin-request-naming/tasks.md

- Source: openspec/changes/unify-platform-admin-request-naming/tasks.md
- Lines: 1-14
- SHA256: ae77bc67220a133b6dc708c42cc08a8c219f5e309204e33ac3504fc9ec768e6a

```md
# Tasks — unify-platform-admin-request-naming（#271 platform admin 批）

## 1. 重命名 + alias
- [ ] 1.1 14 个定义文件：struct+impl+测试 `XxxBuilder`→`XxxRequestBuilder`；`#[cfg(test)]` 前加 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`

## 2. 验证
- [ ] 2.1 `cargo build --workspace --all-features` exit 0
- [ ] 2.2 三组 clippy（-D warnings）exit 0
- [ ] 2.3 `cargo test -p openlark-platform` 0 failed
- [ ] 2.4 `cargo fmt --all -- --check` exit 0
- [ ] 2.5 grep 14 RequestBuilder struct + 14 alias + 0 残留

## 3. CHANGELOG
- [ ] 3.1 CHANGELOG v0.18 breaking 段记录
```

## openspec/changes/unify-platform-admin-request-naming/specs/platform-admin-request-naming/spec.md

- Source: openspec/changes/unify-platform-admin-request-naming/specs/platform-admin-request-naming/spec.md
- Lines: 1-26
- SHA256: f8f06094fa239be5e15af67b0e2beab1a409f091389b1a08416bfe19035c4175

```md
## ADDED Requirements

### Requirement: admin 请求 builder 统一 RequestBuilder 后缀
platform admin 子系统 14 个请求类型 builder SHALL 统一使用 `RequestBuilder` 后缀。

#### Scenario: 14 个重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（CreateBadge/CreateBadgeGrant/CreateBadgeImage/DeleteBadgeGrant/GetBadge/GetBadgeGrant/ListAdminDeptStat/ListAdminUserStat/ListAuditInfo/ListBadge/ListBadgeGrant/ResetPassword/UpdateBadge/UpdateBadgeGrant）
- **THEN** 14 个新 `XxxRequestBuilder` struct 存在

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 14 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: 旧名 alias 存在且标 deprecated
- **WHEN** grep 14 个 `pub type XxxBuilder =`
- **THEN** 14 个 alias 带 `#[deprecated]`，均在 `#[cfg(test)]` 前

### Requirement: 重命名不破坏构建、lint、测试、格式
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0
```

