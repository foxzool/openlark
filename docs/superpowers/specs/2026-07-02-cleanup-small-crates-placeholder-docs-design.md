---
comet_change: cleanup-small-crates-placeholder-docs
role: technical-design
canonical_spec: openspec
---

# Design: cleanup-small-crates-placeholder-docs

替换 5 个中小 crate（mail/workflow/meeting/user/hr）共 335 行 `/// 待补充文档。` 占位为有意义 doc，并修正 struct 占位位置。承接 #273 #1（analytics recipe）+ #4（codegen 闭环）+ #2（死测试）。批量第 3 个 change（最后一个），与已归档的 docs/application change 同源。

## 探勘事实

5 crate 全量勘探（并行 workflow 确认）：

| crate | 占位 | 文件 | fn | struct | field | module | impl | struct 位置 bug | data | 命名字段 |
|-------|------|------|-----|--------|-------|--------|------|----------------|------|----------|
| mail | 104 | 15 | 45 | 35 | 22 | 0 | 0 | 35 | 15 | 7 |
| workflow | 78 | 29 | 48 | 7 | 15 | 8 | 0 | 7 | 0 | 11 |
| meeting | 65 | 41 | 50 | 2 | 13 | 0 | 0 | 2 | 0 | 11 |
| user | 47 | 7 | 21 | 15 | 8 | 0 | 0 | 15 | 7 | 1 |
| hr | 41 | 3 | 9 | 15 | 8 | 0 | 6 | 4 | 0 | 8 |
| **合计** | **335** | **95** | **173** | **74** | **66** | **8** | **6** | **63** | **22** | **38** |

| 维度 | 事实 |
|------|------|
| item 类型 | fn 173 / struct 74 / field 66 / module 8 / impl 块 6（**0 enum variant**） |
| 位置 bug | 63 struct 占位紧跟单 `#[derive(...)]` 后，**0 多属性叠加** |
| field 分布 | 22 `data`（机械）+ 38 named（33 去重名） |
| recipe_match | 5 crate 全 mechanical |

全部 95 文件有 `//!` 头（recipe 标题来源）。样本：mail `mail/v1/mailgroup/manager/list.rs`、workflow `approval/v4/task/query.rs`、meeting `calendar/v4/calendar/get.rs`、user `personal_settings/v1/system_status/get.rs`、hr `corehr/v2/probation/edit.rs`——全部匹配 #1 analytics 同构模式。

**关键**：0 enum variant → 全机械模式（同 application 578），非 docs 的 74 variant 语义工作。recipe 已被 application 实证。

## Recipe（#1 机械模式，application patched + impl 块行）

每条占位 doc = `<API 中文名（取自文件 //! 头）>+<item 角色>`：

| item | doc 文案 | 触发条件 |
|------|---------|---------|
| Request struct | `<API>的请求。`（+ 位置交换） | 占位下一条是 `pub struct XxxRequest` |
| Response struct | `<API>的响应。`（+ 位置交换） | 占位下一条是 `pub struct XxxResponse` |
| 其它 struct（Body/Data/DeviceInfo 等） | `<结构中文名>。`（+ 位置交换） | 占位下一条是其它 `pub struct` |
| field `data` | `响应数据。` | 占位下一条是 `pub data:` |
| field（named） | `<字段中文名>。` | 占位下一条是其它 `pub <name>:`，查翻译表 |
| `fn new` | `创建请求实例。` | 占位下一条是 `pub fn new` |
| `fn execute` | `执行<API>请求。` | 占位下一条是 `pub async fn execute` |
| `fn execute_with_options` | `带自定义请求选项执行。` | 占位下一条是 `pub async fn execute_with_options` |
| `fn <field>`（builder setter） | `设置<字段中文名>。` | 占位下一条是 `pub fn <field>(mut self, ...) -> Self` |
| module | `<子模块 API 说明>。` | 占位下一条是 `pub mod`（workflow 8 处） |
| **impl 块** | `<API>请求构建器实现。` | 占位下一条是 `impl XxxRequest {`（hr 6 处，recipe 新增行） |

mod.rs factory 行（application 有 1 处）在本 change = 0，不需要。implementer 读文件确认 item 类型后套对应行；遇罕见角色（如 impl 块多行占位）按角色语义写合理中文 doc。

### 命名字段翻译表（33 去重名，跨 crate 一致）

implementer 按字段名 + 飞书常识填写，同名跨 crate 必须一致：

