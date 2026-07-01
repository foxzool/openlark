---
change: fix-analytics-missing-docs
design-doc: docs/superpowers/specs/2026-07-01-fix-analytics-missing-docs-design.md
base-ref: ab61f9c82b2f9dfe92d220112d4aec8665ee3174
---

# fix-analytics-missing-docs 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实施。步骤用 `- [ ]` 复选框跟踪。

**Goal:** 移除 `openlark-analytics` 的 crate 级 `#![allow(missing_docs)]`，回补其压制的 122 个未文档化公开项，并把 3 个 missing_docs 验证测试接进 CI。

**Architecture:** 纯增量、非破坏性。122 项不是 122 个独立创作，而是 ~18 个叶子 API 文件 × 同一 recipe。每文件从自己的 `//!` 标题取 API 名，doc 派生为「<标题>请求。 / <标题>响应。 / 创建新的请求构建器。 / 执行<标题>请求。」等。先 pilot 1 个文件验证 recipe，再按域 4 组批量回补，最后移除 allow + CI 接线 + 全局验证。

**Tech Stack:** Rust（doc comment `///`、`//!`）、cargo doc / clippy、Python unittest、GitHub Actions。

## 事实源 / 关键引用

- **Design Doc（事实源）**：`docs/superpowers/specs/2026-07-01-fix-analytics-missing-docs-design.md` §3 recipe、§4 执行计划、§5 决策、§6 测试策略。
- **tasks.md**：`openspec/changes/fix-analytics-missing-docs/tasks.md`（6 组 15 任务）。
- **workspace doc 基准**：`crates/openlark-communication/src/contact/contact/old/default/v2/task/get.rs`（每项一行有意义中文 + 文件级 `//!` + docPath）。
- **工作样例**：`crates/openlark-analytics/src/search/search/v2/schema/create.rs`（Design Doc §3 已给出回补后的完整样例）。

## 全局约束（每个任务隐式遵守）

- **Recipe（D1）**——每个公开项一行 `///` 中文，模板按下表；标题取自该文件 `//!` 第一行：

  | item 类型 | doc 模板 |
  |-----------|---------|
  | Request struct | `/// <标题>请求。` |
  | Response struct | `/// <标题>响应。` |
  | Response 命名字段 | `/// <字段含义>。`（`data` → `/// 响应数据。`） |
  | API 容器 struct（如 `QueryApi`、`UserSearchApi`） | `/// <标题> API。` |
  | builder struct（如 `SearchRequest`、`SuggestRequest`） | `/// <标题>请求构建器。` |
  | `pub fn new`（关联函数） | `/// 创建新的请求构建器。` |
  | `pub async fn execute` | `/// 执行<标题>请求。` |
  | `pub async fn execute_with_options` | `/// 使用指定请求选项执行<标题>请求。` |
  | builder 方法（如 `query`、`page_size`、`search_term`） | `/// <动作>。`（`query` → `/// 设置查询关键字。`） |
  | API 容器方法（如 `search()`、`suggest()`） | `/// 创建<动作>请求构建器。` |

- **docPath 位置（决策 A）**：只留文件级 `//!` 第二行的 docPath；**不在** `execute` 方法 doc 里重复 docPath。
- **trait impl（决策 D）**：`impl ApiResponseTrait for ...` 及其 `data_format()` 方法**不加 doc**（doc 从 trait 继承，加了反而触发 `redundant_doc`）。
- **禁止占位符（决策 E / D2）**：不得出现 `/// 待补充文档。` / `/// 公开项说明。` / `/// TODO` 等。每条 doc 必须引用该文件真实 API 名（标题）。
- **不动 API 签名/行为**：只新增 `///` 注释、移除 1 行 allow、改 CI yml；不改 struct 字段类型、不改方法体逻辑。
- **保留 line 34**：`#![allow(clippy::module_inception)]`（lib.rs:34）不在范围，只移除 line 35 的 `#![allow(missing_docs)]`。
- **每任务结束提交**：commit message 中文，带 `(#fix-analytics-missing-docs)` 标记。

## 文件清单（Create / Modify）

回补 doc 的 18 个叶子 API 文件（按 Design Doc §4 分 4 组）：

