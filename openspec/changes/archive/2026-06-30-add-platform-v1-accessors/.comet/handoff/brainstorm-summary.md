# Brainstorm Summary

- Change: add-platform-v1-accessors
- Date: 2026-06-30

## 确认的技术方案

为 platform v1 三入口（AdminV1/ApaasV1/DirectoryV1）补 full-depth 链式子 API 访问器，对齐 SparkV1 三级范式：

```
service.admin().v1()   → AdminV1 { Arc<PlatformConfig> }
  .badge()             → BadgeService { Arc<PlatformConfig> }   // Arc::clone
    .create()          → CreateBadgeRequestBuilder              // owned Config 喂 builder
```

- D1 每级子模块一个 `XxxService`，full-depth 链到叶子 builder
- D2 config 流转：`Arc<PlatformConfig>` 在上 → 叶子 service `arc.as_ref().clone()` 成 owned `Config` → clone 进叶子 builder（builder 签名不动）
- D3 访问器返回值类型 service（`#[derive(Debug, Clone)]`）
- D4 手写 service，不引宏（Spark 本就手写；每 builder 参数不同，宏收益低；不过早抽象）
- D5 admin facade `audit.rs`/`users.rs` 已有 `AuditApi`/`UsersApi` → 只装访问器，不新建类型
- D6 每入口仿 `test_spark_v1_directory_access` 补 access 测试；apaas 深链走到叶子

可达性已验证：`AdminService/AppEngineService/DirectoryService.v1()` 返回三入口，default/full feature 下可达。

## 关键取舍与风险

- 30+ service 类型 → 按 admin→directory→apaas 分批，每批 access 测试 + clippy 门控
- apaas 3-4 层深嵌套（`application().object().record()`）→ 命名统一 `{Resource}Service` 规避歧义
- dead_code 误报 → 每个新 service 必须被上一级访问器消费；CI `clippy -W dead_code` 硬门控兜底
- facade 是 runtime stub → 装访问器只给导航可达性，不改 stub 行为
- **#275 拆分**：原打包 #274+#275，design 发现 ai crate 4 个目标 struct 零外部引用（多套并行导航链、`#![allow(module_inception)]`），是孤儿类型 → #275 另起 change 先 untangle ai crate

## 测试策略

- 每入口 access 测试：构造 config → 链式调用到最末级 → `let _ = ...` 证可达
- apaas 至少一条深链走到叶子 builder
- CI 红线：`cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo clippy -W dead_code`（openlark-platform 无新告警）

## Spec Patch

无。delta spec `v1-sub-api-accessors` 已含 platform 全部需求（ai 部分已移除，留给 #275 follow-up 扩展同 capability）。

## Open Questions（build 阶段处理）

- D4 宏抽象是否触发？默认不引入，除非重复证明不可承受
- apaas 深嵌套资源同名歧义？逐级核对，`{Resource}Service` 命名规避
