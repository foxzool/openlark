# Issue #41 整改清单与优先级

> **目标**: 统一 OpenLark 全仓库 API 实现规范，消除风格混用  
> **范围**: 18 个业务模块 crates，1,560+ APIs  
> **状态**: P0 已完成，P1 已完成 40/40（RequestOption 透传），剩余 P1/P2 评估完成待执行

---

## 1. 混用点概览（按优先级排序）

### P0 - 阻塞级（影响 API 可用性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 1 | **函数式 API 与 Builder 模式混用** | `crates/openlark-docs/src/ccm/explorer/v2/mod.rs` | 使用 `pub async fn get_folder_meta(config: &Config, ...)` 函数式 API，而非 Builder 模式 | **已完成**: 删除 8 个函数式 API，统一使用 Builder 模式的 Request |
| 2 | **函数式 API 与 Builder 模式混用** | `crates/openlark-docs/src/ccm/permission/v2/mod.rs` | 同上，使用函数式 API `check_member_permission(config, params)` | **已完成**: 删除 3 个函数式 API，统一使用 Builder 模式 |
| 3 | **缺少 `execute_with_options`** | `crates/openlark-meeting/src/calendar/calendar/v4/*` | 44 个 API 仅提供 `execute()`，无 `execute_with_options(RequestOption)` | **已验证**: 实际检查显示 calendar/v4 所有 59 个文件均已包含 `execute_with_options`，无需修改 |

### P1 - 高风险级（影响一致性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 4 | **RequestOption 透传不一致** | `openlark-platform` (16 files), `openlark-helpdesk` (24 files) | 部分 API 使用 `Transport::request(..., None)`，部分使用 `Some(option)` | **已完成**: 统一将 `execute()` 委托给 `execute_with_options(RequestOption::default())`，消除 40 个文件中的代码重复 |
| 5 | **端点定义方式混用** | `openlark-docs` vs `openlark-communication` | docs 使用 `enum ApiEndpoint`，communication 使用常量 `IM_V1_MESSAGES` | **评估结果**: 保持现状，各 crate 内部风格统一即可 |
| 6 | **validate_required! 使用不完整** | `openlark-docs`(37), `openlark-communication`(3), `openlark-hr`(2), `openlark-platform`(2) | 44 个 API 文件 / 101 个必填字段缺少 `validate_required!` 校验 | **待执行**: 补充 44 个文件的必填字段校验 |

### P2 - 中风险级（影响可维护性）

| # | 混用类型 | 具体位置 | 问题描述 | 整改动作 |
|---|---------|---------|---------|---------|
| 7 | **mod.rs 导出风格不一致** | `openlark-cardkit`, `openlark-docs`, `openlark-hr` | 38 个 mod.rs 文件仅使用 `pub mod models;`，缺少显式 `pub use` 导出 | **待执行**: 补充 38 个文件的显式导出 |
| 8 | **serde_json::Value 过度使用** | `calendar/v4/exchange_binding/*`, `calendar/v4/freebusy/*` | 5 个目标 API 使用 `serde_json::Value` 作为请求/响应体 | **待执行**: 优先替换 5 个目标 API 为强类型结构体；全仓库 191 个文件存在类似情况，分阶段处理 |
| 9 | **文档注释格式不统一** | 全仓库分散 | 1658 个 API 文件缺少完整文档注释（仅 18/1676 完整） | **待执行**: 分阶段补充 docPath 和 doc 链接 |

---

## 2. 评估详情

### 2.1 validate_required! 不完整（P1 #6）

**评估方法**: 检查 `execute_with_options` 方法中是否使用 `is_empty()`/`is_none()` 手工校验，但未使用 `validate_required!` 宏

**结果**:
- 高置信缺失项: **44 文件 / 101 字段**
- 按 crate 分布:
  - `openlark-docs`: 37 文件
  - `openlark-communication`: 3 文件
  - `openlark-hr`: 2 文件
  - `openlark-platform`: 2 文件

**建议**: 将手工 `is_empty()` 校验统一替换为 `validate_required!` 宏，提升一致性

