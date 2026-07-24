# ADR: ws 端点发现收口到 Transport（关闭 ws_client 第二条 HTTP 出口）

- **状态**: Proposed（2026-07-24 `/improve-codebase-architecture` → `/grilling` 达成共识，待实施）
- **日期**: 2026-07-24
- **决策者**: 架构评审 + 用户 grilling 共识
- **来源**: 架构评审候选 1（ws_client 端点发现绕过 core Transport）
- **breaking 窗口**: 目标 0.19。`WsClientError` 移除 `ServerError` / `ClientError` 两 variant、`RequestError` 负载由 `reqwest::Error` 换为 `CoreError`（公开枚举 SemVer-major）。`LarkWsClient::open` 返回类型不变；`get_conn_url` 私有；ws_client 新增对 `core::http::Transport` 的依赖（已依赖 openlark-core 取 `Config`，方向正确，无公开 API 影响）。

## 背景

ws_client 建立长连接前需先 POST `/callback/ws/endpoint` 拿到 WS 地址与 `ClientConfig`。这次 HTTP 调用目前**完全绕过** core 的 `Transport`，手搓了一条出口（均已核实，`crates/openlark-client/src/ws_client/client.rs`）：

- `get_conn_url`（`client.rs:111-139`）：自建 `reqwest::Client`（`:119-123`）→ 手 POST `{base_url}/callback/ws/endpoint`（`:126-131`）→ 手解析信封（`:133-138`）。
- 信封 `WsEndpointApiResponse<T>`（`client.rs:18-26`，顶层 `code/msg/data`）。
- `map_ws_api_error`（`client.rs:28-33`）：按 magic `1000040343` 拆 `ServerError`/`ClientError`。
- `extract_endpoint_response`（`client.rs:35-51`）：`code!=0` → error、缺 `URL`/`data` → error。
- `END_POINT_URL = "/callback/ws/endpoint"`（`client.rs:53`，字面量常量）。

这是 ADR-0002 刚从 `openlark-auth` 拔掉的同形模式：`fetch_token_via_http`（绕过 Transport 直接 reqwest post + 手解析 code/msg/token）→ 委托既有构建路径、均经 `Transport::request`（commit `74cfc0494`）。ws_client/client.rs:111-139 是该模式的存活残留。

**与 ADR-0002 的关键差异**：auth 在 core 内、说 `CoreError` 母语；ws_client 在 openlark-client、公开错误词汇是 `WsClientError`（session 语义：close reason / handler panic / backlog 不适合折进 `CoreError`）。

### 已核实约束

- **ServerError / ClientError 零外部消费者**：全仓仅 `client.rs` 自身（`map_ws_api_error` + 2 测试 `:180,:201`）引用，无 crate 外 match。
- **ws_client 当前零 Transport 耦合**：`rg "Transport" crates/openlark-client/src/ws_client/` 空；唯一 core import 是 `Config`（作标量袋：`app_id`/`app_secret`/`base_url`/`req_timeout`/`max_response_size`）。
- **`Transport::do_send` 是 `pub(crate)`**（`http.rs:173`），`UnifiedRequestBuilder::build` 是 `pub`（`request_execution/mod.rs:27`），`Transport::request_typed` 是 `pub`（`http.rs:124`）。→ openlark-client 唯一可达的 Transport 公开入口是完整 `request_typed` 管线；resend 式 bootstrap 旁路（`pub(crate)`）够不到。
- **端点响应是标准 `code/msg/data` 信封**（`client.rs:18-26`）→ 走 `Transport::request_typed::<EndPointResponse>` + `ApiResponseTrait` 默认 `Data` 格式即可，无需特殊解码。
- **`API_PATH_PREFIX = "/open-apis/"`**（`constants.rs:30`）。`/callback/ws/endpoint` 不带该前缀 → `ApiRequest::api_path()` 返回全路径、`UnifiedRequestBuilder::build_url` 正确拼到 `base_url`；无 middleware 假设 `/open-apis/`。
- **20 个 full-session 测试用 wiremock mock 该端点**（`full_session_tests.rs:69-87` mount `/callback/ws/endpoint`），config 设 `base_url=mock_server.uri()` + `allow_custom_base_url(true)` + `app_id/app_secret`（`:96-105`）。路由后 Transport 走同一 `base_url`+`config.http_client` 命中同一 mock；`validate()` 对非空 `app_id/app_secret` + `None` token 放行 → 测试存活。
- **默认 token 路径会触发拉 token**：`ApiRequest` 默认声明 `[User, Tenant]`（`api/mod.rs:303`）；`determine_token_type([User,Tenant], ∅, cache=true)` → `Tenant`（`policy.rs:108-110`）；`AuthHandler::apply_auth(_, Tenant, …)` 在 option 无 token + cache 时调 `token_provider().get_token()`（`acquisition.rs:51-72`）。测试 config 默认 `NoOpTokenProvider` → `get_token` 返 `Err`（`acquisition.rs:392-415`）→ **不声明 `[None]` 则请求在碰 endpoint 前就挂、且 20 测试全红**；生产则多一次拉 tenant_token 的往返 + 挂多余 `Authorization: Bearer`。声明 `[None]` 后 `determine_token_type` 返 `None`、`apply_auth` 直接 `Ok(req_builder)`（`acquisition.rs:22`）不拉 token。