| 组 | 文件（路径相对 `crates/openlark-analytics/src/`） | 数量 |
|----|------|------|
| Group A data_source | `search/search/v2/data_source/{create,get,patch,delete,list}.rs` + `search/search/v2/data_source/item/{create,get,delete}.rs` | 8 |
| Group B schema（pilot 后） | `search/search/v2/schema/{create,get,patch,delete}.rs` | 4 |
| Group C search-rest | `search/search/v2/{app/create,message/create,doc_wiki/search,user,query}.rs` | 5 |
| Group D report | `report/report/v1/rule/query.rs`、`report/report/v1/rule/view/remove.rs`、`report/report/v1/task/query.rs` | 3 |

其他修改：
- Modify: `crates/openlark-analytics/src/lib.rs:35`（移除 allow）——Task 9
- Modify: `.github/workflows/ci.yml:111-114`（CI 接线）——Task 10

---

## Phase 0 — 基线锁定（主会话）

### Task 1: 记录移除 allow 前的 missing_docs 基线

**Files:**
- 验证: `crates/openlark-analytics/src/lib.rs:35`（确认 `#![allow(missing_docs)]` 仍在）
- 输出: 本任务步骤 2 把数字记入 commit message

**Interfaces:** 无（只读基线）

- [x] **Step 1: 确认 allow 仍在**

Run: `grep -n '#!\[allow(missing_docs)\]' crates/openlark-analytics/src/lib.rs`
Expected: `35:#![allow(missing_docs)]`（line 35，line 34 是 module_inception）

- [x] **Step 2: 跑基线 cargo doc，记录被压制的警告数**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep -c 'warning: missing documentation'
```
Expected: `0`（因为 allow 仍压制）。这一步确认 allow 在压制——真正计数靠 Step 3。

- [x] **Step 3: 临时移除 allow 计数真实基线**

编辑 `crates/openlark-analytics/src/lib.rs`：临时删除 line 35 `#![allow(missing_docs)]`，然后：
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'warning: missing documentation' | wc -l
```
Expected: `122`（Design Doc §1 基线：68 struct / 36 method / 18 associated fn）。
记录分布：
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'warning: missing documentation for' | sed -E 's/.*for (an? )?//' | sort | uniq -c
```
Expected: 输出按 struct / fn / associated function 分组，总数 122。

- [x] **Step 4: 还原 allow（基线采集完，恢复压制状态进入 Phase 1）**

```bash
git checkout -- crates/openlark-analytics/src/lib.rs
```
Run: `grep -n '#!\[allow(missing_docs)\]' crates/openlark-analytics/src/lib.rs`
Expected: `35:#![allow(missing_docs)]`（已还原）。

> **注：** 本 task 不 commit（只是采集基线，工作树还原）。

### Task 2: 确认文件级 `//!` 覆盖 + 标记 doc_wiki 空 docPath

**Files:**
- 验证: 18 个叶子文件（见「文件清单」）
- 标记: `crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs:2`（空 docPath，Group C 处理）

- [x] **Step 1: 确认 18 个叶子文件都有文件级 `//!` 标题**

Run:
```bash
for f in crates/openlark-analytics/src/search/search/v2/{data_source/create,data_source/get,data_source/patch,data_source/delete,data_source/list,data_source/item/create,data_source/item/get,data_source/item/delete,schema/create,schema/get,schema/patch,schema/delete,app/create,message/create,doc_wiki/search,user,query}.rs crates/openlark-analytics/src/report/report/v1/{rule/query,rule/view/remove,task/query}.rs; do
  printf '%-90s ' "$(basename $(dirname $f))/$(basename $f)"; head -1 "$f"
done
```
Expected: 每行输出形如 `schema/create.rs //! 创建数据范式`（每个文件第一行都是 `//!` 标题，标题即 API 名）。

- [x] **Step 2: 找出空 docPath（应为 doc_wiki/search.rs:2）**

Run:
```bash
grep -rn '^//! docPath: *$' crates/openlark-analytics/src/ || grep -rn '^//! docPath:<' crates/openlark-analytics/src/
```
Expected: 命中 `search/search/v2/doc_wiki/search.rs:2`（`//! docPath:` 后为空）。记下，Task 7（Group C）会补全。

- [x] **Step 3: 确认 doc_wiki/search.rs 的真实 docPath URL**

