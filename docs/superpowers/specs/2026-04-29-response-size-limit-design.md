# 响应体大小限制设计

**Issue**: #153
**日期**: 2026-04-29
**级别**: Security (High)

## 问题

HTTP 响应体和 WebSocket 帧无大小上限保护。恶意或异常大的响应可导致 OOM/DoS。

## 方案

统一配置 `max_response_size`，默认 100MB，覆盖 HTTP 和 WebSocket。

## 变更

### 1. Config — `openlark-core/src/config.rs`

新增字段：

```rust
max_response_size: u64  // 默认 100 * 1024 * 1024
```

ConfigBuilder 提供 `.max_response_size(u64)` setter。

### 2. HTTP 响应保护 — `openlark-core/src/response_handler.rs`

两层保护：

- **预检**：检查 `content_length` header，超限直接返回 `CoreError::ResponseTooLarge`
- **流式累计**：无 content-length 时用 `response.chunk()` 逐块读取并累计，超限中断

新增辅助函数 `read_response_body_with_limit(response, max_size) -> Result<Vec<u8>>`，替换现有的 `.text().await` 和 `.bytes().await`。

### 3. WebSocket 帧保护 — `openlark-client/src/ws_client/client.rs`

将 `connect_async()` 替换为 `connect_async_with_config()`，配置：

```rust
tungstenite::protocol::WebSocketConfig {
    max_message_size: Some(max_response_size as usize),
    max_frame_size: Some(max_response_size as usize),
    ..Default::default()
}
```

### 4. 错误类型 — `openlark-core/src/error/`

新增 `CoreError` 变体：

```rust
ResponseTooLarge { limit: u64, actual: u64 }
```

## 默认值

100MB（104_857_600 字节）。飞书单文件上传限制通常为 100MB，响应体应在此范围内。

## 不在范围内

- 流式下载 API（后续 feature）
- 分组件独立大小限制