## 决策

端点 POST 经 `Transport::request_typed` 收口；tungstenite WS upgrade（`client.rs:102 connect_async`）保持独立——那是另一种传输协议，core Transport 是 HTTP-only，**它是合理第二传输，不动**。七项子决策：

| # | 子决策 | 结论 |
|---|--------|------|
| 1 | 收口范围 | 端点 POST 走 `Transport::request_typed`；WS upgrade 不动（合理第二传输） |
| 2 | 管线深度 | **完整 `request_typed` 管线**（validate + auth policy + tracing + decode + recovery）。不开新 pub bootstrap seam——`do_send`/resend 旁路是 `pub(crate)`，为单调用点开 pub = 单实现扩展点（踩 CLAUDE.md §3）。recovery 对端点是 no-op（端点不归 app_ticket 门控、不会回 10012）且端点不在 recovery 回路 → 无递归风险 |
| 3 | token 类型 | 端点 `ApiRequest` 显式 `with_supported_access_token_types(vec![None])`（`api/mod.rs:257`）。否则默认 `[User,Tenant]`+cache → 拉 tenant token + 挂多余 Bearer + NoOpTokenProvider 下测试全挂 |
| 4 | 错误边界 | 保留 `WsClientError` 为 `LarkWsClient::open` 返回类型（WS 语义塞不进 `CoreError`）；加包裹 variant 透传 `CoreError`（白拿 request_id）；session 侧 variant 纹丝不动 |
| 5 | variant 收并 | 移除 `ServerError` / `ClientError`（`session/types.rs:24-39`），`RequestError` 负载由 `#[from] reqwest::Error` 换为 `#[from] CoreError`（复用名字、最小文档漂移；reqwest::Error 经 Transport 已吞成 `CoreError::Network`，旧 `#[from]` 路径消失）。三 variant 的唯一生产者本是 `get_conn_url`，零外部消费者。保留 `UnexpectedResponse`（post-decode 缺字段业务规则：缺 URL/缺 client_config/缺 service_id，`client.rs:43-48,86-89`，不是 `CoreError`） |
| 6 | 测试 | 删 `client.rs` 全部 3 个低 seam 单测（均测被删的内部函数）；在 `full_session_tests.rs` 的 open seam 加 2 个——`open_endpoint_business_error_wraps_core_error_with_request_id`（code!=0+`X-Tt-Logid` → `RequestError(CoreError::Api)` 且 request_id 保住）+ `open_endpoint_success_without_url_is_unexpected_response`（code:0 缺 URL → `UnexpectedResponse`，替代原调 `extract_endpoint_response` 的私有测，因 #524 AC 要求 open-seam）。20 个 full-session 测试行为不变；其中 `full_session_oversized_frame_is_rejected` 的 `TINY_MAX` 因 decision #1 副作用（端点 bootstrap 响应现受 `max_response_size` 守护）从 64 抬到 512（须 >~200 字节 bootstrap 响应、仍 << 4096 测试帧，意图不变） |
| 7 | 落地 | 独立 PR（ws_client 局部，不捆未决的 error 系统候选 2/3）；先落本 ADR；breaking 计入 0.19 CHANGELOG |

分层结果：`Transport::request_typed`（HTTP 唯一出口，已深）← `get_conn_url`（~10 行：构 `ApiRequest`([None] + `locale` header + json body) → 委托 → 缺字段校验）← `LarkWsClient::open_with`（WS upgrade 独立）。

## 理由

