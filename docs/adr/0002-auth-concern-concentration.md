# ADR: 鉴权 concern 浓缩进 auth/（决策 / 获取 / 恢复同居 + resend bootstrap 旁路）

- **状态**: Proposed（2026-07-24 `/improve-codebase-architecture` → `/grilling` 达成共识，待实施）
- **日期**: 2026-07-24
- **决策者**: 架构评审 + 用户 grilling 共识
- **来源**: 架构评审候选 #1（auth 策略泄漏到 Transport seam 之外）
- **breaking 窗口**: 无。纯 `openlark-core` 内部移动，零公开 API 破坏。

## 背景

「给一个请求做鉴权」这件事目前摊在 3 个文件、2 个模块里（均已核实）：

**决策（用哪种 token）—— `http.rs` 自由函数**

- `determine_token_type`（`http.rs:239`，~80 行）：marketplace / token-cache 规则。
- `validate_token_type`（`http.rs:210`）：tenant↔user / app↔tenant 冲突校验。
- `validate`（`http.rs:320`）的 auth 授权部分（`L336-366`）：无缓存时 token 必在、Marketplace+Tenant 必带 tenant_key、User 类型必带 user_access_token。

**获取（去 provider 拿 token）—— `request_execution/auth_handler.rs`**

- `AuthHandler::apply_auth`（`auth_handler.rs:15`）→ `apply_app_auth` / `apply_tenant_auth` 调 `config.token_provider().get_token()`。

**恢复（app_ticket 失效重推）—— `auth/app_ticket.rs`，触发点埋在 transport**

- `apply_app_ticket`（`app_ticket.rs:14`，reqwest 直连在 `:21`）。
- 触发：`http.rs:160-162`，`do_request` 内 `if !resp.is_success() && resp.raw_response.code == ERR_CODE_APP_TICKET_INVALID { apply_app_ticket(config).await?; }`。

### 诚实校正：原评审卡片两处承诺经不起代码核对

| 评审卡声称 | 实际 |
|-----------|------|
| 「给 auth 策略一个 test interface（现为不可测自由函数）」 | 三个决策函数是**纯函数**，已被 ~40 个单测覆盖（`http.rs` test 模块 1078 行）。可测性**不是**本 ADR 的驱动力。 |
| 「违背 CI 守卫的『唯一出口』不变式」 | `check_reqwest_boundary.sh` 只查业务 crate 的 `Cargo.toml`，`openlark-core` 在白名单（`tools/check_reqwest_boundary.sh:17`）。`apply_app_ticket` 直连 reqwest **不被 CI 捕获**，且 core 用 reqwest 本就合法。所谓「第二出口」是 **core 内部 seam 不一致**，非跨 crate 泄漏。 |

### 已核实约束

- `AuthHandler` + 三个决策函数**零外部调用者**（`request_execution` 为 `pub(crate)`，全仓 grep 无 crate 外引用）→ 全程内部移动，**零公开 API 破坏**。
- `APPLY_APP_TICKET_PATH = /open-apis/auth/v3/app_ticket/resend`（`constants.rs:41`），`ERR_CODE_APP_TICKET_INVALID = 10012`（`:116`）。resend 是**治愈** 10012 的端点，自身不被 app_ticket 门控 → 不会回 10012。
- core 测试基建齐全：`TestServer`（`testing/mock_server.rs`）+ `TestConfigBuilder`（`testing/fixtures.rs`）+ `transport_request_contract.rs`（集成回归测）。

## 决策

浓缩「决策 + 获取 + 恢复」整条鉴权链进 `auth/`，`Transport` 退回纯 HTTP。五项子决策：

| # | 子决策 | 结论 |
|---|--------|------|
| 1 | resend 出口 | **UnifiedRequestBuilder bootstrap 旁路**——resend 作为 None-token bootstrap 经标准构建路径（header/tracing/timeout 都有），但**不经** validate/determine/recovery → 构造上无递归，关掉 ad-hoc reqwest 出口 |
| 2 | 恢复触发边界 | **α-delegate**——`do_request` 调 `auth::recover_app_ticket_if_needed(&resp, config)`；条件（code==10012）+ 动作 + 出口全在 auth，Transport 按名委托 |
| 3 | `validate()` 拆分 | auth 授权三块（`L336-366`）→ `auth/policy.rs`；Config 校验（`app_id`/`app_secret`，`L325-334`）+ 禁用 header 校验（`L368-379`）**留 Transport**（它们是 config/request 卫生，非鉴权） |
| 4 | 获取环节归位 | `AuthHandler` 从 `request_execution/auth_handler.rs` 搬到 `auth/acquisition.rs` |
| 5 | 模块成形 | `auth/{policy,acquisition,app_ticket,token_provider}.rs`；**无状态化对象、无新 trait、无单实现泛型**（合规 §3） |

分层结果：`auth/`（鉴权策略+获取+恢复）← `request_execution/`（编排：url/header/body/multipart，调 auth）← `http.rs` Transport（入口）。

