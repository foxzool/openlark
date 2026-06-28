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
