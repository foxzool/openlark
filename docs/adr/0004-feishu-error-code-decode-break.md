# ADR: 飞书错误码解码断裂修复（ApiError 承载 raw_code + 映射收敛）

- **状态**: Proposed（2026-07-25 `/improve-codebase-architecture` → `/grilling` 达成共识，待实施）
- **日期**: 2026-07-26
- **决策者**: 架构评审 + 用户 grilling 共识
- **来源**: 架构评审候选 1（error 解码断裂使 ErrorCode 分类在生产路径失效）+ grilling 三项决策；父 spec #542；本 ADR 为 #543
- **breaking 窗口**: 目标 0.19，与 ADR-0002/0003 同批。公开 breaking：`ApiError.status: u16` 移除、`raw_code: i32` 新增；自由函数 `api_error` 与相关构造器签名 `u16 → i32`；`ErrorCode::from_feishu_code` 与 `openlark_client::error::from_feishu_response` 删除。`code: ErrorCode` / `endpoint` / `message` / `source` / `ctx` 字段保留。

## 背景

SDK 使用者调用飞书 API 失败时拿到 `CoreError::Api`，但**无法区分**「token 过期」「权限不足」「用户会话失效」等任何失败类型——`ErrorCode` 分类在生产路径上恒为 `Unknown`。

### 解码断裂（已核实）

飞书业务错误码是 9 位 `i32`（如 `99991663` = tenant_access_token 失效）。生产路径在构造 `ApiError` 时做 `as u16` 截断：

| 位置 | 行为 |
|------|------|
| `Response::decode`（`api/responses.rs:249`） | `api_error(raw.code as u16, …)` |
| 自由函数 `api_error`（`error/core.rs:1238-1257`） | 参数 `status: u16`，经 `ErrorCode::from_http_status(status)` 分类 |
| `CoreError::api_error` 兼容构造器（`error/core.rs:724-736`） | 参数已是 `i32`，却 `status as u16` 再丢给自由函数 |

截断示例：`99991663 as u16` → `49263`。`ErrorCode::from_http_status(49263)` 走不到任何已知臂，结果恒为 `Unknown`。

讽刺的是，`ErrorCode::from_code`（`codes.rs:214`）完整躺着飞书码映射表（含 `99991663 → TenantAccessTokenInvalid` 等）——它从来够不到真实数据。`ApiError.status: u16` 字段携带截断垃圾值，`Display`（`core.rs:575-578`）输出形如 `49263 response: …`，日志排障被误导。

错误码分类是 `CoreError` 对调用方承诺的核心 interface，当前它在生产路径上**整体失效**。

### 三条并行映射路径（已核实）

| # | 路径 | 位置 | 状态 |
|---|------|------|------|
| 1 | `ErrorCode::from_code(i32)` | `codes.rs:214` | **完整**：HTTP status 臂 + 飞书业务码臂；本应是唯一路径 |
| 2 | `ErrorCode::from_feishu_code(i32) → Option` | `codes.rs:669-690` | `from_code` 飞书臂的**子集复刻**；decode 接通后即死 |
| 3 | `from_feishu_response` | `openlark-client/src/error.rs:58-91` | 调 `from_feishu_code`，再含**第三份** category→status magic match；测试外零调用 |

### RawResponse 双域共槽（已核实）

`RawResponse.code: i32`（`responses.rs:10-12`）是双域共槽：

- **飞书业务信封**：装入飞书 `code` 字段（可为 9 位 i32）；
- **HTTP 非 2xx 且无信封**：装入合成 HTTP status（如 429/500）。

这是既有设计；`from_code` 同时含 HTTP 臂与飞书臂，正是双域共槽行为正确的依据。本 ADR **不拆**共槽（见非目标）。

### 已核实约束

- `is_retryable` / `retry_delay` 对 `CoreError::Api` 当前匹配 `api.status` 的 u16 范围（`core.rs:808-830`：`429 | 500..=599`）。截断后飞书业务码落入垃圾 u16，**碰巧**多数不可重试（行为「看起来对」），但语义建立在截断垃圾上，不可依赖。
- `ErrorCode::is_retryable` 已按 variant 判定（`codes.rs:615-629`），含 `TooManyRequests` + 5xx 族 + 网络类——`CoreError::Api` 的 `is_retryable` 切到 `api.code.is_retryable()` 后可复用。`retry_delay` 仍保留 `CoreError::Api` 既有 `1 << attempt.min(5)` 公式（与 `suggested_retry_delay` 数值不同，不混用）。
- openlark-client error 转发壳（`client/src/error.rs`）对 `api_error` 仅签名转发；`from_feishu_response` 测试外零调用。
- transport contract seam（`TestServer` + wiremock）与 `Response::decode` 单元 seam 已存在；当前**无**断言「飞书业务码 → 正确 ErrorCode」的测试——本修复的主盲区。
- 与 ADR-0002/0003 的关系：本 ADR 是 ADR-0002 推迟候选、ADR-0003 推迟 error 候选相关部分的兑现起点；候选 2（error 死装置清扫）的删除判别标准依赖本 ADR 落地后分类表成为真职责。

