# Brainstorm Summary

- Change: remove-deprecated-tenant-token-legacy-chain
- Date: 2026-06-29

## 确认的技术方案

方案 A（整体移除 legacy 路径），用户已确认。

- **execute_with_options 简化**：移除 `if self.app_access_token.is_empty() { legacy 两步 } else { clone }`，改为 `validate_required!(self.app_access_token)` + `validate_required!(self.tenant_key)`，body 直接 move 字段（`app_access_token: self.app_access_token, tenant_key: self.tenant_key`，去 `.clone()`）。canonical POST 不变。
- **删除项**：3 deprecated 方法（84-103）+ 3 legacy 字段 + `new()` legacy 初始化 + `LegacyAppAccessTokenBody` + `AppAccessTokenResponseData` import + legacy 测试（285-344）。
- **测试调整**：`test_tenant_access_token_builder_new` 删 3 行 legacy 字段断言。canonical 正向测试不变。

## 关键取舍与风险

- **clone 移除安全性（已对抗验证）**：`validate_required!` 宏源码（openlark-core/src/lib.rs:53-59）= `if is_empty_trimmed(&$field)` → **借用非 move**，验证后 move 字段合法。实证：agent 应用改动后 `cargo build -p openlark-auth` 成功 + canonical 测试通过。
- **[Breaking]** 外部 legacy 链编译失败 → CHANGELOG 两步迁移指引。
- **[行为移除]** legacy 链非死代码（真实两步换取），但 canonical 流程已覆盖等价能力（第二步；第一步改由用户用 AppAccessTokenBuilder 显式完成）。
- **scope 完全 contained**（已对抗验证）：仅 tenant_access_token.rs 引用 LegacyAppAccessTokenBody / legacy_* 字段 / 3 deprecated 方法；其余 `.app_id(` 等调用在 Config/InternalRequestBuilder/AppAccessTokenBuilder/TokenRequest 上。
- 回滚：`git revert`。

## 测试策略

- `grep -rn '#\[deprecated' crates/ --include='*.rs'` = 0（全仓清零，v0.18 完成）
- `grep 'LegacyAppAccessTokenBody' <file>` = 0；`grep '#\[deprecated' <file>` = 0
- canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization` 通过
- 三组 clippy `-- -Dwarnings -A missing_docs` exit 0 + `cargo test --workspace` 通过

## Spec Patch

无。delta spec 验收场景（全仓 deprecated 清零 + 文件级 grep + canonical 行为不变 + clippy/test）经对抗验证确认 sound，无需回写。