## 理由

1. **deletion test 通过**：浓缩把鉴权故事移进单一模块，复杂度**集中**而非平移——这正是深化的信号。
2. **第二条 reqwest 出口是真 locality 隐患**：将来给所有请求加 tracing/retry 时，`apply_app_ticket` 会**静默漏掉**（它绕过 `UnifiedRequestBuilder`）。现在收口，避免这种坑。
3. **bootstrap 旁路比「走完整 Transport::request」更稳**：resend 本就是无鉴权 bootstrap，不该 traverse policy+recovery 管线；且因不走恢复路径而**构造上不可能递归**，不靠「端点语义」赌。
4. **α-delegate + 具体命名函数合规 §3**：`recover_app_ticket_if_needed` 是具体命名、单调用点，非泛 `post_process` hook（后者是单实现泛型 = 死扩展点，踩 §3 红线）。Transport 调用点名出 concern、但不编码条件。
5. **诚实定位测试赢点**：policy 函数可测性非新赢点（已测）；本 ADR 真正补上的是**恢复副作用**的测试面（`app_ticket.rs` 当前 0 测试 → 全覆盖）。
6. **`validate()` 拆分避免换地方再 smear**：把 config/header 校验误归为 auth 等于归类错误；Split 让 auth/ 诚实只收「用哪种 token + 这请求对该 token 鉴权齐了吗」。

## 后果

### 正面

- `Transport` 减 ~140 行策略 + 测试，聚焦 HTTP；鉴权 concern 收敛到一个可导航模块。
- 恢复副作用从「埋在 `do_request` 的不可测隐式行为」变为命名可测函数。
- 关闭 core 内部第二条 reqwest 出口；`ARCHITECTURE.md`「唯一 HTTP 出口」措辞对 intra-core 也成真（顺手修正表述，见迁移阶段 6）。
- 分层清晰：auth ← request_execution ← Transport，各司其职。

### 负面

- core 内部 import 图变动：`request_execution` 改为依赖 `auth`（方向正确，auth 更基础）。
- 多一个**有理由的**内部入口（bootstrap resend 经 `UnifiedRequestBuilder`）——bootstrap vs 正常请求是真区分，非死扩展点。
- 决策函数从 `http.rs` 私有跨模块后升 `pub(crate)`（内部，无公开影响）。

### 非目标（范围守卫）

- `app_id`/`app_secret` 每请求查一次的 Config smell（本该在 `Config::build()` 查一次）→ **另案**，本 ADR 不碰，避免范围蔓延。
- 评审候选 #2（endpoint catalog）/ #3（codegen 重连）→ **不碰**，与本 ADR 互不阻塞。
- leaf builder API → **不动**（ADR-0001 硬约束；本 ADR 纯 core-internal）。

## 迁移路径（分阶段，TDD）

1. **`auth/policy.rs`**——搬 `determine_token_type` + `validate_token_type` + `validate()` 的 auth 授权块；测试跟着搬，`validate()` 测试按归属拆分。
2. **`auth/acquisition.rs`**——搬 `AuthHandler` + 其测试（StaticTokenProvider / NoOpTokenProvider adapter 测原样存活）。
3. **`auth/app_ticket.rs`**——新增 `recover_app_ticket_if_needed`（条件 + 动作）+ resend bootstrap 经 `UnifiedRequestBuilder`。**TDD**：先写红测（条件单测 + `TestServer` 集成测断言 resend 真发出），再实现转绿。
4. **`http.rs`**——`do_request` 改调 `auth::recover_app_ticket_if_needed`；`validate()` 瘦身为 config + header 校验；删三个自由函数。
5. **`request_execution/mod.rs`**——`AuthHandler` 改从 `auth` re-import；更新 `UnifiedRequestBuilder` 调用点。
6. **`ARCHITECTURE.md`**——轻量修正「Transport HTTP 边界」措辞，反映 intra-core 出口收口（仅表述，不扩范围）。
7. **`just check-all`** 等价验证（fmt + clippy×2 + test + doc + machete + msrv）。

## 遵循

- **CLAUDE.md §3**（无投机抽象 / 无死扩展点）：具体命名函数，无新 trait，无单实现泛型。
- **CLAUDE.md §4**（外科手术式改动）：leaf API 不动，逐阶段最小 diff。
- **CLAUDE.md §5**（验证）：恢复逻辑 TDD，红→绿。
- **ADR-0001**（leaf builder API 100% 冻结）：本 ADR 纯 core-internal，不触该冻结。
- **AGENTS.md 反模式**「不要硬编码 URL」：resend 复用既有 `APPLY_APP_TICKET_PATH` 常量。
- **`check_reqwest_boundary.sh`** 范围不变（core 仍在白名单；本 ADR 收口的是 intra-core 出口，非该守卫目标）。

## 执行记录

_（待实施后填写：各阶段 PR / commit、CI 三元组 + doc + machete + msrv 验证结果）_
