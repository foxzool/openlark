---
change: type-okr-v2-responses
design-doc: docs/superpowers/specs/2026-07-04-type-okr-v2-responses-design.md
base-ref: 9fc91e3e2cdc3bdc8227ea3b45a233d9f062eb95
---

# type-okr-v2-responses Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 `openlark-hr` okr/v2 的 25 个叶子 `execute()` 返回类型从 `SDKResult<serde_json::Value>` 改为各自动态派生的 typed Response struct（从飞书 apiSchema 程序化派生，非手工转录 doc）。

**Architecture:** 每叶 inline 定义 typed Response struct + 嵌套 struct（对齐 okr v1 / attendance 模式），envelope `{code, msg}` 由现有 `ApiResponseTrait::ResponseFormat::Data` 提取，typed Response = `data` 的 shape。schema 源 = `tools/schema_cache/cache.py::get_or_fetch`（零鉴权 urllib 拉 `open.feishu.cn/document_portal/v1/document/get_detail`）。先批量预取 25 schema 落盘 + 生成字段清单，再以 `objective/get` 为试点确立模板，按 6 资源批次铺开，最后全量验证。

**Tech Stack:** Rust（serde Deserialize），openlark-core（`ApiRequest`/`ApiResponseTrait`/`ResponseFormat`/`Transport`），Python（schema_cache + 字段 dump，仅 Task 0 用）。

## Global Constraints

- **breaking 窗口**：v0.18 breaking（okr/v2 是零外部引用的导航终点，#327 已确认）
- **范围严控**：不动 `okr/okr/v2/mod.rs` 资源 accessor 链；不动端点 URL；不动其他 7 域；不动 okr/v1
- **端点构造保持原样**（D4）：`cycle/list.rs` 用 `OkrApiV2` enum，其余 24 叶用 inline `format!`/字面量，**各叶保持当前用法不统一**
- **请求 body 策略**（D3）：response typed = 硬目标；请求 body 单层简单 struct 一并 typed，嵌套复杂 body 保留 `serde_json::Value` + `// TODO: typed body` 注释
- **schema 源 = apiSchema**（D2）：每叶字段从 `apiSchema.responses["200"].content["application/json"].schema.properties → data.properties` 派生，非 doc 转录；`openlark-api-field-verify` 仅作辅助抽样核对
- **不引入新依赖**：沿用 `serde` / `openlark_core`
- **禁用 `unwrap()`/`expect()` 于库代码**
- **每批一个 commit + 增量 build/test**（D5 + tasks.md）
- **MSRV Rust 1.88+**；构建命令固定 `cargo build/test -p openlark-hr --all-features`

---

## File Structure

**改动文件清单（全部位于 `crates/openlark-hr/src/okr/okr/v2/`）—— 25 个叶子，每叶 inline 加 typed Response + 改 execute 返回类型：**

| 资源 | 叶子 | 文件 | HTTP | 当前 body | 当前 endpoint |
|------|------|------|------|-----------|---------------|
| alignment | get | `alignment/get.rs` | GET | — | inline `format!` |
| alignment | delete | `alignment/delete.rs` | DELETE | — | inline `format!` |
| category | list | `category/list.rs` | GET | — | inline 字面量 |
| cycle | list | `cycle/list.rs` | GET | — | **`OkrApiV2` enum**（保持） |
| cycle | objective/create | `cycle/objective/create.rs` | POST | Value | inline `format!` |
| cycle | objective/list | `cycle/objective/list.rs` | GET | — | inline `format!` |
| cycle | objectives_position | `cycle/objectives_position.rs` | PUT | Value | inline `format!` |
| cycle | objectives_weight | `cycle/objectives_weight.rs` | PUT | Value | inline `format!` |
| indicator | patch | `indicator/patch.rs` | PATCH | Value | inline `format!` |
| key_result | get | `key_result/get.rs` | GET | — | inline `format!` |
| key_result | delete | `key_result/delete.rs` | DELETE | — | inline `format!` |
| key_result | patch | `key_result/patch.rs` | PATCH | Value | inline `format!` |
| key_result | indicator/list | `key_result/indicator/list.rs` | GET | — | inline `format!` |
| key_result | progress/list | `key_result/progress/list.rs` | GET | — | inline `format!` |
| objective | get（试点） | `objective/get.rs` | GET | — | inline `format!` |
| objective | delete | `objective/delete.rs` | DELETE | — | inline `format!` |
| objective | patch | `objective/patch.rs` | PATCH | Value | inline `format!` |
| objective | alignment/create | `objective/alignment/create.rs` | POST | Value | inline `format!` |
| objective | alignment/list | `objective/alignment/list.rs` | GET | — | inline `format!` |
| objective | indicator/list | `objective/indicator/list.rs` | GET | — | inline `format!` |
| objective | key_result/create | `objective/key_result/create.rs` | POST | Value | inline `format!` |
| objective | key_result/list | `objective/key_result/list.rs` | GET | — | inline `format!` |
| objective | key_results_position | `objective/key_results_position.rs` | PUT | Value | inline `format!` |
| objective | key_results_weight | `objective/key_results_weight.rs` | PUT | Value | inline `format!` |
| objective | progress/list | `objective/progress/list.rs` | GET | — | inline `format!` |

**辅助产物（Task 0 生成，gitignore 不入库）：**
- `tools/schema_cache/.cache/<api_id>.json` × 25（持久 schema 缓存）
- `reports/okr-v2-fields.md`（25 叶 response.data 字段清单，实现期参考）

**不动文件：** `okr/okr/v2/mod.rs`、`okr/okr/v2/<resource>/mod.rs`、各叶 `path =` 构造行、各叶 `Request` struct 与 builder 方法。

