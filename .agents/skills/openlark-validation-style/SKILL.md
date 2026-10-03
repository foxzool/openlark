---
name: openlark-validation-style
description: OpenLark Rust SDK 的 validate() 写法规范（必填校验 / 空白字符串处理）。当需要统一或评审 feature crate 中 validate(&self) -> SDKResult<()> 的实现、确定何时用 validate_required! 宏、空白字符串是否视为空、列表字段同时校验非空与限长等问题时使用。
argument-hint: "[crate-name|path|request-file]"
allowed-tools: Read, Grep
---

> 权威源：`crates/openlark-core/AGENTS.md`，本文件与之冲突时以它为准。

# OpenLark Validation Style

## 🧭 技能路由指南

**本技能适用场景：**
- 需要统一/评审 `validate()` 方法写法
- 不确定字符串字段是否要 `.trim()`、空白字符串算不算空
- 列表字段需要同时校验“非空 + 最大长度”
- 多条校验如何组织（连续宏快速失败 / 用构建器聚合）

**其他技能：**
- 项目级代码规范检查（架构/API/导出/校验一体）→ `Skill(openlark-code-standards)`
- 添加/重构 API → `Skill(openlark-api)`
- 审查整体设计规范 → `Skill(openlark-design-review)`

### 关键词触发映射

- validate、必填校验、validate_required、validate_required_list、空白字符串、校验聚合 → `openlark-validation-style`
- 代码规范、规范检查、风格一致性、体检 → `openlark-code-standards`
- 架构设计、public API、收敛方案、feature gating、兼容策略 → `openlark-design-review`
- 新增 API、重构 API、Builder、Request/Response、mod.rs 导出 → `openlark-api`
- 覆盖率、缺失 API、实现数量、CSV 对比 → `openlark-api-validation`

### 双向跳转规则

- 若校验问题已扩展到命名/导出/端点体系，转 `openlark-code-standards`。
- 若校验争议本质是架构范式冲突（例如 Request/Service 边界），转 `openlark-design-review`。

---

## 目标

在各 feature crate 的请求/Builder `validate(&self) -> SDKResult<()>` 中统一：

- 必填字段校验写法（减少样板代码）
- 空白字符串是否视为缺失（避免不同 crate 行为漂移）
- 失败时返回的错误类型与消息风格

## 规则

### 1) 字符串字段默认用 `validate_required!` 宏——字符串已自动 trim，传 `self.field` 即可

`validate_required!` 宏内部调用 `Validatable::is_empty_trimmed`（`crates/openlark-core/src/lib.rs:53-59`）。
对 `&str` / `String` 的实现是 `self.trim().is_empty()`（`crates/openlark-core/src/validation/validatable.rs:7-17`，提交 `6fe4a6ca6`）。

也就是说：**字符串字段会自动 `trim`，纯空白字符串视为空。直接传 `self.field`，不要再额外 `.trim()`。**

适用场景：必填校验失败就应该立即返回 `Err(...)`（快速失败）。

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    // 字符串字段：自动 trim，空白算空，直接传字段本身
    openlark_core::validate_required!(self.app_token, "app_token 不能为空");
    openlark_core::validate_required!(self.table_id, "table_id 不能为空");
    Ok(())
}
```

> 现码惯例一致，例如 `crates/openlark-ai/src/ai/document_ai/v1/bank_card/recognize.rs:30`
> `validate_required!(self.file, "file 不能为空");` —— 字符串字段未手动 `.trim()`。

非字符串容器（如 `Vec<T>` / `&[T]`）的 `is_empty_trimmed` 退化为“长度是否为 0”（`validatable.rs:25-35`），同样直接传字段即可：

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required!(self.items, "items 不能为空");
    Ok(())
}
```

> 若列表字段还需要“最大长度”限制，应改用下面的 `validate_required_list!`，不要在 `validate_required!` 之外再手写长度判断。

### 2) 列表字段用 `validate_required_list!` 同时校验非空 + 最大长度

`validate_required_list!(field, max_len, msg)` 一次完成两项检查：先判空、再判是否超过 `max_len`，任一失败即快速返回（`crates/openlark-core/src/lib.rs:72-82`）。msg 用于两种失败场景，建议写明两条约束。

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
    Ok(())
}
```

> 广泛使用于 hr/docs/ai/workflow/mail/communication/user 等，例如
> `crates/openlark-ai/src/ai/translation/v1/text/translate.rs:32`
> `validate_required_list!(self.texts, 50, "texts 不能为空且不能超过 50 个");`，
> `crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_update.rs:114`
> `validate_required_list!(self.records, 1000, "records 不能为空且单次最多 1000 条");`。

### 3) 多条校验：连续写多个宏（遇首个失败即 return），或用 `DefaultValidateBuilder` 聚合

项目**没有统一的“聚合多错误”宏**。两种推荐写法：

**a) 连续宏——快速失败（绝大多数场景首选）**

宏在失败时直接 `return Err(...)`，因此按字段顺序连续书写即可；首个失败字段处返回。

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required!(self.app_token, "app_token 不能为空");
    openlark_core::validate_required!(self.table_id, "table_id 不能为空");
    openlark_core::validate_required_list!(self.records, 1000, "records 不能为空且单次最多 1000 条");
    Ok(())
}
```

**b) 需要收集多个错误一次性返回——用 `DefaultValidateBuilder`**

`openlark_core::validation::DefaultValidateBuilder` 实现 `ValidateBuilder` trait，链式 `required` / `length` / `custom` 累积错误，`build()` 返回 `Result<String, Vec<String>>`（`crates/openlark-core/src/validation/core.rs:154-266`）。

```rust
use openlark_core::validation::{DefaultValidateBuilder, ValidateBuilder};

fn validate(&self) -> openlark_core::SDKResult<()> {
    let result = DefaultValidateBuilder::new()
        .required(Some(self.name.clone()), "name")
        .length(self.code.clone(), 1, 32, "code")
        .build();
    match result {
        Ok(_) => Ok(()),
        Err(errs) => Err(openlark_core::CoreError::validation_msg(errs.join("; "))),
    }
}
```

> 注意：`DefaultValidateBuilder` 当前仅在 core 内定义与使用，feature crate 的现码一律采用方式 (a) 的连续宏写法。优先沿用 (a)；确有聚合需求时再评估 (b)。

### 4) 禁止在 feature crate 内重复定义 `validate_required!` / `validate_required_list!`（或同语义宏）

统一复用 `openlark_core::validate_required!` 与 `openlark_core::validate_required_list!`，避免各 crate 的判空规则/错误类型不一致。

> 同步约束：`openlark_core::validation::validate_required`（函数）**已删除**，不要引用或照抄旧文档里的函数写法，否则编译失败（见 `crates/openlark-core/AGENTS.md:91,125` 反模式）。

## 速查

- 字符串必填 → `validate_required!(self.field, "...")`；**自动 trim，空白算空，不要手动 `.trim()`**
- 列表必填（不限长）→ `validate_required!(self.items, "...")`
- 列表必填 + 限长 → `validate_required_list!(self.items, max_len, "不能为空且不能超过 N 个")`
- 多条校验按顺序快速失败 → 连续写多个宏（首个失败即 return）
- 必须一次性返回全部错误 → `DefaultValidateBuilder`（`openlark_core::validation::DefaultValidateBuilder`），项目无统一聚合宏
