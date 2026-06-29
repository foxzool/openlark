---
comet_change: unify-application-docs-request-naming
role: technical-design
canonical_spec: openspec
---

# Design — unify-application-docs-request-naming（#271 application+docs 批次）

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/unify-application-docs-request-naming/specs/application-docs-request-naming/spec.md` 为 canonical。

## 1. 背景与目标

#271 命名统一后续批次。auth pilot（PR #280 已归档）验证了模式：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` type alias。本批次应用到 application+docs 的 4 个裸 Builder（精确摸底：application 3、docs 1；`RecordFieldsBuilder` 是真 builder 排除）。

## 2. 关键核实（design 阶段）

| 断言 | 验证 | 结论 |
|------|------|------|
| 4 个是请求类型（有 execute） | grep execute | ✅ |
| 4 个撞 body 模型名 | `XxxRequest` 已存在 | ✅ 全 →RequestBuilder |
| 无 trait impl | grep `impl Trait for XxxBuilder` | ✅ 4 个均 0，alias 无兼容性顾虑 |
| 无 service 方法返回 | grep 返回类型 | ✅ 仅 docs 1 个 re-export 待同步 |
| RecordFieldsBuilder 是真 builder | 无 execute | ✅ 排除 |

## 3. 实现步骤（每类型，同 auth pilot）

### 3.1 重命名 struct + impl
`pub struct XxxBuilder`+`impl XxxBuilder` → `XxxRequestBuilder`。方法/字段/逻辑不变。
- application: AccessDataSearchBlock / AccessDataSearchCustom / AccessDataSearchWorkplace（各自 search.rs）
- docs: PatchFormFieldQuestion（base/bitable/v1/app/table/form/field/patch.rs）

### 3.2 #[deprecated] type alias（放 #[cfg(test)] 前）
```rust
#[deprecated(note = "renamed to XxxRequestBuilder, will be removed in v1.0 (#271)")]
pub type XxxBuilder = XxxRequestBuilder;
```

### 3.3 re-export 同步（仅 docs PatchFormFieldQuestion）
`docs/base/bitable/mod.rs` 双块：新名 + `#[allow(deprecated)] pub use ...::PatchFormFieldQuestionBuilder;`。application 3 个无 re-export。

### 3.4 测试 + CHANGELOG
测试引用改新名；CHANGELOG v0.18 breaking 段记录 4 个重命名。

## 4. 测试策略

- `cargo build --workspace --all-features` exit 0
- 三组 clippy（-D warnings）exit 0
- `cargo test -p openlark-application -p openlark-docs` 0 failed
- **`cargo fmt --all -- --check` exit 0**（auth pilot CI 教训，push 前必跑）
- grep：4 RequestBuilder struct + 4 deprecated alias + RecordFieldsBuilder 未动

## 5. 风险与回滚

软 breaking（alias 源码兼容）。回滚 = revert。v1.0 移除 alias 才硬 breaking。