**关键参考模式（不改，仅对照）：**
- typed 域 + 嵌套 struct + 拆 models.rs：`crates/openlark-hr/src/attendance/attendance/v1/group/{create.rs, models.rs}`
- typed 域 inline（无 models.rs，本 change 采用此模式）：`crates/openlark-hr/src/okr/okr/v1/progress_record/get.rs`
- 现状 Value 叶子模板：`crates/openlark-hr/src/okr/okr/v2/objective/get.rs`

---

## 通用转换模式（每叶共用，对照 Design Doc）

每叶按下面 4 步机械转换。**试点（Task 1）给完整代码；其余叶子的字段从 Task 0 dump 派生，字段类型规则统一如下：**

**字段类型规则（apiSchema type → Rust 类型）：**

| apiSchema type | required=true | required=false / 缺失 |
|----------------|---------------|----------------------|
| `string` | `String` | `Option<String>` + `#[serde(default)]` |
| `integer` | `i32`（或 `i64`，看 format/example） | `Option<i32>` + `#[serde(default)]` |
| `number` | `f64` | `Option<f64>` + `#[serde(default)]` |
| `boolean` | `bool` | `Option<bool>` + `#[serde(default)]` |
| `array` | `Vec<T>`（T 简单时直接 typed） | `Option<Vec<T>>` + `#[serde(default)]` |
| `object` 且子字段全部简单（depth≤2） | typed 嵌套 struct | `Option<TypedStruct>` + `#[serde(default)]` |
| `object` 且深度嵌套（document block tree 等） | `serde_json::Value` + `// TODO: 深嵌套结构暂留 Value` | `Option<serde_json::Value>` + `#[serde(default)]` |

**execute 签名规则：**
- GET/DELETE 叶：`pub async fn execute(self) -> SDKResult<XxxResponse>` + `execute_with_options(self, option) -> SDKResult<XxxResponse>`
- POST/PUT/PATCH 叶（10 个写操作）：**保留 body 参数为 `serde_json::Value`**（D3：嵌套复杂 body 不 typed），仅改返回类型为 `SDKResult<XxxResponse>`。body 参数签名不变，避免接口二次 breaking。

**imports 变更（每叶一致）：**
```rust
// 旧
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
// 新（加 ApiResponseTrait/ResponseFormat，加 serde::Deserialize）
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
```

**测试模板（每叶加一个反序列化测试，字段从 dump 取）：**
```rust
#[test]
fn test_xxx_response_deserialize() {
    let json = serde_json::json!({
        // 从 Task 0 dump 的 example 字段拷贝真实 shape
        "field_a": "value_a",
        "field_b": 1,
    });
    let resp: XxxResponse = serde_json::from_value(json).expect("反序列化失败");
    assert_eq!(resp.field_a, "value_a");
}
```

---

## Task 0: 批量预取 25 个 okr/v2 apiSchema + 生成字段清单

**Files:**
- Create: `tools/schema_cache/dump_okr_v2.py`（一次性脚本，gitignore 不入库）
- Create: `reports/okr-v2-fields.md`（实现期参考，gitignore 不入库）
- Cache: `tools/schema_cache/.cache/<api_id>.json` × 25（已 gitignore，objective/get 已存在）

**Interfaces:**
- Consumes: `tools/schema_cache/cache.py::get_or_fetch`、`tools/api_contracts/official.py::load_api_identities`、`api_list_export.csv`、`reports/missing_shards/grp/openlark-hr__okr.*__0.json`
- Produces: `.cache/<api_id>.json` × 25（供 Task 1-7 离线读 schema 派生字段）+ `reports/okr-v2-fields.md`（人类可读字段清单）

- [x] **Step 1: 编写 dump 脚本**

Create `tools/schema_cache/dump_okr_v2.py`:

