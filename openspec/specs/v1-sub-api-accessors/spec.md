# v1-sub-api-accessors Specification

## Purpose
TBD - created by archiving change add-platform-v1-accessors. Update Purpose after archive.
## Requirements
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

