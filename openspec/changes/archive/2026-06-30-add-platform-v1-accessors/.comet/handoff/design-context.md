# Comet Design Handoff

- Change: add-platform-v1-accessors
- Phase: design
- Mode: compact
- Context hash: 2afba3981e3faccd1920bb85475922f054f158dc18da0e67b1d546deda8e351d

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/add-platform-v1-accessors/proposal.md

- Source: openspec/changes/add-platform-v1-accessors/proposal.md
- Lines: 1-40
- SHA256: f4850d886fbddc68867fefb0b049c26e6cd4edec02405779464c305c62408aac

```md
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
```

## openspec/changes/add-platform-v1-accessors/design.md

- Source: openspec/changes/add-platform-v1-accessors/design.md
- Lines: 1-89
- SHA256: 99b0c6d59b7b5f99f6af174261d6bac4a7ec7b41afe4f2d98ba60c62b836df2a

[TRUNCATED]

```md
## Context

SparkV1 已实现完整的三级链式访问范式，是本 change 的参考蓝本：

```
service.spark().v1()        → SparkV1                { config: Arc<PlatformConfig> }
  .directory()              → SparkDirectoryService  { config: Arc<PlatformConfig> }   // Arc::clone
    .user()                 → DirectoryUserService   { config: Config (owned) }        // Arc deref + clone
      .id_convert()         → DirectoryUserIdConvertRequestBuilder                     // Config::clone 喂 builder
```

当前 3 个 platform v1 入口（AdminV1/ApaasV1/DirectoryV1）只持有 config（临时 `_config`），子模块是纯操作集合（`pub mod create/get/...`），无 service 入口类型，故无法链式导航。叶子 builder（如 `CreateBadgeRequestBuilder::new(config: Config)`）已存在且可用，本 change 仅在其上方铺设 service 层。

**可达性已验证**：`AdminService.v1()`/`AppEngineService.v1()`/`DirectoryService.v1()` 均返回对应 V1 入口（`crates/openlark-platform/src/{admin,app_engine,directory/mod.rs}`），三入口在 default/full feature 下可达，装访问器不会制造 dead_code。

约束：
- platform 链全程使用 `Arc<PlatformConfig>`
- 叶子 builder 签名固定（`new(config: Config)`），不得修改
- apaas 存在 3-4 层深嵌套（`application → object → record`、`application → role → member`）
- CI 红线：`cargo fmt --check` + `cargo clippy -W dead_code`

**facade 发现**：admin 的 `audit.rs`/`users.rs` 是 facade 单文件，**已有** service 入口类型 `AuditApi`/`UsersApi`（持 `Arc<PlatformConfig>`，当前是 runtime stub）。这两个无需新建类型，只装访问器。

## Goals / Non-Goals

**Goals:**
- 3 个 platform v1 入口支持 full-depth 链式导航，对齐 SparkV1 体验
- 每一级子模块拥有 service 入口类型，config 逐级下传到叶子 builder
- 3 处 `_config` 恢复为 `config` 并被消费
- 纯加法、非破坏

**Non-Goals:**
- 不改叶子 builder 实现、endpoint URL、请求/序列化逻辑
- 不引入新依赖
- 不重构为宏（见 Decisions D4）
- 不处理 #277（dead_code inner-attribute，独立 change）
- **不做 ai v1 入口（#275）**——见下方"#275 拆分理由"

## #275 拆分理由（design 发现）

原计划打包 #274+#275，design 探索发现 ai crate 存在**多套并行导航链**（`src/ai/` 树、`src/service.rs` AiClient 链、`src/document_ai/`+`src/speech_to_text/` 顶层域树、`src/common/chain.rs`），文件自带 `#![allow(clippy::module_inception)]`。#275 点名的 4 个 struct（`src/ai/document_ai/v1/DocumentAiV1`、`.../image/Image`、`.../speech/Speech`、`.../text/Text`）**零外部引用**——是半完成迁移的孤儿类型（例如 `DocumentAiV1` 有两个定义，聚合 `V1` 用的是同文件内 line 43 那个，不是 #275 点名带 18 子模块那个）。

给孤儿类型装访问器不产生用户价值且与反 dead_code 目标矛盾。#275 正解是先 untangle ai crate 架构，属独立重构，故拆出另起 change。

## Decisions

### D1: 每级子模块一个 service 入口类型，full-depth 链

照搬 SparkV1——每一级子模块建一个 `XxxService`，持 config，暴露下一级访问器或叶子 builder 构造方法。备选（仅顶层 service / service 仅作 config 容器）被否，因达不到 full-depth 链式体验。

