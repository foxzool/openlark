# Issue #199 飞书 API 变动检测新增接口 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 issue #199 报告的 10 个新增 API（完整强类型），新建 openlark-bot crate，并更新 api_list_export.csv 基准。

**Architecture:** 按方案 A 分三个独立 Block —— 新建 openlark-bot crate（1 API）、扩展 openlark-communication（1 API）、扩展 openlark-mail（8 API）。每个 API 提供强类型 Request（builder 模式）+ Response（serde 结构体）。复用各 crate 现有的资源结构体 / 端点常量 / api_utils 模式。

**Tech Stack:** Rust, serde, openlark-core（Config/ApiRequest/Transport/validate_required!）

**参考 spec:** `docs/superpowers/specs/2026-06-15-issue-199-api-drift-design.md`

---

## 文件结构总览

### Block 1 — 新建 openlark-bot crate
- Create: `crates/openlark-bot/Cargo.toml`
- Create: `crates/openlark-bot/src/lib.rs`
- Create: `crates/openlark-bot/src/service.rs`
- Create: `crates/openlark-bot/src/bot/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/v4/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/v4/bot/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/v4/bot/search.rs`
- Modify: `Cargo.toml`（workspace members + workspace deps + 根 crate features/deps）
- Modify: `crates/openlark-client/Cargo.toml` + `src/lib.rs`（注册 bot feature + 导出）

### Block 2 — openlark-communication 加 im/v2/chat/search
- Create: `crates/openlark-communication/src/im/im/v2/chat/mod.rs`
- Create: `crates/openlark-communication/src/im/im/v2/chat/search.rs`
- Modify: `crates/openlark-communication/src/im/im/v2/mod.rs`
- Modify: `crates/openlark-communication/src/endpoints/im.rs`（加端点常量）

### Block 3 — openlark-mail 加 8 个 API
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/profile.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/search.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/send_status.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/mod.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/recall.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/get_recall_detail.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/cancel_scheduled_send.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/get_signatures.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/multi_entity/mod.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/multi_entity/search.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs`（挂载 profile/search/message/send_status）
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/mod.rs`（挂载 send_status + recall）
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/mod.rs`（挂载 cancel_scheduled_send）
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/mod.rs`（挂载 get_signatures）
- Modify: `crates/openlark-mail/src/mail/mail/v1/mod.rs`（挂载 multi_entity）

### 收尾
- Modify: `api_list_export.csv`（追加 10 行）

---

## Block 1：新建 openlark-bot crate

### Task 1: 创建 openlark-bot crate 骨架

**Files:**
- Create: `crates/openlark-bot/Cargo.toml`
- Create: `crates/openlark-bot/src/lib.rs`
- Create: `crates/openlark-bot/src/service.rs`
- Create: `crates/openlark-bot/src/bot/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/v4/mod.rs`
- Create: `crates/openlark-bot/src/bot/bot/v4/bot/mod.rs`

- [ ] **Step 1: 创建 Cargo.toml**

仿照 `crates/openlark-mail/Cargo.toml` 最小模板：

```toml
[package]
name = "openlark-bot"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
documentation.workspace = true
description = "OpenLark 机器人模块 - 提供飞书机器人搜索 API"

[lints]
workspace = true

[dependencies]
openlark-core = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
reqwest = { workspace = true }
tokio = { workspace = true, optional = true }
tracing = { workspace = true }

[dev-dependencies]
tokio = { workspace = true, features = ["full"] }

[features]
default = ["v4", "async"]
async = ["tokio"]

# 机器人 API 版本
v4 = []

# 完整功能集
full = ["v4", "async"]

[package.metadata.cargo-machete]
ignored = ["reqwest", "tokio", "tracing"]
```

- [ ] **Step 2: 创建 src/lib.rs**

```rust
#![allow(dead_code)]
#![allow(clippy::module_inception)]
//! # OpenLark 机器人模块
//!
//! OpenLark SDK 的机器人模块，提供飞书机器人搜索 API。
//!
//! ## 功能特性
//!
//! - **机器人搜索**: 按关键词搜索当前用户可见的机器人

mod service;

// bot 模块
#[cfg(feature = "v4")]
/// 机器人 API 模块。
pub mod bot;

// 重新导出核心服务
/// 机器人服务统一入口。
pub use service::BotService;

/// 机器人服务客户端类型别名（统一命名为 `XxxClient`）。
pub type BotClient = BotService;

/// 机器人模块版本信息
/// 当前 crate 版本号。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_ne!(VERSION, "");
    }
}
```

- [ ] **Step 3: 创建 src/service.rs**

```rust
use openlark_core::config::Config;
use std::sync::Arc;

/// BotService：机器人服务的统一入口
///
/// 提供对机器人 API v4 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct BotService {
    config: Arc<Config>,
}

impl BotService {
    /// 创建新的机器人服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v4")]
    /// 访问机器人 API。
    pub fn bot(&self) -> crate::bot::bot::Bot {
        crate::bot::bot::Bot::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }
}
```

- [ ] **Step 4: 创建模块层级文件**

`crates/openlark-bot/src/bot/mod.rs`:
```rust
//! Bot 模块

pub mod bot;
```

`crates/openlark-bot/src/bot/bot/mod.rs`:
```rust
//! bot 资源模块

pub mod v4;

use openlark_core::config::Config;
use std::sync::Arc;

/// Bot：机器人资源入口
#[derive(Clone)]
pub struct Bot {
    config: Arc<Config>,
}

impl Bot {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v4 API。
    pub fn v4(&self) -> v4::V4 {
        v4::V4::new(self.config.clone())
    }
}
```

`crates/openlark-bot/src/bot/bot/v4/mod.rs`:
```rust
//! Bot v4 模块

pub mod bot;

use openlark_core::config::Config;
use std::sync::Arc;

/// V4：机器人 v4 版本入口
#[derive(Clone)]
pub struct V4 {
    config: Arc<Config>,
}

impl V4 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 bot 资源。
    pub fn bot(&self) -> bot::BotResource {
        bot::BotResource::new(self.config.clone())
    }
}
```

