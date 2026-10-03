# Runtime API Stub Audit

本文件细化 issue `#107`：追踪仍然留在用户可见运行时代码中的 TODO API stubs。

这些接口与 `tests/` 里的占位测试不同：它们已经在公开 crate 中暴露给用户，但 `execute()` 仍只返回占位 JSON。

## 范围

本轮覆盖以下仍在追踪的 source-level TODO API stubs（原 18 个中多项已删除）：

| Crate | File | Stub methods | Count | 状态 |
| --- | --- | --- | ---: | --- |
| `openlark-analytics` | `search/search/v2/query.rs`、`user.rs` | `SearchRequest`/`SuggestRequest`/`SearchUserRequest::execute` | 3 | ✅ #350 删除：无已验证飞书端点，恒 `Err` + setter 死值是接口撒谎 |
| `openlark-user` | settings/preferences 链 | 7 个 `*Request::execute` | 7 | ✅ #311 删除 |
| `openlark-platform` | `admin/v1/settings.rs` | `Get/Update/ListSettingsRequest::execute` | 3 | 待 #110 |
| `openlark-platform` | `admin/v1/users.rs` | `List/Disable/Enable*Request::execute` | 3 | 待 #110 |
| `openlark-platform` | `admin/v1/audit.rs` | `Query/GetAuditLog*Request::execute` | 2 | 待 #110 |

## 风险分组

### 1. Analytics 搜索 stubs（✅ 已由 #350 解决）

**状态**：已移除。`query.rs` / `user.rs` 的 `QueryApi`/`UserSearchApi`/`SearchRequest`/`SuggestRequest`/`SearchUserRequest`
曾以 builder + `execute()` 形状暴露，但无已验证的飞书 runtime 端点（#108/#2fab71234 约束：不发明未验证端点），
`execute()` 恒 `Err`，setter 存值从不读取——#350 P9「接口形状撒谎」。与 #308 删除 `Search`/`SearchV2` 门面死链一致：
未接线 surface 直接删，不保留恒失败脚手架。调用方继续用已实现的 `doc_wiki`/`schema`/`app`/`message`/`data_source`。

跟踪 issue：

- `#108` Implement analytics search runtime TODO stubs（由 #350 关闭方向：stub 删除，不再待实现）
- `#350` P9 接口形状撒谎（analytics 子项）

### 2. User 设置 / 偏好 stubs（✅ 已由 #311 解决）

**状态**：已移除。`openlark-user` 的 settings/preferences stub 链（7 个 `*Request::execute`）连同
`SettingsService` / `PreferencesService` 门面与 `settings` / `preferences` / `v1` 等 feature 一并删除
（v0.18 breaking）。门面 `UserService` 改补 `system_status()` accessor，指向真实 system_status
资源（7 个 live 请求构建器）。`UserService::new` 误导 `SDKResult` 签名由 #360/#350 改为 `Self`。

跟踪 issue：

- `#109` Implement user settings and preferences runtime TODO stubs（由 #311 关闭：stub 删除，不再待实现）
- `#350` P9 user 子项（`UserService::new` → #360）

### 3. Platform admin stubs

影响：

- 平台后台管理接口（settings / users / audit）名称清晰、语义明确
- 当前占位实现会造成“存在接口、但并未真正接线”的错觉
- 注：#350 platform 子项已修 `PlatformService::new` 误导 `SDKResult` 签名（#373）；admin 恒 `Err` stub 仍归 #110

跟踪 issue：

- `#110` Implement platform admin runtime TODO stubs

## 当前结论

- analytics / user runtime TODO stubs 已删除；剩余主要为 platform admin 8 处（#110）
- 未接线 surface 优先删除或改签名，不保留恒失败/`success: true` 的撒谎脚手架

## 与 TODO 总审计的关系

- 总量审计见：`docs/TODO_AUDIT_SUMMARY.md`
- 本文件只聚焦 `source_api_stubs` 这一个 p1 桶