## 决策

接通解码断裂，让分类系统从摆设变成真职责。三项 grilling 共识（与 #542 Implementation Decisions 对齐，无漂移）：

| # | 子决策 | 结论 |
|---|--------|------|
| 1 | **ApiError 字段形态（方案 A）** | 移除 `status: u16`，新增 `raw_code: i32`；`code: ErrorCode`、`endpoint`、`message`、`source`、`ctx` 保留。命名为 `raw_code` 而非 `feishu_code`——`RawResponse.code` 是双域共槽，命名必须诚实 |
| 2 | **映射收敛** | 三条路径收敛为一条：`api_error` 改收 `raw_code: i32`，经 `ErrorCode::from_code(raw_code)` 不截断分类；删除 `from_feishu_code` + `from_feishu_response`（均 pub，计入 breaking） |
| 3 | **共槽处理** | **不拆** `RawResponse` 双域共槽；在共槽处加注释点明 `from_code` 双域映射是行为正确的依据，防止后来者误拆 |

配套子决策（由三项共识派生，同属 #542 Implementation Decisions）：

| # | 子决策 | 结论 |
|---|--------|------|
| 4 | decode | `Response::decode` 传 `raw.code` 原值，去掉 `as u16` |
| 5 | retry 判定 | `is_retryable` 改匹配 `api.code.is_retryable()`（覆盖 `TooManyRequests` + 5xx 族等）；`retry_delay` **仅谓词切 variant**，延迟公式保持既有 `1 << attempt.min(5)` 秒（不换成 `suggested_retry_delay` 的 60s 固定值，避免「等价回归」口径被破坏）。对 HTTP 合成码与飞书业务码两域行为等价 |
| 6 | Display | 打印 `raw_code`（真实错误码，非截断垃圾） |
| 7 | 落地 | 先落本 ADR；再 TDD 实施（#544 → #545/#546 → #547）；**每 ticket 独立全绿可合入**（见迁移路径中间态约束）；breaking 计入 0.19 CHANGELOG；单批 PR 链 |

分层结果：`ErrorCode::from_code`（唯一映射）← `api_error(raw_code: i32, …)`（分类入口）← `Response::decode` / 调用方构造（传原值，不截断）。

## 理由

1. **方案 A 最小正确**：`status: u16` 字段本身就是截断垃圾的容器；换成 `raw_code: i32` 一次修字段语义、分类输入与 Display，无需并行保留假 `status`。
2. **命名诚实优于领域浪漫**：`feishu_code` 在共槽场景撒谎（HTTP 合成码不是飞书码）；`raw_code` 对两域都成立。
3. **`from_code` 已是完整映射**：HTTP 臂 + 飞书臂俱在；接通后 `from_feishu_code` 是纯子集复刻、修复后即死——删除而非保留「兼容别名」避免双写。
4. **`from_feishu_response` 是第三份 magic**：category→status 反推 + 测试外零调用；删它消减并行装置，符合 CLAUDE.md §3（无死扩展点）。
5. **retry 切 variant 与分类单一事实源**：当前按 u16 范围匹配「碰巧」对飞书码多数不可重试，但语义建立在截断上；改匹配 `ErrorCode` variant 后，429/5xx 无论来自 HTTP 合成还是（若将来信封出现）业务码，行为一致，且飞书业务码仍不可重试。
6. **共槽不拆是范围守卫**：修复后行为已正确；拆共槽（新增 `http_status`、重塑 decode）是纯语义瑕疵，另案，避免本批范围蔓延。
7. **与 ADR-0002/0003 同批 0.19**：公开 API breaking 集中窗口，CHANGELOG 迁移表一次说清。

## 后果

### 正面

- 生产路径分类恢复真职责：`99991663 → TenantAccessTokenInvalid`、`99991672 → PermissionMissing` 等可程序化分支。
- `ApiError` 可读完整原始错误码，对照飞书文档排障。
- `Display` 输出真实码，日志不再误导。
- 飞书码 → ErrorCode 映射只剩 `from_code` 一处；新增错误码只改一处。
- retry 语义与错误分类单一事实源；HTTP/业务两域行为等价。
- 为候选 2（error 死装置清扫）提供删除判别前置：分类表成为真职责后才可安全清扫。

### 负面 / Breaking 清单（目标 0.19）

| 变更 | 迁移 |
|------|------|
| `ApiError.status: u16` **移除** | 改读 `raw_code: i32`（原始码）或 `code: ErrorCode`（分类） |
| `ApiError.raw_code: i32` **新增** | 新字段；对照飞书文档 / 日志用 |
| 自由函数 `api_error(status: u16, …)` → `api_error(raw_code: i32, …)` | 传入完整 i32，勿再 `as u16` |
| `CoreError::api` / `CoreError::api_error` / prelude 宏 / ErrorBuilder 相关签名同步 | 同上；client 转发壳同步 |
| `ErrorCode::from_feishu_code` **删除** | 改用 `ErrorCode::from_code(code)`（完整映射，未知 → `Unknown` 而非 `None`） |
| `openlark_client::error::from_feishu_response` **删除** | 改用 core `api_error(raw_code, …)` 或直接构造 `CoreError::Api` |

