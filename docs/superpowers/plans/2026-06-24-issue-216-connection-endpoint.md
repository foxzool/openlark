# Issue #216 获取长连接在线数量接口 + 目录漂移归零 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现飞书新增的 `GET /open-apis/event/v1/connection`（获取长连接在线数量）强类型 SDK 接口，并刷新 `api_list_export.csv` 基准吸收 44 条目录元数据漂移，关闭 #216。

**Architecture:** 新增 `connection` 子模块，对齐 `contact/v3/job_family` 的强类型范式（`ApiRequest<R>` 中 `R` 是 data 业务载荷；`code`/`msg` 由 `RawResponse` envelope 承载）。基准 CSV 用 `tools/export_server_api_list.py` 全量重生。两部分相互独立、可独立提交。

**Tech Stack:** Rust，openlark-core（`ApiRequest`/`Transport`/`Response<R>`/`extract_response_data`），serde，Python（仅 CSV 刷新）。

**参考文件（只读，对齐范式）:**
- `crates/openlark-communication/src/contact/contact/v3/job_family/get.rs` — 强类型 GET 范式
- `crates/openlark-communication/src/event/event/v1/outbound_ip/list.rs` — 同级 event 接口（无类型版，作目录参照）
- `crates/openlark-core/src/api/responses.rs:114-121` — `Response<T> = { data: Option<T>, raw_response }`
- `crates/openlark-communication/src/common/api_utils.rs:50-58` — `extract_response_data<T>` 签名
- `crates/openlark-communication/src/endpoints/event.rs` — 端点常量定义处
- 设计文档：`docs/superpowers/specs/2026-06-24-issue-216-connection-endpoint-design.md`

---

## 文件结构

| 动作 | 路径 | 职责 |
|---|---|---|
| 新建 | `crates/openlark-communication/src/event/event/v1/connection/mod.rs` | 声明 `pub mod get;` |
| 新建 | `crates/openlark-communication/src/event/event/v1/connection/get.rs` | 请求 struct + 强类型响应 + 单测 |
| 修改 | `crates/openlark-communication/src/event/event/v1/mod.rs` | 追加 `pub mod connection;` |
| 修改 | `crates/openlark-communication/src/endpoints/event.rs` | 追加 `EVENT_V1_CONNECTION` 常量 + 测试断言 |
| 修改（脚本生成） | `api_list_export.csv` | 全量刷新（1723 → 1724，吸收 44 条漂移） |

---

## Task 1: 端点常量 EVENT_V1_CONNECTION

**Files:**
- Modify: `crates/openlark-communication/src/endpoints/event.rs`（在 `EVENT_V1_OUTBOUND_IP` 之后、`#[cfg(test)]` 之前插入）

- [ ] **Step 1: 追加常量定义**

在 `crates/openlark-communication/src/endpoints/event.rs` 中，找到：

```rust
/// 事件推送出口 IP 查询接口。
pub const EVENT_V1_OUTBOUND_IP: &str = "/open-apis/event/v1/outbound_ip";
```

在其**下方**插入（保留上方原有常量不动）：

```rust

/// 获取长连接在线数量接口。
pub const EVENT_V1_CONNECTION: &str = "/open-apis/event/v1/connection";
```

- [ ] **Step 2: 追加测试断言**

在同一文件 `#[cfg(test)]` 的 `test_event_endpoints` 函数末尾（`}` 闭合 test body 之前）追加：

```rust
        assert!(EVENT_V1_CONNECTION.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_CONNECTION.ends_with("/connection"));
```

- [ ] **Step 3: 编译并跑测试**

Run: `cargo test -p openlark-communication --lib endpoints::event::tests::test_event_endpoints`
Expected: PASS（1 个 test 通过）

- [ ] **Step 4: Commit**

```bash
git add crates/openlark-communication/src/endpoints/event.rs
git commit -m "feat(event): 新增 EVENT_V1_CONNECTION 端点常量 (#216)"
```

---

## Task 2: connection 子模块骨架 + mod.rs 挂载

**Files:**
- Create: `crates/openlark-communication/src/event/event/v1/connection/mod.rs`
- Modify: `crates/openlark-communication/src/event/event/v1/mod.rs`

- [ ] **Step 1: 创建 connection/mod.rs**

Create `crates/openlark-communication/src/event/event/v1/connection/mod.rs` with content:

```rust
//! Event v1 connection（长连接在线数量）子模块

pub mod get;
```

- [ ] **Step 2: 挂载到 event/event/v1/mod.rs**

Modify `crates/openlark-communication/src/event/event/v1/mod.rs` — 现内容为单行：

```rust
pub mod outbound_ip;
```