```python
#!/usr/bin/env python3
"""一次性脚本：预取 okr/v2 全部 25 个 apiSchema 到 .cache/，并生成字段清单。

产物：
  tools/schema_cache/.cache/<api_id>.json   （schema 缓存，gitignore）
  reports/okr-v2-fields.md                  （25 叶字段清单，实现期参考，gitignore）

运行：python3 tools/schema_cache/dump_okr_v2.py
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.official import load_api_identities  # noqa: E402
from tools.schema_cache.cache import get_or_fetch, record_error  # noqa: E402

CSV_PATH = REPO_ROOT / "api_list_export.csv"
SHARD_GLOB = "reports/missing_shards/grp/openlark-hr__okr.*__0.json"
OUT_REPORT = REPO_ROOT / "reports" / "okr-v2-fields.md"


def _type_str(prop: dict) -> str:
    t = prop.get("type", "?")
    return f"{t}" if not prop.get("properties") else f"object({len(prop['properties'])} fields)"


def _dump_response_props(props: list, indent: int = 0) -> list[str]:
    lines = []
    pad = "  " * indent
    for p in props:
        name = p.get("name", "?")
        typ = _type_str(p)
        req = "required" if p.get("required") else "optional"
        desc = (p.get("description") or "").replace("\n", " ")[:80]
        lines.append(f"{pad}- `{name}` ({typ}, {req}) — {desc}")
        if p.get("properties") and indent < 2:
            lines.extend(_dump_response_props(p["properties"], indent + 1))
    return lines


def main() -> int:
    identities = {a.api_id: a for a in load_api_identities(CSV_PATH, skip_old_versions=False)}
    shards = sorted((REPO_ROOT).glob(SHARD_GLOB))
    v2_apis: list[dict] = []
    for s in shards:
        for a in json.loads(s.read_text(encoding="utf-8")):
            if a.get("version") == "v2":
                a["_identity"] = identities.get(a["api_id"])
                v2_apis.append(a)
    v2_apis.sort(key=lambda x: x["expected_file"])

    OUT_REPORT.parent.mkdir(parents=True, exist_ok=True)
    lines = ["# okr/v2 字段清单（25 叶，从 apiSchema 派生）\n"]
    cache_dir = REPO_ROOT / "tools" / "schema_cache" / ".cache"
    ok, fail = 0, 0

    for a in v2_apis:
        api = a["_identity"]
        if api is None:
            lines.append(f"\n## {a['expected_file']}\n\n[SKIP] api_id {a['api_id']} 不在 CSV\n")
            fail += 1
            continue
        lines.append(f"\n## {a['expected_file']}\n")
        lines.append(f"- api_id: `{a['api_id']}`\n- url: `{a['url']}`\n- name: {a['name']}\n")
        try:
            result = get_or_fetch(api, cache_dir=cache_dir)
            schema = result.api_schema
            resp200 = schema.get("responses", {}).get("200", {})
            content = resp200.get("content", {}).get("application/json", {})
            props = content.get("schema", {}).get("properties", [])
            data_prop = next((p for p in props if p.get("name") == "data"), None)
            lines.append(f"\n### response.data 字段（source={result.source}）\n")
            if data_prop and data_prop.get("properties"):
                lines.extend(_dump_response_props(data_prop["properties"]))
            else:
                lines.append("- (data 无子字段 / 空 data)\n")
            # 同步 dump requestBody（写操作参考）
            req_body = schema.get("requestBody")
            if req_body:
                lines.append("\n### requestBody（D3：单层可 typed，嵌套保留 Value）\n")
                lines.append(f"```json\n{json.dumps(req_body, ensure_ascii=False, indent=2)[:1500]}\n```\n")
            ok += 1
        except Exception as exc:
            record_error(cache_dir, api, exc)
            lines.append(f"\n[ERROR] {exc}\n")
            fail += 1

    OUT_REPORT.write_text("\n".join(lines), encoding="utf-8")
    print(f"完成 {ok}/{len(v2_apis)}（失败 {fail}），缓存目录 {cache_dir}，清单 {OUT_REPORT}")
    return 0 if fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
```

- [x] **Step 2: 运行脚本预取 25 schema**

Run:
```bash
cd /Users/zool/workspace/openlark && python3 tools/schema_cache/dump_okr_v2.py
```
Expected: `完成 25/25（失败 0）`，`tools/schema_cache/.cache/` 下 25 个 `<api_id>.json`，`reports/okr-v2-fields.md` 生成。

- [x] **Step 3: 校验字段清单可读**

Run:
```bash
wc -l /Users/zool/workspace/openlark/reports/okr-v2-fields.md
grep -c "^## " /Users/zool/workspace/openlark/reports/okr-v2-fields.md
```
Expected: 报告 ≥ 25 个 `## ` 段落（每叶一段）。

- [x] **Step 4: 确认两个产物均被 gitignore（不入库）**

Run:
```bash
cd /Users/zool/workspace/openlark && git check-ignore tools/schema_cache/.cache/7644764969658567628.json reports/okr-v2-fields.md tools/schema_cache/dump_okr_v2.py
```
Expected: 三行均输出（已被忽略）。`reports/` 和 `tools/schema_cache/.cache/`、`tools/schema_cache/dump_okr_v2.py` 若未被忽略，**不要**改 `.gitignore`（脚本是一次性的，删除即可；cache 已约定 gitignore；report 不入库）。如 `dump_okr_v2.py` 未被忽略，实现末尾删除该脚本即可。

- [x] **Step 5: 不 commit（产物均 gitignore）**

本 task 无源码变更，不产生 commit。`reports/okr-v2-fields.md` 供 Task 1-7 参考。

---

## Task 1: 试点 objective/get（确立 inline typed 模板）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/get.rs`

**Interfaces:**
- Consumes: Task 0 的 `.cache/7644764969658567628.json`（objective/get schema，已存在并已验证）
- Produces: `GetObjectiveResponse` + `Objective` + `ObjectiveOwner` struct（inline 于 get.rs），`impl ApiResponseTrait for GetObjectiveResponse`，`execute()/execute_with_options()` 返回 `SDKResult<GetObjectiveResponse>`。该文件成为 Task 2-7 的复制模板。

**字段映射（从 apiSchema data.objective.properties 派生）：**
- `id` (string, required) → `id: String`
- `create_time` (string, required) → `create_time: String`
- `update_time` (string, required) → `update_time: String`
- `owner` (object, required; 子字段 `owner_type` string required、`user_id` string optional) → typed `ObjectiveOwner`
- `cycle_id` (string, required) → `cycle_id: String`
- `position` (integer, required) → `position: i32`
- `content` (object, optional; 飞书文档 block 深度嵌套) → `Option<serde_json::Value>` + TODO 注释
- `score` (number, optional) → `Option<f64>`
- `notes` (object, optional; 飞书文档 block 深度嵌套) → `Option<serde_json::Value>` + TODO 注释
- `weight` (number, optional) → `Option<f64>`
- `deadline` (string, optional) → `Option<String>`
- `category_id` (string, optional) → `Option<String>`

- [x] **Step 1: 写失败测试（先加 typed Response 反序列化测试）**

Replace `crates/openlark-hr/src/okr/okr/v2/objective/get.rs` 的全部内容为：