查 `crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs:1` 的 `//!` 标题（应为「搜索文档」）。对应 Feishu docPath：`https://open.feishu.cn/document/server-docs/docs/search-docs`（与同 crate 其他 search API 同域）。
> **若不确定 URL**：在该文件 step 用 `//! docPath: <待 Group C 确认>` 占位标记，并在 Task 7 步骤里写明「以 Feishu 文档实际 URL 为准」。但**最终 doc 不得为空**。

> 本 task 不 commit（只读勘探）。

---

## Phase 1 — Pilot（主会话）：验证 recipe

### Task 3: 回补 `search/v2/schema/create.rs`（pilot）

**Files:**
- Modify: `crates/openlark-analytics/src/search/search/v2/schema/create.rs`

**Interfaces:**
- Produces: recipe 的「锚定样例」。Group A/B/C/D 全部参照本 task 的产出格式。

- [x] **Step 1: 读取文件，确认 `//!` 标题与公开项**

Run: `head -2 crates/openlark-analytics/src/search/search/v2/schema/create.rs`
Expected:
```
//! 创建数据范式
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/create>
```
标题 = 「创建数据范式」。公开项（已确认）：`CreateSchemaRequest`(struct) / `CreateSchemaResponse`(struct + `data` field) / `impl CreateSchemaRequest` 的 `new`/`execute`/`execute_with_options`。`impl ApiResponseTrait` 不加 doc。

- [x] **Step 2: 给 `CreateSchemaRequest` 加 doc**

在 `pub struct CreateSchemaRequest {` 上一行插入：
```rust
/// 创建数据范式请求。
```

- [x] **Step 3: 给 `CreateSchemaResponse` 及其字段加 doc**

在 `pub struct CreateSchemaResponse {` 上一行：
```rust
/// 创建数据范式响应。
```
在 `pub data: Option<serde_json::Value>,` 上一行：
```rust
    /// 响应数据。
```

- [x] **Step 4: 给 `impl CreateSchemaRequest` 的 3 个方法加 doc**

- `pub fn new` 上一行：`    /// 创建新的请求构建器。`
- `pub async fn execute` 上一行：`    /// 执行创建数据范式请求。`
- `pub async fn execute_with_options` 上一行：`    /// 使用指定请求选项执行创建数据范式请求。`

**不要**给 `impl ApiResponseTrait for CreateSchemaResponse` 或 `fn data_format()` 加 doc。

- [x] **Step 5: 单文件自验——cargo doc grep 该文件无 warning**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'schema/create.rs' | grep 'warning: missing documentation'
```
Expected: 空输出（该文件的 6 个原 warning 已清零）。allow 仍压制其他文件，所以**总数**从 122 降到 116 是正常的，无需检查总数。

- [x] **Step 6: 占位符守门（D2）**

Run: `grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/schema/create.rs`
Expected: 空输出。

- [x] **Step 7: fmt 自查**

Run: `cargo fmt -p openlark-analytics -- --check`
Expected: 无 diff（只加了注释，不影响格式）。

- [x] **Step 8: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/schema/create.rs
git commit -m "docs(analytics): pilot 回补 schema/create.rs missing_docs (#fix-analytics-missing-docs)"
```

> **Pilot 审查点（主会话暂停）**：本 task 完成后，对照 Design Doc §3 工作样例人工核对产出。确认 recipe 锁定后再进 Phase 2。

---

## Phase 2 — 批量回补（per-domain）

> **调度说明**：Group A/B/C/D 之间无依赖（不同文件），可并行 4 个 subagent。每个 subagent 自验用 Step N 的 grep 命令（grep 自己组的文件名）。**每文件都参照 Task 3 pilot 产出**。
>
> **跨组一致性规则**：trait impl（`impl ApiResponseTrait` + `data_format`）一律不加 doc；`//!` 文件头不动（Group C 的 doc_wiki/search.rs 例外，仅补 docPath URL 不改标题）。

### Group A: data_source 组（8 文件）

**Files:**
- Modify: `crates/openlark-analytics/src/search/search/v2/data_source/{create,get,patch,delete,list}.rs`
- Modify: `crates/openlark-analytics/src/search/search/v2/data_source/item/{create,get,delete}.rs`

**Interfaces:** 参照 Task 3 产出格式。

#### Task 4: data_source 5 个顶层文件（create/get/patch/delete/list）

- [x] **Step 1: 对每个文件套 recipe**

