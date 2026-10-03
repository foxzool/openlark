# 设计：实现 issue #199 飞书 API 变动检测新增接口

**Issue**: [foxzool/openlark#199](https://github.com/foxzool/openlark/issues/199)
**日期**: 2026-06-15
**前置参考**: #195（2026-06-08 API 变动检测，已实现 34 个 API）

## 1. 范围

### 做什么

实现 issue #199 报告的 **10 个新增 API**（完整强类型：请求体 + 响应体均建模），并更新 `api_list_export.csv` 基准。

### 不做什么（明确排除）

- **34 处字段变化** —— 全部是文档元数据噪声，对代码零影响：
  - `url` 字段加 METHOD 前缀（如 `/open-apis/...` → `GET:/open-apis/...`）
  - `fullPath` / `docPath` 格式变化（docPath 多数被清空）
  - `meta.Resource` 加 `okr.` 前缀（如 `alignment` → `okr.alignment`）
  - `isCharge` false → true
  - 涉及的 aily/okr/task/vc API 在 #195 已实现，代码使用硬编码 URL 常量、不依赖 resource 名 / isCharge，无需改动。

- **覆盖率报告** —— 由实现增量自动体现，无需单独改动。

### 10 个 API 分布

| Crate | 数量 | API |
|-------|------|-----|
| openlark-bot（**新建**） | 1 | bot/v4/bot/search |
| openlark-communication | 1 | im/v2/chats/search |
| openlark-mail | 8 | profile, mail/search, recall, recall_detail, send_status, cancel_scheduled_send, signatures, multi_entity/search |

## 2. 三个 crate 的结构落地

### Block 1：新建 `openlark-bot` crate（1 个 API）

新建 crate 需在 **4 个注册点**对齐：

```
crates/openlark-bot/
├── Cargo.toml          # 仿照 openlark-mail 的最小模板
└── src/
    ├── lib.rs          # pub use BotService; prelude
    └── bot/
        ├── mod.rs
        └── bot/v4/
            ├── mod.rs
            └── bot/
                ├── mod.rs
                └── search.rs   # POST /open-apis/bot/v4/bot/search
```

**4 个注册点改动：**
1. 根 `Cargo.toml` `[workspace.members]` 加 `"crates/openlark-bot"`
2. 根 `Cargo.toml` workspace `[dependencies]` 加 `openlark-bot = { path = "crates/openlark-bot", version = "0.17.0" }`
3. 根 `Cargo.toml` 根 crate 加 feature `bot = ["auth", "dep:openlark-bot", "openlark-client/bot"]` + `openlark-bot = { workspace = true, optional = true }`
4. `src/lib.rs` 加 `#[cfg(feature = "bot")] pub use openlark_bot as bot;`

> bot 目录采用 `bot/bot/v4/bot/` 多层嵌套（与 openlark-communication 的 `aily/aily/v1/agent/` 模式一致），这是仓库的版本化约定。

### Block 2：`openlark-communication` 加 1 个 API

`im/v2/chat/` 目录当前为空，正好落 `search`：

```
crates/openlark-communication/src/im/im/v2/
├── chat/                    # 新建（当前不存在）
│   ├── mod.rs
│   └── search.rs            # POST /open-apis/im/v2/chats/search
└── mod.rs                   # 已存在，需加 `pub mod chat;`
```

### Block 3：`openlark-mail` 加 8 个 API（按现有子域归类）

mail crate 已有完善的 `user_mailbox/{message,setting,draft}` 结构。8 个 API 归类如下：

```
mail/v1/user_mailbox/
├── profile.rs               # 新建：GET .../profile（顶层，参数只支持 me）
├── search.rs                # 新建：POST .../user_mailboxes/:id/search
├── message/
│   ├── send_status.rs       # 新建：GET .../messages/:id/send_status
│   └── (现有 send.rs 等)
├── message/recall/          # 新建子目录
│   ├── mod.rs
│   ├── recall.rs            # POST .../messages/:id/recall（撤回邮件）
│   └── get_recall_detail.rs # GET .../messages/:id/recall（撤回进度）
├── draft/
│   ├── cancel_scheduled_send.rs  # 新建：POST .../messages/:id/cancel_scheduled_send
│   └── (现有 create.rs 等)
└── setting/
    ├── get_signatures.rs    # 新建：GET .../settings/signatures
    └── (现有 send_as.rs)
mail/v1/multi_entity/        # 新建顶层子域（不属于 user_mailbox）
├── mod.rs
└── search.rs                # POST /open-apis/mail/v1/multi_entity/search
```

**归类依据：**
- recall 的 POST（撤回）和 GET（进度）是同一资源的两个方法，放 `message/recall/` 子目录与飞书 URL `messages/:id/recall` 对齐。
- multi_entity 路径无 `user_mailboxes` 段，独立成顶层模块。

## 3. API 建模规范（完整强类型）

每个 API 遵循 #195 的实现骨架，但**请求体和响应体都用 serde 结构体**（不再用 `serde_json::Value`）。

### 通用骨架（以 bot search 为例）

```rust
//! 搜索机器人
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v4/bot/search

use openlark_core::{SDKResult, config::Config, req_option::RequestOption, validate_required};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索机器人请求。
#[derive(Debug, Clone, Serialize, Default)]
pub struct Request {
    config: Arc<Config>,
    /// 查询参数
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 请求体（单独序列化，不随 query 发送）
    body: SearchBotRequestBody,
}

/// 请求体
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchBotRequestBody {
    pub query: Option<String>,
    pub filter: Option<BotSearchFilter>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct BotSearchFilter {
    pub chat_ids: Option<Vec<String>>,
    pub has_chatter: Option<bool>,
}

/// 响应
#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub code: i32,
    pub msg: String,
    pub data: Option<SearchBotData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchBotData {
    pub items: Option<Vec<BotSearchItem>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
    pub notice: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotSearchItem {
    pub id: Option<String>,
    pub display_info: Option<String>,
    pub meta_data: Option<BotSearchMeta>,
}
// ... meta_data / execute() / execute_with_options()
```

### 关键设计决策

| 决策点 | 选择 | 理由 |
|--------|------|------|
| 字段可空性 | **全部 `Option<T>`** | 飞书响应字段非必返；请求体字段除少数 required 外也多可选 |
| 必填字段校验 | `validate_required!` 宏 | 对齐 #195（如 multi_entity/search 的 `query` 标注 required） |
| 命名 | serde `#[serde(rename_all = "snake_case")]` | 飞书字段已是 snake_case，但显式声明保险 |
| 枚举 | 用 `String` + 文档注释列举可选值 | 避免 enum 版本耦合（如 recall_status 的 `in_progress`/`done`） |
| 嵌套对象 | 独立 struct | 可读性 + 可测试性 |
| 类型映射 | `integer→i32/i64`, `string→String`, `boolean→bool`, `array→Vec`, `map→HashMap` | 标准映射 |

### 兜底原则

结构化的部分强类型；飞书 schema 里类型不明确的最内层值（如 signatures 的 `user_fields` value、metadata 扩展字段）用 `serde_json::Value` 兜底，避免猜测。

## 4. 10 个 API 的字段清单

以下字段定义来源于飞书 `document_portal/v1/document/get_detail` 的 apiSchema（已全部获取）。

### 1. bot/v4/bot/search（新建 bot crate）

- **POST** `/open-apis/bot/v4/bot/search`，user_access_token，仅 Custom App
- 查询参数：`page_size: int`, `page_token: string`, `user_id_type: string`
- 请求体：`query: string`, `filter: {chat_ids: string[], has_chatter: bool}`
- 响应 data：`items: [{id, display_info, meta_data: {tenant_id, enable_join_group, chat_id, is_agent}}]`, `has_more`, `page_token`, `notice`

### 2. im/v2/chats/search（communication）

- **POST** `/open-apis/im/v2/chats/search`，tenant/user_access_token
- 查询参数：`page_size`, `page_token`, `user_id_type`
- 请求体：`query`, `filter: {search_types: string[], member_ids: string[], is_manager: bool, disable_search_by_user: bool, chat_modes: string[]}`, `sorter: string`
- 响应 data：`items: [{id, display_info, meta_data: {chat_id, create_time, update_time, external, chat_mode, description, avatar, name, owner_id, owner_id_type, tenant_key, chat_status}}]`, `total: int`, `has_more`, `page_token`, `notice`

### 3. mail profile（mail/user_mailbox/profile）

- **GET** `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/profile`，user_access_token
- 路径参数：`user_mailbox_id`（只支持 `me`）
- 响应 data：`primary_email_address: string`

### 4. mail search（mail/user_mailbox/search）

- **POST** `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/search`，user_access_token
- 请求体：`query: string`（0-50 字符），`filter: {from: string[], to: string[], ...}`（filter 含发件人/收件人/文件夹/时间等多维过滤，字段较多，完整建模）
- 响应 data：`items: [{id, display_info, meta_data: {...}}]`（分页结构）

### 5. mail recall（POST 撤回邮件）

- **POST** `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/recall`
- 路径参数：`user_mailbox_id`, `message_id`，无请求体
- 响应 data：`recall_status: string`(unavailable/available), `recall_restriction_reason: string`

### 6. mail recall_detail（GET 撤回进度）

- **GET** `.../messages/:message_id/recall`
- 响应 data：`recall_status: string`(in_progress/done), `recall_result: string`（撤回结果枚举，含 all_success 等值），`details: [{recipient, status, ...}]`

### 7. mail send_status（GET 发送状态）

- **GET** `.../messages/:message_id/send_status`
- 响应 data：`message_id: string`, `details: [{recipient: {mail_address, name}, status, ...}]`

### 8. mail cancel_scheduled_send（POST 取消定时发送）

- **POST** `.../messages/:message_id/cancel_scheduled_send`
- 路径参数：`user_mailbox_id`, `message_id`，无请求体
- 响应 data：`{}`（空对象）

### 9. mail signatures（GET 签名列表）

- **GET** `.../user_mailboxes/:user_mailbox_id/settings/signatures`
- 响应 data：`signatures: [{id, name, content, signature_type, signature_device, template_json_keys, images: [{image_name, file_key, cid, file_size, image_width, image_height, download_url}], user_fields: map}]`, `usages: [{email_address, send_mail_signature_id, reply_signature_id}]`

### 10. mail multi_entity/search（POST 多实体搜索）

- **POST** `/open-apis/mail/v1/multi_entity/search`，user_access_token
- 请求体：`query: string`（1-50 字符，**required**）, `size: int`（默认 20，1-20）
- 响应 data：`items: [{type: string(user/chat...), id: string, name: string, email: string}]`

> fields 3-9 的 `user_mailbox_id` 路径参数大多支持 `me` 占位符。注意 field #10（multi_entity/search）路径无 `user_mailboxes` 段，不需要该参数。

## 5. 实施顺序与 CSV 更新

### 实施顺序（方案 A：按 crate 分区，块间独立）

**Block 1 — 新建 openlark-bot crate（最重，先做）**
1. 创建 crate 骨架 + Cargo.toml + 4 个注册点
2. 实现 `bot/v4/bot/search`
3. `cargo build -p openlark-bot` + 根 crate `--features bot` 验证

**Block 2 — communication crate（1 API）**
4. 实现 `im/v2/chat/search`
5. `cargo build -p openlark-communication` 验证

**Block 3 — mail crate（8 APIs，按子域批次）**
6. 顶层简单 API 先行：`profile`、`multi_entity/search`
7. message 子域：`send_status`、`recall/recall` + `recall/get_recall_detail`
8. draft/setting 子域：`cancel_scheduled_send`、`get_signatures`
9. 每批 `cargo build -p openlark-mail` 验证

**收尾**
10. 全量验证：`build --workspace` / `test` / `clippy` / `fmt`
11. 更新 `api_list_export.csv`：从飞书 catalog 拉取这 10 个 API 的完整 18 列数据（id/name/bizTag/meta.*/fullPath/url 等），追加到 CSV 末尾（与 #195 一致）
12. 关联 issue #199（commit message 使用 `fixes #199`）

### 验证标准（对齐 #195）

- `cargo build --workspace` ✅
- `cargo test --workspace --lib --tests` ✅
- `cargo clippy --workspace --all-targets --all-features` ✅
- `cargo fmt -- --check` ✅

### 不验证的项（诚实声明）

- **不跑真实 API 调用** —— 需要飞书租户凭证（.env），不具备。
- **mail search 的 filter 全部子字段** —— schema 显示字段较多（from/to/folder/时间），实现时从 portal schema 完整提取；若个别字段类型不明用 `Option<serde_json::Value>` 兜底。

## 6. 字段来源说明

本设计的全部字段定义来自飞书开放平台的两个数据接口（非猜测）：

- **文档页已发布**（bot search、im/v2 chat search、mail profile、mail signatures）：通过 `https://open.feishu.cn/document/<slug>.md` 获取完整 Markdown 文档。
- **文档页未发布**（其余 6 个 mail API）：通过 `https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=...` 的 `apiSchema` 字段获取 `parameters`/`requestBody`/`responses` 完整定义。

API 的 `fullPath` 来源于 `https://open.feishu.cn/api_explorer/v1/api_catalog`（即 issue #199 检测脚本的数据源）。