```rust
//! 获取目标详细信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取目标详细信息请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    objective_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            objective_id: String::new(),
        }
    }

    /// 设置路径参数 `objective_id`。
    pub fn objective_id(mut self, val: impl Into<String>) -> Self {
        self.objective_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetObjectiveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetObjectiveResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = format!("/open-apis/okr/v2/objectives/{}", self.objective_id);
        let req: ApiRequest<GetObjectiveResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标详细信息", "响应数据为空")
        })
    }
}

/// 获取目标详细信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse {
    /// 目标详情。
    pub objective: Objective,
}

impl ApiResponseTrait for GetObjectiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// OKR 目标。
#[derive(Debug, Clone, Deserialize)]
pub struct Objective {
    /// 目标的 ID。
    pub id: String,
    /// 目标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 目标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: ObjectiveOwner,
    /// 目标的用户周期 ID。
    pub cycle_id: String,
    /// 目标的序号：从 1 开始计数。
    pub position: i32,
    /// 目标的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 目标的分数：[0,1]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的备注。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub notes: Option<serde_json::Value>,
    /// 目标的权重：[0,1]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 目标的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
    /// 目标的分类 ID。
    #[serde(default)]
    pub category_id: Option<String>,
}

/// 目标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectiveOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_get_objective_response_deserialize() {
        let json = serde_json::json!({
            "objective": {
                "id": "O-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "cycle_id": "C-1",
                "position": 1,
                "score": 0.8,
                "weight": 0.5,
                "deadline": "1700000000000",
                "category_id": "cat-1"
            }
        });
        let resp: GetObjectiveResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.objective.id, "O-123");
        assert_eq!(resp.objective.position, 1);
        assert_eq!(resp.objective.owner.owner_type, "user");
        assert_eq!(resp.objective.score, Some(0.8));
        assert!(resp.objective.content.is_none());
    }
}
```

- [x] **Step 2: 运行 build 验证编译通过**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -20
```
Expected: 编译通过，无错误（可能有未使用 import 警告，下个 step 消除）。

- [x] **Step 3: 运行测试验证通过**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo test -p openlark-hr --all-features objective::get 2>&1 | tail -15
```
Expected: `test result: ok. 2 passed`（`builder_initializes` + `test_get_objective_response_deserialize`）。

- [x] **Step 4: 全 workspace build 确认无外部破坏**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build --workspace --all-features 2>&1 | tail -10
```
Expected: 全 workspace 编译通过（okr/v2 是零外部引用，理论上无破坏）。

- [x] **Step 5: fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: fmt 无 diff，clippy 无 warning。

- [x] **Step 6: commit 试点**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/objective/get.rs && git commit -m "feat(hr/okr/v2): objective/get 返回 typed GetObjectiveResponse（试点）

确立 inline typed 模板：每叶 inline 定义 Response + 嵌套 struct，深度嵌套
文档结构保留 Value + TODO。剩余 24 叶按此模板铺开。

Refs: change type-okr-v2-responses Task 1"
```

---

## Task 2: alignment 批次（2 叶：get, delete）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs`（api_id `7644764969658305484`）
- Modify: `crates/openlark-hr/src/okr/okr/v2/alignment/delete.rs`（api_id `7644764969658420172`）

**Interfaces:**
- Consumes: Task 1 模板（`objective/get.rs`）；Task 0 dump（`reports/okr-v2-fields.md` 的 alignment/get 与 alignment/delete 段落）
- Produces: `GetAlignmentResponse`、`DeleteAlignmentResponse`（字段按 dump 派生），各叶 `impl ApiResponseTrait`

- [x] **Step 1: 派生 alignment/get 的 Response 字段**

Open `reports/okr-v2-fields.md`，定位 `## okr/okr/v2/okr/alignment/get.rs` 段落。按「通用转换模式」的字段类型规则，从 `response.data 字段` 列表派生 `GetAlignmentResponse` 及嵌套 struct（如有 `alignment` object 包一层则加 `Alignment` struct）。

将派生结果填入 `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs`，套用 Task 1 模板：
- imports 改为 `api::{ApiRequest, ApiResponseTrait, ResponseFormat}` + `use serde::Deserialize;`
- `execute()`/`execute_with_options()` 返回 `SDKResult<GetAlignmentResponse>`
- `let req: ApiRequest<GetAlignmentResponse> = ApiRequest::get(path);`（path 构造行**不动**）
- 加 `impl ApiResponseTrait for GetAlignmentResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }`
- 加 `test_get_alignment_response_deserialize` 测试（JSON fixture 从 dump 的 example 派生）

- [x] **Step 2: 派生 alignment/delete 的 Response 字段**

Open `reports/okr-v2-fields.md`，定位 `## okr/okr/v2/okr/alignment/delete.rs` 段落。DELETE 操作的 response.data 通常为空 object `{}` 或仅含删除标记；若 dump 显示 data 无字段，定义空结构体：

```rust
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteAlignmentResponse {}
impl ApiResponseTrait for DeleteAlignmentResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}
```

按 Task 1 模板修改 `alignment/delete.rs`：返回类型 `SDKResult<DeleteAlignmentResponse>`，path 构造行不动。

- [x] **Step 3: build + test alignment**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -10 && cargo test -p openlark-hr --all-features alignment 2>&1 | tail -10
```
Expected: build 通过，alignment 相关 test 全部 PASS。

- [x] **Step 4: fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 无 diff、无 warning。

- [x] **Step 5: commit alignment 批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/alignment/ && git commit -m "feat(hr/okr/v2): alignment get/delete 返回 typed Response

Refs: change type-okr-v2-responses Task 2"
```

---