逐文件读取 `//!` 标题，按下表补 doc（标题列从各文件 `head -1` 取）：

| 文件 | 标题 | Request→doc | Response→doc |
|------|------|------|------|
| `data_source/create.rs` | 「创建数据源」 | `/// 创建数据源请求。` | `/// 创建数据源响应。` |
| `data_source/get.rs` | 「获取数据源」 | `/// 获取数据源请求。` | `/// 获取数据源响应。` |
| `data_source/patch.rs` | 「更新数据源」 | `/// 更新数据源请求。` | `/// 更新数据源响应。` |
| `data_source/delete.rs` | 「删除数据源」 | `/// 删除数据源请求。` | `/// 删除数据源响应。` |
| `data_source/list.rs` | 「获取数据源列表」 | `/// 获取数据源列表请求。` | `/// 获取数据源列表响应。` |

每个文件还要：
- Response 的 `pub data:` 字段 → `    /// 响应数据。`
- 若文件有额外 pub struct（如 `get.rs` 的 `DataSourceData` 含 `data_source_id`/`name`/`description`）：
  - struct → `/// 数据源详情。`
  - `data_source_id` → `    /// 数据源 ID。`
  - `name` → `    /// 数据源名称。`
  - `description` → `    /// 数据源描述。`
- `pub fn new` → `    /// 创建新的请求构建器。`
- `pub async fn execute` → `    /// 执行<标题>请求。`
- `pub async fn execute_with_options` → `    /// 使用指定请求选项执行<标题>请求。`
- 不动 `impl ApiResponseTrait`。

> **每文件先 `head -2` 读标题再落 doc**——标题以上表为准，若与文件实际 `//!` 不符，以**文件实际**为准（表只列已勘探的）。

- [x] **Step 2: 自验 5 文件无 warning**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep -E 'data_source/(create|get|patch|delete|list)\.rs' | grep 'warning: missing documentation'
```
Expected: 空输出。

- [x] **Step 3: 占位符守门**

Run: `grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/data_source/`
Expected: 空输出。

- [x] **Step 4: fmt**

Run: `cargo fmt -p openlark-analytics -- --check`
Expected: 无 diff。

- [x] **Step 5: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/data_source/
git commit -m "docs(analytics): 回补 data_source 顶层 5 文件 missing_docs (#fix-analytics-missing-docs)"
```

#### Task 5: data_source/item 3 文件（create/get/delete）

- [x] **Step 1: 对每个文件套 recipe**

| 文件 | 标题（已勘探） | Request→doc | Response→doc |
|------|------|------|------|
| `item/create.rs` | 「创建数据源单项」 | `/// 创建数据源单项请求。` | `/// 创建数据源单项响应。` |
| `item/get.rs` | 「获取数据源单项」 | `/// 获取数据源单项请求。` | `/// 获取数据源单项响应。` |
| `item/delete.rs` | 「删除数据源单项」 | `/// 删除数据源单项请求。` | `/// 删除数据源单项响应。` |

> **每文件先 `head -1` 读标题**；若文件实际标题与表不同，以文件实际 `//!` 为准套同一模板。

每文件同 Task 4 Step 1 规则：Response `data` 字段 → `/// 响应数据。`；`new`/`execute`/`execute_with_options` 套模板；不动 trait impl。

- [x] **Step 2: 自验 3 文件无 warning**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'data_source/item/' | grep 'warning: missing documentation'
```
Expected: 空输出。

- [x] **Step 3: 占位符守门 + fmt**

Run:
```bash
grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/data_source/item/
cargo fmt -p openlark-analytics -- --check
```
Expected: 第一条空输出；第二条无 diff。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/data_source/item/
git commit -m "docs(analytics): 回补 data_source/item 3 文件 missing_docs (#fix-analytics-missing-docs)"
```

### Group B: schema 组剩余（3 文件，pilot 已做 create）

#### Task 6: schema/{get,patch,delete}

- [x] **Step 1: 对每个文件套 recipe**

| 文件 | 标题（已勘探） | Request→doc | Response→doc |
|------|------|------|------|
| `schema/get.rs` | 「获取数据范式」 | `/// 获取数据范式请求。` | `/// 获取数据范式响应。` |
| `schema/patch.rs` | 「更新数据范式」 | `/// 更新数据范式请求。` | `/// 更新数据范式响应。` |
| `schema/delete.rs` | 「删除数据范式」 | `/// 删除数据范式请求。` | `/// 删除数据范式响应。` |

