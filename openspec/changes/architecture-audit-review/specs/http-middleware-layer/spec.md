## ADDED Requirements

### Requirement: Transport 支持可插拔的请求中间件管道
Transport SHALL 支持通过中间件层组合请求处理逻辑（重试、限流、日志、指标等），而非将所有横切关注点硬编码在 Transport 内部。

#### Scenario: 添加自定义日志中间件
- **WHEN** 用户通过 Config 配置自定义中间件
- **THEN** 每个 HTTP 请求经过中间件管道处理

#### Scenario: 默认无中间件时行为不变
- **WHEN** 未配置任何中间件
- **THEN** Transport 行为与当前完全一致

### Requirement: 内置重试中间件
SHALL 提供内置的重试中间件，支持指数退避策略，对可重试错误（5xx、网络超时、限流）自动重试。

#### Scenario: 5xx 错误自动重试
- **WHEN** API 返回 500 错误且配置了重试中间件
- **THEN** 中间件按指数退避策略自动重试，最多重试配置的次数

#### Scenario: 4xx 错误不重试
- **WHEN** API 返回 400 错误
- **THEN** 中间件直接返回错误，不进行重试

### Requirement: 内置限流感知中间件
SHALL 提供内置的限流中间件，当 API 返回 429 时自动等待 `Retry-After` 头指定的时间后重试。

#### Scenario: 限流后等待重试
- **WHEN** API 返回 429 且包含 `Retry-After: 5` 头
- **THEN** 中间件等待 5 秒后自动重试
