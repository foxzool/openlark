# Client 配置构建深化设计

**状态**：已确认

**日期**：2026-07-10

**范围**：`openlark-core` 配置构建与 `openlark-client` Client 构造

## 背景

`openlark-client` 当前通过私有 `ClientBuildConfig` 镜像 core `Config` 的字段、默认值、环境变量解析和校验，再把结果逐字段转换成 core `Config`。该浅层 module 已发生实际漂移：`ClientBuilder` 暴露 `allow_custom_base_url`，但转换时没有把它写入 core `Config`。

与此同时，`Client::with_core_config` 使用另一份较弱的重复校验，未执行 core 的域名白名单规则。这与 issue #150 已确立的安全契约冲突：非 Feishu/Lark 域名必须通过 `allow_custom_base_url(true)` 显式放行。

本设计将配置状态、环境解释和通用校验收拢到 core 配置 module，同时保留 Client 构造的严格策略和现有 caller interface。

## 目标

- 删除 `ClientBuildConfig` 及其字段镜像、环境解析、转换和重复校验。
- 让 core `ConfigBuilder` 成为配置状态的唯一 implementation。
- 保持 `ClientBuilder` 的公开 interface、30 秒默认超时和严格构建语义。
- 让 `ClientBuilder::build` 与 `Client::with_core_config` 穿过同一个私有构造 seam。
- 让 `Config::validate` 成为凭据、URL、域名白名单和重试次数校验的唯一 implementation。
- 修复 `allow_custom_base_url` 丢失和 `with_core_config` 绕过白名单的问题。
- 让测试只穿过真实 interface，并在删除浅层 module 后继续保护行为。

## 非目标

- 不改变 core `ConfigBuilder::build()` 的宽松语义。
- 不改变 core 默认的 `req_timeout = None`。
- 不新增公开 policy、profile、port 或 adapter。
- 不改变 Client registry、token provider 或业务 crate 的构造方式。
- 不承诺旧验证错误文案逐字兼容。

## 设计决策

### 1. Core ConfigBuilder 持有 canonical 状态

`ConfigBuilder` 直接持有一份私有 `ConfigInner`，不再维护平行的 `Option<T>` 字段集合：

```rust
pub struct ConfigBuilder {
    inner: ConfigInner,
}
```

所有 setters 直接修改 `inner`，`build()` 只把该状态移入 `Config`。默认值、字段保存、环境覆盖和 header 合并因此具有单一 locality。

公开 interface 保持现有 setters，并增加两个纯加法入口：

```rust
impl ConfigBuilder {
    pub fn load_from_env(self) -> Self;

    pub fn add_header(
        self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self;
}
```

`Config::from_env()`、`Config::load_from_env()` 和 `ConfigBuilder::load_from_env()` 共用同一个私有环境覆盖 implementation。不得为 builder 再复制一份环境变量匹配表。

`ConfigBuilder::build()` 继续返回 `Config`，不自动调用 `validate()`。

### 2. ClientBuilder 只保留 Client 策略

`ClientBuilder` 改为直接持有 core `ConfigBuilder`：

```rust
pub struct ClientBuilder {
    config: openlark_core::config::ConfigBuilder,
}
```

现有公开 setters 全部保留并委托 core builder。`ClientBuilder::new()` 只增加一项 Client 策略：

```rust
Config::builder().req_timeout(Duration::from_secs(30))
```

这不是第二套默认值模型。core 默认仍为无超时，30 秒只属于 Client 构造 interface。

`ClientBuilder` 继续实现 `Clone` 和脱敏 `Debug`。`Debug` 不得输出 `app_secret`、token provider 或 header 内容，只输出安全摘要。

### 3. 两个公开入口共用一个私有构造 seam

`ClientBuilder::build()` 与 `Client::with_core_config()` 均调用 crate-private `with_checked_core_config`：

```text
ClientBuilder::build
                     ┐
                     ├─> with_checked_core_config
Client::with_core_config
```

`with_checked_core_config(config, operation)` 依次执行：

1. `Config::validate()`；
2. Client 特有的零超时校验；
3. 为错误附加调用入口 operation context；
4. 初始化 registry；
5. 注入 token provider；
6. 组装 `Client`。

删除当前 `with_validated_core_config` 的“调用方必须先验证”隐含顺序约束。构造 seam 自己拥有并执行全部前置条件。

## 状态流与覆盖顺序

### 默认值

- core `ConfigBuilder` 默认 `req_timeout = None`。
- `ClientBuilder::new()` 默认 `req_timeout = Some(30s)`。
- `Client::with_core_config()` 接受 `None`，拒绝 `Some(Duration::ZERO)`。

### 环境变量覆盖

环境覆盖发生在链式调用出现的位置，后写覆盖前写：

```rust
Client::builder()
    .timeout(Duration::from_secs(10))
    .from_env(); // 有效 OPENLARK_TIMEOUT 覆盖 10 秒

Client::builder()
    .from_env()
    .timeout(Duration::from_secs(10)); // 显式 setter 覆盖环境值
```

兼容规则：