每文件同 Task 4 规则：`data` 字段 → `/// 响应数据。`；`new`/`execute`/`execute_with_options` 套模板；不动 trait impl。`schema/create.rs` 已在 Task 3 完成，本 task 不再动。

- [x] **Step 2: 自验 schema 全组无 warning（含 pilot 的 create）**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'schema/' | grep 'warning: missing documentation'
```
Expected: 空输出（4 文件全清）。

- [x] **Step 3: 占位符守门 + fmt**

Run:
```bash
grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/schema/
cargo fmt -p openlark-analytics -- --check
```
Expected: 第一条空；第二条无 diff。

- [x] **Step 4: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/schema/
git commit -m "docs(analytics): 回补 schema 剩余 3 文件 missing_docs (#fix-analytics-missing-docs)"
```

### Group C: search-rest 组（5 文件，含补 doc_wiki 空 docPath）

#### Task 7: doc_wiki/search.rs（含补空 docPath）

- [ ] **Step 1: 读文件，确认标题与当前空 docPath**

Run: `head -2 crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs`
Expected:
```
//! 搜索文档
//! docPath:
```
标题 = 「搜索文档」。docPath 为空，需补。

- [ ] **Step 2: 补全 line 2 docPath**

把 `//! docPath:` 改为（URL 以飞书搜索文档实际地址为准；若该 crate 其他 search API 同域参考之）：
```
//! docPath: <https://open.feishu.cn/document/server-docs/docs/search-docs>
```
> **校验 URL**：执行 `grep -rn 'docPath' crates/openlark-analytics/src/search/ | head`，参考同域已有 URL 格式确认本 URL 形态合法。若该文件标题「搜索文档」对应的飞书 URL 与上述不同，**以飞书文档实际 URL 替换**——但最终不得为空。

- [ ] **Step 3: 给公开项套 recipe（此文件结构同 schema/create.rs）**

- Request struct → `/// 搜索文档请求。`
- Response struct → `/// 搜索文档响应。`
- Response `data` 字段 → `    /// 响应数据。`
- `pub fn new` → `    /// 创建新的请求构建器。`
- `pub async fn execute` → `    /// 执行搜索文档请求。`
- `pub async fn execute_with_options` → `    /// 使用指定请求选项执行搜索文档请求。`
- 不动 trait impl。

- [ ] **Step 4: 自验 + 占位符 + fmt**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'doc_wiki/search.rs' | grep 'warning: missing documentation'
grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs
cargo fmt -p openlark-analytics -- --check
```
Expected: 第一条空；第二条空；第三条无 diff。另外确认 docPath 不再为空：
```bash
grep -n '^//! docPath: *$' crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs
```
Expected: 空输出（无空 docPath）。

- [ ] **Step 5: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/doc_wiki/search.rs
git commit -m "docs(analytics): 回补 doc_wiki/search + 补空 docPath (#fix-analytics-missing-docs)"
```

#### Task 8: app/create、message/create、user、query

> **注意变体**：`user.rs` 和 `query.rs` 不是单 struct-pair 文件——它们有 API 容器 struct + builder struct + builder 方法。recipe 仍覆盖（见全局约束表的「API 容器 struct / builder struct / builder 方法」行）。

- [ ] **Step 1: 对每个文件读标题 + 落 doc**

**app/create.rs**（标题「创建搜索应用」之类，`head -1` 确认）：
- Request struct → `/// <标题>请求。`
- Response struct → `/// <标题>响应。`
- Response `data` → `/// 响应数据。`
- `new`/`execute`/`execute_with_options` 套模板（execute 用 `<标题>`）。
- 不动 trait impl。

**message/create.rs**（同 app/create 模式）。

