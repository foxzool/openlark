# 响应体大小限制 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为 HTTP 响应和 WebSocket 帧添加统一的大小限制，防止 OOM/DoS 攻击。

**Architecture:** 在 `openlark-core` 的 `Config` 中新增 `max_response_size` 字段（默认 100MB），通过 `Transport` 层传递到 `ImprovedResponseHandler`，在读取响应体之前进行大小检查。WebSocket 通过 `connect_async_with_config` 配置 `max_message_size` 和 `max_frame_size`。

**Tech Stack:** Rust, reqwest, tokio-tungstenite, thiserror

---

### Task 1: 添加 ResponseTooLarge 错误变体

**Files:**
- Modify: `crates/openlark-core/src/error/core.rs`
- Modify: `crates/openlark-core/src/error/codes.rs`
- Modify: `crates/openlark-core/src/error/mod.rs`

- [ ] **Step 1: 在 ErrorCode 枚举中添加 ResponseTooLarge**

在 `crates/openlark-core/src/error/codes.rs` 的 `ErrorCode` 枚举中添加：

```rust
/// 响应体大小超过限制
ResponseTooLarge = 41300,
```

同时在 `ErrorCode` 的相关方法（`from_http_status`, `severity`, `category`, `is_retryable` 等）中添加对应分支。`severity` 返回 `ErrorSeverity::Warning`，`category` 返回 `ErrorCategory::Network`，`is_retryable` 返回 `false`。

- [ ] **Step 2: 在 CoreError 枚举中添加 ResponseTooLarge 变体**

在 `crates/openlark-core/src/error/core.rs` 中，`CoreError` 枚举（在 `Internal` 之前）添加：

```rust
/// 响应体大小超过限制
#[error("响应体过大: {actual} 字节超过限制 {limit} 字节")]
ResponseTooLarge {
    /// 配置的大小限制
    limit: u64,
    /// 实际大小（已知时），否则为 0
    actual: u64,
    /// 错误上下文
    ctx: Box<ErrorContext>,
},
```

- [ ] **Step 3: 更新 CoreError 的所有 match 分支**

需要更新以下方法中的 match 分支（在 `crates/openlark-core/src/error/core.rs`）：

1. `Clone for CoreError` — 添加 `Self::ResponseTooLarge { limit, actual, ctx } => Self::ResponseTooLarge { limit: *limit, actual: *actual, ctx: ctx.clone() },`
2. `code()` — 添加 `Self::ResponseTooLarge { .. } => ErrorCode::ResponseTooLarge,`
3. `ctx()` — 添加 `Self::ResponseTooLarge { ctx, .. } => ctx,`
4. `map_context()` — 添加对应的 match arm
5. `is_network_error()` — 无需修改（使用默认 false 即可）

- [ ] **Step 4: 更新 ErrorBuilder 的 BuilderKind 和 build()**

在 `BuilderKind` 中不添加新变体（ResponseTooLarge 不走 builder 路径）。添加一个便捷构造函数：

在 `crates/openlark-core/src/error/core.rs` 的 `impl CoreError` 中添加：

```rust
/// 响应体过大错误
pub fn response_too_large(limit: u64, actual: u64) -> Self {
    Self::ResponseTooLarge {
        limit,
        actual,
        ctx: Box::new(ErrorContext::new()),
    }
}
```

在 `crates/openlark-core/src/error/mod.rs` 的导出中确认 `CoreError` 已导出（已经是）。

- [ ] **Step 5: 运行测试确认编译通过**