### 非目标（范围守卫）

- **`RawResponse` 双域共槽拆分**（新增 `http_status` 字段、decode 管线重塑）——修复后行为已正确，纯语义瑕疵，另案。
- **error 系统死装置清扫**（架构评审候选 2：core/error 并行装置、client error 转发壳其余拆除）——本 ADR 是其前置，本批不碰。
- **endpoint catalog / codegen 结构化**（评审候选 3）——不碰。
- **leaf builder 公开 API**——不动（ADR-0001 硬约束）。
- **基于新分类的自动 token 刷新分派**——本 ADR 只让分派成为可能，不实现分派本身。

## 迁移路径（分阶段，TDD；对应 #544–#547）

**中间态约束**：每个 ticket 合入后 `cargo test --workspace --all-features` 必须全绿。`ApiError.status` 删除会立刻弄断所有字面构造与 `matches!(api.status, …)`——这些机械同步属于引入字段变更的同一 ticket，不可留给后续阶段。

1. **本 ADR（#543）**——固化背景 / 三项决策 / 理由 / 后果 / 非目标 / 迁移路径 / 遵循；评审者可凭此单独判断后续实施 ticket 是否忠实于共识。
2. **ApiError + decode（#544）**——TDD 红测先行；字段形态 + **全仓编译同步**：
   - transport contract seam（`TestServer`）：mock 业务错误信封 `code=99991663` + `X-Tt-Logid` → 断言 `TenantAccessTokenInvalid`、`raw_code` 原样、`request_id` 透传；
   - `Response::decode` 单元 seam：业务错误 Response → 分类与 `raw_code`；
   - 实现：`ApiError` 字段形态方案 A；`api_error` 改收 `i32` + `from_code`；`decode` 去 `as u16`；Display 打 `raw_code`；共槽处注释；
   - **编译同步（必做，否则 #544 无法独立全绿）**：
     - 所有 `ApiError { status, … }` 字面构造（含 `from_feishu_response`、ErrorBuilder、`CoreError::api*`）改 `raw_code`；
     - `is_retryable` / `retry_delay` 中对 `api.status` 的匹配改为对 `api.raw_code` 的 `429 | 500..=599`（HTTP 合成码仍可重试；飞书 9 位码仍不可重试——行为与现状等价）。语义切到 ErrorCode variant 留给 #545。
3. **retry 判定（#545）**——谓词切 variant，延迟公式不动：
   - `is_retryable` → `api.code.is_retryable()`（覆盖 `TooManyRequests` + `InternalServerError` / `BadGateway` / `ServiceUnavailable` / `GatewayTimeout` 等）；
   - `retry_delay` → 仍用既有 `Duration::from_secs(1 << attempt.min(5))`，**不**换成 `suggested_retry_delay`（后者对 429 固定 60s，破坏等价回归）；
   - 合成 429 可重试 + 飞书业务码不可重试 的回归测。
4. **映射收敛删除（#546）**——删 `from_feishu_code` + `from_feishu_response` 及其测试；全仓 grep 源码无残留；**同步改写 `ARCHITECTURE.md`**「错误码对齐与优先级」段（当前写「优先 `from_feishu_code`，未命中再用 status」及示例）为 `from_code` 单路径 + `raw_code` 语义。
5. **CHANGELOG + 执行记录（#547）**——0.19 breaking 迁移表以**上文 Breaking 清单全部行**为准（字段移除/新增、`api_error` 与 `CoreError::api*` / ErrorBuilder / prelude 签名、两处删除），含 before/after 示例；勿缩成笼统「四项」而漏 ErrorBuilder；本 ADR 补执行记录；`just check-all` 等价验证（fmt + clippy×2 + test + doc + machete + msrv）。

## 遵循

- **#542 Implementation Decisions**：三项 grilling 共识与配套子决策原样固化，无漂移。
- **ADR-0002 / ADR-0003**：同批 0.19 breaking 窗口；先 ADR 后 TDD 实施先例；结构对齐（状态/日期/背景/决策/理由/后果/非目标/迁移路径/遵循）。
- **ADR-0001**（leaf builder API 100% 冻结）：本 ADR 不触 leaf builder。
- **CLAUDE.md §3**（无投机抽象 / 无死扩展点）：收敛映射为一条；删除死路径，不开兼容别名双写。
- **CLAUDE.md §4**（外科手术式改动）：范围守卫明确；共槽拆分 / 死装置清扫 / 自动刷新分派另案。
- **CLAUDE.md §5**（验证）：transport contract + decode 单元 + retry seam 红→绿；全仓回归。
- **AGENTS.md**：错误处理统一 `CoreError`；库代码不 `unwrap`/`expect`；公开 API breaking 走 CHANGELOG 迁移表。

## 执行记录

_待 #544–#547 落地后由 #547 补齐各 ticket commit 与验证结果。_