## Task 3: category 批次（1 叶：list）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/category/list.rs`（api_id `7644764969658469324`，当前 endpoint 用 inline 字面量 `"/open-apis/okr/v2/categories"` —— 保持不动）

**Interfaces:**
- Consumes: Task 0 dump `## okr/okr/v2/okr/category/list.rs` 段落
- Produces: `ListCategoryResponse`（字段按 dump 派生；预期含 `categories: Vec<Category>` 列表字段）

- [x] **Step 1: 派生 + 改 list.rs**

Open `reports/okr-v2-fields.md`，定位 `## okr/okr/v2/okr/category/list.rs` 段落。按通用规则派生 `ListCategoryResponse` + 嵌套 `Category` struct。

修改 `category/list.rs` 套用 Task 1 模板。**注意 endpoint 用字面量（不用 `format!`），保持原样不动**：
```rust
let req: ApiRequest<ListCategoryResponse> = ApiRequest::get("/open-apis/okr/v2/categories");
```

加 `test_list_category_response_deserialize` 测试（list 响应 fixture 含数组字段）。

- [x] **Step 2: build + test + fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -5 && cargo test -p openlark-hr --all-features category 2>&1 | tail -5 && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 全部 PASS，无 warning。

- [x] **Step 3: commit category 批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/category/list.rs && git commit -m "feat(hr/okr/v2): category/list 返回 typed ListCategoryResponse

Refs: change type-okr-v2-responses Task 3"
```

---

## Task 4: cycle 批次（5 叶：list, objective/create, objective/list, objectives_position, objectives_weight）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/list.rs`（api_id `7644863390543989724`，**当前用 `OkrApiV2` enum —— 保持不动**）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objective/create.rs`（api_id `7644863390543973340`，POST 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs`（api_id `7644764969658272716`）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs`（api_id `7644764969658321868`，PUT 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs`（api_id `7644764969658338252`，PUT 写操作）

**Interfaces:**
- Consumes: Task 0 dump 上述 5 段落；Task 1 模板
- Produces: 5 个 typed Response（`ListCycleResponse` / `CreateCycleObjectiveResponse` / `ListCycleObjectiveResponse` / `UpdateCycleObjectivesPositionResponse` / `UpdateCycleObjectivesWeightResponse`）

**关键约束：**
- `cycle/list.rs` 的 endpoint `OkrApiV2::Cycles`（或类似 enum 变体）**保持不动**，只把 `ApiRequest::<serde_json::Value>::get(...)` 改为 `ApiRequest::<ListCycleResponse>::get(...)`，enum 调用方式不变（D4）
- 3 个写操作叶（create / objectives_position / objectives_weight）：`execute(body: serde_json::Value)` 签名**保留 Value**（D3），仅改返回类型

- [x] **Step 1: 派生 cycle/list（GET，enum endpoint）**

Open dump `## okr/okr/v2/okr/cycle/list.rs`。派生 `ListCycleResponse` + 嵌套 `Cycle` struct（若有 `cycles: Vec<Cycle>` 字段）。

修改 `cycle/list.rs`：
- imports 加 `ApiResponseTrait, ResponseFormat` + `Deserialize`
- `ApiRequest::<serde_json::Value>::get(...)` → `ApiRequest::<ListCycleResponse>::get(...)`，**enum 用法不动**
- 返回类型改 `SDKResult<ListCycleResponse>`
- 加 `impl ApiResponseTrait` + 反序列化测试

- [x] **Step 2: 派生 cycle/objective/list（GET）**

Open dump `## okr/okr/v2/okr/cycle/objective/list.rs`。派生 `ListCycleObjectiveResponse`（预期含 `objectives: Vec<...>`）。修改 `cycle/objective/list.rs` 套用模板，path 构造不动。

- [x] **Step 3: 派生 cycle/objective/create（POST，body 保留 Value）**

Open dump `## okr/okr/v2/okr/cycle/objective/create.rs`。派生 `CreateCycleObjectiveResponse`。

修改 `cycle/objective/create.rs`：
- **`execute(self, body: serde_json::Value)` 签名不动**（D3：POST body 多含 objective 嵌套，留 Value + 不加 TODO，因为这是入参而非响应）
- 返回类型改 `SDKResult<CreateCycleObjectiveResponse>`
- `ApiRequest::<serde_json::Value>::post(path)` → `ApiRequest::<CreateCycleObjectiveResponse>::post(path)`
- 加 `impl ApiResponseTrait` + 反序列化测试

- [x] **Step 4: 派生 cycle/objectives_position（PUT，body 保留 Value）**

Open dump `## okr/okr/v2/okr/cycle/objectives_position.rs`。派生 `UpdateCycleObjectivesPositionResponse`（PUT 更新操作响应通常为空 data 或简单确认；若 dump 显示 data 空，用空 struct + `Default`，参 Task 2 Step 2 模式）。

修改 `cycle/objectives_position.rs`：body 签名保留 Value，返回类型改 typed，加 impl + 测试。

- [x] **Step 5: 派生 cycle/objectives_weight（PUT，body 保留 Value）**

同 Step 4 模式处理 `cycle/objectives_weight.rs`，派生 `UpdateCycleObjectivesWeightResponse`。

- [x] **Step 6: build + test cycle 批次**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -10 && cargo test -p openlark-hr --all-features cycle 2>&1 | tail -10
```
Expected: 5 叶全部 build 通过、test PASS。

- [x] **Step 7: fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 无 diff、无 warning。

- [x] **Step 8: commit cycle 批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/cycle/ && git commit -m "feat(hr/okr/v2): cycle 5 叶返回 typed Response（list/objective_create/objective_list/objectives_position/objectives_weight）

写操作 body 保留 Value（D3）；cycle/list 的 OkrApiV2 enum 保持不动（D4）。

Refs: change type-okr-v2-responses Task 4"
```