`crates/openlark-bot/src/bot/bot/v4/bot/mod.rs`:
```rust
//! bot 资源 v4

pub mod search;

use openlark_core::config::Config;
use std::sync::Arc;

/// BotResource：机器人资源
#[derive(Clone)]
pub struct BotResource {
    config: Arc<Config>,
}

impl BotResource {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建搜索机器人请求。
    pub fn search(&self) -> search::SearchBotRequest {
        search::SearchBotRequest::new(self.config.clone())
    }
}
```

- [ ] **Step 5: 验证骨架编译（search.rs 用最小占位）**

先放一个最小 `search.rs` 让骨架编译：
```rust
//! 搜索机器人

use openlark_core::config::Config;
use std::sync::Arc;

/// 搜索机器人请求。
#[derive(Debug, Clone)]
pub struct SearchBotRequest {
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl SearchBotRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
```

- [ ] **Step 6: 提交**

```bash
git add crates/openlark-bot
git commit -m "feat(bot): 新建 openlark-bot crate 骨架"
```

---

### Task 2: 注册 openlark-bot 到 workspace

**Files:**
- Modify: `Cargo.toml`（根，3 处）
- Modify: `crates/openlark-client/Cargo.toml`
- Modify: `crates/openlark-client/src/lib.rs`

- [ ] **Step 1: 根 Cargo.toml 加 workspace member**

在 `[workspace.members]` 列表中（约第 3-45 行），在 `"crates/openlark-mail"` 行之后添加：
```toml
    "crates/openlark-bot",            # 机器人服务
```

- [ ] **Step 2: 根 Cargo.toml 加 workspace dependency**

在 workspace `[dependencies]` 区（约第 142 行 `openlark-mail` 之后）添加：
```toml
openlark-bot = { path = "crates/openlark-bot", version = "0.17.0" }
```

- [ ] **Step 3: 根 Cargo.toml 根 crate 加 dep + feature**

在根 crate `[dependencies]` 区（约第 203 行 `openlark-mail` 之后）添加：
```toml
openlark-bot = { workspace = true, optional = true }
```

在 `[features]` 区（约第 335 行 `mail = [...]` 之后）添加：
```toml
bot = ["auth", "dep:openlark-bot", "openlark-client/bot"]
```

- [ ] **Step 4: 根 src/lib.rs 加导出**

在 `src/lib.rs` 第 78 行（`pub use openlark_mail as mail;` 之后）添加：
```rust
#[cfg(feature = "bot")]
pub use openlark_bot as bot;
```

- [ ] **Step 5: openlark-client 注册 bot**

`crates/openlark-client/Cargo.toml`：
- 在 `[dependencies]`（约第 54 行 `openlark-mail` 之后）加：
```toml
openlark-bot = { workspace = true, optional = true }
```
- 在 `[features]`（约第 81 行 `mail = [...]` 之后）加：
```toml
bot = ["auth", "dep:openlark-bot"]
```

`crates/openlark-client/src/lib.rs`：
- 在第 360 行（`pub use openlark_mail::MailClient;` 之后）加：
```rust
#[cfg(feature = "bot")]
pub use openlark_bot::BotClient;
```
- 在 prelude 模块（约第 504 行 `pub use openlark_mail::MailClient;` 之后）加：
```rust
    #[cfg(feature = "bot")]
    pub use openlark_bot::BotClient;
```

- [ ] **Step 6: 验证编译**

Run: `cargo build -p openlark-bot && cargo build -p openlark-client --features bot && cargo build --features bot`
Expected: 全部 PASS，无错误

- [ ] **Step 7: 提交**

```bash
git add Cargo.toml src/lib.rs crates/openlark-client/Cargo.toml crates/openlark-client/src/lib.rs
git commit -m "feat(bot): 注册 openlark-bot 到 workspace 和 client"
```

---

### Task 3: 实现 bot/v4/bot/search API（完整强类型）

**Files:**
- Modify: `crates/openlark-bot/src/bot/bot/v4/bot/search.rs`

- [ ] **Step 1: 实现完整 search.rs**

替换 Task 1 Step 5 的占位实现：

```rust
//! 搜索机器人
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v4/bot/search

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索机器人请求。
#[derive(Debug, Clone)]
pub struct SearchBotRequest {
    config: Arc<Config>,
    /// 查询参数
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 请求体
    body: SearchBotRequestBody,
}

/// 搜索机器人请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchBotRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<BotSearchFilter>,
}

/// 机器人搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct BotSearchFilter {
    /// 群聊ID列表，查询指定群聊内的机器人（0-100）。
    pub chat_ids: Option<Vec<String>>,
    /// 是否和机器人聊过天，true 只返回有聊天的机器人。
    pub has_chatter: Option<bool>,
}

/// 搜索机器人响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchBotResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<SearchBotData>,
}

/// 搜索机器人响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchBotData {
    /// 搜索结果列表。
    pub items: Option<Vec<BotSearchItem>>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 机器人搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct BotSearchItem {
    /// 机器人ID。
    pub id: Option<String>,
    /// 包含机器人基本信息的卡片，关键词命中的文本片段用 <h></h> 包裹标注。
    pub display_info: Option<String>,
    /// 机器人元信息。
    pub meta_data: Option<BotSearchMeta>,
}

/// 机器人元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct BotSearchMeta {
    /// 租户ID。
    pub tenant_id: Option<String>,
    /// 是否允许加入群聊。
    pub enable_join_group: Option<bool>,
    /// 机器人所属的群聊ID。
    pub chat_id: Option<String>,
    /// 是否是智能体。
    pub is_agent: Option<bool>,
}

impl SearchBotRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            body: SearchBotRequestBody::default(),
        }
    }

    /// 设置分页大小（查询参数）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（查询参数）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）：open_id / union_id / user_id。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置搜索关键词（请求体，0-50 字符）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件（请求体）。
    pub fn filter(mut self, filter: BotSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchBotResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchBotResponse> {
        let mut req: ApiRequest<SearchBotResponse> =
            ApiRequest::post("/open-apis/bot/v4/bot/search");

        // 拼接查询参数
        let mut query_params: Vec<(String, String)> = Vec::new();
        if let Some(ps) = self.page_size {
            query_params.push(("page_size".to_string(), ps.to_string()));
        }
        if let Some(pt) = &self.page_token {
            query_params.push(("page_token".to_string(), pt.clone()));
        }
        if let Some(uit) = &self.user_id_type {
            query_params.push(("user_id_type".to_string(), uit.clone()));
        }
        for (k, v) in &query_params {
            req = req.query_param(k, v);
        }

        // 请求体
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::request_serialization_error("搜索机器人", e)
        })?;
        req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("搜索机器人", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let request = SearchBotRequest::new(config);
        assert!(request.body.query.is_none());
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = SearchBotRequest::new(config)
            .query("会议助手")
            .page_size(10)
            .user_id_type("open_id");
        assert_eq!(request.body.query, Some("会议助手".to_string()));
        assert_eq!(request.page_size, Some(10));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn body_serializes_correctly() {
        let body = SearchBotRequestBody {
            query: Some("会议助手".to_string()),
            filter: Some(BotSearchFilter {
                chat_ids: Some(vec!["oc-123".to_string()]),
                has_chatter: Some(false),
            }),
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["query"], "会议助手");
        assert_eq!(json["filter"]["chat_ids"][0], "oc-123");
    }

    #[test]
    fn response_deserializes_from_json() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "id": "7890123456abcdef",
                    "display_info": "飞书<h>搜索</h>助手",
                    "meta_data": {
                        "tenant_id": "7010970696222244883",
                        "enable_join_group": false,
                        "chat_id": "oc-7890123456abcdef",
                        "is_agent": false
                    }
                }],
                "has_more": true,
                "page_token": "token123"
            }
        }"#;
        let resp: SearchBotResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.id, Some("7890123456abcdef".to_string()));
        assert_eq!(item.meta_data.unwrap().is_agent, Some(false));
    }
}
```