**user.rs**（变体文件，公开项：`UserSearchApi`、`SearchUserRequest` + builder 方法 `query`/`page_size` + `new`/`execute`/`execute_with_options`）：
- `//!` 标题确认（应为「用户搜索」类）。以**文件实际 `//!` 标题**作为 `<标题>`。
- `pub struct UserSearchApi` → `/// 用户搜索 API。`
- `impl UserSearchApi` 的 `pub fn new` → `    /// 创建用户搜索 API 实例。`
- `impl UserSearchApi` 的 `pub fn search` → `    /// 创建搜索用户请求构建器。`
- `pub struct SearchUserRequest` → `/// 搜索用户请求构建器。`
- builder 方法 `query` → `    /// 设置查询关键字。`
- builder 方法 `page_size` → `    /// 设置每页返回数量。`
- `pub async fn execute` → `    /// 执行用户搜索请求。`
- `pub async fn execute_with_options` → `    /// 使用指定请求选项执行用户搜索请求。`
> 若 `SearchUserRequest` 有 `pub fn new`，则 `    /// 创建新的请求构建器。`

**query.rs**（变体文件，公开项：`QueryApi`、`SearchRequest` + `SuggestRequest` + builder 方法 `search_term`/`search_type`/`page_size`/`query` + execute*）：
- `//!` 标题确认（应为「搜索与建议」类）。
- `pub struct QueryApi` → `/// 搜索查询 API。`
- `impl QueryApi` 的 `pub fn new` → `    /// 创建搜索查询 API 实例。`
- `impl QueryApi` 的 `pub fn search` → `    /// 创建搜索请求构建器。`
- `impl QueryApi` 的 `pub fn suggest` → `    /// 创建搜索建议请求构建器。`
- `pub struct SearchRequest` → `/// 搜索请求构建器。`
- builder 方法 `search_term` → `    /// 设置搜索词。`
- builder 方法 `search_type` → `    /// 设置搜索类型。`
- builder 方法 `page_size`（SearchRequest 内）→ `    /// 设置每页返回数量。`
- `pub async fn execute`（SearchRequest）→ `    /// 执行搜索请求。`
- `pub async fn execute_with_options`（SearchRequest）→ `    /// 使用指定请求选项执行搜索请求。`
- `pub struct SuggestRequest` → `/// 搜索建议请求构建器。`
- builder 方法 `query`（SuggestRequest 内）→ `    /// 设置建议查询关键字。`
- `pub async fn execute`（SuggestRequest）→ `    /// 执行搜索建议请求。`
- `pub async fn execute_with_options`（SuggestRequest）→ `    /// 使用指定请求选项执行搜索建议请求。`

> **执行者责任**：每个文件先 `head -1` 确认 `//!` 标题；若实际标题与本计划措辞不同，**以文件实际标题代入**模板（doc 的具体中文措辞允许微调以匹配标题，但结构/句式必须一致）。

- [ ] **Step 2: 自验 4 文件无 warning**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep -E 'search/v2/(app/create|message/create|user|query)\.rs' | grep 'warning: missing documentation'
```
Expected: 空输出。

- [ ] **Step 3: 占位符守门 + fmt**

Run:
```bash
grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/search/search/v2/app/ crates/openlark-analytics/src/search/search/v2/message/ crates/openlark-analytics/src/search/search/v2/user.rs crates/openlark-analytics/src/search/search/v2/query.rs
cargo fmt -p openlark-analytics -- --check
```
Expected: 第一条空；第二条无 diff。

- [ ] **Step 4: Commit**

```bash
git add crates/openlark-analytics/src/search/search/v2/app/create.rs crates/openlark-analytics/src/search/search/v2/message/create.rs crates/openlark-analytics/src/search/search/v2/user.rs crates/openlark-analytics/src/search/search/v2/query.rs
git commit -m "docs(analytics): 回补 search-rest 4 文件 missing_docs (#fix-analytics-missing-docs)"
```

### Group D: report 组（3 文件）

#### Task 9: report/{rule/query, rule/view/remove, task/query}

- [ ] **Step 1: 对每个文件读标题 + 落 doc**

| 文件 | 标题（已勘探） | Request→doc | Response→doc |
|------|------|------|------|
| `report/v1/rule/query.rs` | 「查询规则」 | `/// 查询规则请求。` | `/// 查询规则响应。` |
| `report/v1/rule/view/remove.rs` | 「移除规则看板」 | `/// 移除规则看板请求。` | `/// 移除规则看板响应。` |
| `report/v1/task/query.rs` | 「查询任务」 | `/// 查询任务请求。` | `/// 查询任务响应。` |

