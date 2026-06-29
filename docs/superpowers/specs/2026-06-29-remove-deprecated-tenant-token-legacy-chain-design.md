---
comet_change: remove-deprecated-tenant-token-legacy-chain
role: technical-design
canonical_spec: openspec
---

# Design — remove-deprecated-tenant-token-legacy-chain

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/remove-deprecated-tenant-token-legacy-chain/specs/no-deprecated-tenant-token-legacy-chain/spec.md` 为 canonical。

## 1. 背景与目标

`TenantAccessTokenBuilder`（商店应用获取 tenant_access_token）当前有两条执行路径：

- **canonical**：`new(config).app_access_token(..).tenant_key(..).execute()`——直接传 `app_access_token`。
- **legacy（deprecated）**：`new(config).app_id(..).app_secret(..).app_ticket(..).tenant_key(..).execute()`——`app_access_token` 为空时，先用 app_id/secret/ticket 调 `/auth/v3/app_access_token` 换取，再换 tenant_access_token。

3 个 deprecated 方法（`app_id`/`app_secret`/`app_ticket`）注释"用于编译兼容"。本 change 移除整条 legacy 路径，完成 v0.18 全仓 `#[deprecated]` 清零（前 6 个 change 已归档）。

## 2. 关键技术验证（对抗验证结论）

| 断言 | 验证方式 | 结论 |
|------|---------|------|
| scope 仅 tenant_access_token.rs | 全仓 grep LegacyAppAccessTokenBody / legacy_* 字段 / 3 deprecated 方法 | ✅ 全 confined；其余 `.app_id(` 在 Config/InternalRequestBuilder/AppAccessTokenBuilder/TokenRequest |
| 唯一调用点是 legacy 测试 | grep + 接收者类型核对 | ✅ 仅 tenant_access_token.rs:326-329 |
| execute 简化可编译 | agent 实证：应用改动→`cargo build -p openlark-auth`→canonical 测试→还原 | ✅ build 成功，canonical 测试通过 |
| clone 移除合法 | 读 `validate_required!` 宏源码 | ✅ `if is_empty_trimmed(&$field)` 借用非 move；验证后 move 字段合法 |
| AppAccessTokenResponseData 未孤儿化 | 读 app_access_token.rs | ✅ 类型在其定义文件内仍完整使用 |
| CHANGELOG house style | 对照既有 breaking 条目 | ✅ 一致 |

## 3. 实现步骤

### 3.1 删除 legacy 方法与字段

- 删 3 deprecated 方法（line 84-103：`app_id`/`app_secret`/`app_ticket`，含 `#[deprecated]` 属性与 doc 注释）。
- 删 `TenantAccessTokenBuilder` 的 `legacy_app_id`/`legacy_app_secret`/`legacy_app_ticket` 字段，并从 `new()` 移除其初始化。

### 3.2 删除 legacy 结构体与 import

- 删 `LegacyAppAccessTokenBody` 结构体（line 27-32）。
- 删 `use super::app_access_token::AppAccessTokenResponseData;`（line 3）——该类型在 app_access_token.rs 仍完整使用，仅本文件 import 移除。

### 3.3 简化 execute_with_options

移除 legacy 两步换取分支，简化为：

```rust
pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<TenantAccessTokenResponseData> {
    validate_required!(self.app_access_token, "应用访问凭证不能为空");
    validate_required!(self.tenant_key, "租户标识不能为空");

    use crate::common::api_endpoints::AuthApiV3;
    let api_endpoint = AuthApiV3::TenantAccessToken;

    let request_body = TenantAccessTokenBody {
        app_access_token: self.app_access_token,   // move（validate_required! 借用，已结束）
        tenant_key: self.tenant_key,                // move
    };

    let api_request: ApiRequest<TenantAccessTokenResponseData> =
        ApiRequest::post(api_endpoint.path())
            .body(serde_json::to_value(&request_body)?)
            .with_supported_access_token_types(vec![AccessTokenType::None]);

    let response = Transport::request(api_request, &self.config, Some(option)).await?;
    response.data.ok_or_else(|| {
        openlark_core::error::validation_error("获取商店应用 tenant_access_token", "响应数据为空")
    })
}
```

移除：`let app_access_token = if ... { legacy 两步 }`、二次 HTTP 请求、`LegacyAppAccessTokenBody` 构造、`app_token_response` 处理。canonical POST `/auth/v3/tenant_access_token` body `{app_access_token, tenant_key}` 不变。

### 3.4 测试清理

- 删 `test_execute_legacy_chain_fetches_app_token_then_tenant_token`（line 285-344，含 `#[allow(deprecated)]`）。
- `test_tenant_access_token_builder_new`：删 3 行 legacy 字段断言（`legacy_app_id.is_empty()` 等），保留 `app_access_token`/`tenant_key` 断言。
- canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization` 不变（覆盖简化后的 execute）。

## 4. CHANGELOG

追加于 `## [Unreleased]` > `### Breaking Changes`（im::im 条目之后），镜像 v0.18 house style（中文、整标签加粗、反引号、全角括号、内联 `→`、`关联 #<issue>` 尾注）：

```
- **Removed deprecated tenant_access_token legacy 链**：移除 `TenantAccessTokenBuilder`
  的 `app_id`/`app_secret`/`app_ticket` 旧链式入口及两步换取逻辑（deprecated legacy
  chain）→ 迁移：先 `AppAccessTokenBuilder` 取 `app_access_token`，再
  `TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)`。关联 #278。
```

## 5. 测试策略

| 验证 | 命令 | 期望 |
|------|------|------|
| 全仓 deprecated 清零 | `grep -rn '#\[deprecated' crates/ --include='*.rs'` | 0 |
| legacy 结构体移除 | `grep 'LegacyAppAccessTokenBody' <file>` | 0 |
| 文件内 deprecated 移除 | `grep '#\[deprecated' <file>` | 0 |
| canonical 流程保留 | `grep 'pub fn app_access_token\|pub fn tenant_key' <file>` | 2 |
| 三组 clippy | `cargo clippy --workspace --all-targets [--all-features\|--no-default-features] -- -Dwarnings -A missing_docs` | exit 0 |
| 测试 | `cargo test --workspace` | 0 failed |

## 6. 风险与回滚

- **[Breaking，外部]** legacy 链移除 → 外部编译失败 → CHANGELOG 两步迁移指引。
- **[行为移除]** legacy 链非死代码，但 canonical 等价覆盖（用户用 AppAccessTokenBuilder 显式完成第一步）。
- **回滚**：`git revert`。

## 7. 非目标（明确不动）

`TenantAccessTokenInternalRequestBuilder`（独立 builder，内部应用标准流程，非 deprecated）/ `AppAccessTokenBuilder`（合法 `app_ticket`）/ `Config::builder()` 的 `app_id`/`app_secret` / canonical `app_access_token+tenant_key` 流程的网络行为。