---

## Task 5: indicator 批次（1 叶：patch，写操作含 body）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs`（api_id `7644764969658207180`，PATCH）

**Interfaces:**
- Consumes: Task 0 dump `## okr/okr/v2/okr/indicator/patch.rs` 段落
- Produces: `PatchIndicatorResponse`

- [x] **Step 1: 派生 + 改 indicator/patch.rs**

Open dump `## okr/okr/v2/okr/indicator/patch.rs`。派生 `PatchIndicatorResponse`（PATCH 响应通常为空 data 或回显 indicator；按 dump 实际 shape）。

修改 `indicator/patch.rs`：
- `execute(self, body: serde_json::Value)` 签名**保留**（D3）
- 返回类型改 `SDKResult<PatchIndicatorResponse>`
- imports 加 `ApiResponseTrait, ResponseFormat` + `Deserialize`
- `ApiRequest::<serde_json::Value>::patch(path)` → `ApiRequest::<PatchIndicatorResponse>::patch(path)`
- 加 `impl ApiResponseTrait for PatchIndicatorResponse` + 反序列化测试
- path 构造 `format!("/open-apis/okr/v2/indicators/{}", self.indicator_id)` 不动

- [x] **Step 2: build + test + fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -5 && cargo test -p openlark-hr --all-features indicator 2>&1 | tail -5 && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 全部 PASS，无 warning。

- [x] **Step 3: commit indicator 批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs && git commit -m "feat(hr/okr/v2): indicator/patch 返回 typed PatchIndicatorResponse

Refs: change type-okr-v2-responses Task 5"
```

---

## Task 6: key_result 批次（5 叶：get, delete, patch, indicator/list, progress/list）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/get.rs`（api_id `7644764969658534860`）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/delete.rs`（api_id `7644764969658551244`）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs`（api_id `7644764969658256332`，PATCH 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs`（api_id `7644764969658403788`）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/progress/list.rs`（api_id `7644764969658371020`）

**Interfaces:**
- Consumes: Task 0 dump 上述 5 段落；Task 1 模板
- Produces: `GetKeyResultResponse` / `DeleteKeyResultResponse` / `PatchKeyResultResponse` / `ListKeyResultIndicatorResponse` / `ListKeyResultProgressResponse`

- [x] **Step 1: 派生 key_result/get（GET）**

Open dump，派生 `GetKeyResultResponse` + 嵌套 `KeyResult` struct（预期字段：id / kr_id / objective_id / content / score / weight / progress 等，深度嵌套 content/notes 按 Task 1 模式留 Value + TODO）。

修改 `key_result/get.rs` 套用模板，path `format!("/open-apis/okr/v2/key_results/{}", self.key_result_id)` 不动。

- [x] **Step 2: 派生 key_result/delete（DELETE）**

Open dump，派生 `DeleteKeyResultResponse`（DELETE 响应通常空 data，用空 struct + Default，参 Task 2 Step 2）。修改 `key_result/delete.rs` 套用模板。

- [x] **Step 3: 派生 key_result/patch（PATCH，body 保留 Value）**

Open dump，派生 `PatchKeyResultResponse`。修改 `key_result/patch.rs`：`execute(body: serde_json::Value)` 签名保留（D3），返回类型改 typed。

- [x] **Step 4: 派生 key_result/indicator/list（GET）**

Open dump，派生 `ListKeyResultIndicatorResponse`（预期含 `indicators: Vec<...>`）。修改 `key_result/indicator/list.rs` 套用模板。

- [x] **Step 5: 派生 key_result/progress/list（GET）**

Open dump，派生 `ListKeyResultProgressResponse`（预期含 `progresses: Vec<...>`）。修改 `key_result/progress/list.rs` 套用模板。

- [x] **Step 6: build + test key_result 批次**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -10 && cargo test -p openlark-hr --all-features key_result 2>&1 | tail -10
```
Expected: 5 叶全部 build 通过、test PASS。

- [x] **Step 7: fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 无 diff、无 warning。

- [x] **Step 8: commit key_result 批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/key_result/ && git commit -m "feat(hr/okr/v2): key_result 5 叶返回 typed Response（get/delete/patch/indicator_list/progress_list）

Refs: change type-okr-v2-responses Task 6"
```

---

## Task 7: objective 剩余 10 叶

**Files（objective/get 已在 Task 1 完成，本 task 处理剩余 10 个）：**
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/delete.rs`（api_id `7644764969658452940`，DELETE）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/patch.rs`（api_id `7644764969658436556`，PATCH 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/alignment/create.rs`（api_id `7644764969658289100`，POST 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs`（api_id `7644764969658223564`，GET）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs`（api_id `7644764969658387404`，GET）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_result/create.rs`（api_id `7644764969658239948`，POST 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_result/list.rs`（api_id `7644764969658518476`，GET）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_position.rs`（api_id `7644764969658485708`，PUT 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_weight.rs`（api_id `7644764969658502092`，PUT 写操作）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/progress/list.rs`（api_id `7644764969658354636`，GET）

**Interfaces:**
- Consumes: Task 0 dump 上述 10 段落；Task 1 模板（同目录的 `objective/get.rs`）
- Produces: 10 个 typed Response

**关键约束：**
- 4 个写操作叶（patch / alignment/create / key_result/create / key_results_position / key_results_weight —— 实际 5 个写）：`execute(body: serde_json::Value)` 签名保留（D3）
- 端点构造全部为 inline `format!`，保持不动（D4）