1. **deletion test 通过**：HTTP 故事移进单一深模块（Transport），复杂度**集中**而非平移——深化信号。`get_conn_url` 从 ~30 行手搓 HTTP 缩成 ~10 行委托。
2. **精确先例**：ADR-0002 对 auth `fetch_token_via_http` 做了同一判定（commit `74cfc0494`），路径已验证、风险已知。本 ADR 是其同源兄弟。
3. **白拿增益**：log_id / tracing span（`http.rs:56-63 info_span`）/ feishu_code 映射（`Response::decode`）/ request_id 全自动覆盖；旧手搓路径全无。
4. **不开 pub seam 合规 §3**：`do_send` 的 `pub(crate)` 是有意的——为单调用点开 pub bootstrap 入口是单实现扩展点。端点不在 recovery 回路，traverse 完整管线无递归风险，resend「绕 policy 防递归」的理由不适用。
5. **`[None]` 是正确性强制项**：不声明则默认路径拉 tenant token，既坏测试（NoOpTokenProvider）又坏生产（多余往返 + 多余 Bearer）。
6. **三 variant 收并诚实**：`ServerError`/`ClientError`/`RequestError` 唯一生产者是 `get_conn_url`、零消费者；保留它们等于把 `CoreError::Api` 再拆回 `{code,message}`、扔掉 request_id——正是本 ADR 要消除的损失。`UnexpectedResponse` 留下因它是解出的 `EndPointResponse` 上的业务规则，非 `CoreError`。

## 后果

### 正面

- 关闭 intra-workspace 第二条 HTTP 出口；HTTP 故事单一入口（Transport）。
- `get_conn_url` 减 ~20 行手搓；删 `map_ws_api_error` / `extract_endpoint_response` / `WsEndpointApiResponse` 信封（Transport 的 `RawResponse`+`decode` 吸收）。
- 端点错误带 `request_id`（旧 `ServerError`/`ClientError` 不带）——净增益。
- ws_client 自动获得 tracing span / feishu_code 映射。
- `WsClientError` endpoint 错误从三种形状收敛为一种。

### 负面

- `WsClientError` 公开 variant 增减 = SemVer-major（移 2 + 负载换型 1）→ 须搭 0.19 窗口 + CHANGELOG 迁移项。
- ws_client 新增对 `core::http::Transport` 的依赖（方向正确 client→core，但耦合度从「借 Config 标量袋」升为「走 HTTP 管线」；端点是唯一 HTTP 点，耦合局部）。
- variant 名 `RequestError` 现承载 `CoreError`，语义略宽（含 feishu 业务码失败，非纯传输错）；doc 需相应调整。若评审偏好更直白命名（如 `EndpointTransport(CoreError)`）可在实施期定，不影响决策。

### 非目标（范围守卫）

- **WS upgrade / session / frame_handler / dispatcher 一律不动**——只收口端点发现的 HTTP POST。
- **error 系统候选 2/3（core/error 4882 行并行装置、client/error.rs 转发壳）→ 不碰**，与本 ADR 互不阻塞、另案 grilling。
- **leaf builder API → 不动**（ADR-0001 硬约束；本 ADR 纯 ws_client + 一个 core 公开入口复用）。
- **ws_client 的 `reqwest` 直接依赖是否可随之下沉**：路由后 ws_client 不再自建 `reqwest::Client`；是否能让 openlark-client 去掉直接 reqwest 依赖由 `cargo machete` 判定，属实施期清点，非本 ADR 决策。

## 迁移路径（分阶段，TDD）

1. **`session/types.rs`**——`RequestError` 负载换 `#[from] CoreError`；删 `ServerError`/`ClientError` 两 variant；更新 doc。此步单独可编译（无生产者报错即证明零消费者）。
2. **`client.rs` 类型**——`EndPointResponse` 加 `ApiResponseTrait`（默认 `Data` 格式，标准信封）；删 `WsEndpointApiResponse` / `map_ws_api_error` / `extract_endpoint_response`。
3. **`client.rs::get_conn_url` 重写**——构 `ApiRequest::post(END_POINT_URL).with_supported_access_token_types(vec![None]).header("locale","zh").body(RequestData::Json(json!({"AppID":…,"AppSecret":…})))` → `Transport::<EndPointResponse>::request_typed(req, config, None, "ws endpoint").await?` → post-decode 缺字段校验（URL/client_config/service_id）→ `UnexpectedResponse`。`?` 经新 `#[from] CoreError` 自动转 `WsClientError::RequestError`。
4. **测试**——删 `client.rs` 3 个低 seam 单测；open seam 加 2 个（钉 request_id 透传 + 缺 URL→UnexpectedResponse）；`full_session_oversized_frame_is_rejected` 上限 64→512（decision #1 副作用，见决策 #6）。20 full-session 测试行为不变。
5. **CHANGELOG**——0.19 breaking 条目（`WsClientError` variant 变更 + 迁移说明）。
6. **`just check-all` 等价验证**（fmt + clippy×2 [`--all-features` + `--no-default-features`] + test + doc [`-D rustdoc::broken_intra_doc_links`] + machete + msrv）。msrv：无依赖变更预期，但若 machete 发现 reqwest 可从 openlark-client 直接依赖移除，须同步 `.github/msrv/Cargo.lock`。

