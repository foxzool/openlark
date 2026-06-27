# platform-service-access Specification

## Purpose
TBD - created by archiving change fix-platform-v1-feature-gating. Update Purpose after archive.
## Requirements
### Requirement: Platform services expose their API surface under default features
openlark-platform 的 `AdminService`、`AppEngineService`、`DirectoryService`、`SparkService` SHALL 在 `default` feature 组合下编译并暴露其 API 实现入口（不再是无 API 的空壳 facade）。

#### Scenario: AdminService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`（`cargo build -p openlark-platform`）
- **THEN** `AdminService` 暴露其 v1 API 入口并成功编译，`admin/admin/**` 子树参与编译

#### Scenario: AppEngineService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`
- **THEN** `AppEngineService` 暴露其 API 入口，`app_engine/apaas/**` 子树参与编译

#### Scenario: DirectoryService 与 SparkService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`
- **THEN** `DirectoryService` 与 `SparkService` 各自暴露 API 入口，`directory/directory/**` 与 `spark/spark/**` 子树参与编译

### Requirement: Platform services expose their API surface under full features
四个 platform service SHALL 在 `full`（`--all-features`）feature 组合下同样暴露其完整 API 实现。

#### Scenario: full 构建覆盖全部 platform API
- **WHEN** 以 `--all-features` 构建 `openlark-platform`
- **THEN** 四个 service 的全部 96 个 API 实现均参与编译，无空壳 facade

### Requirement: Feature-gating 测试门控不回归
本变更 SHALL 不破坏现有基于 feature flag 的 clippy 测试门控。

#### Scenario: no-default-features clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- **THEN** 命令以 exit 0 结束

#### Scenario: all-features clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **THEN** 命令以 exit 0 结束

### Requirement: 公开 API 符号不被移除
本变更 SHALL 不移除任何现有公开 API 符号；仅让原本不可达的 API 变为可达，视为行为补全而非破坏性变更。

#### Scenario: 现有公开类型保持可用
- **WHEN** 变更后以 `default` 或 `full` feature 编译依赖 openlark-platform 的代码
- **THEN** 变更前可访问的公开类型与方法仍可访问（仅新增可达性，无删除）