### D2: config 流转——Arc 在上、owned Config 在叶

入口与中间级 service 持 `Arc<PlatformConfig>`，访问器返回新 service 时 `Arc::clone`；叶子 service（其子是 .rs 操作）持 owned `Config`，由父级 `arc.as_ref().clone()` 得到；叶子 service 访问器调用已存在 builder 的 `new(self.config.clone())`。与 SparkV1 流转完全一致；builder 签名 `new(config: Config)` 决定叶级必须 owned。

### D3: 访问器返回值类型

访问器返回**值类型** service（`pub fn badge(&self) -> BadgeService`），service 为 `#[derive(Debug, Clone)]`。与 SparkV1 一致；值返回避免生命周期标注，链式调用顺畅。

### D4: 手写 service，不引入宏

本 change 全部手写，照 SparkV1 模板复制。30+ service 虽重复，但每叶子 builder 名字/参数不同，宏抽象收益有限；SparkV1 本身手写，保持一致；遵守 "copy-paste twice before abstract"。若 build 阶段证明重复不可承受再评估宏（Open Question）。

### D5: facade 模块复用已有类型

admin 的 `audit.rs`/`users.rs` 已有 `AuditApi`/`UsersApi` service 类型——只装 `pub fn audit()/users()` 访问器返回它们，不新建类型、不套第二层 service。

### D6: 测试范式

每个入口补 access 测试，仿 `test_spark_v1_directory_access`——构造 config、链式调用到最末级、`let _ = ...` 证明可达。apaas 深嵌套至少一条链走到叶子。

## Risks / Trade-offs

- **[Scope: 30+ service 类型]** → 按 crate 子树分批（admin → directory → apaas），每批随附 access 测试 + clippy 门控
- **[深嵌套命名/路径错配]** → service 命名统一 `{Resource}Service`，与模块名一一对应
- **[dead_code 误报]** → 每个新 service 必须被上一级访问器消费；CI `clippy -W dead_code` 硬门控兜底
- **[facade stub 语义]** → `AuditApi`/`UsersApi` 是 runtime stub（返回未接线错误），装访问器不改变其 stub 行为，仅提供导航可达性

## Migration Plan

1. platform admin（6 操作集合 + 2 facade）→ 装访问器 + access 测试 + `_config`→`config`
```

Full source: openspec/changes/add-platform-v1-accessors/design.md

## openspec/changes/add-platform-v1-accessors/tasks.md

- Source: openspec/changes/add-platform-v1-accessors/tasks.md
- Lines: 1-33
- SHA256: 02eea77ba765bb1f18ac2ee4d2453fe86700ff1f08659e0a9851b22f90be5106

```md
# Tasks

## 1. platform admin v1（AdminV1，6 操作集合 + 2 facade，浅）

- [ ] 1.1 为 admin v1 操作集合子模块建 service 入口类型：`badge`/`badge_image`/`password`/`admin_dept_stat`/`admin_user_stat`/`audit_info` 各建 `XxxService { config }` + 暴露叶子 builder 构造方法（如 `badge().create()`）
- [ ] 1.2 facade `audit.rs`/`users.rs` 复用已有 `AuditApi`/`UsersApi`，不新建类型
- [ ] 1.3 `AdminV1`：`_config` → `config`，装 `pub fn badge()/badge_image()/password()/admin_dept_stat()/admin_user_stat()/audit_info()/audit()/users()` 访问器
- [ ] 1.4 补 access 测试（仿 `test_spark_v1_directory_access`），链式到叶子 builder + facade 访问器
- [ ] 1.5 admin 范围 `cargo clippy -W dead_code` + `cargo fmt` 通过

## 2. platform directory v1（DirectoryV1，8 子模块，浅）

- [ ] 2.1 为 directory v1 子模块建 service：`department`/`departments`/`users`/`employee`/`sync`/`collaboration_share_entity`/`collaboration_tenant`/`collaboration_rule`
- [ ] 2.2 `DirectoryV1`：`_config` → `config`，装访问器
- [ ] 2.3 access 测试链到叶子 builder
- [ ] 2.4 directory 范围 clippy + fmt 通过

## 3. platform apaas v1（ApaasV1，8 顶层 + 深嵌套）

- [ ] 3.1 顶层 8 service：`app`/`approval_task`/`approval_instance`/`application`/`user_task`/`seat_activity`/`seat_assignment`/`workspace`
- [ ] 3.2 `application` 深嵌套逐级 service：`object`→`record`、`role`→`member`、`record_permission`→`member`、`environment_variable`/`function`/`flow`/`audit_log`
- [ ] 3.3 `workspace` 嵌套 service：`table`/`view`/`enum_mod`
- [ ] 3.4 `ApaasV1`：`_config` → `config`，装顶层访问器
- [ ] 3.5 深链 access 测试：`application().object().record()` 等走到叶子 builder
- [ ] 3.6 apaas 范围 clippy + fmt 通过（含深嵌套无 dead_code）

