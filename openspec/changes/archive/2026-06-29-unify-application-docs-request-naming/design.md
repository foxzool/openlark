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