改为：

```rust
pub mod connection;
pub mod outbound_ip;
```

- [ ] **Step 3: 预期编译失败（get.rs 尚不存在）**

Run: `cargo build -p openlark-communication 2>&1 | head -20`
Expected: FAIL — `file not found for module 'connection'` / `get` 未声明。这是 TDD 红灯，下一步实现 get.rs 转绿。

- [ ] **Step 4: 暂不提交**（与 Task 3 的 get.rs 一起提交，避免仓库中间态不可编译）

---

## Task 3: get.rs 强类型实现 + 单测（TDD）

**Files:**
- Create: `crates/openlark-communication/src/event/event/v1/connection/get.rs`

> 类型语义（务必对齐，否则编译不过）：`Transport::request` 返回 `Response<R>`，`extract_response_data` 解出 `R`。`R` = data 业务载荷；`code`/`msg` 属于 envelope，**不**进载荷 struct。

- [ ] **Step 1: 先写 get.rs 的完整内容（含测试）**

Create `crates/openlark-communication/src/event/event/v1/connection/get.rs` with content:

```rust
//! 获取长连接在线数量
//!
//! 查询应用的长连接在线数量。应用由请求头中的 tenant_access_token 确定。
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/event-v1/connection/get

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data, endpoints::EVENT_V1_CONNECTION,
};

/// 获取长连接在线数量响应（data 业务载荷）
///
/// 官方 apiSchema 响应体：code (int, envelope) / msg (string, envelope) /
/// data.online_instance_cnt (int)。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConnectionOnlineCountResponse {
    /// 在线连接数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_instance_cnt: Option<i64>,
}

/// 获取长连接在线数量请求
pub struct GetConnectionOnlineCountRequest {
    config: Config,
}

impl GetConnectionOnlineCountRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/event-v1/connection/get
    pub async fn execute(self) -> SDKResult<GetConnectionOnlineCountResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetConnectionOnlineCountResponse> {
        // url: GET:/open-apis/event/v1/connection
        let req: ApiRequest<GetConnectionOnlineCountResponse> = ApiRequest::get(EVENT_V1_CONNECTION);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取长连接在线数量")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化往返
        let resp = GetConnectionOnlineCountResponse {
            online_instance_cnt: Some(42),
        };
        let json = serde_json::to_string(&resp).expect("序列化失败");
        let back: GetConnectionOnlineCountResponse =
            serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(back.online_instance_cnt, Some(42));
    }

    #[test]
    fn test_deserialization_from_official_payload() {
        // 官方 data 载荷形状：{"online_instance_cnt": <int>}
        // code/msg 属于 envelope，不在此载荷内。
        let json = r#"{"online_instance_cnt": 128}"#;
        let resp: GetConnectionOnlineCountResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(resp.online_instance_cnt, Some(128));
    }

    #[test]
    fn test_deserialization_missing_field_uses_default() {
        // 缺字段时 serde(default) 应回落为 None，不破坏调用方
        let json = r#"{}"#;
        let resp: GetConnectionOnlineCountResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(resp.online_instance_cnt, None);
    }
}
```

- [ ] **Step 2: 编译通过（Task 2 的红灯转绿）**

Run: `cargo build -p openlark-communication`
Expected: PASS（无编译错误）

- [ ] **Step 3: 跑单测**

Run: `cargo test -p openlark-communication --lib event::event::v1::connection`
Expected: PASS — 3 个 test（roundtrip / official payload / missing field）

- [ ] **Step 4: Commit（Task 2 + Task 3 合并提交）**

```bash
git add crates/openlark-communication/src/event/event/v1/connection/ \
        crates/openlark-communication/src/event/event/v1/mod.rs
git commit -m "feat(event): 实现获取长连接在线数量接口 (GET event/v1/connection, 强类型) (#216)

- 新增 connection/get.rs：GetConnectionOnlineCountRequest + 强类型响应
- 响应载荷仅含 online_instance_cnt (Option<i64> + serde(default))，
  code/msg 由 RawResponse envelope 承载（对齐 job_family 范式）
- 挂载到 event/event/v1/mod.rs
- 3 个单测：序列化往返 / 官方载荷反序列化 / 缺字段 default 回落"
```

---

## Task 4: Lint + 全量构建验证

**Files:** 无改动

- [ ] **Step 1: Clippy（零 warning）**

Run: `cargo clippy -p openlark-communication --all-targets -- -D warnings`
Expected: PASS（零 warning；如有 `missing_docs` 等告警，补文档注释后重跑）

- [ ] **Step 2: 格式化检查**

