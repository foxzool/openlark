# Runtime API Stub Audit

本文件细化 issue `#107`：追踪仍然留在用户可见运行时代码中的 TODO API stubs。

这些接口与 `tests/` 里的占位测试不同：它们已经在公开 crate 中暴露给用户，但 `execute()` 仍只返回占位 JSON。

## 范围

本轮覆盖以下 11 个仍在追踪的 source-level TODO API stubs（原 18 个中 `openlark-user` 的 7 个 settings/preferences stub 已由 #311 删除：stub 链移除 + 门面补 `personal_settings()` accessor 指向真实 system_status）：

| Crate | File | Stub methods | Count |
| --- | --- | --- | ---: |
| `openlark-analytics` | `crates/openlark-analytics/src/search/search/v2/query.rs` | `SearchRequest::execute`, `SuggestRequest::execute` | 2 |
| `openlark-analytics` | `crates/openlark-analytics/src/search/search/v2/user.rs` | `SearchUserRequest::execute` | 1 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/settings.rs` | `GetSettingRequest::execute`, `UpdateSettingRequest::execute`, `ListSettingsRequest::execute` | 3 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/users.rs` | `ListAdminUsersRequest::execute`, `DisableUserRequest::execute`, `EnableUserRequest::execute` | 3 |
| `openlark-platform` | `crates/openlark-platform/src/admin/admin/v1/audit.rs` | `QueryAuditLogsRequest::execute`, `GetAuditLogRequest::execute` | 2 |

## 风险分组

### 1. Analytics 搜索 stubs

影响：

- 看起来像可用的搜索 API，但默认只返回占位结构
- 容易让调用方误以为已经接入了真实搜索服务

跟踪 issue：

- `#108` Implement analytics search runtime TODO stubs

### 2. User 设置 / 偏好 stubs（✅ 已由 #311 解决）

**状态**：已移除。`openlark-user` 的 settings/preferences stub 链（7 个 `*Request::execute`）连同
`SettingsService` / `PreferencesService` 门面与 `settings` / `preferences` / `v1` 等 feature 一并删除
（v0.18 breaking）。门面 `UserService` 改补 `personal_settings()` accessor，指向真实 system_status
资源（7 个 live 请求构建器）。原影响（占位返回误导集成方）随之消除。

跟踪 issue：

- `#109` Implement user settings and preferences runtime TODO stubs（由 #311 关闭：stub 删除，不再待实现）

### 3. Platform admin stubs

影响：

- 平台后台管理接口（settings / users / audit）名称清晰、语义明确
- 当前占位实现会造成“存在接口、但并未真正接线”的错觉

跟踪 issue：

- `#110` Implement platform admin runtime TODO stubs

## 当前结论

- 这些 runtime TODO stubs **不应继续处于无主状态**
- 本轮不强行在一个 issue 里实现全部 18 处
- 已按业务域拆成 3 个 follow-up issues，便于分批收敛

## 与 TODO 总审计的关系

- 总量审计见：`docs/TODO_AUDIT_SUMMARY.md`
- 本文件只聚焦 `source_api_stubs` 这一个 p1 桶