- 缺失或空环境值不修改当前状态；
- 非法数字和未知 `AppType` 静默忽略；
- 最终配置是否合法，由 Client 构造 seam 判断；
- 环境变量测试必须串行隔离进程状态。

### Header 覆盖

- `add_header(k, v)` 增量写入，同名键后写覆盖；
- `header(map)` 整体替换；
- 两者混用时，调用顺序决定最终结果。

## 校验与错误契约

`Config::validate()` 负责：

- `app_id` / `app_secret` 非空；
- `base_url` scheme 合法；
- Feishu/Lark 域名白名单；
- `retry_count <= 10`。

Client 构造 seam 仅追加一条规则：`req_timeout == Some(Duration::ZERO)` 时返回 validation 错误，`field = "timeout"`。

非白名单域名默认拒绝。只有显式 `allow_custom_base_url(true)` 才能放行。该规则同时适用于 `ClientBuilder::build()` 和 `Client::with_core_config()`。

错误契约：

- 统一使用 `CoreError`；
- 保证 validation 类别和准确 `field`；
- 文案采用 core 规范格式，不保证旧字符串逐字兼容；
- operation context 分别保留 `ClientBuilder::build` 与 `Client::with_core_config`；
- registry 初始化错误继续包含调用入口和 `service_loading` context。

## 测试设计

### Core 配置 interface

- core 默认超时保持 `None`；
- `load_from_env()` 与 setters 的前后覆盖顺序；
- 非法环境值不覆盖已有状态；
- `add_header()` 的增量、同名覆盖及与 `header(map)` 的顺序；
- `allow_custom_base_url` 在构建后保持原值；
- builder 的 `Clone` 与脱敏 `Debug` 行为。

### Client 构造 interface

- `ClientBuilder` 默认超时为 30 秒；
- 自定义域名未显式允许时失败，允许后成功；
- `Client::with_core_config()` 使用同一白名单规则；
- core 配置的 `None` 超时可通过，零超时失败；
- validation 类别、`field` 和 operation context 正确；
- setters 与 `from_env()` 的前后覆盖顺序；
- header 增量与覆盖结果通过最终 `Client::config()` 观察。

删除直接读取 builder 私有字段或直接测试 `ClientBuildConfig` 的测试。测试不得越过公开 interface 验证 implementation 细节。

## 删除计划

- 删除 `crates/openlark-client/src/client_build_config.rs`；
- 删除 `openlark-client` 中对应 `mod` 声明和 import；
- 删除 `validate_core_config()`、`validate_base_url()` 和逐字段 `build_core_config()`；
- 删除浅层 module 的字段级测试；
- 将仍有价值的行为断言迁移到 core builder 或 Client 构造 interface。

Deletion test：删除 `ClientBuildConfig` 后，配置复杂度不会回流到 caller，而是由既有 core 配置 module 承担并集中。若删除 core `ConfigBuilder`，默认值、环境覆盖、header 合并和字段状态才会重新散落，因此 core module 提供真实 depth 和 leverage。

## 兼容性与发布说明

- `ClientBuilder` 公开 interface 不变，现有 caller 无需迁移；
- core 只增加 `load_from_env()` 和 `add_header()`，属于纯加法；
- `ConfigBuilder::build()` 与 core 默认值不变；
- 唯一行为收紧是修复非白名单域名的错误放行；
- 合法自定义域名继续通过 `allow_custom_base_url(true)` 使用；
- CHANGELOG 记录安全修复、`allow_custom_base_url` 传播修复和错误文案规范化。

## 接受的取舍

- `ConfigBuilder` 会比当前 implementation 更早创建默认 `reqwest::Client`。正常 caller 最终都会构建 `Config`，该成本换取单一 canonical 状态和更高 locality。
- `ClientBuilder` 仍保留一组委托 setters。这是稳定 caller interface，不是第二套状态 implementation。
- 环境变量解析继续容错静默，避免引入额外兼容变化。
- 不引入 `ConfigPolicy`、`overlay()` 或 `build_with()`；当前没有第二个真实策略 caller，这些 seam 只会增加 interface 复杂度。

## 被拒绝的方案

### 保留 ClientBuildConfig，仅修复漏字段

只能修复当前症状，未来新增字段仍可能再次漂移；删除测试不成立。

### 公开 ConfigPolicy 与组合式 overlay

灵活性高，但当前只有一个真实 Client 策略。新增 policy、组合类型和构建入口会形成 hypothetical seam，降低 depth。

### 让 core ConfigBuilder 自动严格校验

会破坏 core 独立 caller 的现有宽松构建契约，超出本次范围。

## 完成标准

- `ClientBuildConfig` 和重复校验 implementation 完全删除；
- `allow_custom_base_url` 在 ClientBuilder 路径中正确传播；
- 两个 Client 构造入口执行同一白名单规则和零超时规则；
- core 与 Client 默认超时分别保持 `None` 和 30 秒；
- 环境与 header 覆盖顺序由 interface 测试锁定；
- 错误类别、`field` 与 operation context 由 interface 测试锁定；
- 相关 core/client 测试、fmt、双模式 clippy 与 workspace 检查通过；
- CHANGELOG 准确记录安全与行为修复。
