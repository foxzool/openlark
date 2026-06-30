## Why

platform v1 的 `AdminV1`/`ApaasV1`/`DirectoryV1` 入口 struct 虽持有 config，却**没有任何子 API 访问器**——用户无法走 `service.admin().v1().badge()` 式链式导航，只能用冗长的模块路径 `admin::admin::v1::badge::create::CreateBadgeRequestBuilder::new(config)`。这与已具备完整链式访问的 `SparkV1`（`service.spark().v1().directory().user().id_convert()`）体验割裂。

同时 cleanup-dead-code-allows（#267）为这 3 个入口的 config 字段做了临时 `_config` 下划线处理等待闭环；本 change 装回访问器后即可恢复为 `config` 并被真正消费。

解 issue #274（P1）。

**范围说明**：原计划打包 #274 + #275，但 design 阶段探索发现 #275 点名的 4 个 ai struct 是孤儿类型（ai crate 存在多套并行导航链、零外部引用），需先 untangle ai crate 架构——已拆分，#275 另起 change 处理。本 change 只做 platform。

## What Changes

- 为 platform v1 三入口（admin/apaas/directory）的**每一级子模块**新增 service 入口类型（`BadgeService`/`ApplicationService`/`DepartmentService` …），full-depth 链到叶子 builder，对齐 SparkV1 三级范式
- admin 的 facade `audit.rs`/`users.rs` 已有 `AuditApi`/`UsersApi` 类型——只装访问器，不新建类型
- 3 个入口 struct 的临时 `_config` 字段恢复为 `config`，由新增访问器消费（消除 dead_code 例外来源）
- 入口 struct 新增 `pub fn xxx()` 访问器方法，返回子 service；子 service 持 config 并暴露下一级访问器或叶子 builder 构造方法
- 各入口补 access 测试（仿 `test_spark_v1_directory_access`）
- **非破坏**：仅新增访问器与 service 类型，叶子 builder 实现、endpoint URL、请求逻辑均不变；`_config` 为私有字段（非公共合约），重命名无外部影响；现有模块路径调用保持可用

## Capabilities

### New Capabilities

- `v1-sub-api-accessors`: platform v1（admin/apaas/directory）入口 struct SHALL 通过链式访问器暴露其全部子 API，范式对齐 SparkV1，深度一路链到叶子请求 builder；各级 service 持 config 并下传。（ai v1 入口由后续 #275 follow-up change 同能力扩展）

### Modified Capabilities

<!-- 无。现有 platform-service-access 规约的是 feature-gating 可达性（default/full feature 下 service 不再是空壳 facade，编译期关注点），与本 change 的链式导航形状（API 人体工学）是不同层面，故不在此修改。 -->

## Impact

- **代码**：
  - `crates/openlark-platform/src/admin/admin/v1/**`（8 直接子模块：6 操作集合 + audit/users facade）
  - `crates/openlark-platform/src/app_engine/apaas/v1/**`（8 直接子模块 + 深嵌套：`application → object → record`、`application → role → member` 等 3-4 层）
  - `crates/openlark-platform/src/directory/directory/v1/**`（8 直接子模块）
  - 新增约 30+ service 入口类型（platform 范围）
- **公开 API**：纯加法——新增 service 类型 + 访问器方法；3 处私有 `_config` → `config` 重命名（非公共合约）
- **依赖/系统**：无新增依赖
- **CI 红线**：须过 `cargo fmt --check` + `cargo clippy -W dead_code`（不因新 service 新增 dead_code 告警）
- **关联**：闭环 cleanup-dead-code-allows（#267）platform 部分；解 issue #274（P1）。#275（ai）已拆分另起
