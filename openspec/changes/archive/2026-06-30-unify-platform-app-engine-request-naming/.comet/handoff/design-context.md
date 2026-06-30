# Comet Design Handoff

- Change: unify-platform-app-engine-request-naming
- Phase: design
- Mode: compact
- Context hash: f23adec90b060a87dd211be4a68b04179bc20bd2cb1a83549e501c2ed77dc722

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-platform-app-engine-request-naming/proposal.md

- Source: openspec/changes/unify-platform-app-engine-request-naming/proposal.md
- Lines: 1-14
- SHA256: c8b9bb7026a487b703b07648d3bbc51ad68a2efed044d9de86fce04143a2d7bb

```md
## Why
issue #271 platform app_engine 批（第 7 批，**最后一批**）。前 6 批已归档（63 类型）。app_engine 51 个裸 Builder，全在 apaas 子目录，全最简（无 trait impl/re-export/service）。

## What Changes
- 51 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 全在 platform/app_engine/apaas/ 子目录
- **BREAKING**（软）：alias 源码兼容

## Capabilities
### New Capabilities
- `platform-app-engine-request-naming`: platform app_engine 子系统请求 builder SHALL 统一 RequestBuilder 后缀。

## Impact
- openlark-platform app_engine/apaas 51 个定义文件改名 + alias。无 re-export/service/trait impl。
```

## openspec/changes/unify-platform-app-engine-request-naming/design.md

- Source: openspec/changes/unify-platform-app-engine-request-naming/design.md
- Lines: 1-10
- SHA256: 99ebf7b5372e1112c572ce2a0389094048399842f2210f706a338370f5170d8c

```md
## Context
#271 最后一批（app_engine 51）。模式 6 批验证。全最简（无 re-export/service/trait impl）。

## Decisions
1. Builder→RequestBuilder（#271 既定方向）
2. #[deprecated] alias（放 #[cfg(test)] 前）
3. 无 re-export/service → 仅定义文件改

## Risks
alias 放 #[cfg(test)] 前；push 前 cargo fmt --check。51 个量大但机械化脚本统一处理。
```

## openspec/changes/unify-platform-app-engine-request-naming/tasks.md

- Source: openspec/changes/unify-platform-app-engine-request-naming/tasks.md
- Lines: 1-7
- SHA256: d7afef5ace6d8b4cb21c69044229701ae26220ea62f42b580ea4e9acef06ab06

```md
# Tasks — unify-platform-app-engine-request-naming（#271 app_engine 批，最后一批）
## 1. 重命名 + alias
- [ ] 1.1 51 定义文件 struct+impl+测试→RequestBuilder；#[cfg(test)] 前加 #[deprecated] alias
## 2. 验证
- [ ] 2.1 build --all-features + clippy×3 + test + fmt + grep
## 3. CHANGELOG
- [ ] 3.1 CHANGELOG v0.18 breaking 段记录
```

## openspec/changes/unify-platform-app-engine-request-naming/specs/platform-app-engine-request-naming/spec.md

- Source: openspec/changes/unify-platform-app-engine-request-naming/specs/platform-app-engine-request-naming/spec.md
- Lines: 1-22
- SHA256: 410807c49ccbd1c5e6b46cf14ead3dd0d92945b2e0604bc7537a0fd9b587950e

```md
## ADDED Requirements

### Requirement: app_engine 请求 builder 统一 RequestBuilder 后缀
platform app_engine/apaas 子系统 51 个请求类型 builder SHALL 统一 `RequestBuilder` 后缀。

#### Scenario: 51 重命名
- **WHEN** grep `pub struct XxxRequestBuilder`（app_engine 51 类型）
- **THEN** 51 个存在

### Requirement: 旧名 #[deprecated] alias
51 个旧 `XxxBuilder` SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: alias 存在
- **WHEN** grep 51 个 `pub type XxxBuilder =`
- **THEN** 51 个带 `#[deprecated]`

### Requirement: 不破坏 build/clippy/test/fmt
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全绿
- **WHEN** build --all-features / clippy×3 / test / fmt --check
- **THEN** 均 exit 0
```