Run: `cargo fmt -p openlark-communication -- --check`
Expected: 无 diff。若有 diff，运行 `cargo fmt -p openlark-communication` 后回到 Step 1。

- [ ] **Step 3: 定向测试再确认**

Run: `cargo test -p openlark-communication --lib event`
Expected: 所有 event 相关 test 通过（含 Task 1 的端点断言 + Task 3 的 3 个单测）

- [ ] **Step 4: 如有 fmt/clippy 修正，补提交**

```bash
# 仅当 Step 1/2 产生改动时执行
git add -A
git commit -m "style(event): connection 接口 fmt/clippy 修正 (#216)"
```

---

## Task 5: 刷新 api_list_export.csv 基准（吸收 44 条漂移）

**Files:**
- Modify（脚本生成）: `api_list_export.csv`

> 该脚本调用飞书开放 catalog 接口，无需凭证。刷新后数据行 1723 → 1724（纳入新增 connection 接口；`wc -l` 含表头 = 1724 → 1725），并吸收 44 条元数据漂移，使下周 watch 不再重复报告。

- [ ] **Step 1: 全量重新生成 CSV**

Run:
```bash
python3 tools/export_server_api_list.py \
  --output api_list_export.csv \
  --max-workers 16 \
  --timeout 30 \
  --retries 3
```
Expected: 脚本正常退出，无异常堆栈。

- [ ] **Step 2: 验证行数与新增接口**

CSV 含表头行。刷新前为 1723 条数据（`wc -l` 含表头 = 1724）；纳入 connection 后应为 1724 条数据（`wc -l` 含表头 = 1725）。

Run:
```bash
echo "总行数（含表头，应为 1725）:"; wc -l < api_list_export.csv
echo "connection 接口是否已纳入（应 >=1）:"; grep -c "event/v1/connection" api_list_export.csv
```
Expected: `wc -l` = 1725（含表头）；connection 计数 ≥ 1。

- [ ] **Step 3: 本地对比验证（漂移已归零）**

Run:
```bash
# 用 compare 工具对刷新后的 CSV 自比，应无 added/removed/changed
python3 tools/compare_api_catalogs.py \
  --baseline api_list_export.csv \
  --current api_list_export.csv \
  --report /tmp/self-diff.md
cat /tmp/self-diff.md
```
Expected: 报告显示「未检测到飞书服务端 API 清单变化」（自比无差异）。

- [ ] **Step 4: Commit**

```bash
git add api_list_export.csv
git commit -m "chore: 刷新 api_list_export.csv 基准，吸收 #216 的 44 条目录元数据漂移

纳入新增 event/v1/connection 接口（数据行 1723 -> 1724）。
漂移均为飞书侧元数据标准化（fullPath 加 /document/ 前缀、url 加 METHOD: 前缀、
docPath 清空、bizTag/chargingMethod/isCharge 回填、OKR meta.Resource 加 okr. 前缀），
SDK 侧无需改动。"
```

---

## Task 6: 关闭 issue #216

**Files:** 无代码改动

- [ ] **Step 1: 推送分支并开 PR**

```bash
# 确认在特性分支上（如非，先建分支）
git checkout -b fix/issue-216-connection-endpoint
git push -u origin fix/issue-216-connection-endpoint

gh pr create \
  --title "feat(event): 获取长连接在线数量接口 + 刷新基准 CSV (fixes #216)" \
  --body "## 关闭 #216

- 实现 \`GET /open-apis/event/v1/connection\`（获取长连接在线数量）强类型接口
- 刷新 \`api_list_export.csv\` 吸收 44 条目录元数据漂移（纯飞书侧标准化，无 SDK 行为变化）
- 设计文档：\`docs/superpowers/specs/2026-06-24-issue-216-connection-endpoint-design.md\`

Closes #216"
```

- [ ] **Step 2: PR 合并后确认 issue 关闭**

Run: `gh issue view 216 --repo foxzool/openlark --json state`
Expected: `"state": "CLOSED"`（PR 标题含 `fixes #216`，合并后自动关闭）

---

## 自检（Self-Review）

- **Spec 覆盖**：✅ 工作流 A = Task 5；工作流 B = Task 1–3；测试 = Task 3 Step1 测试 + Task 4；DoD 每条均映射到 Task。
- **占位符扫描**：✅ 无 TBD/TODO；每个代码 step 含完整代码块；每条命令含 expected。
- **类型一致性**：✅ 全程使用 `GetConnectionOnlineCountResponse`（载荷类型，单 struct，仅 `online_instance_cnt`），与修正后的 spec 及 `Response<R>` 范式一致；`EVENT_V1_CONNECTION` 常量名贯穿 Task 1/3。
