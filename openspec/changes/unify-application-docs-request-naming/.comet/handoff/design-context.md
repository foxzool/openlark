# Comet Design Handoff

- Change: unify-application-docs-request-naming
- Phase: design
- Mode: compact
- Context hash: b009a0b152d71df89dd3d031cd9e26ea281d5601aedab58b1fac4e9530af33f6

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-application-docs-request-naming/proposal.md

- Source: openspec/changes/unify-application-docs-request-naming/proposal.md
- Lines: 1-29
- SHA256: 1d1a2d31931251f67c659b5572f8f0ca57665dc9dc78a12590af3c49d0b2010b

```md
## Why

issue #271（命名统一）的后续批次。auth pilot（PR #280，已归档）已验证模式：请求 builder 统一 `XxxRequestBuilder` + `#[deprecated]` type alias 软迁移。本批次把同一模式应用到 application+docs 两个 crate 剩余的不一致 builder。

精确摸底（裸 `XxxBuilder`，非 RequestBuilder）：application 3 个、docs 1 个（`RecordFieldsBuilder` 是真 builder 无 execute，排除）。共 4 个目标，**全部撞 body 模型名**（`crate::models` 已有 `XxxRequest`）→ 全部 → `XxxRequestBuilder`（与 auth pilot 撞名类型同方向）。

## What Changes

- 将 4 个请求 builder `XxxBuilder` 重命名为规范 `XxxRequestBuilder`（均有 `execute()`，确为请求类型）：
  - application: `AccessDataSearchBlockBuilder`→`AccessDataSearchBlockRequestBuilder`、`AccessDataSearchCustomBuilder`→`AccessDataSearchCustomRequestBuilder`、`AccessDataSearchWorkplaceBuilder`→`AccessDataSearchWorkplaceRequestBuilder`
  - docs: `PatchFormFieldQuestionBuilder`→`PatchFormFieldQuestionRequestBuilder`
- 旧名作 `#[deprecated(note="...")] pub type XxxBuilder = XxxRequestBuilder;`（v0.18→v1.0 软迁移，源码兼容 + warning）
- 同步 docs 的 `PatchFormFieldQuestionBuilder` re-export（`docs/base/bitable/mod.rs`，双块：新名 + `#[allow(deprecated)]` 旧名 alias）；application 3 个无 re-export
- **BREAKING**（软）：公开类型重命名，`#[deprecated]` alias 保证源码兼容（编译通过 + warning），v1.0 移除 alias

## Capabilities

### New Capabilities
- `application-docs-request-naming`: openlark-application 与 openlark-docs 的请求类型 builder SHALL 统一 `RequestBuilder` 后缀；旧 `Builder` 名 SHALL 作 `#[deprecated]` type alias 保留至 v1.0。

### Modified Capabilities
<!-- 无 -->

## Impact

- **openlark-application**：3 个 struct+impl 重命名 + 3 个 `#[deprecated]` alias + 内部引用/测试同步（无 re-export）。
- **openlark-docs**：1 个 struct+impl 重命名 + 1 alias + `docs/base/bitable/mod.rs` re-export 双块 + 测试同步。
- **破坏性**：软 breaking——alias 保证源码兼容（编译 + warning）。v1.0 移除 alias。
- **非目标**：不动 `RecordFieldsBuilder`（docs 真 builder，无 execute）；不动 body 模型 `XxxRequest`；不动其他 crate（platform 97 待后续 change）。
```

## openspec/changes/unify-application-docs-request-naming/design.md

- Source: openspec/changes/unify-application-docs-request-naming/design.md
- Lines: 1-36
- SHA256: 09a2c75493f4a99a4b2dee5d32bc22e97d5863caf98414cc23819f85c00b52e8

```md
## Context

#271 命名统一后续批次。auth pilot（PR #280）已确立并验证模式：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` type alias（body 模型 `XxxRequest` 不动）。本批次应用到 application+docs 的 4 个裸 Builder。

约束：v0.18 breaking 窗口；`#[deprecated]` alias 机制已在 auth pilot + /tmp spike 实证（旧名触发 warning、新名无 warning、方法经 alias 可调用）。

## Goals / Non-Goals

**Goals:** application(3)+docs(1) 共 4 个请求 builder 统一 `XxxRequestBuilder` + `#[deprecated]` alias 软迁移。

**Non-Goals:** 不动 `RecordFieldsBuilder`（真 builder）；不动 body 模型；不动其他 crate（platform 97 后续）；不在 v0.18 硬移除旧名。

## Decisions

### 决策 1：方向 Builder → RequestBuilder（沿用 auth pilot）
4 个目标全部撞 body 模型名（`XxxRequest` 已存在）→ builder 统一 `XxxRequestBuilder`，body 保持 `XxxRequest`。零撞名，对齐 helpdesk/auth pilot。