- [ ] **Step 1: 派生 objective/delete（DELETE）**

Open dump `## okr/okr/v2/okr/objective/delete.rs`。派生 `DeleteObjectiveResponse`（DELETE 响应通常空 data，用空 struct + Default）。修改 `objective/delete.rs` 套用 Task 1 模板，path 不动。

- [ ] **Step 2: 派生 objective/patch（PATCH，body 保留 Value）**

Open dump `## okr/okr/v2/okr/objective/patch.rs`。派生 `PatchObjectiveResponse`（PATCH 响应通常回显 objective 或空 data）。

修改 `objective/patch.rs`：
- `execute(self, body: serde_json::Value)` 签名**保留**（D3，objective body 含嵌套 content，留 Value）
- 返回类型改 `SDKResult<PatchObjectiveResponse>`
- imports 加 `ApiResponseTrait, ResponseFormat` + `Deserialize`
- `ApiRequest::<serde_json::Value>::patch(path)` → `ApiRequest::<PatchObjectiveResponse>::patch(path)`
- 加 `impl ApiResponseTrait` + 反序列化测试

- [ ] **Step 3: 派生 objective/alignment/list（GET）**

Open dump，派生 `ListObjectiveAlignmentResponse`（预期含 `alignments: Vec<Alignment>`）。修改 `objective/alignment/list.rs` 套用模板。

- [ ] **Step 4: 派生 objective/alignment/create（POST，body 保留 Value）**

Open dump，派生 `CreateObjectiveAlignmentResponse`。修改 `objective/alignment/create.rs`：body 签名保留，返回类型改 typed。

- [ ] **Step 5: 派生 objective/indicator/list（GET）**

Open dump，派生 `ListObjectiveIndicatorResponse`（预期含 `indicators: Vec<Indicator>`）。修改 `objective/indicator/list.rs` 套用模板。

- [ ] **Step 6: 派生 objective/key_result/list（GET）**

Open dump，派生 `ListObjectiveKeyResultResponse`（预期含 `key_results: Vec<KeyResult>`）。修改 `objective/key_result/list.rs` 套用模板。**注意命名避免与 key_result/get.rs 的 `KeyResult` 冲突**：本叶用模块路径区分（`super::super::key_result::get::KeyResult` 复用）或定义本模块专属 `ObjectiveKeyResult`，按 dump 字段差异决定（若字段一致优先复用，否则定义本叶专属 struct）。

- [ ] **Step 7: 派生 objective/key_result/create（POST，body 保留 Value）**

Open dump，派生 `CreateObjectiveKeyResultResponse`。修改 `objective/key_result/create.rs`：body 签名保留，返回类型改 typed。

- [ ] **Step 8: 派生 objective/key_results_position（PUT，body 保留 Value）**

Open dump，派生 `UpdateObjectiveKeyResultsPositionResponse`（PUT 响应通常空 data）。修改 `objective/key_results_position.rs`：body 签名保留，返回类型改 typed。

- [ ] **Step 9: 派生 objective/key_results_weight（PUT，body 保留 Value）**

同 Step 8 模式处理 `objective/key_results_weight.rs`，派生 `UpdateObjectiveKeyResultsWeightResponse`。

- [ ] **Step 10: 派生 objective/progress/list（GET）**

Open dump，派生 `ListObjectiveProgressResponse`（预期含 `progresses: Vec<...>`）。修改 `objective/progress/list.rs` 套用模板。

- [ ] **Step 11: build + test objective 批次**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -10 && cargo test -p openlark-hr --all-features objective 2>&1 | tail -10
```
Expected: 10 叶（含 Task 1 已做的 get）全部 build 通过、test PASS。

- [ ] **Step 12: fmt + clippy**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt -p openlark-hr --check && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -5
```
Expected: 无 diff、无 warning。

- [ ] **Step 13: commit objective 剩余批次**

```bash
cd /Users/zool/workspace/openlark && git add crates/openlark-hr/src/okr/okr/v2/objective/ && git commit -m "feat(hr/okr/v2): objective 剩余 10 叶返回 typed Response（delete/patch/alignment_*/indicator_list/key_result_*/key_results_position/key_results_weight/progress_list）

Refs: change type-okr-v2-responses Task 7"
```

---

## Task 8: 最终验证（issue 验收）

**Files:** 无源码改动；仅运行验证命令 + 必要修复。

**Interfaces:**
- Consumes: Task 1-7 全部完成（25 叶已 typed）
- Produces: 验证通过证据（verify 阶段引用）

- [ ] **Step 1: 全 feature build**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build -p openlark-hr --all-features 2>&1 | tail -5
```
Expected: 编译通过。

- [ ] **Step 2: 全 feature test**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo test -p openlark-hr --all-features 2>&1 | tail -15
```
Expected: 所有现有 test + 25 个新反序列化 test 全部 PASS，无回归。

- [ ] **Step 3: cargo doc（验证 markdown / 链接）**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo doc -p openlark-hr --all-features --no-deps 2>&1 | tail -10
```
Expected: 无 bare_url / broken markdown 警告（参考 MEMORY：cargo doc 盲点 = 须人工抽样 doc 渲染）。

- [ ] **Step 4: fmt --check（CI lint 第一步，必跑）**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo fmt --check 2>&1 | tail -5
```
Expected: 无 diff（参考 MEMORY：clippy 通过 ≠ fmt 通过）。

- [ ] **Step 5: clippy（全 feature + 全 targets，含 --no-default-features 隐含覆盖）**