每文件同规则：Response `data` → `/// 响应数据。`；`new`/`execute`/`execute_with_options` 套模板（execute 用该文件 `<标题>`）；不动 trait impl。
> `view/remove.rs` 注意：当前无 docPath（只有 `//! 移除规则看板`），保留现状不加 docPath（Design Doc 未要求给 report 加 docPath，只补 doc_wiki/search.rs 那一处空值）。

- [ ] **Step 2: 自验 report 全组无 warning**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'report/' | grep 'warning: missing documentation'
```
Expected: 空输出。

- [ ] **Step 3: 占位符守门 + fmt**

Run:
```bash
grep -rnE '待补充文档|公开项说明|TODO|TBD' crates/openlark-analytics/src/report/
cargo fmt -p openlark-analytics -- --check
```
Expected: 第一条空；第二条无 diff。

- [ ] **Step 4: Commit**

```bash
git add crates/openlark-analytics/src/report/
git commit -m "docs(analytics): 回补 report 3 文件 missing_docs (#fix-analytics-missing-docs)"
```

---

## Phase 3 — 收尾（主会话）

### Task 10: 移除 crate 级 `#![allow(missing_docs)]`

**Files:**
- Modify: `crates/openlark-analytics/src/lib.rs:35`

**Interfaces:** 无（移除 1 行）

- [ ] **Step 1: 移除 line 35**

把
```rust
#![allow(clippy::module_inception)]
#![allow(missing_docs)]
```
改为
```rust
#![allow(clippy::module_inception)]
```
**保留 line 34（module_inception）**——它不在范围。

- [ ] **Step 2: 验证 crate 级 0 missing_docs 警告（allow 移除后真验证）**

Run:
```bash
cargo doc -p openlark-analytics --all-features 2>&1 | grep 'warning: missing documentation' | wc -l
```
Expected: `0`（122 个已全部回补）。

- [ ] **Step 3: 验证 grep 无残留 crate 级 allow**

Run: `grep -rn '#!\[allow(missing_docs)\]' crates/`
Expected: 空输出（workspace 无 crate 级 missing_docs 抑制；protocol 的 item 级 `#[allow]` 不匹配 `#!`）。

- [ ] **Step 4: fmt**

Run: `cargo fmt -p openlark-analytics -- --check`
Expected: 无 diff。

- [ ] **Step 5: Commit**

```bash
git add crates/openlark-analytics/src/lib.rs
git commit -m "refactor(analytics): 移除 crate 级 #![allow(missing_docs)] (#fix-analytics-missing-docs)"
```

### Task 11: CI 接线——加 missing_docs 测试

**Files:**
- Modify: `.github/workflows/ci.yml`（在 lint job 现有 `test_check_mod_reachability` 步骤旁加一步）

**Interfaces:** 无

- [ ] **Step 1: 找到锚点步骤**

Run: `grep -n 'test_check_mod_reachability' .github/workflows/ci.yml`
Expected: 命中约 line 111-114 的 `Check mod reachability` step：
```yaml
      - name: Check mod reachability (no new orphan src files)
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 tools/check_mod_reachability.py
```

- [ ] **Step 2: 在该 step 之后插入 missing_docs 测试 step**

在 `Check mod reachability` step 之后（`Check no #[allow(dead_code)]` step 之前）插入：
```yaml
      - name: Check workspace missing_docs (no warnings, no crate-level suppressions)
        run: python3 -m unittest tools.tests.test_workspace_missing_docs
```
> 这会运行 `tools/tests/test_workspace_missing_docs.py` 的全部 3 个测试方法：
> - `test_workspace_has_no_missing_docs_warnings`（跑 `cargo test --workspace --all-features --no-run` 检查无 `warning: missing documentation for `）
> - `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions`（grep 无 `#![allow(missing_docs)]`）
> - `test_workspace_item_level_missing_docs_exception_is_protocol_generated_module_only`（item 级 allowlist 仅 protocol）

- [ ] **Step 3: 本地复现 CI——3 测试全绿**

Run:
```bash
python3 -m unittest tools.tests.test_workspace_missing_docs -v
```
Expected: `test_workspace_has_no_missing_docs_warnings ... ok` / `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions ... ok` / `test_workspace_item_level_missing_docs_exception_is_protocol_generated_module_only ... ok`，结尾 `OK`。
> **注**：第一个测试会跑 `cargo test --workspace --all-features --no-run`，耗时较长（首次几分钟，CI 复用缓存更快）。这是 Design Doc 决策 C 接受的成本。