### 决策 2：#[deprecated] type alias 软迁移（沿用 auth pilot）
`pub struct XxxRequestBuilder` + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`。源码兼容 + warning，v1.0 移除。

### 决策 3：re-export 双块（仅 docs PatchFormFieldQuestion）
docs 的 `PatchFormFieldQuestionBuilder` 有 re-export（`docs/base/bitable/mod.rs`）→ 双块：新名 + `#[allow(deprecated)]` 旧名 alias。application 3 个无 re-export，仅定义文件内改。

## Risks / Trade-offs

- **alias 放 `#[cfg(test)]` 前**（auth pilot clippy `items_after_test_module` 教训）：alias 必须在 test 模块之前。
- **fmt**（auth pilot CI 教训）：改完跑 `cargo fmt --all -- --check` 再 push。
- 软 breaking，alias 保证源码兼容。回滚 = revert。

## Migration Plan

v0.18：struct 重命名 + alias。v1.0：移除 alias。CHANGELOG v0.18 breaking 段记录。

## Open Questions

4 个是否全 pub（决定 alias 数量）；docs 是否有 service 方法返回类型需改——build 阶段核实。
```

## openspec/changes/unify-application-docs-request-naming/tasks.md

- Source: openspec/changes/unify-application-docs-request-naming/tasks.md
- Lines: 1-16
- SHA256: 4cb86f68c9124385cd3f946512126afda20a18fd8188e3dbd4c6bda1997c5c87

```md
# Tasks — unify-application-docs-request-naming（#271 批次）

> application(3)+docs(1) 共 4 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。auth pilot 模式直接应用。

## 1. 重命名 + alias + re-export

- [ ] 1.1 application 3 个（AccessDataSearchBlock/Custom/Workplace）：struct+impl+测试重命名 + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（alias 放 `#[cfg(test)]` 前）；无 re-export
- [ ] 1.2 docs 1 个（PatchFormFieldQuestion）：struct+impl+测试重命名 + alias；同步 `docs/base/bitable/mod.rs` re-export 双块（新名 + `#[allow(deprecated)]` 旧名）

## 2. 验证

- [ ] 2.1 `cargo build --workspace --all-features` exit 0
- [ ] 2.2 三组 feature clippy（default / --all-features / --no-default-features + `-D warnings`）均 exit 0
- [ ] 2.3 `cargo test -p openlark-application -p openlark-docs` 0 failed
- [ ] 2.4 **`cargo fmt --all -- --check` exit 0**（auth pilot CI 教训：push 前必跑）
- [ ] 2.5 grep 确认 4 RequestBuilder struct + 4 deprecated alias + RecordFieldsBuilder 未动
```

## openspec/changes/unify-application-docs-request-naming/specs/application-docs-request-naming/spec.md

- Source: openspec/changes/unify-application-docs-request-naming/specs/application-docs-request-naming/spec.md
- Lines: 1-42
- SHA256: 3183b534a183fa2f305d27048a0b888cc9f6d02e33f871d0d76068683c25293e

```md
## ADDED Requirements

### Requirement: application+docs 请求 builder 统一 RequestBuilder 后缀
openlark-application 与 openlark-docs 的请求类型 builder SHALL 统一使用 `RequestBuilder` 后缀（对齐 auth pilot 与 helpdesk 模式）。4 个原 `XxxBuilder`（均撞 body 模型 `XxxRequest`）SHALL 重命名为 `XxxRequestBuilder`。

#### Scenario: 4 个请求类型重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（AccessDataSearchBlock/AccessDataSearchCustom/AccessDataSearchWorkplace/PatchFormFieldQuestion）
- **THEN** 4 个新 `XxxRequestBuilder` struct 存在

#### Scenario: RecordFieldsBuilder 不被误改
- **WHEN** 检查 `crates/openlark-docs/src/base/bitable/v1/field_types.rs`
- **THEN** `RecordFieldsBuilder`（真 builder，无 execute）保留原样

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 4 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（v0.18→v1.0 软迁移）。alias 放 `#[cfg(test)]` 前（避免 clippy items_after_test_module）。

#### Scenario: 旧名 alias 存在且标 deprecated
- **WHEN** grep `pub type XxxBuilder`（4 个旧名）
- **THEN** 4 个 type alias 存在，各自带 `#[deprecated]`

#### Scenario: alias 放在 test 模块前
- **WHEN** 检查 4 个定义文件的 alias 与 `#[cfg(test)]` 顺序
- **THEN** alias 在 `#[cfg(test)]` 之前（不触发 clippy items_after_test_module）

### Requirement: 重命名不破坏构建、lint 与测试
本次重命名 + alias SHALL 不破坏 workspace 构建/clippy/测试，SHALL 同步 docs 的 PatchFormFieldQuestion re-export。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** clippy default / --all-features / --no-default-features + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: application+docs 测试通过
- **WHEN** `cargo test -p openlark-application -p openlark-docs`
- **THEN** 0 failed

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0
```