Run: `cargo check -p openlark-core --all-features`
Expected: 编译成功

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-core/src/error/
git commit -m "feat(error): 添加 ResponseTooLarge 错误变体 (#153)"
```

---

### Task 2: 在 openlark-core Config 中添加 max_response_size

**Files:**
- Modify: `crates/openlark-core/src/config.rs`

- [ ] **Step 1: 在 ConfigInner 中添加 max_response_size 字段**

在 `crates/openlark-core/src/config.rs` 的 `ConfigInner` 结构体中添加字段（约第 62 行之后）：

```rust
/// 响应体最大大小（字节），超过返回 ResponseTooLarge 错误
/// 默认 100MB
pub(crate) max_response_size: u64,
```

在 `Default for ConfigInner` 中添加：

```rust
max_response_size: 100 * 1024 * 1024, // 100MB
```

- [ ] **Step 2: 在 Config 中添加 getter 和 builder 支持**

在 `impl Config` 中添加 getter：

```rust
/// 获取响应体最大大小限制
pub fn max_response_size(&self) -> u64 {
    self.inner.max_response_size
}
```

在 `ConfigBuilder` 结构体中添加字段：

```rust
max_response_size: Option<u64>,
```

在 `ConfigBuilder` 的 `build()` 方法中添加：

```rust
max_response_size: self.max_response_size.unwrap_or(default.max_response_size),
```

在 `impl ConfigBuilder` 中添加 builder 方法：

```rust
/// 设置响应体最大大小限制（字节），默认 100MB
pub fn max_response_size(mut self, size: u64) -> Self {
    self.max_response_size = Some(size);
    self
}
```

更新 `with_token_provider()` 中的 `ConfigInner` 构造，添加 `max_response_size: self.max_response_size,`。

更新 `Debug for ConfigInner`，添加 `.field("max_response_size", &self.max_response_size)`。

- [ ] **Step 3: 更新现有测试中直接构造 ConfigInner 的地方**

在 `crates/openlark-core/src/config.rs` 的测试中，所有 `ConfigInner { ... }` 构造需要添加 `max_response_size` 字段。有以下测试需要更新：

1. `test_config_creation` — 添加 `max_response_size: 100 * 1024 * 1024,`
2. `test_config_clone` — 同上
3. `test_config_with_custom_header` — 使用 `..ConfigInner::default()` 已覆盖
4. `test_config_with_different_app_types` — 同上
5. `test_config_with_timeout_variations` — 同上
6. `test_config_arc_efficiency_simulation` — 使用 `Config::default()` 已覆盖

对于使用 `ConfigInner { ... }` 直接构造的测试，添加 `max_response_size` 字段。

- [ ] **Step 4: 运行测试**

Run: `cargo test -p openlark-core --all-features`
Expected: 全部通过

- [ ] **Step 5: Commit**

```bash
git add crates/openlark-core/src/config.rs
git commit -m "feat(config): 添加 max_response_size 配置项，默认 100MB (#153)"
```

---

### Task 3: HTTP 响应体大小限制保护

**Files:**
- Modify: `crates/openlark-core/src/http.rs`
- Modify: `crates/openlark-core/src/response_handler.rs`

- [ ] **Step 1: 添加响应体大小限制检查辅助函数**

在 `crates/openlark-core/src/response_handler.rs` 中添加辅助函数（在 `ImprovedResponseHandler` impl 之前）：

```rust
/// 默认响应体最大大小
const DEFAULT_MAX_RESPONSE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