- [ ] **Step 4: yaml 语法自查**

Run: `python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/ci.yml'))" && echo YAML_OK`
Expected: `YAML_OK`。

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: 接入 missing_docs 验证测试进 lint job (#fix-analytics-missing-docs)"
```

### Task 12: 全局验证 + 收尾

**Files:** 无（只跑验证命令）

- [ ] **Step 1: workspace cargo doc 0 missing_docs**

Run:
```bash
cargo doc --workspace --all-features 2>&1 | grep 'warning: missing documentation' | wc -l
```
Expected: `0`。

- [ ] **Step 2: cargo fmt --check + just lint**

Run:
```bash
cargo fmt --all -- --check
just lint
```
Expected: fmt 无 diff；`just lint`（`cargo clippy --workspace --all-targets --all-features -- -Dwarnings` + 其他检查）通过 0 错误。
> **memory 提醒**：CI lint job 第一步是 `cargo fmt --check`，本地必须显式跑（clippy 通过 ≠ fmt 通过）。

- [ ] **Step 3: 现有测试不破**

Run: `cargo test -p openlark-analytics`
Expected: 全部通过（本 change 只加注释，不改逻辑，测试不应受影响）。

- [ ] **Step 4: 占位符全局守门（D2 最终）**

Run: `grep -rnE '待补充文档|公开项说明' crates/openlark-analytics/src/`
Expected: 空输出。

- [ ] **Step 5: CI 全链路 dry-run（可选，确认接线）**

Run: `python3 -m unittest tools.tests.test_workspace_missing_docs -v && python3 -m unittest tools.tests.test_check_mod_reachability -v`
Expected: 两个模块全绿。

- [ ] **Step 6: 最终提交（若前面有未提交的 fmt 修正等）**

```bash
git status   # 确认工作树干净
git log --oneline -10   # 复核本 change 的 8 个 commit（pilot + 4 组 + allow + CI + 可能的修正）
```

> 全部通过后：tasks.md 全部打勾 → 触发 verify guard → 进入 Comet verify 阶段。

---

## Self-Review（计划作者自检，非执行步骤）

**1. Spec coverage**（逐条对 Design Doc / tasks.md）：
- tasks 1.1 基线 → Task 1 ✓
- tasks 1.2 文件级 //! 覆盖 + doc_wiki 空 docPath → Task 2 ✓
- tasks 2.1 data_source 8 文件 → Task 4 + 5 ✓
- tasks 2.2 schema 4 文件 → Task 3（pilot create）+ Task 6 ✓
- tasks 2.3 search-rest 5 文件 + 补 docPath → Task 7 + 8 ✓
- tasks 2.4 report 3 文件 → Task 9 ✓
- tasks 2.5 逐组自验 → 各 Task 的 grep 步骤 ✓
- tasks 3.1 移除 allow → Task 10 ✓
- tasks 3.2 crate 级 0 警告 → Task 10 Step 2 ✓
- tasks 4.1 占位符守门 → 各 Task Step + Task 12 Step 4 ✓
- tasks 5.1 CI 接线 → Task 11 ✓
- tasks 5.2 本地复现 3 测试 → Task 11 Step 3 ✓
- tasks 6.1 workspace doc 0 警告 → Task 12 Step 1 ✓
- tasks 6.2 fmt + just lint → Task 12 Step 2 ✓
- tasks 6.3 analytics 现有测试 → Task 12 Step 3 ✓

**2. Placeholder scan**：无 TBD/TODO/「类似 Task N」；每步有具体 grep/cargo 命令与 doc 文本。

**3. Type/name 一致性**：`CreateSchemaRequest`、`DataSourceData`、`UserSearchApi`、`SearchRequest`、`SuggestRequest` 等名取自实际文件 grep；recipe 模板在「全局约束」单一定义，各 task 引用同一名词。

## 执行交接

计划已保存至 `docs/superpowers/plans/2026-07-01-fix-analytics-missing-docs.md`。两种执行方式：

1. **Subagent-Driven（推荐）** — 我为每个 task 派发新 subagent，task 间审查，迭代快。Group A/B/C/D 4 组无依赖可并行。
2. **Inline Execution** — 本会话内用 executing-plans 逐 task 执行，带 checkpoint 审查。

请选择方式。
