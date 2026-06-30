---
comet_change: add-platform-v1-accessors
role: technical-design
canonical_spec: openspec
---

# Design: platform v1 入口链式子 API 访问器

## 问题

openlark-platform 的 `AdminV1`/`ApaasV1`/`DirectoryV1` 入口 struct 持有 config 但**没有任何子 API 访问器**。用户无法走 `service.admin().v1().badge()` 式链式导航，只能用冗长模块路径 `admin::admin::v1::badge::create::CreateBadgeRequestBuilder::new(config)`。这与已具备完整链式访问的 `SparkV1`（`service.spark().v1().directory().user().id_convert()`）体验割裂。同时 cleanup-dead-code-allows（#267）把这 3 个入口的 config 字段临时改成 `_config`，等本 change 装访问器后恢复。

解 issue #274（P1）。

## 参考蓝本：SparkV1 三级链

```
service.spark().v1()        → SparkV1                { config: Arc<PlatformConfig> }
  .directory()              → SparkDirectoryService  { config: Arc<PlatformConfig> }   // Arc::clone
    .user()                 → DirectoryUserService   { config: Config (owned) }        // Arc deref + clone
      .id_convert()         → DirectoryUserIdConvertRequestBuilder                     // Config::clone 喂 builder
```

本 change 在 3 个 platform v1 入口下复制此范式。叶子 builder（`CreateBadgeRequestBuilder::new(config: Config)` 等）已存在，**签名不动**，仅在上方铺设 service 层。

## 可达性（已验证）

`AdminService.v1()` / `AppEngineService.v1()` / `DirectoryService.v1()`（`crates/openlark-platform/src/{admin.rs, app_engine.rs, directory/mod.rs}`）均返回对应 V1 入口。三入口在 default/full feature 下可达，装访问器不会制造 dead_code。

## 设计决策

### D1：每级子模块一个 service 入口类型，full-depth 链

每一级子模块建 `XxxService`（`BadgeService`/`ApplicationService`/`DepartmentService`…），持 config，暴露下一级访问器或叶子 builder 构造方法。链式导航一路到达叶子 builder，与 SparkV1 一致。

被否备选：仅顶层 service（达不到 full-depth 体验）/ service 仅作 config 容器（叶子仍走模块路径）。

### D2：config 流转——Arc 在上、owned Config 在叶

- 入口与中间级 service 持 `Arc<PlatformConfig>`；访问器返回新 service 时 `Arc::clone`（廉价）
- 叶子 service（其子是 .rs 操作、不再分层）持 owned `Config`，由父级 `arc.as_ref().clone()` 得到
- 叶子 service 访问器调用已存在 builder 的 `new(self.config.clone())`

与 SparkV1 流转一致；builder 签名 `new(config: Config)` 决定叶级必须 owned。

### D3：访问器返回值类型 service

`pub fn badge(&self) -> BadgeService`，返回值类型（非引用）。service 为 `#[derive(Debug, Clone)]`。与 SparkV1 一致，避免生命周期标注，链式调用顺畅。

### D4：手写，不引入宏

全部手写，照 SparkV1 模板复制。30+ service 虽重复，但每叶子 builder 名字/参数不同，宏抽象收益有限；SparkV1 本身手写，保持一致；遵守 "copy-paste twice before abstract"。build 阶段若证明重复不可承受再评估宏。

### D5：facade 模块复用已有类型

admin 的 `audit.rs`/`users.rs` 是 facade 单文件，**已有** service 入口类型 `AuditApi`/`UsersApi`（持 `Arc<PlatformConfig>`，当前是 runtime stub）。这两个只装 `pub fn audit()/users()` 访问器返回它们，不新建类型、不套第二层 service。

### D6：测试范式

每入口补 access 测试，仿 `test_spark_v1_directory_access`：构造 config → 链式调用到最末级 → `let _ = ...` 证可达。apaas 深嵌套至少一条链走到叶子。

## 范围

| 入口 | 子模块 | 深度 |
|------|--------|------|
| AdminV1 | 6 操作集合（badge/badge_image/password/admin_dept_stat/admin_user_stat/audit_info）+ 2 facade（audit/users） | 浅 |
| ApaasV1 | 8 顶层 + 深嵌套（application→object→record、application→role→member、workspace→table/view 等） | 3-4 层 |
| DirectoryV1 | 8 子模块（department/departments/users/employee/sync/collaboration_*） | 浅 |

新增约 30+ service 入口类型。3 处私有 `_config` → `config` 重命名。

## 风险与缓解

- **30+ 类型 diff 体积** → 按 admin→directory→apaas 分批，每批随附 access 测试 + clippy 门控
- **apaas 深嵌套命名歧义** → service 命名统一 `{Resource}Service`，与模块名一一对应
- **dead_code 误报** → 每个新 service 必须被上一级访问器消费；CI `clippy -W dead_code` 硬门控兜底
- **facade stub 语义** → `AuditApi`/`UsersApi` 是 runtime stub（返回未接线错误），装访问器不改变 stub 行为，仅提供导航可达性

## 非目标

- 不改叶子 builder 实现、endpoint URL、请求/序列化逻辑
- 不引入新依赖
- 不重构为宏（D4）
- 不处理 #277（dead_code inner-attribute，独立 change）
- 不做 ai v1 入口（#275）——见下

## #275 拆分说明

原计划打包 #274+#275。design 探索发现 ai crate 存在多套并行导航链（`src/ai/` 树、`src/service.rs` AiClient 链、`src/document_ai/`+`src/speech_to_text/` 顶层域树、`src/common/chain.rs`），文件自带 `#![allow(clippy::module_inception)]`。#275 点名的 4 个 struct（`src/ai/document_ai/v1/DocumentAiV1` 等）**零外部引用**，是半完成迁移的孤儿类型（`DocumentAiV1` 有两个定义，聚合 `V1` 用同文件内 line 43 那个，非 #275 点名带 18 子模块那个）。给孤儿类型装访问器无用户价值且与反 dead_code 目标矛盾。#275 正解是先 untangle ai crate 架构，另起 change。

## 验收

- `service.admin().v1().badge().create().name("x").execute()` 等链式调用可用
- apaas 深链 `service.app_engine().apaas().v1().application().object().record()…` 可用
- 3 个入口无 `_config` 遗留
- `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo clippy -W dead_code`（openlark-platform）全过