Run:
```bash
cd /Users/zoul/workspace/openlark && cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: 无 warning。

- [ ] **Step 6: grep 确认 25 叶无 Value 残留**

Run:
```bash
cd /Users/zool/workspace/openlark && grep -rn "SDKResult<serde_json::Value>" crates/openlark-hr/src/okr/okr/v2/ --include="*.rs"
```
Expected: **零输出**（25 叶全部 typed）。

补充检查 execute 返回类型全 typed：
```bash
grep -rn "pub async fn execute" crates/openlark-hr/src/okr/okr/v2/ --include="*.rs" | grep -v "SDKResult<" | head
```
Expected: 所有 `pub async fn execute` 均返回 `SDKResult<...Response>`（无 `serde_json::Value`）。

- [ ] **Step 7: git diff 确认导航链与端点零改动**

Run:
```bash
cd /Users/zool/workspace/openlark && git diff 9fc91e3e2cdc3bdc8227ea3b45a233d9f062eb95 -- crates/openlark-hr/src/okr/okr/v2/mod.rs
```
Expected: **空 diff**（v2/mod.rs 资源 accessor 链零改动）。

Run:
```bash
cd /Users/zool/workspace/openlark && git diff 9fc91e3e2cdc3bdc8227ea3b45a233d9f062eb95 -- crates/openlark-hr/src/okr/okr/v2/ | grep -E "^\+.*let path|^\-.*let path|^\+.*ApiRequest::(get|post|put|patch|delete)\(" | head -30
```
Expected: 每叶 endpoint 构造行**仅在 ApiRequest 泛型参数上变化**（`ApiRequest::<serde_json::Value>::...` → `ApiRequest::<XxxResponse>::...`），`let path = format!(...)` 行与 `ApiRequest::get/post/...(path)` 调用本身**不变**。若出现 path 改动则须回退（违反 D4 / Non-Goals）。

- [ ] **Step 8: 跨 crate 回归**

Run:
```bash
cd /Users/zool/workspace/openlark && cargo build --workspace --all-features 2>&1 | tail -10
```
Expected: 全 workspace 编译通过（okr/v2 零外部引用，理论上无破坏）。

补充 grep 确认无外部消费 Value：
```bash
grep -rn "okr::v2" crates/ --include="*.rs" | grep -v "crates/openlark-hr/src/okr/" | head
```
Expected: 仅 openlark-hr 内部引用（外部 crate 不直接消费 okr/v2 typed Response）。

- [ ] **Step 9: openlark-api-field-verify 抽样核对（≥3 叶跨资源）**

调用 `openlark-api-field-verify` skill，对以下 3 叶抽样核对 typed Response 字段与飞书 doc 一致：
- `objective/get`（Task 1 试点）
- `cycle/list`（cycle 代表）
- `key_result/get`（key_result 代表）

每叶：playwright 渲染飞书 doc（docPath 已在文件头注释）→ 对比代码字段名/类型/可选性 → 记录差异。若发现字段不一致，回到对应 Task 修正。

Expected: 字段一致（apiSchema 与 doc 同源，理论上零差异；若有则是 apiSchema 与 doc 渲染差异，按 codegen 同源原则以 apiSchema 为准并记录）。

- [ ] **Step 10: 清理一次性脚本（若未被 gitignore）**

Run:
```bash
cd /Users/zool/workspace/openlark && git status --short tools/schema_cache/dump_okr_v2.py reports/okr-v2-fields.md 2>&1
```
若显示 untracked，删除一次性产物：
```bash
rm -f /Users/zool/workspace/openlark/tools/schema_cache/dump_okr_v2.py /Users/zool/workspace/openlark/reports/okr-v2-fields.md
```
（cache `.cache/*.json` 已 gitignore，保留供后续 codegen 复用。）

- [ ] **Step 11: 最终 commit（如有清理）**

```bash
cd /Users/zool/workspace/openlark && git status --short
```
若无源码变更则跳过；若有清理产生的删除，commit：
```bash
git commit -am "chore(hr/okr/v2): 清理 type-okr-v2-responses 一次性脚本

Refs: change type-okr-v2-responses Task 8"
```

---

## Self-Review 备注

**Spec coverage：**
- Design Doc D1（inline + 嵌套 struct）→ Task 1 模板确立，Task 2-7 复用 ✓
- D2（schema 源 = apiSchema）→ Task 0 程序化预取 + dump，Task 1-7 从 dump 派生 ✓
- D3（response typed 硬目标，body 视成本）→ 写操作 5 叶 body 保留 Value 明确写入步骤 ✓
- D4（端点 enum 不动）→ cycle/list 保持 OkrApiV2，Task 4/8 grep 验证 path 零改动 ✓
- D5（6 资源批次）→ Task 2-7 分 6 批 + 增量 build/test/commit ✓
- tasks.md §1 试点 → Task 1 ✓
- tasks.md §2 5 资源批次（注：tasks.md 写"5 资源批次"实为 6 批含 alignment，本计划按 Design D5 6 批对齐）→ Task 2-7 ✓
- tasks.md §3 验证 7 项 → Task 8 Step 1-9 逐项对应 ✓

**Type consistency：**
- `GetObjectiveResponse` / `Objective` / `ObjectiveOwner`（Task 1）—— Task 7 Step 6 明确处理 objective/key_result/list 与 key_result/get 的 `KeyResult` 命名冲突
- 所有 `impl ApiResponseTrait::data_format() -> ResponseFormat::Data` 一致
- imports 变更规则统一（Task 1 模板）

**Placeholder scan：** Task 2-7 的"派生字段"步骤依赖 Task 0 dump 真实数据（非 TBD），属可执行机械步骤；提供字段类型规则表 + 测试模板（非"add appropriate tests"空话）。