### 2.2 mod.rs 导出风格不一致（P2 #7）

**评估方法**: 统计 `pub mod models;` 文件中是否同时存在 `pub use ...models` 显式导出

**结果**:
- 全量 mod.rs: **829 个**
- 包含 `pub mod models;`: **108 个**
- 显式导出（`pub use ...models`）: **70 个**
- 仅模块导出（`pub mod models;`）: **38 个**
- 风格不一致的 crate:
  - `openlark-cardkit`: 1 显式 + 1 仅模块
  - `openlark-docs`: 34 显式 + 2 仅模块
  - `openlark-hr`: 19 显式 + 2 仅模块

**建议**: 将 38 个仅模块导出的文件补充 `pub use models::*;` 或显式导出需要的类型

### 2.3 serde_json::Value 过度使用（P2 #8）

**评估方法**: 检查 `exchange_binding/` 和 `freebusy/` 目录，以及全仓库 `serde_json::Value` 使用情况

**结果**:
- 目标目录 5 个高优先级 API:
  - `exchange_binding/get.rs`: 返回 `SDKResult<serde_json::Value>`
  - `exchange_binding/create.rs`: 入参和返回均为 `serde_json::Value`
  - `exchange_binding/delete.rs`: 返回 `SDKResult<serde_json::Value>`
  - `freebusy/batch.rs`: 入参和返回均为 `serde_json::Value`
  - `freebusy/list.rs`: 入参和返回均为 `serde_json::Value`
- 全仓库范围: 约 **191 个文件** 使用 `serde_json::Value`
- 按 crate 统计 (match/file):
  - `openlark-platform`: 225/89
  - `openlark-mail`: 145/64
  - `openlark-meeting`: 95/20
  - `openlark-security`: 62/20
  - `openlark-communication`: 64/20

**建议**: 
1. 优先处理 5 个目标 API（exchange_binding + freebusy）
2. 评估是否为第三方动态 schema（注释显示部分 API 字段较多/多变，有意采用动态结构）
3. 全仓库 191 个文件分阶段处理，创建子 issue 跟踪

### 2.4 文档注释格式不统一（P2 #9）

**评估方法**: 检查 `pub async fn execute` 文件前 80 行内的文档注释完整性

**结果**:
- 扫描文件数: **1676 个**
- 文档完整（docPath + doc 链接 + 方法描述）: **18 个**（仅占 1.1%）
- 不完整: **1658 个**
- 缺失项分布:
  - 只缺 docURL: **301**
  - 缺 docPath + docURL: **1163**
  - 三项都缺: **194**
- 按 crate 缺失数 TOP:
  - `openlark-hr`: 562
  - `openlark-workflow`: 186
  - `openlark-communication`: 176
  - `openlark-docs`: 145
  - `openlark-meeting`: 117

**建议**: 分阶段补充文档注释，优先处理缺失最严重的 crate

---

## 3. 执行计划建议

### 阶段 1: P1 遗留项（validate_required!）
- 工作量: 中等（44 文件 / 101 字段）
- 影响: 提升 API 健壮性和一致性
- 预计耗时: 1-2 天

### 阶段 2: P2 高价值项（mod.rs 导出 + 5 个 serde_json::Value API）
- mod.rs 导出: 工作量小（38 文件），可快速完成
- serde_json::Value: 工作量中等（5 文件），需设计强类型结构体
- 预计耗时: 1-2 天

### 阶段 3: P2 大规模项（文档注释）
- 工作量: 超大（1658 文件）
- 建议: 按 crate 分批处理，创建子 issue 跟踪
- 预计耗时: 1-2 周

---

## 4. 已完成的提交

| 提交 | 内容 |
|------|------|
| `05827ce55` | refactor: 删除 explorer/permission v2 的函数式 API，统一使用 Builder 模式 |
| `a516073e7` | docs: 更新 issue #41 状态为 P0 已完成 |
| `e8d75e8eb` | refactor: 统一 execute() 委托模式，消除 40 个文件的代码重复 |
| `3f4d6d794` | docs: 更新 issue #41 进度，标记 calendar 和 platform/helpdesk 已完成 |
