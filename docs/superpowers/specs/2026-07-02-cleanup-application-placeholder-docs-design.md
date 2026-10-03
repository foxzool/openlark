---
comet_change: cleanup-application-placeholder-docs
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-02-cleanup-application-placeholder-docs
status: final
---

# Design: cleanup-application-placeholder-docs

替换 `openlark-application` crate 578 行 `/// 待补充文档。` 占位为有意义 doc，并修正 struct 占位位置。承接 #273 #1（analytics recipe）+ #4（codegen 闭环）+ #2（死测试）。批量第 2 个 change（small-crates 待），与已归档的 docs change 同源。

## 探勘事实

| 维度 | 事实 |
|------|------|
| 总量 | 578 占位 / 91 文件 / 全是 `待补充文档。` |
| item 类型 | fn 269 / struct 190 / field 116 / module 3（0 enum、0 variant） |
| 版本分布 | v1=213 / v5=12 / v6=352 / root=1 |
| 位置 bug | 190 struct 占位**全部**紧跟 `#[derive(...)]` 后，0 多属性叠加 |
| field 分布 | 87 `data`（机械）+ 29 named（17 字段名，逐个翻译） |

全部 91 文件在 `crates/openlark-application/src/application/application/v{1,5,6}/<sub-domain>/`，文件 `//!` 头齐全（recipe 标题来源）。样本 `v6/app/create.rs` 完美匹配 #1 analytics 同构模式。

## Recipe（#1 机械模式）

每条占位 doc = `<API 中文名（取自文件 //! 头）>+<item 角色>`：

| item | doc 文案 | 触发条件 |
|------|---------|---------|
| Request struct | `<API>的请求。` | 占位下一条是 `pub struct XxxRequest` |
| Response struct | `<API>的响应。` | 占位下一条是 `pub struct XxxResponse` |
| field `data` | `响应数据。` | 占位下一条是 `pub data:` |
| field（named） | `<字段中文名>。` | 占位下一条是其它 `pub <name>:`，读 name 翻译 |
| `fn new` | `创建请求实例。` | 占位下一条是 `pub fn new` |
| `fn execute` | `执行<API>请求。` | 占位下一条是 `pub async fn execute` |
| `fn execute_with_options` | `带自定义请求选项执行。` | 占位下一条是 `pub async fn execute_with_options` |
| `fn <field>`（builder setter） | `设置<字段中文名>。` | 占位下一条是 `pub fn <field>(mut self, ... -> Self)`（outlier，仅 3 处：app_id/badge/new_owner_id） |
| `fn <api>`（mod.rs 门面） | `返回<API>请求构建器。` | 占位下一条是 `pub fn <api>(&self) -> ...Request`（outlier，仅 1 处：v6/app/mod.rs get） |
| module | `<子模块 API 说明>。` | 占位下一条是 `pub mod` |

named field（17 个）翻译表由 implementer 读字段名按飞书常识填写（如 `app_id`→应用 ID、`app_name`→应用名称、`contacts_range`→通讯录范围、`badge`→徽标）。

## 位置修正

- **struct 占位（190）**：`///` 当前在 `#[derive(...)]` 与 `pub struct` 之间，移到 `#[derive]` **前**。统一 3 行交换：

  ```rust
  // 修正前
  #[derive(Debug, Clone)]
  /// 待补充文档。
  pub struct CreateAppRequest {

  // 修正后
  /// 创建应用的请求。
  #[derive(Debug, Clone)]
  pub struct CreateAppRequest {
  ```

- **fn/field/module 占位（388）**：位置已正确（`///` 在 item 前），仅原位换文案。

已验证 0 多属性边界（所有 190 struct 占位上方恰好一条 `#[derive(...)]`，无 `#[serde]` 等叠加），变换 100% 机械。

## 执行（subagent-driven，分组 B：版本×子域）

~7-8 组，每组 implementer + task reviewer：

| 组 | 范围 | 占位约 |
|----|------|-------|
| G1 | v1（app/app_version/collaborator 等） | ~110 |
| G2 | v1（application/feedback/management/owner/visibility 等） | ~103 |
| G3 | v5（2 文件）+ root mod | ~13 |
| G4-G7 | v6 按 sub-domain 拆 4 组 | ~88/组 |

确切分组由 plan 按 sub-domain 文件清单定界。每组边界 = 一组文件路径，覆盖校验简单。

每组流程：implementer 按 recipe 回补组内文件占位 + 修正 struct 位置 + 自验 `grep` 组内文件占位=0 → task reviewer 审 spec 合规 + 质量。

## 验证

1. 占位守门：`grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/` 输出为空
2. 位置守门：application crate 内 `#[derive(...)]` 后不紧跟 `///`（`grep -rnE -A1 '^#\[derive' crates/openlark-application/src/ | grep '/// 待补充文档'` 为空）
3. `cargo doc --workspace --all-features` missing_docs=0
4. `cargo fmt --check` + `just lint` exit 0
5. application crate 现有测试不破

## 风险

- [规模 578 诱发偷懒] → recipe 强制引用真实 API 名 + 占位 grep 守门 + pilot 先行（1 文件验证 recipe + 位置变换）
- [doc 位置漏修] → 位置守门 grep
- [named field 翻译漂移] → 17 个字段名有限，reviewer 抽样核对飞书术语

## 非目标

不改逻辑；不动其它 crate（docs/application/small-crates 各自 change）；不动作 TestCheck；不升 workspace deny。

## Spec Patch

无。delta spec（`specs/missing-docs-quality/spec.md`）已含 application crate 2 scenarios；主 spec `missing-docs-quality` 由 `cleanup-docs-placeholder-docs` 归档时已建。本 change 仅 ADDED application 场景到既有 capability。