/// 读取响应体，带大小限制保护
///
/// 两层保护：
/// 1. 预检：检查 content_length header，超限直接返回错误
/// 2. 流式累计：无 content_length 时用 chunk() 逐块读取并累计，超限中断
async fn read_body_with_limit(
    response: reqwest::Response,
    max_size: u64,
) -> Result<Vec<u8>, crate::error::CoreError> {
    use crate::error::CoreError;

    // 预检：如果 content_length 已知且超限，直接返回错误
    if let Some(content_length) = response.content_length() {
        if content_length > max_size {
            return Err(CoreError::response_too_large(max_size, content_length));
        }
    }

    // 流式读取并累计大小
    let mut total_size: u64 = 0;
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| crate::error::network_error(e.to_string()))?;
        total_size += chunk.len() as u64;
        if total_size > max_size {
            return Err(CoreError::response_too_large(max_size, total_size));
        }
        body.extend_from_slice(&chunk);
    }

    Ok(body)
}
```

- [ ] **Step 2: 更新 Cargo.toml 添加 futures-util 依赖（如需要）**

检查 `crates/openlark-core/Cargo.toml` 是否已有 `futures-util` 依赖。如果没有，添加：

```toml
futures-util = { workspace = true }
```

如果 workspace 中没有此依赖，则在 `[dependencies]` 中添加 `futures-util = "0.3"`。

- [ ] **Step 3: 修改 handle_response 签名，接受 max_size 参数**

在 `crates/openlark-core/src/response_handler.rs` 中，修改 `handle_response` 方法签名：

```rust
pub async fn handle_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    max_size: u64,
) -> SDKResult<Response<T>> {
```

在方法内部，将 `max_size` 传递给各个子方法调用：

```rust
let result = match T::data_format() {
    ResponseFormat::Data => Self::handle_data_response(response, max_size).await,
    ResponseFormat::Flatten => Self::handle_flatten_response(response, max_size).await,
    ResponseFormat::Binary => Self::handle_binary_response(response, max_size).await,
    ResponseFormat::Text => Self::handle_data_response(response, max_size).await,
    ResponseFormat::Custom => Self::handle_data_response(response, max_size).await,
};
```

- [ ] **Step 4: 修改 handle_data_response 使用大小限制**

修改 `handle_data_response` 签名和实现：

```rust
async fn handle_data_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    max_size: u64,
) -> SDKResult<Response<T>> {
    let tracker = ResponseTracker::start("json_data", response.content_length());

    let body_bytes = read_body_with_limit(response, max_size).await?;
    let response_text = String::from_utf8_lossy(&body_bytes).to_string();

    // ... 其余逻辑不变（response_text 后面的代码）
```

- [ ] **Step 5: 修改 handle_flatten_response 使用大小限制**

```rust
async fn handle_flatten_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    max_size: u64,
) -> SDKResult<Response<T>> {
    let tracker = ResponseTracker::start("json_flatten", response.content_length());

    let body_bytes = read_body_with_limit(response, max_size).await?;
    let response_text = String::from_utf8_lossy(&body_bytes).to_string();
    debug!("Raw response: {response_text}");

    // ... 其余逻辑不变
```

- [ ] **Step 6: 修改 handle_binary_response 使用大小限制**

```rust
async fn handle_binary_response<T: ApiResponseTrait>(
    response: reqwest::Response,
    max_size: u64,
) -> SDKResult<Response<T>> {
    let tracker = ResponseTracker::start("binary", response.content_length());

    let _file_name = response
        .headers()
        .get("Content-Disposition")
        .and_then(|header| header.to_str().ok())
        .and_then(content_disposition::extract_filename)
        .unwrap_or_default();

    tracker.parsing_complete();

    // 用带大小限制的方式读取二进制数据
    let bytes = match read_body_with_limit(response, max_size).await {
        Ok(data) => {
            tracing::debug!("Binary response received: {} bytes", data.len());
            data
        }
        Err(e) => {
            let error_msg = format!("Failed to read binary response: {e}");
            tracker.error(&error_msg);
            return Err(e);
        }
    };

    // ... 其余 Any/downcast 逻辑不变，用 bytes 替换原来的 byte_vec
```

- [ ] **Step 7: 更新 http.rs 中的调用点**

在 `crates/openlark-core/src/http.rs` 中：

修改 `do_send` 签名，接受 `max_response_size`：

```rust
pub async fn do_send(
    raw_request: RequestBuilder,
    body: Vec<u8>,
    multi_part: bool,
    max_response_size: u64,
) -> SDKResult<Response<T>> {
```

修改调用 `handle_response` 的地方：

```rust
ImprovedResponseHandler::handle_response(response, max_response_size).await
```

修改 `do_request` 中调用 `do_send` 的地方：

```rust
let resp = Self::do_send(req, http_req.to_bytes(), !http_req.file().is_empty(), config.max_response_size()).await?;
```

- [ ] **Step 8: 运行编译检查**

Run: `cargo check -p openlark-core --all-features`
Expected: 编译成功

- [ ] **Step 9: 运行测试**

Run: `cargo test -p openlark-core --all-features`
Expected: 全部通过

- [ ] **Step 10: Commit**

```bash
git add crates/openlark-core/src/response_handler.rs crates/openlark-core/src/http.rs crates/openlark-core/Cargo.toml
git commit -m "feat(http): 响应体大小限制保护，防止 OOM (#153)"
```

---

### Task 4: WebSocket 帧大小限制保护

**Files:**
- Modify: `crates/openlark-client/src/config.rs`
- Modify: `crates/openlark-client/src/ws_client/client.rs`

- [ ] **Step 1: 在 openlark-client Config 中添加 max_response_size**

在 `crates/openlark-client/src/config.rs` 的 `Config` 结构体中添加字段：

```rust
/// 响应体最大大小限制（字节），默认 100MB
pub max_response_size: u64,
```

在 `Default for Config` 中添加：

```rust
max_response_size: 100 * 1024 * 1024, // 100MB
```

在 `ConfigBuilder` 中添加 builder 方法：

```rust
/// 设置响应体最大大小限制（字节），默认 100MB
pub fn max_response_size(mut self, size: u64) -> Self {
    self.config.max_response_size = size;
    self
}
```

在 `Debug for Config` 中添加 `.field("max_response_size", &self.max_response_size)`。

在 `ConfigSummary` 中添加字段。

更新 `update_with()` 方法。

更新 `apply_env_var()` 添加 `OPENLARK_MAX_RESPONSE_SIZE` 环境变量支持。

更新 `build_core_config()` 将 `max_response_size` 传入 core config：

```rust
pub fn build_core_config(&self) -> CoreConfig {
    CoreConfig::builder()
        .app_id(self.app_id.clone())
        .app_secret(self.app_secret.clone())
        .base_url(self.base_url.clone())
        .app_type(self.app_type)
        .enable_token_cache(self.enable_token_cache)
        .req_timeout(self.timeout)
        .header(self.headers.clone())
        .max_response_size(self.max_response_size)
        .build()
}
```

- [ ] **Step 2: 修改 WebSocket 客户端使用 connect_async_with_config**

在 `crates/openlark-client/src/ws_client/client.rs` 中：

更新导入（约第 18 行），添加 `connect_async_with_config`：

```rust
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, connect_async_with_config,
    tungstenite::protocol::{Message, frame::coding::CloseCode},
};
```

修改 `open()` 方法（约第 286 行），将 `connect_async(conn_url)` 替换为：

```rust
let ws_config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig {
    max_message_size: Some(config.max_response_size as usize),
    max_frame_size: Some(config.max_response_size as usize),
    ..Default::default()
};

let (conn, _response) = connect_async_with_config(conn_url, Some(ws_config), false).await?;
```

- [ ] **Step 3: 更新 openlark-client Config 相关测试**

在 `crates/openlark-client/src/config.rs` 的测试中，所有直接构造 `Config { ... }` 的地方添加 `max_response_size: 100 * 1024 * 1024,`。

- [ ] **Step 4: 运行编译检查**

Run: `cargo check -p openlark-client --features websocket`
Expected: 编译成功

- [ ] **Step 5: 运行测试**

Run: `cargo test -p openlark-client --features websocket`
Expected: 全部通过

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-client/src/config.rs crates/openlark-client/src/ws_client/client.rs
git commit -m "feat(ws): WebSocket 帧大小限制保护，防止 OOM (#153)"
```

---

### Task 5: 全量编译验证和最终测试

**Files:**
- 无新增修改

- [ ] **Step 1: 全 workspace 编译**

Run: `cargo check --workspace --all-features`
Expected: 编译成功

- [ ] **Step 2: 运行全 workspace 测试**

Run: `cargo test --workspace --all-features`
Expected: 全部通过

- [ ] **Step 3: 运行 clippy**

Run: `cargo clippy --workspace --all-features`
Expected: 无新 warning

- [ ] **Step 4: 格式化检查**

Run: `cargo fmt --check`
Expected: 无差异
