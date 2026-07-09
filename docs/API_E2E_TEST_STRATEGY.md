# API 端到端测试策略（占位测试 → 真实 e2e）

> 关联 issue：[#351 — P10: 占位 serde_json roundtrip 测试 → test_runtime 端到端](https://github.com/foxzool/openlark/issues/351)
> 父 epic：[#314 deep-module](https://github.com/foxzool/openlark/issues/314)
> 本 PR：策略文档 + pilot crate（`openlark-security`）全量改造。

## 1. 问题：占位测试不验证接口层

业务 crate 的接口层（`src/**/v*/` 下的 API 文件）普遍存在两类**占位测试**，它们都不触发真实的 `Builder → execute → Transport` 路径：

1. **`serde_json` roundtrip 占位**：只测 `Request/Response` 结构体的 `serialize/deserialize`，从不构造请求、不发送。
2. **`let _ = request.execute().await` 丢弃式占位**（部分 `openlark-docs` 的 `test_runtime` 用法）：调用了 `execute()`，但把结果丢弃，只断言「线程没 panic」。由于 `Config` 指向真实 `open.feishu.cn` 且无凭证，`execute` 实际返回 `Err`，而测试仍「绿」。

结论：**业务 crate 的真实端到端覆盖近似为 0**。`validate_required!` 之类的校验被测到了，但「URL 是否拼对、method/body/query 是否正确、响应是否被正确解析」全部未验证。

## 2. 目标模式：wiremock 端到端（auth 既定模式）

唯一能真正验证 `Builder → execute → Transport → 响应解析` 的模式，是 `openlark-auth` 已落地的 **wiremock + `base_url` 注入**：

1. `wiremock::MockServer::start()` 起一个本地 mock HTTP 服务。
2. `Mock::given(method + path [+ body])` 挂载期望的请求匹配与响应。
3. `Config::builder().base_url(server.uri())` 把真实 `Transport` 指向 mock。
4. `.execute().await` 跑完整链路。
5. 断言返回的 `data`，并用 `server.received_requests()` 反查**实际发出的 HTTP 请求形状**（method / path / query / body）。

参考实现：
- 历史范本：`crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs::test_execute_sends_app_token_tenant_key_and_no_authorization`
- 本 PR pilot 范本：`crates/openlark-security/src/acs/acs/v1/device/{get,create,list}.rs`

### 2.1 为什么不用 docs 的 `test_runtime` Potemkin 模式

issue 标题写作「→ `test_runtime` 端到端」，但 `test_runtime()` 本身只是 `Tokio Runtime` 的封装（见 `openlark-core/src/testing/mock_context.rs`），**不是一个 mock**。`openlark-docs` 全 crate 有 `wiremock` dev-dep 却 0 处使用，42 处 `execute()` 调用全部是 `let _ = ...execute()` 丢弃式——这正是要消除的反模式，不能作为目标。

目标模式是 **wiremock 端到端**（auth 模式），不是 `test_runtime` 丢弃式。

### 2.2 三个关键细节（避免踩坑）

**① `enable_token_cache(false)`：App/Tenant 类型端点的测试必备**

业务接口多用 `AccessTokenType::App`/`Tenant`。`Config::default()` 的 `enable_token_cache = true`，此时 `Transport` 会调用 `TokenProvider` 去获取 token；默认的 `NoOpTokenProvider` 直接返回 `ConfigurationError`，请求根本到不了 mock。

测试中把 token 注入视为正交关注点（auth crate 已充分覆盖），配置加 `.enable_token_cache(false)`：在 `determine_token_type`（`openlark-core/src/http.rs`）中，缓存关闭 + `App` + 未显式传 token → 退化为 `None`，不发 token 请求、不加 `Authorization` 头，请求直达 mock。

```rust
let config = Config::builder()
    .app_id("ci_app_id")
    .app_secret("ci_app_secret")
    .base_url(server.uri())
    .enable_token_cache(false)   // ← 关键：绕过 token 获取，直达 mock
    .build();
```

**② 响应信封：`ResponseFormat::Data` → `{"code":0,"data":<值>}`**

业务 crate 的响应类型多为 `serde_json::Value`，其 `ApiResponseTrait` blanket impl 返回 `ResponseFormat::Data`（`openlark-core/src/api/responses.rs:211`）。`execute()` 返回的就是 `data` 字段的**内容**。故 mock body 必须是：

```json
{ "code": 0, "msg": "success", "data": { ...断言的内容... } }
```

`code: 0` 表示成功；非 0 会被 `Transport` 当作 API 错误。

**③ wiremock 是 per-crate dev-dependency**

`openlark-core` 的 `TestServer` 封装是 `#[cfg(test)]`，**不能跨 crate 使用**（见 [`TESTSERVER_LIMITATIONS.md`](./TESTSERVER_LIMITATIONS.md)）。业务 crate 必须各自在 `[dev-dependencies]` 加 `wiremock = { workspace = true }`，并在测试内直接 `use wiremock::{Mock, MockServer, ResponseTemplate}`。长期去重方案（独立 `openlark-test-utils` crate）见同文档方案 3，**不在本 PR 范围**。

## 3. 食谱（已验证模板）

### GET + 路径参数

```rust
#[tokio::test]
async fn test_get_device_returns_data_on_success() {
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/acs/v1/devices/dev_001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0, "msg": "success",
            "data": { "device_id": "dev_001", "device_name": "前台门禁机" }
        })))
        .mount(&server).await;

    let config = Config::builder()
        .app_id("ci_app_id").app_secret("ci_app_secret")
        .base_url(server.uri()).enable_token_cache(false).build();

    let data = GetDeviceRequest::new(config, "dev_001")
        .execute().await.expect("获取设备信息应成功");
    assert_eq!(data["device_id"], "dev_001");

    let received = server.received_requests().await.unwrap_or_default();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].url.path(), "/open-apis/acs/v1/devices/dev_001");
}
```

### POST + body（额外校验请求体透传）

```rust
// mock 用 method+path 匹配（不绑死 body，避免脆性）；响应后反查 received body
let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
assert_eq!(sent["device_name"], "新门禁机");
```

### GET + query 参数（校验拼装）

```rust
let query = received[0].url.query().unwrap_or("");
assert!(query.contains("page_size=10"));
assert!(query.contains("device_type=gate"));
```

> 约定：测试命名 `test_<op>_<resource>_returns_data_on_success`，与既有 `rejects_empty_*` 校验测试并列共存（不替换、不删除校验测试）。

## 4. 各 crate 规模与优先级

按「API 文件数（`fn execute`）」排序，端到端改造的体量与建议批次：

| Crate | API 文件数 | 含 mod tests 文件 | 现状 | 优先级 |
|-------|-----------|------------------|------|--------|
| `openlark-security` | 39 | 32 | ✅ pilot 全量 wiremock e2e | P0 |
| `openlark-cardkit` | 10 | 18 | ✅ 全量 wiremock e2e | P1 |
| `openlark-user` | 7 | 10 | ✅ wiremock e2e（幻影 get 见 #377） | P1 |
| `openlark-auth` | 15 | 26 | ✅ 已是 wiremock e2e（范本） | — |
| `openlark-analytics` | 20 | 25 | ✅ 全量 wiremock e2e | P2 |
| `openlark-helpdesk` | 57 | 75 | ✅ 全量 wiremock e2e | P2 |
| `openlark-application` | 96 | 104 | ✅ 真实端点 36 e2e；幻影 stub 见 #382 | P3 |
| `openlark-mail` | 108 | 130 | ✅ 34 真实端点 wiremock e2e | P3 |
| `openlark-platform` | 125 | 131 | ✅ 76 endpoint e2e，占位 0 残留 | P3 |
| `openlark-meeting` | 125 | 134 | ✅ 90 endpoint e2e，占位 0 残留 | P3 |
| `openlark-workflow` | 132 | 154 | ✅ 清 roundtrip 占位；endpoint 已有 to_url/builder 测试，完整 e2e 可后续 | P3 |
| `openlark-docs` | 172 | 263 | ✅ Potemkin 清理 + sheets/docx/wiki/drive/base 等 e2e | P3 |
| `openlark-communication` | 190 | 227 | ✅ 28+35 body 类 wiremock e2e | P3 |
| `openlark-hr` | 599 | 625 | 🚧 **按子域分 PR**：ehr+payroll+compensation ✅ / performance ✅ / okr ✅ / attendance ✅ / 余 hire·feishu_people | P4 |

**选型原则**：
- pilot 选小而全（覆盖 GET/POST/LIST/DELETE/PATCH 等所有形状）的 crate，建立可复制范本 → `openlark-security`。
- 后续按 crate 分批，**每个 crate 一个子 issue/PR**（issue #351 明确「宜按 crate 分批子 issue」）。大 crate（application/mail/hr…）按子域再拆。
- 优先改造「响应类型为强类型 struct」的接口（更能捕获反序列化回归）；`serde_json::Value` 透传接口次之。

## 5. 验收

- [x] 测试策略文档（本文件）
- [x] pilot crate（`openlark-security`）全量替换为 wiremock 端到端（含移除 `src/lib.rs` 中的占位 roundtrip 测试）
- [x] `cargo test -p openlark-security --all-features` 通过（79 项，含 39 个新增 e2e）
- [x] P1–P3 业务 crate 按批完成（cardkit/user/analytics/helpdesk/application/mail/platform/meeting/workflow/docs/communication）
- [ ] P4 `openlark-hr`：按子域分 PR（ehr+payroll+compensation ✅；performance ✅；okr ✅；attendance ✅；后续 hire / feishu_people）

## 6. 相关文档

- [`TESTING.md`](../TESTING.md) — 总测试指南
- [`docs/TEST_ARCHITECTURE_SUMMARY.md`](./TEST_ARCHITECTURE_SUMMARY.md) — 测试架构与覆盖率门禁
- [`docs/TESTSERVER_LIMITATIONS.md`](./TESTSERVER_LIMITATIONS.md) — `TestServer` 跨 crate 限制（为何业务 crate 直接用 wiremock）
- [`docs/TEST_MIGRATION.md`](./TEST_MIGRATION.md) — 向 `TestServer` 迁移（core 内部）
