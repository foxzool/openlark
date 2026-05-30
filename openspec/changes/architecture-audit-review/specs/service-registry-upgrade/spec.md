## ADDED Requirements

### Requirement: ServiceRegistry 查询已启用服务
ServiceRegistry SHALL 提供 `has_service(name: &str) -> bool` 和 `list_services() -> Vec<&ServiceMetadata>` 方法，返回当前编译启用的服务列表及其元信息。

#### Scenario: 查询已启用的 auth 服务
- **WHEN** auth feature 已启用且 registry 已初始化
- **THEN** `registry.has_service("auth")` 返回 `true`

#### Scenario: 查询未启用的服务
- **WHEN** hr feature 未启用
- **THEN** `registry.has_service("hr")` 返回 `false`

### Requirement: Client 使用宏自动生成 feature-gated 初始化
Client 的业务服务字段声明和 `with_config()` 初始化代码 SHALL 由声明宏（macro_rules!）生成，而非手动编写 14+ 个 `#[cfg(feature)]` 块。

#### Scenario: 新增业务模块无需修改 Client 结构体
- **WHEN** 新增一个 `openlark-xxx` 业务 crate
- **THEN** 只需在宏注册表中添加一行声明，Client 自动获得 `pub xxx: XxxClient` 字段

#### Scenario: 未启用的 feature 不生成字段
- **WHEN** `docs` feature 未启用
- **THEN** Client 结构体中不包含 `docs` 字段，编译通过

### Requirement: Registry 元信息包含依赖关系
ServiceMetadata SHALL 包含 `dependencies` 字段，用于表达服务间的依赖关系（如 communication 依赖 auth）。

#### Scenario: communication 依赖 auth
- **WHEN** 查询 communication 服务的 dependencies
- **THEN** 返回包含 `"auth"` 的列表
