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

### D7: apaas 路径参数逐级下传（计划阶段发现）

apaas 叶子 builder 的 `new()` 除 `config` 外还要求路径参数（已核对签名）：
- `RecordCreateRequestBuilder::new(config, namespace, object_api_name)`
- `RoleMemberGetRequestBuilder::new(config, namespace, role_api_name)`

故 apaas service 链在**路径绑定层**的 accessor 带参，逐级持有并下传：

```
service.app_engine().apaas().v1()
  .application("ns")          // ApplicationService { config, namespace }
    .object("obj_api")        // ObjectService { config, namespace, object_api_name }
      .record()               // RecordService { config, namespace, object_api_name }
        .create()             // RecordCreateRequestBuilder::new(config, ns, obj_api)
```

与 SparkV1 的无参链不同，但更优——路径参数在链中语义位置绑定，比强制用户在叶子 builder 处补传更自然。admin/directory 叶子 builder 仅需 `config`，不受影响。中间级 service 在装访问器前可能短暂触发 dead_code（下一级未接），用临时 `#[allow(dead_code)]` 过渡，子级接好后删除。

## Risks / Trade-offs

- **[Scope: 30+ service 类型]** → 按 crate 子树分批（admin → directory → apaas），每批随附 access 测试 + clippy 门控
- **[深嵌套命名/路径错配]** → service 命名统一 `{Resource}Service`，与模块名一一对应
- **[dead_code 误报]** → 每个新 service 必须被上一级访问器消费；CI `clippy -W dead_code` 硬门控兜底
- **[facade stub 语义]** → `AuditApi`/`UsersApi` 是 runtime stub（返回未接线错误），装访问器不改变其 stub 行为，仅提供导航可达性

## Migration Plan

1. platform admin（6 操作集合 + 2 facade）→ 装访问器 + access 测试 + `_config`→`config`
2. platform directory（8 子模块）→ 同上
3. platform apaas（8 顶层 + 深嵌套）→ 逐级 service，最深链测试覆盖
4. 全量 `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo clippy -W dead_code`
5. 回滚：纯加法，回退 commit 即可，无数据/配置迁移

## Open Questions

- D4 宏抽象是否在 build 阶段触发？（默认不引入）
- apaas 深嵌套是否存在资源同名歧义？（build 阶段逐级核对，命名 `{Resource}Service` 规避）