> ⚠️ 实现者注意：`ApiRequest::query_param()` 和 `request_serialization_error` 的确切方法/函数名需在实现时核对 `crates/openlark-core/src/api/` 和 `crates/openlark-core/src/error/` 的实际签名。若 `query_param` 方法名不同（如 `query()`），改用实际方法；若 `request_serialization_error` 签名不符，改用 `openlark_core::error::validation_error` 或 `CoreError` 直接构造。先 grep 确认再写。

- [ ] **Step 2: 验证编译**

Run: `cargo build -p openlark-bot`
Expected: PASS（若有 ApiRequest 方法名不匹配，按 Step 1 注意事项核对并修正）

- [ ] **Step 3: 运行测试**

Run: `cargo test -p openlark-bot`
Expected: 4 个测试 PASS

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-bot/src/bot/bot/v4/bot/search.rs
git commit -m "feat(bot): 实现搜索机器人 API (完整强类型)"
```

---

## Block 2：openlark-communication 加 im/v2/chat/search

### Task 4: 确认 ApiRequest 的 query/body 传递 API

**Files:**
- Read: `crates/openlark-core/src/api/`

- [ ] **Step 1: 核对 ApiRequest 方法签名**

Run: `grep -n "pub fn query\|pub fn body\|pub fn get\|pub fn post" crates/openlark-core/src/api/*.rs`

记录实际的查询参数传递方法名（`query_param` / `query` / 其他）和 body 传递方法名。后续 Task 4/5/6/7/8/9 全部依赖此结论。

- [ ] **Step 2: 核对 error 构造辅助函数**

Run: `grep -n "pub fn request_serialization_error\|pub fn validation_error\|pub fn missing_response_data_error" crates/openlark-core/src/error/*.rs`

记录可用辅助函数签名。后续 Task 全部依赖。

---

### Task 5: 实现 im/v2/chats/search API

**Files:**
- Create: `crates/openlark-communication/src/im/im/v2/chat/mod.rs`
- Create: `crates/openlark-communication/src/im/im/v2/chat/search.rs`
- Modify: `crates/openlark-communication/src/im/im/v2/mod.rs`
- Modify: `crates/openlark-communication/src/endpoints/im.rs`

- [ ] **Step 1: 加端点常量**

在 `crates/openlark-communication/src/endpoints/im.rs` 末尾添加：
```rust
/// IM v2 群组搜索
pub const IM_V2_CHATS_SEARCH: &str = "/open-apis/im/v2/chats/search";
```

- [ ] **Step 2: 创建 chat/mod.rs**

`crates/openlark-communication/src/im/im/v2/chat/mod.rs`:
```rust
//! 群组搜索 v2

pub mod search;
```

- [ ] **Step 3: 创建 chat/search.rs**

```rust
//! 搜索群组
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/chat/search

use crate::endpoints::IM_V2_CHATS_SEARCH;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索群组请求。
#[derive(Debug, Clone)]
pub struct SearchChatsRequest {
    config: Arc<Config>,
    /// 查询参数
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 请求体
    body: SearchChatsRequestBody,
}

/// 搜索群组请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchChatsRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<ChatSearchFilter>,
    /// 排序方式：create_time_desc / update_time_desc / member_count_desc。
    pub sorter: Option<String>,
}

/// 群组搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct ChatSearchFilter {
    /// 群组类型：private / external / public_joined / public_not_joined（0-4）。
    pub search_types: Option<Vec<String>>,
    /// 群成员ID（0-500）。
    pub member_ids: Option<Vec<String>>,
    /// 是否自己创建或者管理的群组。
    pub is_manager: Option<bool>,
    /// 是否关闭以人搜群功能（默认开启）。
    pub disable_search_by_user: Option<bool>,
    /// 群模式筛选器：default（普通群）/ thread（话题群）（0-2）。
    pub chat_modes: Option<Vec<String>>,
}

/// 搜索群组响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchChatsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SearchChatsData>,
}

/// 搜索群组响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchChatsData {
    /// 搜索结果列表。
    pub items: Option<Vec<ChatSearchItem>>,
    /// 搜索命中结果数。
    pub total: Option<i32>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 群组搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatSearchItem {
    /// 群组ID。
    pub id: Option<String>,
    /// 包含群组基本信息的卡片，关键词命中的文本片段用 <h></h> 包裹标注。
    pub display_info: Option<String>,
    /// 群组元信息。
    pub meta_data: Option<ChatSearchMeta>,
}

/// 群组元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatSearchMeta {
    /// 群组 ID。
    pub chat_id: Option<String>,
    /// 创建时间（iso8601）。
    pub create_time: Option<String>,
    /// 更新时间（iso8601）。
    pub update_time: Option<String>,
    /// 是否是外部群。
    pub external: Option<bool>,
    /// 群模式：group / topic。
    pub chat_mode: Option<String>,
    /// 群描述。
    pub description: Option<String>,
    /// 群头像URL。
    pub avatar: Option<String>,
    /// 群名称。
    pub name: Option<String>,
    /// 群主ID。
    pub owner_id: Option<String>,
    /// 群主ID类型。
    pub owner_id_type: Option<String>,
    /// tenant key。
    pub tenant_key: Option<String>,
    /// 群状态：normal / dissolved / dissolved_save。
    pub chat_status: Option<String>,
}

impl SearchChatsRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            body: SearchChatsRequestBody::default(),
        }
    }

    /// 设置分页大小（查询参数）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（查询参数）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置搜索关键词。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件。
    pub fn filter(mut self, filter: ChatSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 设置排序方式。
    pub fn sorter(mut self, sorter: impl Into<String>) -> Self {
        self.body.sorter = Some(sorter.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchChatsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchChatsResponse> {
        let mut req: ApiRequest<SearchChatsResponse> = ApiRequest::post(IM_V2_CHATS_SEARCH);

        if let Some(ps) = self.page_size {
            req = req.query_param("page_size", &ps.to_string());
        }
        if let Some(pt) = &self.page_token {
            req = req.query_param("page_token", pt);
        }
        if let Some(uit) = &self.user_id_type {
            req = req.query_param("user_id_type", uit);
        }

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::request_serialization_error("搜索群组", e)
        })?;
        req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("搜索群组", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = SearchChatsRequest::new(config)
            .query("部门群")
            .page_size(10);
        assert_eq!(request.body.query, Some("部门群".to_string()));
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn response_deserializes_from_json() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "id": "7890123456abcdef",
                    "display_info": "飞书<h>搜索</h>",
                    "meta_data": {
                        "chat_id": "7890123456abcdef",
                        "external": true,
                        "chat_mode": "group",
                        "name": "研发讨论群",
                        "chat_status": "normal"
                    }
                }],
                "total": 10,
                "has_more": true
            }
        }"#;
        let resp: SearchChatsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
        assert_eq!(resp.data.as_ref().unwrap().total, Some(10));
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.meta_data.unwrap().name, Some("研发讨论群".to_string()));
    }
}
```

> ⚠️ 同 Task 3 Step 1 注意事项：`query_param` / `request_serialization_error` 方法名以 Task 4 核对结论为准。

- [ ] **Step 4: 挂载 chat 模块到 v2/mod.rs**

在 `crates/openlark-communication/src/im/im/v2/mod.rs` 的 `pub mod chat_button;` 之后添加：
```rust
pub mod chat;
```

- [ ] **Step 5: 验证编译与测试**

Run: `cargo build -p openlark-communication && cargo test -p openlark-communication --lib search`
Expected: PASS

- [ ] **Step 6: 提交**

```bash
git add crates/openlark-communication/src/im/im/v2/chat crates/openlark-communication/src/im/im/v2/mod.rs crates/openlark-communication/src/endpoints/im.rs
git commit -m "feat(im): 实现搜索群组 API (im/v2/chats/search, 完整强类型)"
```

---

## Block 3：openlark-mail 加 8 个 API

### Task 6: mail profile + multi_entity/search（顶层简单 API）

**Files:**
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/profile.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/multi_entity/mod.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/multi_entity/search.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/mod.rs`

- [ ] **Step 1: 创建 profile.rs**

`crates/openlark-mail/src/mail/mail/v1/user_mailbox/profile.rs`:
```rust
//! 获取用户邮箱信息
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/profile

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取用户邮箱信息请求。
#[derive(Debug, Clone)]
pub struct GetUserMailboxProfileRequest {
    config: Arc<Config>,
    /// 用户邮箱ID（只支持 "me"）。
    user_mailbox_id: String,
}

/// 获取用户邮箱信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserMailboxProfileResponse {
    /// 错误码。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<GetUserMailboxProfileData>,
}

/// 获取用户邮箱信息响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserMailboxProfileData {
    /// 用户主邮箱地址。
    pub primary_email_address: Option<String>,
}

impl GetUserMailboxProfileRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetUserMailboxProfileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetUserMailboxProfileResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/profile",
            self.user_mailbox_id
        );
        let req: ApiRequest<GetUserMailboxProfileResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户邮箱信息", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": { "primary_email_address": "abc@abc.com" }
        }"#;
        let resp: GetUserMailboxProfileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            resp.data.unwrap().primary_email_address,
            Some("abc@abc.com".to_string())
        );
    }
}
```

- [ ] **Step 2: 创建 multi_entity/mod.rs**

`crates/openlark-mail/src/mail/mail/v1/multi_entity/mod.rs`:
```rust
//! 多实体搜索模块

pub mod search;

use openlark_core::config::Config;
use std::sync::Arc;

/// MultiEntity：多实体搜索资源入口
#[derive(Clone)]
pub struct MultiEntity {
    config: Arc<Config>,
}

impl MultiEntity {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建多实体搜索请求。
    pub fn search(&self) -> search::MultiEntitySearchRequest {
        search::MultiEntitySearchRequest::new(self.config.clone())
    }
}
```

- [ ] **Step 3: 创建 multi_entity/search.rs**

```rust
//! 多实体搜索
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/multi_entity/search

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 多实体搜索请求。
#[derive(Debug, Clone)]
pub struct MultiEntitySearchRequest {
    config: Arc<Config>,
    body: MultiEntitySearchRequestBody,
}

/// 多实体搜索请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEntitySearchRequestBody {
    /// 搜索关键词（1-50 字符，必填）。
    pub query: String,
    /// 获取的数据条数（默认 20，支持 1-20）。
    pub size: Option<i32>,
}

/// 多实体搜索响应。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<MultiEntitySearchData>,
}

/// 多实体搜索响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchData {
    /// 搜索结果列表。
    pub items: Option<Vec<MultiEntitySearchItem>>,
}

/// 多实体搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchItem {
    /// 实体类型（如 user、chat 等）。
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    /// 唯一标识 ID。
    pub id: Option<String>,
    /// 名称。
    pub name: Option<String>,
    /// 邮箱地址。
    pub email: Option<String>,
}

impl MultiEntitySearchRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: MultiEntitySearchRequestBody {
                query: String::new(),
                size: None,
            },
        }
    }

    /// 设置搜索关键词（必填，1-50 字符）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置返回条数（默认 20，1-20）。
    pub fn size(mut self, size: i32) -> Self {
        self.body.size = Some(size);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<MultiEntitySearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<MultiEntitySearchResponse> {
        validate_required!(self.body.query, "query 不能为空");

        let req: ApiRequest<MultiEntitySearchResponse> =
            ApiRequest::post("/open-apis/mail/v1/multi_entity/search");
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::request_serialization_error("多实体搜索", e)
        })?;
        let req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("多实体搜索", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": { "items": [{ "type": "user", "id": "691", "name": "张三", "email": "z@b.com" }] }
        }"#;
        let resp: MultiEntitySearchResponse = serde_json::from_str(json).unwrap();
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.entity_type, Some("user".to_string()));
    }
}
```

- [ ] **Step 4: 挂载模块**

`crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs`：在现有 `pub mod setting;` 之后添加：
```rust
/// profile 模块。
pub mod profile;
/// search 模块。
pub mod search;
```

并在 `UserMailbox` impl 块中（现有工厂方法之后）添加：
```rust
    /// 创建获取用户邮箱信息请求。
    pub fn profile(&self) -> profile::GetUserMailboxProfileRequest {
        profile::GetUserMailboxProfileRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
```

> 注意：`search.rs` 模块名与 mail crate 顶层 `mail/mail/v1/user_mailbox` 下新增的 `search.rs` 冲突吗？——不冲突，这是 `user_mailbox/search.rs`（搜索邮件），与 `user_mailbox/mod.rs` 同级。Task 7 实现。

`crates/openlark-mail/src/mail/mail/v1/mod.rs`：添加：
```rust
/// 多实体搜索模块。
pub mod multi_entity;
```

- [ ] **Step 5: 验证编译与测试**

Run: `cargo build -p openlark-mail && cargo test -p openlark-mail --lib profile && cargo test -p openlark-mail --lib multi_entity`
Expected: PASS

- [ ] **Step 6: 提交**

```bash
git add crates/openlark-mail/src/mail/mail/v1/user_mailbox/profile.rs crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs crates/openlark-mail/src/mail/mail/v1/multi_entity crates/openlark-mail/src/mail/mail/v1/mod.rs
git commit -m "feat(mail): 实现获取用户邮箱信息 + 多实体搜索 API"
```

---

### Task 7: mail search（搜索邮件，复杂 filter）

**Files:**
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/search.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs`

- [ ] **Step 1: 创建 search.rs**

```rust
//! 搜索邮件
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/search

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索邮件请求。
#[derive(Debug, Clone)]
pub struct SearchMailRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: SearchMailRequestBody,
}

/// 搜索邮件请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchMailRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<MailSearchFilter>,
}

/// 邮件搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct MailSearchFilter {
    /// 发件人姓名或邮箱地址筛选。
    pub from: Option<Vec<String>>,
    /// 收件人姓名或邮箱地址筛选。
    pub to: Option<Vec<String>>,
    /// 抄送人姓名或邮箱地址筛选。
    pub cc: Option<Vec<String>>,
    /// 密送人姓名或邮箱地址筛选。
    pub bcc: Option<Vec<String>>,
    /// 邮件主题搜索。
    pub subject: Option<String>,
    /// 文件夹名称筛选（系统文件夹：inbox/sent/drafts 等）。
    pub folder: Option<Vec<String>>,
    /// 自定义标签名称筛选（子标签用 parent_name/child_name）。
    pub label: Option<Vec<String>>,
    /// 是否只筛选有附件的邮件。
    pub has_attachment: Option<bool>,
    /// 是否只筛选未读邮件。
    pub is_unread: Option<bool>,
    /// 邮件接收时间范围筛选。
    pub create_time: Option<MailSearchTimeRange>,
}

/// 邮件搜索时间范围。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct MailSearchTimeRange {
    /// 开始时间（iso8601，精确到秒）。
    pub start_time: Option<String>,
    /// 截止时间（iso8601，精确到秒）。
    pub end_time: Option<String>,
}

/// 搜索邮件响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchMailResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SearchMailData>,
}

/// 搜索邮件响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchMailData {
    /// 搜索结果列表。
    pub items: Option<Vec<MailSearchItem>>,
    /// 搜索命中结果数。
    pub total: Option<i32>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 邮件搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSearchItem {
    /// 邮件唯一标识。
    pub id: Option<String>,
    /// 包含邮件基本信息的卡片。
    pub display_info: Option<String>,
    /// 邮件元信息。
    pub meta_data: Option<MailSearchMeta>,
}

/// 邮件搜索元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSearchMeta {
    /// 邮件主题。
    pub title: Option<String>,
    /// 邮件线程 ID。
    pub thread_id: Option<String>,
    /// 邮件接收时间。
    pub create_time: Option<String>,
    /// 邮件唯一标识。
    pub message_biz_id: Option<String>,
    /// 邮件发件人。
    pub from: Option<MailAddress>,
}

/// 邮件地址。
#[derive(Debug, Clone, Deserialize)]
pub struct MailAddress {
    /// 邮件地址。
    pub mail_address: Option<String>,
    /// 名称。
    pub name: Option<String>,
}

impl SearchMailRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: SearchMailRequestBody::default(),
        }
    }

    /// 设置搜索关键词。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件。
    pub fn filter(mut self, filter: MailSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchMailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchMailResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/search",
            self.mailbox_id
        );
        let req: ApiRequest<SearchMailResponse> = ApiRequest::post(&path);
        let body = serde_json::to_value(&self.body)
            .map_err(|e| openlark_core::error::request_serialization_error("搜索邮件", e))?;
        let req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("搜索邮件", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_serializes() {
        let filter = MailSearchFilter {
            from: Some(vec!["user@example.com".to_string()]),
            is_unread: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["from"][0], "user@example.com");
        assert_eq!(json["is_unread"], true);
    }
}
```

- [ ] **Step 2: 挂载到 UserMailbox**

在 `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs` 的 `UserMailbox` impl 块添加：
```rust
    /// 创建搜索邮件请求。
    pub fn search(&self) -> search::SearchMailRequest {
        search::SearchMailRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
```

- [ ] **Step 3: 验证编译与测试**

Run: `cargo build -p openlark-mail && cargo test -p openlark-mail --lib search`
Expected: PASS

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-mail/src/mail/mail/v1/user_mailbox/search.rs crates/openlark-mail/src/mail/mail/v1/user_mailbox/mod.rs
git commit -m "feat(mail): 实现搜索邮件 API (含完整 filter)"
```

---

### Task 8: mail message send_status + recall（撤回邮件 + 撤回进度）

**Files:**
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/send_status.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/mod.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/recall.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/recall/get_recall_detail.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/mod.rs`

- [ ] **Step 1: 创建 send_status.rs**

```rust
//! 查询邮件发送状态
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/send_status

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 查询邮件发送状态请求。
#[derive(Debug, Clone)]
pub struct GetMailSendStatusRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 查询邮件发送状态响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetMailSendStatusResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<MailSendStatusData>,
}

/// 发送状态数据。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSendStatusData {
    /// 邮件业务标识 ID。
    pub message_id: Option<String>,
    /// 收件人投递状态列表。
    pub details: Option<Vec<MailSendStatusDetail>>,
}

/// 收件人投递状态详情。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSendStatusDetail {
    /// 收件人信息。
    pub recipient: Option<MailRecipient>,
    /// 投递状态（1-6）。
    pub status: Option<i32>,
    /// 最后更新时间（Unix 时间戳，秒）。
    pub last_updated_time: Option<i64>,
}

/// 收件人信息。
#[derive(Debug, Clone, Deserialize)]
pub struct MailRecipient {
    /// 邮件地址。
    pub mail_address: Option<String>,
    /// 名称。
    pub name: Option<String>,
}

impl GetMailSendStatusRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailSendStatusResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailSendStatusResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/send_status",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<GetMailSendStatusResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询邮件发送状态", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0, "msg": "success",
            "data": { "message_id": "197c5d72e22e1d78", "details": [{"recipient":{"mail_address":"m@o.com","name":"Mike"},"status":1}] }
        }"#;
        let resp: GetMailSendStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.unwrap().message_id, Some("197c5d72e22e1d78".to_string()));
    }
}
```

- [ ] **Step 2: 创建 recall/mod.rs**

```rust
//! 邮件撤回模块

pub mod get_recall_detail;
pub mod recall;

use openlark_core::config::Config;
use std::sync::Arc;

/// Recall：邮件撤回资源入口
#[derive(Clone)]
pub struct Recall {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

impl Recall {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 撤回已发送邮件（POST）。
    pub fn recall(&self) -> recall::RecallMessageRequest {
        recall::RecallMessageRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            self.message_id.clone(),
        )
    }

    /// 获取邮件撤回进度（GET）。
    pub fn get_recall_detail(&self) -> get_recall_detail::GetRecallDetailRequest {
        get_recall_detail::GetRecallDetailRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            self.message_id.clone(),
        )
    }
}
```

- [ ] **Step 3: 创建 recall/recall.rs（POST 撤回）**

```rust
//! 撤回已发送邮件
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-sent_message/recall

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 撤回已发送邮件请求。
#[derive(Debug, Clone)]
pub struct RecallMessageRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 撤回已发送邮件响应。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallMessageResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<RecallMessageData>,
}

/// 撤回邮件数据。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallMessageData {
    /// 撤回状态：unavailable（不可撤回）/ available（可撤回）。
    pub recall_status: Option<String>,
    /// 不支持撤回的原因：recall_not_enabled / migration_domain / sender_address_not_owned /
    /// already_recalled / not_delivered / exceeded_time_limit（仅 recall_status=unavailable 时返回）。
    pub recall_restriction_reason: Option<String>,
}

impl RecallMessageRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<RecallMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecallMessageResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/recall",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<RecallMessageResponse> = ApiRequest::post(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("撤回邮件", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": { "recall_status": "available" } }"#;
        let resp: RecallMessageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.unwrap().recall_status, Some("available".to_string()));
    }
}
```

- [ ] **Step 4: 创建 recall/get_recall_detail.rs（GET 进度）**

```rust
//! 获取邮件撤回进度
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-sent_message/get_recall_detail

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取邮件撤回进度请求。
#[derive(Debug, Clone)]
pub struct GetRecallDetailRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 获取邮件撤回进度响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetRecallDetailResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<RecallDetailData>,
}

/// 撤回进度数据。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallDetailData {
    /// 整体撤回进度：in_progress / done。
    pub recall_status: Option<String>,
    /// 撤回最终结果（仅 done 时有意义）：all_success / all_fail / some_fail / processing。
    pub recall_result: Option<String>,
    /// 撤回成功的收件人数。
    pub success_count: Option<i32>,
    /// 撤回失败的收件人数。
    pub failure_count: Option<i32>,
    /// 处理中的收件人数。
    pub processing_count: Option<i32>,
    /// 每个收件人的撤回详情列表。
    pub items: Option<Vec<RecallDetailItem>>,
}

/// 收件人撤回详情。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallDetailItem {
    /// 收件人邮箱地址。
    pub recipient_address: Option<String>,
    /// 收件人显示名称。
    pub recipient_name: Option<String>,
    /// 该收件人的撤回状态：success / fail / processing。
    pub status: Option<String>,
    /// 撤回失败原因（仅 status=fail）：message_has_been_read / not_using_lark_mail /
    /// not_in_the_same_tenant / invalid_address / unknown。
    pub fail_reason: Option<String>,
    /// 是否为邮件组地址。
    pub is_mailing_list: Option<bool>,
    /// 邮件组内成功撤回人数（仅 is_mailing_list=true）。
    pub mailing_list_success_count: Option<i32>,
    /// 邮件组内撤回失败人数（仅 is_mailing_list=true）。
    pub mailing_list_failure_count: Option<i32>,
    /// 邮件组完成百分比 0-100（仅 is_mailing_list=true）。
    pub mailing_list_finish_percent: Option<i32>,
}

impl GetRecallDetailRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetRecallDetailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetRecallDetailResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/recall",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<GetRecallDetailResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件撤回进度", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": { "recall_status": "done", "recall_result": "all_success", "success_count": 2 } }"#;
        let resp: GetRecallDetailResponse = serde_json::from_str(json).unwrap();
        let d = resp.data.unwrap();
        assert_eq!(d.recall_status, Some("done".to_string()));
        assert_eq!(d.success_count, Some(2));
    }
}
```

- [ ] **Step 5: 挂载到 message/mod.rs**

在 `crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/mod.rs`：
- 文件顶部 `pub mod` 区添加：
```rust
/// send_status 模块。
pub mod send_status;
/// recall 模块（邮件撤回）。
pub mod recall;
```
- 在 `Message` impl 块添加工厂方法。

  > **重要**：现有 `Message::new(config, mailbox_id)` 只接收 mailbox_id，无 message_id（`get()` 方法才接收 message_id）。因此 send_status / recall 应模仿 `get()` 的签名，在工厂方法上接收 message_id 参数：

```rust
    /// 创建查询发送状态请求。
    pub fn send_status(
        &self,
        message_id: impl Into<String>,
    ) -> send_status::GetMailSendStatusRequest {
        send_status::GetMailSendStatusRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            message_id,
        )
    }

    /// 访问邮件撤回资源。
    pub fn recall(
        &self,
        message_id: impl Into<String>,
    ) -> recall::Recall {
        recall::Recall::new(self.config.clone(), self.mailbox_id.clone(), message_id)
    }
```

- [ ] **Step 6: 验证编译与测试**

Run: `cargo build -p openlark-mail && cargo test -p openlark-mail --lib send_status && cargo test -p openlark-mail --lib recall`
Expected: PASS

- [ ] **Step 7: 提交**

```bash
git add crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/
git commit -m "feat(mail): 实现查询发送状态 + 邮件撤回(撤回+进度) API"
```

---

### Task 9: mail cancel_scheduled_send + signatures

**Files:**
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/cancel_scheduled_send.rs`
- Create: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/get_signatures.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/mod.rs`
- Modify: `crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/mod.rs`

- [ ] **Step 1: 创建 cancel_scheduled_send.rs**

```rust
//! 取消定时发送
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/cancel_scheduled_send

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 取消定时发送请求。
#[derive(Debug, Clone)]
pub struct CancelScheduledSendRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 取消定时发送响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CancelScheduledSendResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<serde_json::Value>,
}

impl CancelScheduledSendRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CancelScheduledSendResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelScheduledSendResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/cancel_scheduled_send",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<CancelScheduledSendResponse> = ApiRequest::post(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("取消定时发送", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": {} }"#;
        let resp: CancelScheduledSendResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
    }
}
```

- [ ] **Step 2: 创建 get_signatures.rs**

```rust
//! 获取用户的签名列表
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-setting/get_signatures

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取签名列表请求。
#[derive(Debug, Clone)]
pub struct GetSignaturesRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

/// 获取签名列表响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSignaturesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SignaturesData>,
}

/// 签名列表数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SignaturesData {
    /// 用户邮箱签名列表。
    pub signatures: Option<Vec<UserMailboxSignature>>,
    /// 用户邮箱签名使用情况列表。
    pub usages: Option<Vec<SignatureUsage>>,
}

/// 用户邮箱签名。
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxSignature {
    /// 签名 ID。
    pub id: Option<String>,
    /// 签名名称。
    pub name: Option<String>,
    /// 签名内容（HTML 格式）。
    pub content: Option<String>,
    /// 签名类型：USER（用户签名）/ TENANT（租户签名）。
    pub signature_type: Option<String>,
    /// 签名适用设备类型：PC / MOBILE。
    pub signature_device: Option<String>,
    /// 企业签名模板变量渲染。
    pub template_json_keys: Option<Vec<String>>,
    /// 签名图片列表。
    pub images: Option<Vec<SignatureImage>>,
    /// 企业签名模版变量值（值结构不明确，用 Value 兜底）。
    pub user_fields: Option<serde_json::Value>,
}

/// 签名图片。
#[derive(Debug, Clone, Deserialize)]
pub struct SignatureImage {
    /// 签名图片名称。
    pub image_name: Option<String>,
    /// 签名图片的文件 key。
    pub file_key: Option<String>,
    /// 签名图片的 Content-ID。
    pub cid: Option<String>,
    /// 签名图片文件大小（字节）。
    pub file_size: Option<String>,
    /// 签名图片宽度（像素）。
    pub image_width: Option<i32>,
    /// 签名图片高度（像素）。
    pub image_height: Option<i32>,
    /// 图片下载 url。
    pub download_url: Option<String>,
}

/// 签名使用情况。
#[derive(Debug, Clone, Deserialize)]
pub struct SignatureUsage {
    /// 邮箱地址。
    pub email_address: Option<String>,
    /// 发送邮件时使用的签名 ID。
    pub send_mail_signature_id: Option<String>,
    /// 回复邮件时使用的签名 ID。
    pub reply_signature_id: Option<String>,
}

impl GetSignaturesRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetSignaturesResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetSignaturesResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/settings/signatures",
            self.mailbox_id
        );
        let req: ApiRequest<GetSignaturesResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取签名列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0, "msg": "success",
            "data": {
                "signatures": [{
                    "id": "sig_xxx", "name": "我的签名", "content": "<div>Best regards</div>",
                    "signature_type": "USER", "signature_device": "PC"
                }],
                "usages": [{ "email_address": "u@e.com", "send_mail_signature_id": "sig_xxx" }]
            }
        }"#;
        let resp: GetSignaturesResponse = serde_json::from_str(json).unwrap();
        let sig = resp.data.unwrap().signatures.unwrap().pop().unwrap();
        assert_eq!(sig.id, Some("sig_xxx".to_string()));
    }
}
```

- [ ] **Step 3: 挂载到 draft/mod.rs 和 setting/mod.rs**

`crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/mod.rs`：顶部添加：
```rust
/// cancel_scheduled_send 模块。
pub mod cancel_scheduled_send;
```
在 `Draft`（或对应资源结构体）impl 块添加（参考现有 draft 资源结构体的工厂方法风格，需接收 message_id）：
```rust
    /// 创建取消定时发送请求。
    pub fn cancel_scheduled_send(
        &self,
        message_id: impl Into<String>,
    ) -> cancel_scheduled_send::CancelScheduledSendRequest {
        cancel_scheduled_send::CancelScheduledSendRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            message_id,
        )
    }
```
> 实现者注意：核对 draft 资源结构体名称和 `mailbox_id` 字段名（阅读 `draft/mod.rs` 现有代码）。若 draft 无独立资源结构体，改为在 `UserMailbox` impl 块暴露。

`crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/mod.rs`：添加：
```rust
/// get_signatures 模块。
pub mod get_signatures;
```
在 `Setting`（或对应资源结构体）impl 块添加：
```rust
    /// 创建获取签名列表请求。
    pub fn get_signatures(&self) -> get_signatures::GetSignaturesRequest {
        get_signatures::GetSignaturesRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
```
> 实现者注意：核对 setting 资源结构体名称和 `mailbox_id` 字段名。当前 `setting/mod.rs` 仅有 `pub mod send_as;` 无资源结构体——若如此，需先在 `setting/mod.rs` 添加 `Setting` 资源结构体（参考 `message/mod.rs` 的 `Message` 结构），或在 `UserMailbox` impl 块直接暴露 `get_signatures()` 方法（参考 `profile` 的挂载方式）。

- [ ] **Step 4: 验证编译与测试**

Run: `cargo build -p openlark-mail && cargo test -p openlark-mail --lib cancel_scheduled_send && cargo test -p openlark-mail --lib get_signatures`
Expected: PASS

- [ ] **Step 5: 提交**

```bash
git add crates/openlark-mail/src/mail/mail/v1/user_mailbox/draft/ crates/openlark-mail/src/mail/mail/v1/user_mailbox/setting/
git commit -m "feat(mail): 实现取消定时发送 + 签名列表 API"
```

---

## 收尾

### Task 10: 全量验证

**Files:** 无（仅验证）

- [ ] **Step 1: 格式检查**

Run: `cargo fmt -- --check`
Expected: PASS（若有格式问题，运行 `cargo fmt` 修正后重新检查）

- [ ] **Step 2: 构建**

Run: `cargo build --workspace --all-features`
Expected: PASS，无错误

- [ ] **Step 3: 测试**

Run: `cargo test --workspace --lib --tests`
Expected: 0 failed

- [ ] **Step 4: Clippy**

Run: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
Expected: 无 warning（若有，修正后重新检查）

- [ ] **Step 5: 修复后如有改动则提交**

若 Step 1-4 修复了任何问题：
```bash
git add -A
git commit -m "fix: 修复全量验证发现的问题"
```

---

### Task 11: 更新 api_list_export.csv

**Files:**
- Modify: `api_list_export.csv`

- [ ] **Step 1: 生成 10 个 API 的 CSV 行**

Run（用脚本从飞书 catalog 拉取这 10 个 API 的完整 18 列数据并追加）:
```bash
python3 -c "
import json, urllib.request, csv, sys

CATALOG_URL = 'https://open.feishu.cn/api_explorer/v1/api_catalog'
TARGET_IDS = {
    '7649652220287716543','7649732836954606572','7648865505080413417',
    '7620282151846448348','7648865505080429801','7650148595703123124',
    '7649297073556982987','7629252749259918546','7629252749259934930',
    '7629252749259951314',
}
req = urllib.request.Request(CATALOG_URL, headers={'User-Agent':'Mozilla/5.0'})
data = json.loads(urllib.request.urlopen(req, timeout=30).read())
items = data.get('data',{}).get('items',[])

def walk(nodes, out):
    for n in nodes:
        children = n.get('children') or []
        if children:
            walk(children, out)
        else:
            s = n.get('apiSummary') or {}
            if str(s.get('id','')) in TARGET_IDS:
                meta = s.get('meta',{}) or {}
                out.append({
                    'id': s.get('id',''),
                    'name': n.get('name',''),
                    'bizTag': s.get('bizTag',''),
                    'meta.Project': meta.get('Project',''),
                    'meta.Version': meta.get('Version',''),
                    'meta.Resource': meta.get('Resource',''),
                    'meta.Name': meta.get('Name',''),
                    'detail': s.get('detail',''),
                    'chargingMethod': s.get('chargingMethod',''),
                    'fullDose': s.get('fullDose',''),
                    'fullPath': s.get('fullPath',''),
                    'url': s.get('url',''),
                    'orderMark': s.get('orderMark',''),
                    'supportAppTypes': json.dumps(s.get('supportAppTypes',''), ensure_ascii=False),
                    'tags': json.dumps(s.get('tags',''), ensure_ascii=False),
                    'updateTime': s.get('updateTime',''),
                    'isCharge': s.get('isCharge',''),
                    'meta.Type': meta.get('Type',''),
                    'docPath': s.get('docPath',''),
                })

rows = []
walk(items, rows)
print(f'找到 {len(rows)} 个 API', file=sys.stderr)
if len(rows) != 10:
    print(f'警告：期望 10 个，实际 {len(rows)} 个', file=sys.stderr)

with open('api_list_export.csv', 'a', newline='', encoding='utf-8') as f:
    writer = csv.DictWriter(f, fieldnames=[
        'id','name','bizTag','meta.Project','meta.Version','meta.Resource','meta.Name','detail',
        'chargingMethod','fullDose','fullPath','url','orderMark','supportAppTypes','tags',
        'updateTime','isCharge','meta.Type','docPath'
    ])
    for r in rows:
        writer.writerow(r)
print('已追加到 api_list_export.csv', file=sys.stderr)
"
```

> 若脚本因网络问题失败，手动从 issue #199 表格 + 飞书 catalog 补齐字段。

- [ ] **Step 2: 验证 CSV 行数**

Run: `tail -10 api_list_export.csv | wc -l` 与 `wc -l api_list_export.csv`
Expected: 新增 10 行

- [ ] **Step 3: 提交**

```bash
git add api_list_export.csv
git commit -m "chore: 追加 10 个新 API 到 api_list_export.csv 基准 (fixes #199)"
```

---

## 完成标准

- [ ] 10 个 API 全部实现（完整强类型请求+响应）
- [ ] openlark-bot crate 新建并注册到 workspace/client/根 crate
- [ ] `cargo build --workspace --all-features` PASS
- [ ] `cargo test --workspace --lib --tests` 0 failed
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` 无 warning
- [ ] `cargo fmt -- --check` PASS
- [ ] `api_list_export.csv` 新增 10 行
- [ ] commit message 关联 `fixes #199`
