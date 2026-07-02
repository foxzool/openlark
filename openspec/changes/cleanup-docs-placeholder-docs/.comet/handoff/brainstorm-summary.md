# Brainstorm Summary

- Change: cleanup-docs-placeholder-docs
- Date: 2026-07-02

## 确认的技术方案

**逐项语义 doc**（非 #1 机械 recipe）。docs crate 144 占位 = 74 enum variant + 58 struct field + 4 struct + 3 const + 3 other + 2 type（92% 是 variant/field）。逐项读名称 + 所在 enum/struct 上下文 + Feishu 常识，写有意义中文描述：
- enum variant `OpenId` → `/// 开放平台用户 ID`
- struct field `app_id` → `/// 应用 ID`
- struct/const/type → 按其职责描述

**位置修正**：仅 4 处 `#[derive]` 后置 `///` → 移前；其余 variant/field 无 derive 问题。

**执行**：14 文件，subagent-driven 按文件/域分组；每 subagent 读 item 语义 + 写 doc + 自验（grep 无占位 + cargo doc 该文件 0 警告）。

## 关键取舍与风险

- **docs 是语义工作（非机械）**：open 阶段 design.md 的"recipe 仿 #1"假设被勘探推翻——docs 占位 92% 是 variant/field，需逐项判断，非 `<//!标题>+<角色>` 模板。Design Doc 修正此点。
- **4/14 文件无 //! 头**：doc 从 item 上下文（非文件标题）派生。
- **语义质量靠 review**：非机械可验（grep 只能验"无占位"，不能验"有意义"）；final review + cargo doc 兜底。
- **application/small-crates 是 #1 机械模式**（占位在 new/execute/struct）——docs 是异类。先做 docs 验证语义方法。

## 测试策略

- grep 守门：`grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/` 为空。
- 编译：`cargo doc -p openlark-docs --all-features` 0 警告；`cargo doc --workspace --all-features` 0。
- 位置守门：`#[derive]` 后不紧跟 `///`。
- 回归：`cargo fmt --check` + `just lint`；docs 现有测试不破。
- 语义质量：final code review 抽查 variant/field doc 是否有意义（非名称堆砌）。

## Spec Patch

无。delta `missing-docs-quality`（ADDED：公开项 MUST 有有意义 doc 非 placeholder + docs crate 无占位场景 + doc 在 #[derive] 前场景）已覆盖。"有意义"由 review 守，grep 守"非占位"。
