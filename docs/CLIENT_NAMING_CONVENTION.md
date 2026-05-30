# 业务 Crate Client 类型命名规范

## 规则

所有业务 crate 必须导出一个名为 `XxxClient` 的类型作为主入口：

```rust
// ✅ 推荐：每个 crate 导出 XxxClient
pub use service::WorkflowClient;  // 或 type alias
```

### 具体要求

1. **命名**: `XxxClient`（PascalCase，与 crate 名匹配）
2. **类型**: 可以是 `struct` 或 `pub type XxxClient = XxxService;`（type alias）
3. **构造**: 必须提供 `new(config: Config) -> Self` 构造函数
4. **禁止**: 不使用 `Arc<XxxServices>` 包装（Arc 应在内部处理）

### 当前映射

| Crate | 内部类型 | 导出名称 |
|-------|----------|----------|
| openlark-auth | `AuthService` + `AuthenService` + `OAuthService` | `AuthClient` (wrapper struct in openlark-client) |
| openlark-communication | `CommunicationClient` | `CommunicationClient` ✅ |
| openlark-docs | `DocsClient` | `DocsClient` ✅ |
| openlark-hr | `HrClient` | `HrClient` ✅ |
| openlark-meeting | `MeetingClient` | `MeetingClient` ✅ |
| openlark-cardkit | `CardkitClient` | `CardkitClient` ✅ |
| openlark-ai | `AiClient` | `AiClient` ✅ |
| openlark-workflow | `WorkflowService` | `WorkflowClient` (type alias) |
| openlark-platform | `PlatformService` | `PlatformClient` (type alias) |
| openlark-application | `ApplicationService` | `ApplicationClient` (type alias) |
| openlark-helpdesk | `HelpdeskService` | `HelpdeskClient` (type alias) |
| openlark-mail | `MailService` | `MailClient` (type alias) |
| openlark-analytics | `AnalyticsService` | `AnalyticsClient` (type alias) |
| openlark-user | `UserService` | `UserClient` (type alias) |
| openlark-security | `SecurityServices` | `SecurityClient` (struct wrapper) |
