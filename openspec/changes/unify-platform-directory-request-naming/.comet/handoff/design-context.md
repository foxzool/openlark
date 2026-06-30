# Comet Design Handoff

- Change: unify-platform-directory-request-naming
- Phase: design
- Mode: compact
- Context hash: 5ce3449014bf31defab28bc2de9a895410d82486aeeb3cb703578870d867eb99

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-platform-directory-request-naming/proposal.md

- Source: openspec/changes/unify-platform-directory-request-naming/proposal.md
- Lines: 1-11
- SHA256: 9e7711a140755c4ada66095e6d75cd07ecce745b8c00f8dd17865128a9b1b137

```md
## Why
issue #271 platform directory 批（第 6 批）。前 5 批已归档（42 类型），模式验证。directory 21 个裸 Builder，全最简（无 trait impl/re-export/service）。CollaborationTenantListBuilder 撞名已排除（platform 根不 re-export，不同模块路径）。

## What Changes
- 21 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 类型：CollaborationRule(Create/Delete/List/Update)、CollaborationShareEntityList、CollaborationTenantList、Department(Create/Delete/Filter/Mget/Patch/Search)、Employee(Create/Delete/Filter/Mget/Patch/Regular/Resurrect/Search/ToBeResigned)
- **BREAKING**（软）：alias 源码兼容

## Capabilities
### New Capabilities
- `platform-directory-request-naming`: platform directory 子系统请求 builder SHALL 统一 RequestBuilder 后缀。
```

## openspec/changes/unify-platform-directory-request-naming/design.md

- Source: openspec/changes/unify-platform-directory-request-naming/design.md
- Lines: 1-11
- SHA256: 35ad1f1bb125bb03678c2d0a84b5e4586dde813b7cc2f05654d8f65ff78d2bb8

```md
## Context
#271 platform directory 批。模式 5 批验证。21 个全无 trait impl/re-export/service → 最简。

## Decisions
1. Builder→RequestBuilder（#271 既定方向）
2. #[deprecated] alias（放 #[cfg(test)] 前）
3. 无 re-export/service → 仅定义文件改
4. CollaborationTenantListBuilder 不撞名（不同模块路径，platform 根不 re-export）

## Risks
alias 放 #[cfg(test)] 前；push 前 cargo fmt --check。
```

## openspec/changes/unify-platform-directory-request-naming/tasks.md

- Source: openspec/changes/unify-platform-directory-request-naming/tasks.md
- Lines: 1-7
- SHA256: fb55027fdef6ee78d1cf0d387626b667beb70fa945328432ef090afecbf7826b

```md
# Tasks — unify-platform-directory-request-naming（#271 directory 批）
## 1. 重命名 + alias
- [ ] 1.1 21 定义文件 struct+impl+测试→RequestBuilder；#[cfg(test)] 前加 #[deprecated] alias
## 2. 验证
- [ ] 2.1 build --all-features + clippy×3 + test + fmt + grep
## 3. CHANGELOG
- [ ] 3.1 CHANGELOG v0.18 breaking 段记录
```

## openspec/changes/unify-platform-directory-request-naming/specs/platform-directory-request-naming/spec.md

- Source: openspec/changes/unify-platform-directory-request-naming/specs/platform-directory-request-naming/spec.md
- Lines: 1-20
- SHA256: 2cd4d403805e2ff7c029bf636857fc0e9fa2d81c6c2bb62f0a8751e21da638e9

```md
## ADDED Requirements

### Requirement: directory 请求 builder 统一 RequestBuilder 后缀
platform directory 子系统 21 个请求类型 builder SHALL 统一 `RequestBuilder` 后缀。

#### Scenario: 21 重命名
- **WHEN** grep `pub struct XxxRequestBuilder`（directory 21 类型）
- **THEN** 21 个存在

### Requirement: 旧名 #[deprecated] alias
21 个旧 `XxxBuilder` SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: alias 存在
- **WHEN** grep 21 个 `pub type XxxBuilder =`
- **THEN** 21 个带 `#[deprecated]`

### Requirement: 不破坏 build/clippy/test/fmt
#### Scenario: build/clippy/fmt 全绿
- **WHEN** build --all-features / clippy×3 / test / fmt --check
- **THEN** 均 exit 0
```