## 遵循

- **ADR-0002**（auth concern concentration）：本 ADR 是其同源兄弟，复用「None-token bootstrap 经 Transport」「单一 HTTP 出口」判定。
- **ADR-0001**（leaf builder API 100% 冻结）：本 ADR 纯 ws_client + core 公开入口复用，不触该冻结。
- **CLAUDE.md §3**（无投机抽象 / 无死扩展点）：不开新 pub bootstrap seam；`[None]` 显式声明而非新机制。
- **CLAUDE.md §4**（外科手术式改动）：独立 PR，逐阶段最小 diff，不碰 session/WS。
- **CLAUDE.md §5**（验证）：request_id 透传 TDD，红→绿；20 full-session 回归。
- **AGENTS.md 反模式**「不要硬编码 URL」：复用既有 `END_POINT_URL` 命名常量（不新增字面量）。

## 执行记录

ADR-0003 已落地（branch `refactor/0003-ws-endpoint-via-transport`，单 commit 实现 + 测试 + 验证）：

| 阶段 | 产出 |
|------|------|
| variant 收并 | `WsClientError`：移除 `ServerError`/`ClientError`；`RequestError` 负载 `reqwest::Error` → `#[from] CoreError`；`UnexpectedResponse` + 全部 WS 会话 variant 不动 |
| 端点类型 | `EndPointResponse` 加 `impl ApiResponseTrait`（默认 `Data` 解码）；删 `WsEndpointApiResponse` / `map_ws_api_error` / `extract_endpoint_response`（被 `Transport` 的 `RawResponse` + `Response::decode` 吸收） |
| `get_conn_url` 重写 | `ApiRequest::post(END_POINT_URL).with_supported_access_token_types(vec![None]).header("locale","zh").body(json!{AppID/AppSecret})` → `Transport::<EndPointResponse>::request_typed(req, config, None, "ws endpoint")`；`?` 经 `#[from] CoreError` 落 `WsClientError::RequestError` |
| 测试 | 删 `client.rs` 3 个低 seam 单测；在 `full_session_tests.rs` open seam 加 2 个（code!=0+`X-Tt-Logid` → `RequestError(CoreError::Api)` 且 request_id 保住；code:0 缺 URL → `UnexpectedResponse`）；`full_session_oversized_frame_is_rejected` 上限 64→512（端点 bootstrap 响应现受 `max_response_size` 守护，须大于 ~200 字节、仍远小于 4096 测试帧） |
| 依赖清点 | openlark-client 不再直接依赖 `reqwest`（`cargo machete` 清点；移除 Cargo.toml 的 `reqwest` dep 行 + `websocket` feature 条目；仍作 core 传递依赖存在）；`Cargo.lock` 与 `.github/msrv/Cargo.lock` 同步 |
| 文档 | CHANGELOG 0.19 breaking 条目 |

验证（本地全绿）：
- `cargo build --workspace --all-features`；
- `cargo fmt --check`（workspace）；
- `cargo clippy --workspace --all-targets --all-features -- -Dwarnings` 与 `--no-default-features` 两模式均 clean；
- `cargo test --workspace --all-features`（全 `0 failed`，含 ws_client 54 测 + 全仓）；
- `cargo machete`（workspace，无 unused dep）；
- `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --workspace --all-features`（无断链）。
- msrv：移除 openlark-client 直接 reqwest 依赖（解析图未变、reqwest 0.13.3 仍在），已同步 `.github/msrv/Cargo.lock`。

**实施期发现（超出 ADR 原始范围、已在 CHANGELOG 记录）**：端点 bootstrap 响应现与所有 Transport 响应一致地受 `Config::max_response_size` 守护——源于该旋钮在 ws_client 的既有双重复用（HTTP 响应上限 + WS 帧上限，client.rs:66-67），非本 ADR 引入；对真实用户无影响（默认 100MB），仅波及把上限设到极端小的测试值。彻底解耦（WS 帧尺寸独立于 `max_response_size`）另案。