| 字段名 | 中文 | 出现 crate |
|--------|------|-----------|
| manager_ids | 管理员 ID 列表 | mail |
| managers | 管理员 | mail |
| manager_id | 管理员 ID | mail |
| manager_email | 管理员邮箱 | mail |
| download_url | 下载地址 | mail |
| expire_time | 过期时间 | mail |
| id | ID | workflow |
| name | 名称 | workflow |
| level | 层级 | workflow |
| has_sub_district | 是否含下级区划 | workflow |
| parent_districts | 上级区划 | workflow |
| version | 版本 | workflow |
| has_more | 是否有更多 | workflow |
| page_token | 分页标记 | workflow |
| items | 列表项 | workflow, hr |
| district_ids | 区划 ID 列表 | workflow |
| keyword | 关键词 | workflow |
| calendar | 日历 | meeting |
| capacity | 容量 | meeting |
| capacity_max | 最大容量 | meeting |
| description | 描述 | meeting |
| device_id | 设备 ID | meeting |
| device_name | 设备名称 | meeting |
| device_type | 设备类型 | meeting |
| devices | 设备列表 | meeting |
| room_id | 会议室 ID | meeting |
| room_name | 会议室名称 | meeting |
| status | 状态 | meeting |
| user_ids | 用户 ID 列表 | user |
| employee_id | 员工 ID | hr |
| probation | 试用期 | hr |
| from_date | 开始日期 | hr |
| to_date | 结束日期 | hr |

builder setter（14 去重名）用 `设置<字段中文名>。`：`manager_ids`→设置管理员 ID 列表、`user_id`→设置用户 ID、`topic`→设置主题、`user_id_type`→设置用户 ID 类型、`page_size`→设置分页大小、`page_token`→设置分页标记、`root_district_id`→设置根区划 ID、`list_type`→设置列表类型、`locale`→设置语言、`district_ids`→设置区划 ID 列表、`keyword`→设置关键词、`status_id`→设置状态 ID、`user_ids`→设置用户 ID 列表、`body`→设置请求体。implementer 按飞书常识微调。

## 位置修正

- **struct 占位（63）**：`///` 当前在 `#[derive(...)]` 与 `pub struct` 之间，移到 `#[derive]` **前**：

  ```rust
  // 修正前
  #[derive(Debug, Clone)]
  /// 待补充文档。
  pub struct XxxRequest {

  // 修正后
  /// <API>的请求。
  #[derive(Debug, Clone)]
  pub struct XxxRequest {
  ```

- **fn/field/module/impl 占位（272）**：位置已正确（`///` 在 item 前），仅原位换文案。

已验证 0 多属性边界（所有 63 struct 占位上方恰好一条 `#[derive(...)]`，无 `#[serde]` 等叠加），变换 100% 机械。hr `edit.rs` 的 struct 已在正确位置（`///` 在 `#[derive]` 前），作为正确顺序参照。

## 执行（subagent-driven，按 crate 5 组）

| 组 | crate | 占位 | 文件 | 备注 |
|----|-------|------|------|------|
| G1 | mail | 104 | 15 | 最大；pilot 1 文件验证 recipe + 位置变换 |
| G2 | workflow | 78 | 29 | struct/field 集中在 district/list.rs(23)+search.rs(12)；余多单 fn new 文件 |
| G3 | meeting | 65 | 41 | struct 位置 bug 仅 responses.rs 2 处；多单 fn 文件 |
| G4 | user | 47 | 7 | 全在 system_status/7 文件 |
| G5 | hr | 41 | 3 | 仅 3 文件；含 6 处 impl 块（新 recipe 角色） |

每组 = 一个 crate（独立编译单元）。mail 104 < application G5=138 实证可行，无需拆分。每组流程：implementer 按 recipe 回补 crate 全部占位 + 修正 struct 位置 + 自验 `grep` crate 占位=0 + `cargo doc -p openlark-<crate>` 无 warning。

## 验证

1. 占位守门：`grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-{mail,workflow,meeting,user,hr}/src/` 输出为空
2. 位置守门：5 crate 内 `#[derive(...)]` 后不紧跟 `/// 待补充文档`（grep 为空）
3. 逐 crate `cargo doc -p openlark-<crate>` 无 warning
4. `cargo doc --workspace --all-features` missing_docs=0
5. `cargo fmt --check` + `just lint` exit 0
6. 5 crate 现有测试不破

## 风险

- [规模 335 诱发偷懒] → recipe 强制引用真实 API 名 + 占位 grep 守门 + mail pilot 先行
- [doc 位置漏修] → 位置守门 grep
- [跨 crate 同名字段翻译漂移] → 共享翻译表（33 字段 + 14 setter），reviewer 抽样核对
- [impl 块是新 recipe 角色] → recipe 新增行 + hr 组自验 cargo doc

## 非目标

不改逻辑；不动其它 crate（docs/application 各自 change 已归档）；不动作 TestCheck；不升 workspace deny。

## Spec Patch

无。delta spec（`specs/missing-docs-quality/spec.md`）open 阶段已写 small-crates 2 scenarios（无占位 + 位置）；主 spec missing-docs-quality 现 2 requirements（docs + application）。本 change 仅 ADDED small-crates 场景（已存在），归档时合并为第 3 requirement。