## 4. 全局验证 + 闭环

- [ ] 4.1 `cargo fmt --check`（workspace）
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 4.3 `cargo clippy -W dead_code` 于 openlark-platform，无新增告警
- [ ] 4.4 grep 确认 3 个 platform 入口无 `_config` 遗留
- [ ] 4.5 闭环 cleanup-dead-code-allows：移除 3 个 platform 入口的"待装访问器"reserved 注释
```

## openspec/changes/add-platform-v1-accessors/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/add-platform-v1-accessors/specs/v1-sub-api-accessors/spec.md
- Lines: 1-57
- SHA256: 3ec917af3f113382ff94b75b0971deb9d11858528c8073edc3bcffd335c67395

```md
## ADDED Requirements

### Requirement: platform v1 入口暴露链式子 API 访问器

openlark-platform 的 `AdminV1`、`ApaasV1`、`DirectoryV1` SHALL 通过 `pub fn` 访问器暴露其每一级子 API，链式导航一路到达叶子请求 builder，范式对齐 `SparkV1`。每一级子模块 SHALL 拥有一个 service 入口类型（如 `BadgeService`、`ApplicationService`、`DepartmentService`），持有 config 并暴露下一级访问器或叶子 builder 构造方法。

#### Scenario: AdminV1 链式访问叶子 builder

- **WHEN** 调用 `service.admin().v1().badge().create()` 设置 name 后 execute
- **THEN** 返回 `CreateBadgeRequestBuilder` 并可完成请求构建，链式导航可用

#### Scenario: AdminV1 facade 访问器复用已有类型

- **WHEN** 调用 `service.admin().v1().audit()` 或 `service.admin().v1().users()`
- **THEN** 返回已存在的 `audit::AuditApi` / `users::UsersApi`（facade 模块已有 service 入口类型，仅装访问器，不新建类型）

#### Scenario: ApaasV1 深嵌套链式访问

- **WHEN** 调用 `service.app_engine().apaas().v1().application().object().record()` 及更深层级
- **THEN** 每级 service 入口可达，链式导航覆盖 application→object→record、application→role→member 等 3-4 层深嵌套

#### Scenario: DirectoryV1 链式访问

- **WHEN** 调用 `service.directory().v1().department()` 等子模块访问器
- **THEN** 返回对应 service 入口，链式导航可用

### Requirement: 访问器 config 流转对齐 SparkV1 范式

各级 service SHALL 持有 config 并向下传递：入口与中间级 service 持 `Arc<PlatformConfig>`，叶子 service 持 owned `Config`（由 `arc.as_ref().clone()` 得到）并 clone 喂给已存在的请求 builder 的 `new(config: Config)` 构造器。不得修改叶子 builder 的现有签名。

#### Scenario: config 类型与流转

- **WHEN** 检查 platform service 链各级 config 字段类型
- **THEN** 入口与中间级使用 `Arc<PlatformConfig>`，叶子 service 解引用并 clone 为 owned `Config` 传入 builder，与 `SparkV1` 范式一致

### Requirement: 入口 config 字段恢复并被访问器消费

3 个 platform v1 入口 struct（AdminV1/ApaasV1/DirectoryV1）的临时 `_config` 字段 SHALL 恢复为 `config`，且 SHALL 被新增访问器消费（不再有下划线前缀或 dead_code 例外）。

#### Scenario: 无 _config 遗留

- **WHEN** 变更后检查 3 个 platform 入口 struct 的字段命名
- **THEN** 不存在 `_config` 前缀字段，config 被访问器读取使用

#### Scenario: 不新增 dead_code 告警

- **WHEN** 运行 `cargo clippy -W dead_code` 于 openlark-platform
- **THEN** 新增 service 入口类型不产生 dead_code 告警（均被访问器链消费）

### Requirement: 非破坏性补全

本变更 SHALL 为纯加法：现有模块路径调用（如 `admin::admin::v1::badge::create::CreateBadgeRequestBuilder::new(config)`）与叶子 builder 的公共签名 SHALL 保持可用；仅新增 service 类型与访问器方法，不移除任何现有公开符号。

#### Scenario: 现有模块路径调用保持可用

- **WHEN** 变更后以原有模块路径构造叶子 builder
- **THEN** 调用方式与签名不变，编译通过
```

