# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

> WebSocket 公开 API 破坏性变更随 **0.18.0** workspace 版本一并发出（见下 Breaking）。

### Changed

- **client：剩余业务域迁入编译期能力目录（#436 / #423）**：
  `hr` / `ai` / `workflow` / `platform` / `application` / `helpdesk` / `mail` /
  `analytics` / `user` 亦由统一 `capability` catalog 生成；legacy
  `registry/catalog` 业务条目清空。修正 **AI** 诊断 `dependencies` 为 `["auth"]`，
  与 Cargo `ai = ["auth", ...]` 一致（不再误报 `communication`）。
  `Client::registry()` listing 与 catalog / Client 字段集合一致。

- **client：foundational 域迁入编译期能力目录（#435 / #423）**：
  `auth` / `communication` / `docs` / `cardkit` / `meeting` / `security` 的
  Client 字段与 registry 诊断元数据改由统一 `capability` catalog 生成，不再维护
  Client/registry 双声明。禁用 feature 时两处均不产生字段或 entry。

- **client：编译期能力目录 tracer（#434 / #423）**：新增 `capability` catalog，
  以 `bot` 为 tracer 用同一声明同时生成 `Client::bot` 字段与 registry 诊断元数据；
  启用 `bot` feature 时 `client.bot` 可用且 `registry.has_service("bot")`；禁用时
  两处均不可用。其余业务域仍走 `declare_client!` + `registry/catalog.rs` 旧路径。

- **core：加深 `Transport::request` 请求执行（#422 / #430–#433）**：
  - 内部收敛为 `request_execution` 深模块（构建 + 认证 + 解码）；删除纯委托
    `ReqTranslator` 与一行式 `HeaderBuilder`。
  - `ApiResponseTrait` 增加 `requires_payload` / `from_binary` / `from_text` /
    `from_custom`；`ResponseFormat::Text|Custom` 不再静默走 Data 路径；Binary
    不再 `TypeId` 猜测。
  - **行为（0.18 可能波及业务 crate）**：Data/Flatten 在业务 `code==0` 时：
    - 缺少可解析的必需 payload → `Err`（不再 `Ok(data: None)`）。
    - 无 `data` 字段时**仅**通过 `ApiResponseTrait::empty_success()` 提供空成功
      （删除类空 struct 显式 `Some(Self {})`）；**不再**用「能否反序列化 `{}`」探测。
    - 无体成功请用 `()`（`requires_payload() == false`）。
  - 契约测试仅经 `Transport::request` + wiremock。
  - **decode 收尾（Codex review #451）**：`ResponseFormat::as_label` 收为
    `pub(crate)`；Binary/Text/Custom 先识别 HTTP 非 2xx 与业务错误 envelope，
    不再伪装 `code:0`；保留 `X-Request-Id` 等请求标识；Text 使用严格 UTF-8。

### Security

- **Client 构造统一校验 seam（#416 / #413）**：`ClientBuilder::build` 与
  `Client::with_core_config` 共用私有 `with_checked_core_config`——一律执行
  `Config::validate()`（凭据 / URL / Feishu·Lark 域名白名单 / retry）以及 Client
  特有的零超时拒绝（`req_timeout == Some(0)` 失败，`None` 允许）。修复
  `with_core_config` 此前可绕过域名白名单的 SSRF 缺口；非白名单域名须显式
  `allow_custom_base_url(true)`。删除 `client_build_config` 重复弱校验（#415 已将
  配置状态迁至 core `ConfigBuilder`）。校验文案以 core 规范为准（不保证与旧
  client 字符串逐字兼容）。

### Fixed

- **WebSocket 会话结果可观察（#426）**：`LarkWsClient::open` 在远端关闭或传输失败时
  通过 `Result` 返回 `ConnectionClosed` / `WsError`（此前会话几乎总是以
  `Ok(())` 结束）。新增本地 endpoint + WebSocket peer 的完整会话测试 seam。

- **WebSocket 数据帧单一会话路径（#427）**：分包组装 → 事件派发 → 同会话 sink
  写回；移除每帧临时 channel。完整会话测试覆盖多包乱序与缺包。

- **WebSocket 控制帧 / 心跳（#428）**：pong 经内部 `interpret_control_frame`
  解释并更新 ping 间隔；malformed pong / 非法会话状态经
  `MalformedControlFrame` / `InvalidStateTransition` 返回；心跳超时可测。

- **`allow_custom_base_url` 与构造入口一致性（#415–#416）**：Client 两条公开构造路径
  均完整传播自定义域名放行标志，并执行同一白名单规则。

- **`utils::create_config_from_env` 委托 core env 解释（#413 收尾）**：删除手写
  `OPENLARK_*` 二次解析，改为 `check_env_config` 预检 + `Config::from_env()`。
  与 `ConfigBuilder::load_from_env` / `ClientBuilder::from_env` 共用规则；未设
  `OPENLARK_ENABLE_LOG` 时与 core 一致默认为 `true`（此前该工具函数默认为 `false`）。

### Breaking

- **WebSocket 公开面收缩（#429）与单一 Session（#421）→ 0.18.0**：`ws_client`
  仅 re-export `LarkWsClient` / `EventDispatcherHandler` / `EventHandler` /
  `WsClientError` / `WsClientResult` / `WsCloseReason` / `InvalidStateKind`。
  内部为单 `select!` 会话 + **串行** handler worker（`spawn_blocking`，保序）。

  **设计收缩，非常规废弃**（对照 `docs/PUBLIC_API_STABILITY_POLICY.md`：主动将
  实现渗漏移出 public API，属跨 minor 可接受的公开面收敛，而非安全/正确性紧急
  例外；0.18 直接移除而非先 `#[deprecated]`）。**迁移表**：

  | 旧 import | 替代 |
  |-----------|------|
  | `FrameHandler` / `FrameType` | 勿直接用；经 `LarkWsClient::open` |
  | `WebSocketStateMachine` / `ConnectionState` / `StateMachineEvent` | 内部；观察 `open` 的 `Result` |
  | `ClientConfig` / `EndPointResponse` / `WsEvent` | 内部 |
  | `open().await?` 期望常驻 | 匹配 `Err(ConnectionClosed{..})` 作为正常断开 |
  | 状态错误字符串匹配 | `Err(InvalidStateTransition { kind })` + `InvalidStateKind`（#428 可 match） |

  `InvalidStateKind` 为 #428「状态错误可 match」保留的公开枚举；配合
  `WsClientError::InvalidStateTransition`，勿依赖其 `Display` 文案做分支。

- **WebSocket 协议错误更严格**：malformed pong、未知 frame method 结束会话
  （`MalformedControlFrame` / `InvalidFrameMethod`）；另增 `HandlerPanicked` /
  `BacklogFull`。

- **#350 P9 接口形状撒谎修正（workflow + analytics；platform/user 已先行）**：
  - **workflow**：`approve_task`/`reject_task`/`resubmit_task` 原丢弃真实响应并恒返回
    `ApprovalTaskActionResult { success: true }`（`success: false` 永不达）。改为
    `SDKResult<()>`——成功/失败只由 `Result` 表达；删除 `ApprovalTaskActionResult`。
    飞书 approval v4 同意/拒绝/重提响应 data 为空，与 `()` 一致。**迁移**：
    `let r = service.approve_task(...).await?; r.success` → `service.approve_task(...).await?`。
  - **analytics**：
    1. 删除 `search/v2/query.rs` 与 `search/v2/user.rs` 恒 `Err` runtime stub
       （`QueryApi`/`UserSearchApi`/`SearchRequest`/`SuggestRequest`/`SearchUserRequest`）。
       无已验证飞书端点（与 #2fab71234 / #108 约束一致：不发明未验证端点）；setter 死值 +
       `execute()` 恒失败是接口撒谎。与 #308 删除 `Search`/`SearchV2` 门面死链同向收口。
       **迁移**：这些 stub 从未接线，不是其它 leaf 的别名——请改用已实现的 search leaf
       （`doc_wiki`/`schema`/`app`/`message`/`data_source`）。**用户搜索仍无 surface**。
    2. `AnalyticsService::new` 误导签名 `SDKResult<Self>` 但函数体永远 `Ok(...)` → 改为 `Self`
       （同 platform #373 / user #360）。**迁移**：`AnalyticsService::new(config)?` / client
       facade 去 `?`。
  - **platform / user（已合入）**：`PlatformService::new`（#373）、`UserService::new`（#360）
    误导 `SDKResult<Self>` → `Self`，不在本变更重复。

- **meeting_room 17 叶 `execute()` 返回类型 `Value` → typed Response**（#349）：
  `meeting_room/{building,room,country,district,freebusy,instance,summary}` 全部
  `execute()` / `execute_with_options()` 从 `SDKResult<serde_json::Value>` 改为 typed
  Response（如 `ListBuildingResponse` / `BatchGetFreebusyResponse` / `DeleteBuildingResponse`）。
  字段对齐飞书历史版文档 Response body example（该类文档为 GuideDocumentType，无结构化
  apiSchema）；无 `data` 的写操作（update/delete/instance reply）在缺省时返回
  `Default` 空响应。**v0.18 breaking**：调用方需按新 typed Response 取值，不可再按
  `resp["field"]` 索引。请求 body 仍为 `serde_json::Value`（后续可独立 typed）。

- **删除 `openlark-user` 幻影 `SystemStatusResource::get()` / `SystemStatusGetRequest`（#377）**：
  飞书 `personal_settings/v1/system_status` 仅有 6 个 API（batch_close / batch_open /
  create / delete / list / patch），无 `get`（"获取系统状态"对应 `list`）。既有
  `get` 实现 URL 畸形（双段 `personal-settings` + 连字符 + `/get` 后缀），调用即失败。
  直接移除 `get.rs`、`SystemStatusGetRequest`/`SystemStatusGetResponse`、以及
  `SystemStatusResource::get()` accessor；文档与契约测试改为 6 个真实构建器。
  **无迁移路径**：旧 `get` 对真实飞书本就 404；请改用 `list()` /
  `SystemStatusListRequest`。

- **删除 `openlark-helpdesk` 幻影/孤儿 API（#380）**：#351 helpdesk e2e catalog 核对发现
  (1) `faq/faq_image` 与 `faq/image` 生产代码完全重复且从未 re-export/挂到 `Faq`——删除孤儿
  `faq_image` 模块，保留 `image`（含 wiremock e2e）；(2) `notification/list`
  （`GET /open-apis/helpdesk/v1/notifications`，`HelpdeskApiV1::NotificationList`）不在
  `api_list_export.csv` 的 8 个 notification 端点中，官方文档亦无 list——删除 list 源文件、
  `Notification::list()`、公开 re-export 与 endpoint 变体。**无迁移路径**：list 对真实飞书
  无 catalog/文档支撑；FAQ 图片请继续用 `Faq::image()`。

- **删除 `openlark-application` 幻影/残破 stub（#382）**：#351 e2e 化对照
  `api_list_export.csv` 发现约 56 个从未可用的 stub——path 多一层
  （`/open-apis/application/application/...`）、create/delete/patch 滥用 GET、
  或指向 catalog 不存在的端点（`applications/recommended`、`frequently_used`、
  `owner/transfer`、`app_versions/.../contacts_range`、
  `applications/{}/recommend_rules` 等），以及 3 个 method 错误的重复壳
  （`message_push_overview` GET 壳、两处 `contacts_range` GET 壳）。
  批量移除 v1/v6 下对应目录与 `AppApiV1` 幻影 endpoint 枚举；正确实现保留在
  `v6/application/` 资源树及 `app_badge` / `app_recommend_rule` / `scope`。
  **无迁移路径**：旧 path/method 对真实飞书本就 404 或语义错误，调用方应改用
  catalog 对齐的 v6 类型。

### Changed

- **hr attendance 39 真实端点占位测试 → wiremock e2e**（#351 第 15 批，P4 hr attendance）：
  attendance/v1 全子域（group/shift/user_flow/user_task/user_approval/user_daily_shift/
  user_setting/user_stats_*/file/archive_rule/leave_*/approval_info 39）占位 roundtrip →
  wiremock 端到端。覆盖 path 参数 builder、必填 list/date 校验、以及 enum path 中字面 `{}`
  的 `.replace` 行为。域级 mod.rs 聚合占位删除。hire/feishu_people 按子域后续 PR。
  **非 breaking**：纯测试替换/新增。

- **hr okr 37 真实端点占位测试 → wiremock e2e**（#351 第 14 批，P4 hr okr）：
  okr/v1（progress_record/period/period_rule/image/okr/review/user 12）+ okr/v2 全子域
  （category/cycle/objective/key_result/alignment/indicator 25）占位 roundtrip → wiremock 端到端。
  v2 含 Arc<Config> + path builder + `execute(body: Value)` 形态；`user/okr/list` 按代码实际
  path（`/users/{id}/okrs`，非 enum 的 user_okrs/list）mock。域级 mod.rs 聚合占位删除。
  **非 breaking**：纯测试替换/新增。

- **hr performance 21 真实端点占位测试 → wiremock e2e**（#351 第 13 批，P4 hr performance）：
  performance/v1（semester/stage_task/review_data 4）+ performance/v2 全子域（activity/
  additional_information*/indicator/metric_*/question/review_*/reviewee/user_* 17）
  占位 `serde_json` roundtrip → wiremock 端到端。模式同 PR A（Config 非 Arc + enum to_url +
  enable_token_cache(false)）。域级 `mod.rs` 聚合占位删除。okr/attendance/hire/feishu_people
  按子域后续 PR。**非 breaking**：纯测试替换/新增。

- **hr ehr/payroll/compensation 35 真实端点占位测试 → wiremock e2e**（#351 第 12 批，P4 hr PR A）：
  ehr/v1（employee list + attachment get 2）+ payroll/v1 全子域（acct_item/cost_allocation_*/
  datasource/datasource_record/paygroup/payment_activity*/payment_detail 12）+
  compensation/v1 全子域（archive/change_reason/indicator/item*/lump_sum_payment/plan/
  recurring_payment/social_* 21）占位 `serde_json` roundtrip → wiremock 端到端。
  模式：Config 非 Arc + `XxxApiV1` enum `to_url()` + `response.data.ok_or_else` +
  `.enable_token_cache(false)`。已有真实 builder/validation 测试的文件（attachment get、
  social_plan query、datasource_record save）保留并追加 e2e；三域 `mod.rs` 聚合占位测试删除。
  performance/okr/attendance/hire/feishu_people 留后续 PR。**非 breaking**：纯测试替换/新增。

- **docs 清除 38 个 Potemkin 丢弃式测试 + 15 wiremock e2e**（#351 第 11 批，P3 docs PR A）：
  baike（entity extract/match 2）+ ccm/drive（export_task/file version/import_task/media/permission/member 13）
  共 15 文件，每文件含 `let _ = request.execute().await` + `assert!(result.is_ok())` 的 Potemkin 丢弃式——调 execute
  但丢弃返回值，只断言"线程没 panic"（Config 指向真实飞书无凭证，execute 实际返 Err 被丢弃 → 假绿）。
  删除这些假绿测试，保留真实 builder/validation 测试，加 wiremock e2e（Config 非 Arc + enum + extract_response_data）。
  这是 #351 issue 标题「→ test_runtime 端到端」点名的核心反模式（test_runtime 是 Potemkin 封装）。
  e2e 暴露 latent bug（按代码实际行为处理）：`batch_get_tmp_download_url` execute 手拼重复 query
  （core HashMap 不支持重复 key），url `?` 被 Transport encode 成 `%3F`。docs 的 ~107 roundtrip 占位留后续 PR。
  **非 breaking**：纯测试替换。

- **workflow 清除 8 个占位 serde_json roundtrip 测试**（#351 第 10 批，P3 workflow）：
  7 个 `v2/*/models.rs`（纯聚合 struct，0 execute）删除整个 `mod tests` 块；`service.rs` 删除 2 个
  roundtrip 占位保留 4 个真实 builder/action 测试。workflow crate 特殊现状：endpoint 普遍已有
  `test_*_url`（`to_url()` 断言）+ builder 测试（334 测试，部分覆盖 URL 拼装与 builder），**非 roundtrip 占位**，
  故本批只清 roundtrip；endpoint 的完整 wiremock e2e（execute → Transport → 响应解析）作为后续议题。
  **非 breaking**：纯测试删除。

- **platform directory/admin/mdm/tenant/trust_party 32 真实端点占位测试 → wiremock e2e**（#351 第 9 批，P3 platform PR B）：
  directory/v1（department/employee/collaboration_rule 15）+ admin/v1/badge·password（6）+ mdm（3）+ tenant/v2（2）+
  trust_party/v1（4）占位 → wiremock 端到端。admin/v1 audit·users 是显式 stub（PlatformConfig + business_error
  "尚未接入"），删占位保留 stub 测试。common/mod.rs 聚合占位删除。至此 **platform crate 占位测试 0 残留**
  （PR A+B 合计 76 endpoint e2e）。e2e 暴露 latent bug：`tenant/product_assign_info/query.rs` execute 手工
  `url.push('?')` 拼 query 与 Transport 不兼容（应用 `.query()` 方法），测试简化不设 query 参数，bug 留后续。
  **非 breaking**：纯测试新增 + stub 占位删除。

- **platform app_engine/apaas 44 真实端点占位测试 → wiremock e2e**（#351 第 9 批，P3 platform PR A）：
  app_engine/apaas/v1 全子域（application/audit_log/environment_variable/flow/function/object/record/
  record_permission/role + approval_instance/approval_task/user_task + workspace）占位 `serde_json`
  roundtrip → wiremock 端到端。模式：Config 非 Arc + url 字符串 + `response.data.ok_or_else`。
  e2e 覆盖 query 拼装（手工 `params.push`）、POST body 透传、强类型 Response 嵌套 struct 断言。
  directory/admin/mdm/tenant/trust_party 留 PR B。**非 breaking**：纯测试新增。

- **meeting calendar/v4 + meeting_room 40 真实端点占位测试 → wiremock e2e**（#351 第 8 批，P3 meeting PR B）：
  calendar/v4 全子域（calendar / event / exchange_binding / freebusy / setting / timeoff_event）+
  meeting_room 全子域（building / freebusy / instance / room / summary）占位 `serde_json` roundtrip →
  wiremock 端到端。calendar 用 `CalendarApiV4` enum + `to_url()`；meeting_room 用常量 path。
  4 个聚合文件（calendar responses.rs/responses_new.rs、common/chain.rs、meeting_room responses.rs）
  纯 struct 占位测试删除。至此 **meeting crate 占位测试 0 残留**（PR A+B 合计 90 endpoint e2e）。
  e2e 暴露 latent bug（按代码实际行为 mock，不修）：calendar `primary.rs` enum 注 GET 但 execute POST、
  meeting_room 多处 path 单复数不一致 + update 用 POST 非 PATCH。**非 breaking**：纯测试新增。

- **meeting vc/v1 视频会议 50 真实端点占位测试 → wiremock e2e**（#351 第 8 批，P3 meeting PR A）：
  vc/v1 全子域（export/meeting/participant/report/reserve/reserve_config/room/room_level/
  room_config/scope_config/resource_reservation_list）占位 `serde_json` roundtrip → wiremock 端到端。
  vc 统一模式：`VcApiV1` enum + `to_url()` + `extract_response_data` + Config 非 Arc + query 断言。
  信封按 Response struct（裸 Value/强类型单层、`GetDailyReportResponse` 内含 `data` 双层）。
  e2e 暴露 2 个 latent bug（本次按代码实际行为 mock，不修）：`export/download` 硬编码 path 缺 task_id 参数
  （飞书真实 `:task_id/download`）、`get_active_meeting` path 与 enum 不一致（`get_active_meeting` vs `active_meeting`）。
  calendar/v4 + meeting_room 留 PR B。**非 breaking**：纯测试新增 + execute 未动。

- **mail 34 真实端点占位测试 → wiremock e2e + 修 7 个丢弃响应 bug**（#351 第 7 批，P3 mail）：
  mailgroup 的 `patch` / `alias-delete` / `member-delete` / `manager-batch_create` / `manager-batch_delete` /
  `permission_member-delete` / `permission_member-batch_delete`（7 个）execute 用
  `let _resp = ...; Ok(XxxResponse { data: None })` 丢弃响应，修成 `extract_response_data(response, ...)`。
  34 个 API 文件（user_mailbox / mailgroup / public_mailbox / user 全子域）占位 `serde_json` roundtrip →
  wiremock 端到端。mail crate 健康（catalog 107 端点 vs 代码 116 文件，幻影少；无衍生清理 issue）。
  **非 breaking**：丢弃响应修复使行为变正确；e2e 测试纯新增。

- **application 36 真实端点占位测试 → wiremock e2e + 修 27 个丢弃响应 bug**（#351 第 6 批，P3
  application）：`v6/application`（23）/ `v5`（2）/ `v6` 非 app（2）的 `execute_with_options` 用
  `let _resp = Transport::request(...).await?; Ok(XxxResponse { data: None })` 丢弃响应（永远返回空
  data），修成 `resp.data.ok_or_else(|| validation_error(...))`。36 个经 catalog 核对为真实端点的
  API 文件：占位 `serde_json` roundtrip 测试 → wiremock 端到端（Builder → execute → Transport →
  mock → 断言 data + path）。`v1/` + `v6` 非 application 子域 ~60 残破 stub（双 `application/` path
  + create/delete 用 GET）归 #382（v0.18 清理）。**非 breaking**：丢弃响应修复使行为变正确（旧返回
  空 data，无可依赖）；e2e 测试纯新增。

- **ADR 0001 导航壳重设计执行完成**（10 crate / 12 PR：#353-#366）：bot/meeting/mail/helpdesk/analytics/
  user/platform-facade/docs/cardkit/application/workflow 全部按 5 项判定落地（细节见各 PR 及
  `docs/adr/0001-navigation-shell-redesign.md` 执行记录）。platform inception 折叠按 ADR 硬约束 line 105
  （「不改模块树…module 重组作为后续独立议题」）另案 #367。**本条为 ADR 状态/执行记录文档更新，非 breaking**。

- **application v1/app 补齐 4 个声明却未接线的端点（ADR 0001 阶段4）**：
  `application/application/v1/app/mod.rs` 声明了 `create`/`delete`/`list`/`patch` 4 个 `pub mod`
  （leaf builder 已存在）但 `App` struct 只暴露 `get()`。补 `App::create()`/`delete()`/`list()`/`patch()`
  accessor，对齐 `get()` 形态（`new(Arc<Config>)`）。**非 breaking**：纯 additive accessor，leaf 不变。

- **platform `mdm`/`tenant`/`trust_party` 宣布 flat-by-design（ADR 0001 阶段2 facade 缺口收口）**：
  这 3 个域叶子 `new(Config)` 无路径参数（对照 spark `SparkAppService.patch(app_id)` 路径参数绑定，ADR
  判定 #3），加 Service 层会是纯转发 shell（反 ADR）。照 analytics 裁决宣布 flat：直路径访问
  （`crate::mdm::v1::*` 等），`PlatformService` 故意不暴露 accessor。lib.rs 模块文档补齐 7 域清单
  （原仅列 4 域）+ 3 域 mod.rs 加 flat-by-design 说明。**非 breaking**：纯文档，无 API 变更。

- **auth `AuthTokenProvider` 手搓 HTTP 改委托 Transport-based RequestBuilder**（#309）：
  `fetch_token_via_http`（绕过 `Transport` 直接 `config.http_client().post()` 手搓 reqwest + 手解析
  code/msg/token）改为委托 4 个既有 RequestBuilder（AppAccessTokenInternal/AppAccessToken/
  TenantAccessTokenInternal/TenantAccessToken，均经 `Transport::request`）。删除 ~50 行手搓逻辑 +
  4 个硬编码 path 字面量（改用 `AuthApiV3` enum）。恢复 `log_id`/request_id、feishu_code→ErrorCode
  映射、ResponseTracker 可观测性、`ERR_CODE_APP_TICKET_INVALID` 的 app-ticket 自动刷新。
  **非 breaking**：`fetch_token_via_http` 是私有方法，token 获取行为不变（同端点同 token），wiremock 测试全过。

- **Lint 清理**：移除全 workspace 392 处 `#[allow(dead_code)]`（376 处 cruft 删除 + 16 个
  不完整脚手架的死 `config` 字段改名为 `_config` + reserved 注释；跨 platform/ai/analytics/
  user/helpdesk/docs/application）。dead_code lint 信号重新生效。`_config` 均为私有字段，
  不影响公开 API。补全访问器/execute 的工作拆至 #274 / #275 / #276。
- **CI 防复发**：`lint` job 新增 `tools/check_no_dead_code_allows.sh` 检查，禁止非测试代码
  引入 `#[allow(dead_code)]`（本地 `just no-dead-code-allows`）；`#[expect(dead_code)]` 为
  受控的预期死代码豁免。闭环 #267。
- **Transport 边界 hygiene**：移除 12 个业务 crate（analytics/auth/bot/application/
  communication/mail/hr/docs/helpdesk/platform/user/workflow）未用的 `reqwest` 依赖声明，
  并清除这些 crate 的 `[package.metadata.cargo-machete] ignored` 列表里对应的 `"reqwest"`
  项（此前用 ignore 列表承认债务而非删除，是 `cargo machete` 假阴性根因）。
  业务 crate 经 core `Transport<T>` 发请求、源码 0 处直接使用 reqwest（#270 实证）。
  保留例外：`openlark-core`（抽象本体）/ `openlark-client`（装配 + websocket）/ `openlark-webhook`
  （by-design 连接池复用例外，见 ARCHITECTURE.md「Transport HTTP 边界」）。新增
  `tools/check_reqwest_boundary.sh` 守卫并接入 just 与 CI lint job 防回归。
  **非 breaking**：不改公开 API（业务 crate 无 re-export reqwest）。

- **analytics Search / SearchV2 / search() 标 `#[deprecated]`**（#308）：三层 `Arc<Config>`
  导航死胡同（Search → SearchV2 无真实 API 落地）标记 deprecated，note 指明替代路径
  （v2 已实现 leaf：`doc_wiki` / `schema` / `app` / `message` / `data_source` 的 `XxxRequest::new`）。
  配合 v0.18 deprecated 清理节奏，下个 breaking 窗口删除（#350 已删恒 `Err` 的 `query`/`user`
  stub）。**非 breaking**：仅 deprecation warning，旧调用仍可编译。

### Breaking Changes

- **openlark-analytics 修 7 个 search/v2 + report 残缺 stub（#351 第 4 批）**：
  `data_source` delete/patch、`data_source.item` create/delete、`schema` delete/patch、`report.rule.view`
  remove 共 7 个 stub 原本残缺——`new(config)` 不收 id 且 URL 为字面 `{}`（无插值），对真实飞书必然 404、
  不可用。补 id 参数 + `format!` 路径插值，并加 wiremock 端到端覆盖（同 #374/#375/#376 食谱）。**breaking**：
  7 个 `XxxRequest::new(config)` → `new(config, id)`（`DeleteDataSourceItemRequest` 加 2 个 id）。**迁移**：
  调用方补传对应 id。全仓零外部消费者（workspace `cargo check --all-targets` 验证无跨 crate 引用；这些
  stub 此前不可用、无人调用）。

- **openlark-platform 修 `PlatformService::new` 误导签名（#350 P9 platform 子项）**：
  签名 `SDKResult<Self>` 但函数体永远 `Ok(...)`（接口撒谎——调用方据 `Result` 写的错误分支永不达）→ 改为 `Self`
 （非 `Result`，同 user #360 `UserService::new` 修正方向）。**breaking**：`PlatformService::new` 返回类型
 `SDKResult<Self>` → `Self`。**迁移**：`openlark-client` facade（`PlatformClient::new(...)?` → 去 `?`）+ 本 crate
 doctest / 6 个 unit test（`.unwrap()` → 去）已改；仓内零外部残留。

- **openlark-platform 折叠 3 个 module_inception（spark/admin/directory，ADR 0001 #367）**：
  `spark::spark`/`admin::admin`/`directory::directory` 三个同名 inception hop（各 3 行 `pub mod v1;`）折叠：
  `x/x/v1/` 上移到 `x/v1/`，删 inception hop + 空目录。路径 `crate::spark::spark::v1::*` → `crate::spark::v1::*`
 （admin/directory 同），13 处引用重连（src/ 6 + 本 crate 契约测试 7）。删 `lib.rs #![allow(clippy::module_inception)]`
 （3 个 inception 全清，allow 不再需要，是本次折叠的收口证据）。**breaking**：pub 模块路径命名空间移动——经
  `openlark::platform` re-alias，`openlark::platform::{spark::spark,admin::admin,directory::directory}::v1::*` 曾可达
 （v0.18 窗口，对照 #336/#340 okr/v2 迁移）；全仓零外部 FQN 消费者（8-agent 勘察 workflow 验证 SOUND）。
  leaf builder + 路径参数绑定层（`ApaasV1.application(ns).workspace(ws).table(id)`，app_engine/apaas 非同名 inception，未动）100% 保留。

- **openlark-cardkit 合并双导航树：砍死 strict 树 + 解决 CardElementResource 命名碰撞（ADR 0001 阶段3）**：
  cardkit 有两套并行导航树——门面链（common/chain.rs：`CardkitClient → CardkitV1Client → CardResource →
  CardElementResource`，`Arc<Config>` + async helpers）和 strict 死树（cardkit/cardkit/v1：`CardkitV1Service →
  CardService → strict CardElementResource`，`Config` by-value）。strict 树根 `CardkitV1Service::new()` 全仓零调用，
  门面直接调 leaf Request，strict 树是自封闭死循环。砍 strict 树 3 壳（`CardkitV1Service`/`CardService`/
  strict `CardElementResource`）+ `CardElementService` 别名；保门面 twin（disjoint 模块路径，rustc 层面从未碰撞，
  无需 rename）。门面目标链 `client.cardkit.v1.card.create(body)` 100% 保留。**breaking**：移除 pub
  `CardkitV1Service`/`CardService`/strict `CardElementResource`/`CardElementService` alias——全仓零外部引用
 （多 agent 勘察 + 双对抗 reviewer 共识 SOUND）；leaf builder（`*Request*`）+ 模块树不变。

- **openlark-docs 砍 5 个 config-holder 子客户端（ADR 0001 阶段3 扁平收口）**：
  `CcmClient`/`BaseClient`/`BitableClient`/`BaikeClient`/`MinutesClient`（common/chain.rs）是纯 config-holder
  （仅 `config()`，BaseClient 多一个 `bitable()` 路由），与 `DocsClient::config()` 等价冗余。砍 5 struct +
  `DocsClient` 的 `ccm`/`base`/`baike`/`minutes` 4 个 pub 字段，统一 `docs.config()` 直路径。原
  `docs.ccm.config()` / `docs.base.bitable().config()` → `docs.config()`。`DocsClient` ~15 个真 async helper
  （`search_bitable_records_all`/`find_wiki_node_by_path`/`folder_children_pager`/...）100% 保留。
  **breaking**：移除 5 pub struct + 4 pub 字段 + `BaseClient::bitable()`。**迁移**：`docs.ccm.config()` /
  `docs.base.bitable().config()` / `docs.baike.config()` → `docs.config()`；docs doctest/example、openlark-client
  facade doc + `docs_feature_contract` test 已改。

- **openlark-workflow 删 `service.task()`/`service.tasklist()` 冗余双入口（ADR 0001 阶段4）**：
  `WorkflowService::task()`/`tasklist()` 与 `service.v2().task()`/`service.v2().tasklist()` 等价（同
  `v2::task::Task` / `v2::tasklist::Tasklist` 类型），是绕过版本层的冗余捷径。ADR「双入口二选一，留版本化
  路径」：删捷径 accessor，统一经 `service.v2()` 版本路由层。原 `service.task().create()` /
  `service.tasklist().search()` → `service.v2().task().create()` / `service.v2().tasklist().search()`。
  **breaking**：移除 pub `WorkflowService::task()`/`tasklist()`。**迁移**：仅 `openlark-client` facade
  test + 本 crate 2 个捷径 test 用到，已改（test 改走 `.v2()` / 捷径 test 删除）；TaskV2 + leaf builder 不变。

- **openlark-user 砍 `PersonalSettingsResource` 中间层 + 修 `UserService::new` 误导签名（ADR 0001 阶段2）**：
  `PersonalSettingsResource`（personal_settings/mod.rs）是 1:1 单转发壳（仅 `system_status()` 一子资源，
  全仓零调用者）砍除；`UserService` 新增 `system_status()` 直达 `SystemStatusResource`（7 个真实构建器）。
  原 `service.personal_settings().system_status().list()` → `service.system_status().list()`。
  同时修 `UserService::new` 误导签名：签名 `SDKResult<Self>` 但函数体永远 `Ok(...)`（#350 P9 接口撒谎）→
  改为 `Self`（非 Result）。**breaking**：移除 pub `PersonalSettingsResource` + `personal_settings()` accessor；
  `UserService::new` 返回类型 `SDKResult<Self>` → `Self`。**迁移**：仓内仅 `openlark-client` facade
  （`UserClient::new(...)?` → 去 `?`）+ 本 crate doctest/test 受影响；`SystemStatusResource` + 7 leaf builder 不变。

- **openlark-analytics 删 deprecated `Search`/`SearchV2`/`search()` 死链（ADR 0001 阶段2 扁平收口）**：
  `AnalyticsService::search()` → `Search` → `SearchV2` 三层 `Arc<Config>` 纯转发死胡同（`SearchV2` 仅持 `_config`
  无任何 accessor；真实 search API 经直路径 `crate::search::search::v2::<resource>::XxxRequest::new(Arc<Config>)`
  访问）。收口为方案 B 扁平：删 pub `Search`/`SearchV2` struct + `AnalyticsService::search()` accessor（#308 已铺
  `#[deprecated]`，本次落地删除）。**breaking**：移除 pub `Search`/`SearchV2` + `search()`。**迁移**：仓内零外部引用
  （client/tests.rs 的 `.search()` 属 workflow tasklist，非本 crate）；`v2::*` leaf builder API 与模块树不变。

- **openlark-helpdesk 砍 `Helpdesk` 域层转发壳 + 统一 v1() accessor（ADR 0001 阶段2）**：
  `Helpdesk`（helpdesk/helpdesk/mod.rs）纯转发壳（仅 `v1()`，全仓零调用者）砍除。`HelpdeskService` 移除
  `helpdesk()`（→Helpdesk 壳）+ `ticket()` 单独快捷（11 资源中仅 1 个有快捷，访问深度不一致），统一为
  `v1()` → `HelpdeskV1`（ticket/agent/category/faq 等 11 资源扇出）。原 `service.helpdesk().v1().ticket()` /
  `service.ticket()` → `service.v1().ticket()`。**breaking**：移除 pub `Helpdesk` + `helpdesk()`/`ticket()`
  accessor。**迁移**：仓内零外部引用；`HelpdeskV1` + leaf builder API 不变。

- **openlark-mail 砍 `Mail` 域层转发壳 + 统一 v1() accessor（ADR 0001 阶段2）**：
  `Mail`（mail/mail/mod.rs）纯转发壳（仅 `v1()`，全仓零调用者）砍除。`MailService` 移除 `mail()`（→Mail 壳）
  + `mailgroup()` 单独快捷（5 资源中仅 1 个有快捷，访问深度不一致），统一为 `v1()` → `MailV1`（mailgroup/
  public_mailbox/user/user_mailbox/multi_entity 扇出 5）。原 `service.mail().v1().mailgroup()` /
  `service.mailgroup()` → `service.v1().mailgroup()`。**breaking**：移除 pub `Mail` + `mail()`/`mailgroup()`
  accessor。**迁移**：仓内零外部引用；`MailV1` + leaf builder API 不变。

- **openlark-meeting 砍 chain.rs 7 空壳 + 修文档谎言（ADR 0001 阶段1 重灾区）**（#353）：
  `common/chain.rs` 的 `CalendarClient`/`CalendarV4Client`/`CalendarResourceClient`/`MeetingRoomClient`
  + `VcRoomResourceClient`/`VcMeetingResourceClient`/`VcReserveResourceClient` 7 个纯转发空壳
  （承诺资源却零方法——字段暗示 room/meeting/reserve/calendar API 却未接线，真实 builder 经 strict 路径
  `crate::vc::vc::v1::<resource>::*` / `crate::calendar::*` 访问）全砍。`MeetingClient.calendar`/`.meeting_room`
  + `VcV1Client.room`/`.meeting`/`.reserve` 字段移除。保留唯一接线的 `VcNoteResourceClient`（get/subscribe/unsubscribe）。
  修文档谎言：`client.meeting.vc.v1.room.create()`（`VcRoomResourceClient` 空壳无 create）→ 实际可达的 `note.get()`。
  Config 内部 Arc-wrapped（clone O(1)，非深拷贝），无需改 Arc。**breaking**：移除 pub 字段 + 7 空壳 struct。
  **迁移**：仓内零外部引用（仅 client/tests.rs 测 note.*，照过）；leaf builder + VcNote API 不变。

- **openlark-bot 删 `Bot`/`V4`/`BotResource` 3 纯转发壳 + `search_bot()` 直达 leaf**（#354，ADR 0001 阶段1）：
  4 层壳包裹唯一 1 个 API（search），`service.bot().v4().bot().search()` 段名重复 4 跳 → `service.search_bot()` 1 跳。
  删 `Bot`/`V4`/`BotResource` struct（保 `pub mod` 模块树维持 leaf 路径）；`BotService` 加 `search_bot()` 直达
  `SearchBotRequest`（保留 `feature=v4` 门控）。**breaking**：移除 pub `Bot`/`V4`/`BotResource`。**迁移**：仓内零外部引用，
  leaf `SearchBotRequest` API 不变（strict-path 用户零影响），v0.17.x 预发布。

- **`serialize_params` / `extract_response_data` / `ensure_success` 下沉 `openlark_core::api`（canonical）+ ai common 私有化 + `api_url!` 去 macro_export**（#330）：
  通用 HTTP 管道 helper 此前在 10 个业务 crate 各有一份 `common/api_utils.rs`（locality 失守），
  现统一到 `openlark_core::api::{serialize_params, extract_response_data, ensure_success}`（rich 诊断：
  operation/resource/request_id）。10 crate 的 `common::api_utils` 改 re-export core canonical（签名不变，
  调用点零改动）；workflow/docs 保留各自域专用的 `missing_response_data_error`/`request_serialization_error`，
  meeting 保留 `validate_required_field`。`openlark-ai` 的 `pub mod common` 改私有（HTTP helper 不再公开泄漏）；
  各 crate `api_url!` 宏移除 `#[macro_export]`（零生产调用，语义等同 `format!`）。**breaking**：
  `openlark_ai::common::*` 不再公开；`api_url!` 宏不再 export。**迁移**：仓内零外部引用，v0.17.x 预发布。

- **openlark-application 删 dead `Application` wrapper + 补 v5/v6/v7/workplace accessor + 独立 feature 门控**（#312）：
  删除死 pass-through `Application` wrapper（service.rs 已直接构造 `ApplicationV1`，wrapper 零引用）。
  `ApplicationService` 补 `v5()/v6()/v7()/workplace()` accessor（对齐 `v1()` 模式，使「统一入口」名副其实），
  新增 entry struct `ApplicationV5`/`ApplicationV7`/`WorkplaceV1`（+ resource 层）收敛真实请求构建器
  （workplace 仅 v1，`workplace()` 直返 `WorkplaceV1`，不引入单版本中间层）。
  v5/v6/v7 在 Cargo.toml 各自独立 feature 门控（不再搭 `v1` feature 车编译/消失——修 pre-existing bug），
  新增 `workplace` feature；版本入口形状统一为 entry struct。**breaking**：移除 pub `Application` wrapper；
  v5/v6/v7 模块不再随 `v1` 自动启用，需显式 `features=["v5"]` 等。**迁移**：仓内零外部引用，v0.17.x 预发布。

- **openlark-user 删 settings/preferences stub 链 + 补 `personal_settings()` accessor**（#311）：
  删除 `SettingsService` / `PreferencesService` / `SettingsV1` / `PreferencesV1` 及 7 个
  `*Request::execute` business_error stub（始终未接真实端点），同步删除 `settings` / `preferences`
  / `settings-core` / `preferences-core` / `v1` feature 及 `default` / `full` / `user` / `all-user`
  组合别名（无 CI matrix / 下游引用，零 ripple）。门面 `UserService` 改补 `personal_settings()`
  accessor → `PersonalSettingsResource` → `system_status()` 收敛 7 个真实 system_status 请求构建器
  （此前须写 `personal_settings::personal_settings::v1::system_status::*` 三重嵌套全路径）。
  README `openlark-user` 行由「✅ 完成 9 API 用户设置」校正为「7 API 个人设置 system_status」。
  **breaking**：移除 pub stub 类型 + feature。**迁移**：仓内零外部引用，v0.17.x 预发布；外部若有
 消费改用 `UserService::personal_settings().system_status()` 访问真实 system_status。

- **okr/v2 25 叶 `execute()` 返回类型 `Value` → typed Response**（#328）：`objective::get` /
  `cycle::list` / `key_result::patch` 等 25 个叶子的 `execute()` / `execute_with_options()` 返回
  从 `SDKResult<serde_json::Value>` 改为 typed Response（如 `SDKResult<GetObjectiveResponse>`），
  字段对齐飞书 OKR v2 官方文档（每叶 `docPath`）。填补 okr/v2「可导航但全无类型」缺口，与另 7 域
  一致收敛到「可导航 + typed」。**breaking**：公开 API 源码级返回类型变更，消费方需改接收类型
  （`Value` → `GetObjectiveResponse` 等）。**迁移**：okr/v2 为零外部引用的导航终点（#327/#328
  已确认），v0.17.x 预发布，影响可控；外部若有消费按新 typed Response 改类型即可。深嵌套字段
  （富文本/备注）的 typed 化见下条 #339。

- **okr/v2 `content` / `notes` 深嵌套字段 `Value` → typed `ContentBlock`**（#339）：`Objective.content` /
  `Objective.notes` / `KeyResult.content` / `KeyResultProgress.content` / `Progress.content` 共 5 个
  `Option<serde_json::Value>` + TODO 字段改为 `Option<ContentBlock>`。`ContentBlock`（14 struct 树：
  blocks → paragraph|gallery → textRun|docsLink|mention → style/color/link/...）从飞书 apiSchema
  `objectName=content_block` 派生，定义在 `common/models.rs` 被 5 字段共享（#336 消重所赐，改一处）。
  判别联合 tag 用 `String`（非 enum）容忍飞书未来新增 block 类型。**breaking**：公开 Response 字段
  类型变更（`Option<Value>` → `Option<ContentBlock>`），消费方读这些字段需改类型。**迁移**：okr/v2
  仓内零外部引用，v0.17.x 预发布，影响可控。至此 okr/v2 grep `Option<serde_json::Value>` 残留为 0。

- **okr/v2 跨叶共享 domain struct 路径统一到 `common::models`**（#336）：`Objective`/`ObjectiveOwner`、
  `Indicator`/`IndicatorOwner`/`IndicatorUnit`、`KeyResult`/`KeyResultOwner`、`Alignment`/`AlignmentOwner`
  9 个 struct 跨 11 叶 byte-identical 重复（#328 typed Response 产物），各只在
  `openlark-hr::okr::okr::v2::common::models` 定义一次；叶子改
  `use crate::okr::okr::v2::common::models::<Struct>;` 显式具名引用（非 glob——repo clippy
  `wildcard_imports` + CI `-D warnings` 会 deny glob）。per-leaf Response wrapper（`GetObjectiveResponse`
  等）保持 inline。纯机械重构，反序列化零变化（4 canonical struct byte-identical 验证）。
  **breaking**：9 struct 公开路径 `<leaf>::<Struct>` → `common::models::<Struct>`（D3 clean break，不留
  `pub use` re-export）。**迁移**：okr/v2 仓内零外部引用（#327/#328 已确认），v0.17.x 预发布；外部若有
  消费按新路径改 import 即可。为 #339 深字段 typed 化扫清 Shotgun Surgery（改一处而非 N 处）。

- **删除 ai 4 个死外导航 struct**（#329）：`DocumentAi` / `OpticalCharRecognition` /
  `SpeechToText` / `Translation` 被 service.rs 的 `*Client` 穿透绕过（pub 声明承诺导航实则无接收者），
  各自带自构造测试（Potemkin）。删除 struct + impl + 自测，保留 `pub mod v1`（真实 API，service.rs
  经 `*Client.v1()` 访问）。**迁移**：零消费方（service.rs 用 `*Client`，不引用导航 struct），删除
  strictly safe。区别于 #275（v1 内层孤儿），同属 ai-crate untangle。

- **删除 `openlark-security::models` Potemkin 层**（#326）：models/（~1085 行：acs.rs /
  security_and_compliance.rs / common.rs + PageRequest / Status 等死类型）零消费——src/acs/ 与
  src/security/（真实实现）从不 import models::*，74 处 execute() 返 Value，4 处 typed 返回用
  API 文件本地类型。唯一 live 类型 `SecurityConfig` carve-out 到 `src/config.rs`。删 794 行契约测试
  （serde 自洽，无 HTTP mock，给死类型盖"CI 绿=活着"戳）。prelude glob → 显式 `SecurityConfig`。
  **迁移**：零消费方，删除 strictly safe。

- **删除 `openlark-protocol` 幻影 crate**（#325）：仓内 protocol 是已发布 crates.io
  `lark-websocket-protobuf`（同作者 ZoOL）的死壳复制品——零 import（除自测），现网 WS 栈全用
  外部 `lark_websocket_protobuf`。删除整 crate（workspace members + workspace dep + core optional
  dep / `websocket` feature / cargo-machete ignore + CI `test_openlark_protocol_missing_docs` 脚本 +
  release publish 步骤 + `.github/msrv/Cargo.lock` 同步）。**迁移**：WebSocket 用户无感知（已用
  `lark-websocket-protobuf`，deletion test 全绿）。crates.io `openlark-protocol` 0.17.0 由 owner
  后续 `cargo yank` + 文档 deprecate（第 2 步，owner 决策）。

- **webhook 统一发送管道 + `WebhookClient` 改薄 wrapper**（#310）：提取共享 `post_payload`
  helper（validate / sign / POST / deserialize），消除 `SendWebhookMessageRequest::execute` 与
  `WebhookClient::send` 的 ~40 行逐字重复。`SendWebhookMessageRequest` 增 `.raw(Value)` +
  `.with_client(reqwest::Client)`（解除 shared_client 限制）。`WebhookClient` 改为 Request 的薄
  wrapper（`send` 委托 `Request::raw + with_client + execute`）。**breaking**：移除 `WebhookClient`
  的 5 个 inline-json 构造器（`send_text` / `send_post` / `send_image` / `send_file` / `send_card`）。
  **迁移**：`client.send_text(url, text)` → `SendWebhookMessageRequest::new(url).text(text).execute()`
  或 `client.send(url, json!({"msg_type":"text","content":{"text":text}}))`。

- **移除 `trait_system` 死 seam 及三处复制宏**（#301 死码清理，归 #277/#299 系列）：
  `openlark-core::trait_system`（`Service` + `ExecutableBuilder` trait）及 `impl_executable_builder!` /
  `impl_executable_builder_owned!` / `impl_full_service!` 等 `#[macro_export]` 宏，全仓**零调用零实现**
  （唯一实现是 core 的 `#[cfg(test)]` mock）。`openlark-docs` / `openlark-meeting` / `openlark-hr` 各自
  复制的 `macros.rs` 同样零调用。所有业务请求 builder 一律用 inherent `execute`，从不走 trait 分发。
  **迁移**：无需迁移——这些 trait/宏从未被内部或外部调用（全仓 grep 零命中），删除是 strictly safe
  的清理。pre-1.0、minor bump 足矣。

- **移除 `openlark-client::types` 孤儿类型层**（#302 死码清理）：
  `openlark-client/src/types/`（`ApiResponse` trait + `ApiResponseData` / `PaginatedResponse` /
  `RequestOptions` + auth 的 `AccessToken` / `TokenInfo` 等 9 个 pub 项）是与 `core::http::Transport`
  和 `openlark-auth` 真实类型平行竞争的孤儿层——全仓（含 client 自身）**零引用**，且其信封形状
  `{data, success, ...}` 无法解析真实飞书 `{code, msg, data}` 响应。删除整个 `types/` 模块。
  **迁移**：无需迁移——零消费方，删除 strictly safe。pre-1.0、minor bump。

- **移除 webhook `robot/v1/models.rs` 死模型集**（#305 死码清理）：
  `TextMessage` / `CardMessage` / `MessageContent` 三类型无 send path（send 管道
  自包含于 `crate::models`），从 prelude 与 robot/v1/mod 移除。**迁移**：无需迁移——
  这些类型本就无法发送，零消费方。prelude 移除 3 类型，minor breaking。

- **移除 `openlark-auth` 死 feature + 死依赖**（#306 死码清理）：
  `[features]` 段 6 feature（cache / encryption / monitoring / oauth / token-management /
  advanced-cache）门控 0 行代码（auth src 零 `cfg(feature)`），却拉入 ring / sha2 / hmac /
  pbkdf2 / url / criterion 死依赖。删 `[features]` + 死依赖 + cargo-machete ignored 对应 6 项。
  cache token_provider 实现保留（不用 `cfg(feature)`）。**迁移**：无需迁移——feature 从未
  被代码门控，删除 strictly safe。

- **openlark-core 移除 `tracing-init` / `otel` feature 及直接依赖**（#277 inner-attribute 收尾）：
  `openlark-core` 的 `tracing-init` 与 `otel` feature 仅门控已删的 `observability.rs` 死代码（0 引用），移除。
  连带删 4 个直接依赖（`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`）
  与 `tracing-subscriber` 直接引用（根 `[workspace.dependencies]` 同步；`tracing-subscriber` 仍可能作为
  `tracing-test` 的传递依赖出现）。`testing` feature **保留**并解耦为 `testing = []`（不再拉 `tracing-init`，
  因 `pub mod testing` 被 hr/docs 测试大量使用）。`observability` 模块现仅保留被 `response_handler` 使用
  的 `ResponseTracker`。根 crate `openlark` 的 `otel = ["openlark-core/otel"]` 转发 feature 同步移除。
  **迁移**：若启用过 `tracing-init`/`otel` feature，直接从 `Cargo.toml` 移除即可，无行为变化
  （原 feature 只编译死代码）。`tracing` 本体与其他 feature 不受影响。

- **platform app_engine 请求类型统一 RequestBuilder**（#271 app_engine 批，软 breaking，**最后一批**）：
  openlark-platform app_engine/apaas 子系统 51 个请求 builder XxxBuilder → XxxRequestBuilder，
  旧名作 #[deprecated] alias。**本批完成 #271 全部 platform crate 统一**。


- **platform directory 请求类型统一 RequestBuilder**（#271 directory 批，软 breaking）：
  openlark-platform directory 子系统 21 个请求 builder XxxBuilder → XxxRequestBuilder（Collaboration/Department/Employee 系列），旧名作 #[deprecated] alias。


- **platform admin 请求类型统一 RequestBuilder**（#271 admin 批，软 breaking）：
  openlark-platform admin 子系统 14 个请求 builder XxxBuilder → XxxRequestBuilder（Badge/Grant/Stat/AuditInfo/ResetPassword 系列），旧名作 #[deprecated] alias。


- **platform 小批请求类型统一 `RequestBuilder` 后缀**（#271 platform 批 1，软 breaking）：
  openlark-platform 的 trust_party/mdm/tenant/spark 子系统 12 个请求 builder `XxxBuilder`
  重命名为 `XxxRequestBuilder`（含 `UserAuthDataRelationBind/Unbind`、`Collaboration*`、
  `CountryRegion*`、`DirectoryUserIdConvert`、`TenantQuery`、`AssignInfoListQuery`、
  `VisibleOrganization`），旧名作 `#[deprecated]` type alias 保留至 v1.0。

- **application+docs 请求类型统一 `RequestBuilder` 后缀**（#271 批次，软 breaking）：
  openlark-application 的 3 个（`AccessDataSearchBlock`/`AccessDataSearchCustom`/`AccessDataSearchWorkplace`）
  与 openlark-docs 的 1 个（`PatchFormFieldQuestion`）请求 builder `XxxBuilder` 重命名为
  `XxxRequestBuilder`，旧名作 `#[deprecated]` type alias 保留至 v1.0。body 模型不动；
  `RecordFieldsBuilder`（真 builder）不动。迁移：`XxxBuilder` → `XxxRequestBuilder`。

- **auth 请求类型统一 `RequestBuilder` 后缀**（#271 pilot，软 breaking）：openlark-auth 的
  12 个请求类型 builder `XxxBuilder` 重命名为规范 `XxxRequestBuilder`，旧名作 `#[deprecated]`
  type alias 保留至 v1.0（调用方用旧名仍可编译，仅 deprecation warning）。涉及
  `AppAccessToken`/`AppAccessTokenInternal`/`AppTicketResend`/`Authorization`/`IdentityCreate`/
  `OidcAccessToken`/`OidcRefreshAccessToken`/`RefreshUserAccessTokenV1`/`TenantAccessToken`/
  `UserAccessTokenV1`/`UserInfo`/`VerificationGet` 各 `Builder` → `RequestBuilder`；
  `TenantAccessTokenInternalRequestBuilder` 已是目标形式（不动）。body 模型（`XxxRequest`）不动。
  `AuthorizationUrlBuilder`（URL builder）不动。迁移：`XxxBuilder` → `XxxRequestBuilder`。
  方向说明：原拟「→Request」，实证发现 5/13 撞 body 模型名，改「→RequestBuilder」对齐 helpdesk。
  v1.0 移除 alias。

- **Removed deprecated wiki Params**：移除 `SearchWikiParams` / `ListWikiSpacesParams` /
  `CreateWikiSpaceParams` / `MoveDocsToWikiParams`（deprecated since 0.16.0）→ 用对应
  `XxxRequest` 流式 Builder。无生产用法（仅兼容测试，一并删除）。关联 #268（B）。

- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移
  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。

- **Removed deprecated tenant_access_token legacy 链**：移除 `TenantAccessTokenBuilder`
  的 `app_id`/`app_secret`/`app_ticket` 旧链式入口及两步换取逻辑（deprecated legacy
  chain）→ 迁移：先 `AppAccessTokenBuilder` 取 `app_access_token`，再
  `TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)`。关联 #278。

- **Removed docs deprecated 方法**：移除 `RecordFieldValue::to_value()`（deprecated since 0.15.0，→ 直接用 `RecordFieldValue` 类型）与 `impl_required_builder!` 宏生成的 `new()`（deprecated since 0.5.0，→ 用 `builder()`）。均零调用/dead。关联 #278（D+C 子集）。

- **Removed deprecated 兼容访问器**：移除 `Hr` 的 8 个 service 访问器方法
  （`attendance()`/`corehr()`/`compensation()`/`payroll()`/`performance()`/`okr()`/`hire()`/`ehr()`）
  与 `SearchV2` 的 `query()`/`user()` 未接线存根（deprecated since 0.15.0）。迁移：

  | 旧（移除） | 新 |
  |---|---|
  | `hr.attendance()` / `corehr()` / ... | `hr.attendance` / `hr.corehr` / ...（字段访问） |
  | `search_v2.query()` | 用 `doc_wiki` / `schema` / `app` / `message` surface |
  | `search_v2.user()` | 无 surface（user-search 未实现）；`UserSearchApi` 经完整路径可达，`execute()` 显式返回未接线错误 |

  `QueryApi`/`UserSearchApi` 类型保留（仅移除便捷存根访问器）。关联 #268。

- **Removed** `openlark_client::Config` / `ConfigBuilder` / `ConfigSummary` (deprecated
  since 0.17.0). All functionality is merged into `openlark_core::config::Config`. The
  root crate `openlark::Config` now re-exports `openlark_core::config::Config` directly.
- **`Client::with_config(client::Config)`** removed — use `Client::with_core_config(core::Config)`
  or `Client::builder()`.
- **`From<client::Config> for Result<Client>`** removed.
- **WebSocket** `LarkWsClient::open` now takes `Arc<openlark_core::config::Config>`
  (was `Arc<openlark_client::Config>`).

### Migration: `client::Config` → `core::Config`

| v0.17 (`openlark_client::Config`) | v0.18 (`openlark_core::config::Config`) |
|---|---|
| `timeout: Duration` (default 30s) | `req_timeout: Option<Duration>` (default `None` = never timeout) |
| `headers: HashMap` | `header: HashMap` (singular) |
| `Config::builder().build()` → `Result<Config>` (validates) | `Config::builder().build()` → `Config` (no validation); call `.validate()` explicitly |
| `Config::from_env()` | `Config::from_env()` (now on core; same `OPENLARK_*` vars) |
| `Client::with_config(cfg)` | `Client::with_core_config(cfg)` |
| `config.app_id` (public field) | `config.app_id()` (accessor; `ConfigInner` fields are `pub(crate)`) |
| base_url whitelist SSRF (client-only) | preserved via `Config::validate()` + `allow_custom_base_url` |

Set `.allow_custom_base_url(true)` on the builder to use a non-whitelisted base_url
(known domains: `*.feishu.cn`, `*.larksuite.com`, `*.larkoffice.com`).

### Added

- `openlark_core::config::Config::validate()` + `is_known_base_url()` — base_url whitelist
  SSRF protection migrated from client (previously client-only).
- `openlark_core::config::Config::from_env()` / `load_from_env()` — env-var loading migrated
  from client; `OPENLARK_TIMEOUT` (seconds) now maps to `req_timeout(Some(Duration))`.
- `openlark_core::config::ConfigSummary` + `Config::summary()` — redacts `app_secret`.
- `openlark_core::config::ConfigInner.allow_custom_base_url` field + builder method.

### Fixed

- **fix(platform)**: 移除 `openlark-platform` 四个 service（Admin/AppEngine/Directory/Spark）
  facade 与 intermediate 层多余的 `#[cfg(feature = "v1")]` 门控。此前 `default`/`full`
  feature 下 service 启用却暴露空壳 facade（四个 service 的全部 v1 API 实现被排除在标准构建外）。
  移除后 "service 启用 = API 可达"，与 hr/communication/meeting 一致。行为补全，非 breaking：
  仅让原本不可达的公开 API 变为可达，不移除任何符号。`v1` feature 保留（测试依赖）。

### Security

- **升级 anyhow 1.0.102 → 1.0.103**（修复 RUSTSEC-2026-0190）：1.0.102 的
  `Error::downcast_mut()` 在 `Error::context` 后调用时违反借用规则（UB）。patch 版本升级，
  无 breaking。CI security-audit（cargo-deny）恢复绿。

## [0.17.0] - 2026-05-30

### Breaking Changes

- **Removed** all v0.15.0 deprecated crate re-exports (`open_lark::openlark_client`,
  `open_lark::openlark_core`, `open_lark::openlark_auth`, etc.)
- **Removed** all v0.15.0 deprecated `*Client` type aliases from root crate
  (`AuthClient`, `DocsClient`, `HrClient`, etc.). Use `client.auth`, `client.docs`,
  `client.hr` field access instead.
- **Cleaned** root `prelude` — no longer exports deprecated `*Client` aliases
- **Cleaned** root and `openlark-client` preludes — no longer export deprecated
  `openlark_client::Config`; use `CoreConfig` or `Client::builder()` instead.
- **`Client.config()` now returns `&openlark_core::config::Config`** (was `&openlark_client::Config`).
  Access fields via methods: `client.config().app_id()`, `client.config().base_url()`, etc.
- **Migration**: Replace `use open_lark::AuthClient` → access via `client.auth` field
  or use `open_lark::auth` module namespace directly. Replace `client.config().app_id`
  → `client.config().app_id()` (method call, not field access)

### Deprecated

- `openlark_client::Config` — planned for removal after the migration window.
  Use `Client::builder()` or `openlark_core::config::Config` directly.

### Added

- `SecurityClient` struct in `openlark-security` — proper wrapper with `Deref`
  to `SecurityServices` (replaces `Arc<SecurityServices>` alias)
- `XxxClient` type aliases in all business crates for consistent naming:
  `WorkflowClient`, `PlatformClient`, `ApplicationClient`, `HelpdeskClient`,
  `MailClient`, `AnalyticsClient`, `UserClient`
- `[package.metadata.docs.rs]` configuration for complete documentation generation
- `docs/CLIENT_NAMING_CONVENTION.md` — naming convention documentation

### Changed

- `openlark-core` no longer enables `testing` feature by default. Crates using
  `openlark_core::testing` in tests must add `openlark-core = { features = ["testing"] }`
  to their `[dev-dependencies]`.
- All business crate `XxxClient` types now exported from source crates instead of
  defined as type aliases in `openlark-client`

### Removed

- `#![allow(async_fn_in_trait)]` from `openlark-client` (MSRV 1.88 no longer needs it)

### Compatibility

#### Typed APIs

#### Helpers & Convenience Methods

#### Breaking Changes

#### Deprecations

#### Migration Notes

### Added

### Changed

### Fixed

## [0.16.1] - 2026-05-20

### Added

- **feat(api)**: 同步 70 个新增飞书 API catalog 条目，并补齐 application v7、docs drive/minutes、HR corehr v2、IM reaction、mail v1、meeting v1、spark v1 等 typed SDK 模块。
- **coverage(api)**: 将 application、communication、docs、hr、mail、meeting、platform 等相关 crate 的 strict API 覆盖率恢复到 0 missing APIs。

### Changed

- **build(deps)**: 放宽 `uuid` 与 `serde_with` 的精确版本锁定，降低下游依赖解析冲突风险。

## [0.16.0] - 2026-05-10

### 🔄 变更

- **style(fmt)**: 统一代码格式化，修复多个文件的格式问题
- **docs(docPath)**: 为 260+ 个 API 文件补充 docPath 官方文档链接
- **refactor(api)**: 删除 explorer/permission v2 的函数式 API，统一使用 Builder 模式
- **refactor(api)**: 统一 platform/helpdesk 40 个文件的 execute() 委托模式，消除代码重复
- **refactor(validate)**: 统一 44 个文件的必填字段校验，使用 validate_required! 宏替换手工校验
- **refactor(types)**: 替换 calendar v4 的 serde_json::Value 为强类型结构体
- **fix(exports)**: 补充 5 个 mod.rs 文件的模型显式导出
- **fix(url)**: 修复 exchange_binding/get.rs 的 API 端点路径拼写错误
- **ci**: 修复 clippy 警告和文档注释缺失
- **build(deps)**: 升级安全相关依赖（tokio-tungstenite、url、reqwest）
- **build(rust)**: 对齐 Rust 2024 / MSRV 1.88

### 🐛 修复

- **fix(ci)**: 修复 CI 持续失败问题（clippy 警告、格式问题）
- **fix(security)**: 添加 max_response_size / ResponseTooLarge HTTP 与 WebSocket 响应大小限制
- **fix(security)**: Token/PII 日志脱敏
- **fix(security)**: path 参数 percent-encoding 安全修复
- **fix(code)**: 生产代码与测试代码 unwrap() 清理
- **fix(auth)**: AuthTokenProvider 多租户缓存 key 修复

## [0.15.0] - 2026-04-05

### 🔄 变更

- **release(root)**: 将工作区版本与根 crate 文案切换到 `0.15.0`
- **docs(release)**: 新增 `0.15` 迁移指南、public API 稳定性策略与正式版发布清障清单
- **ci(examples)**: 为根 README 对齐示例与主推 examples 增加编译校验入口
- **build(metadata)**: 修正 crates.io 元数据中的仓库与文档链接

### 🐛 修复

- **fix(release)**: 修复 GitHub Release 对 RC tag 一律标为正式版的问题
- **fix(docs)**: 修复多个 crate README 中错误的 crate 名、仓库链接和过期版本示例

## [0.15.0-rc.2] - 2026-03-26

### 🔄 变更

- **refactor(root)**: 将 `openlark` 收敛为唯一官方入口 crate，直接导出 `Client`、`ClientBuilder`、`Config`、`RequestOption`、`CoreError` 等高频类型
- **refactor(features)**: 重构根 crate feature 模型，移除面向用户的 `client`/`protocol` 心智，统一为业务 feature、技术 feature 与组合 feature
- **refactor(client)**: 将 `openlark-client` 明确为高级入口，不再作为普通用户的默认接入方式
- **docs(examples)**: 统一 README 和主 examples 到 `openlark` 根入口，修复 `workflow` 示例门槛与实际依赖不一致问题
- **build(lints)**: 将 `workspace.lints` 真正落到各成员 crate，统一工作区 lint 配置

## [0.15.0-rc.1] - 2026-03-17

### ✨ 新增功能

- **feat(webhook)**: 集成 openlark-webhook 模块到工作空间（8 个 API）
  - 自定义机器人、Webhook 事件处理
- **feat(hr)**: 实现 462 个 API (Wave 1-5)，涵盖招聘、CoreHR、考勤、薪酬等模块，总计 562 个 API
- **feat(workflow)**: 完成 workflow 模块 100% API 覆盖（117 个 API）
  - TASK v1 剩余 28 个 API、TASK v2 剩余 24 个 API
  - APPROVAL v4 16 个 API
  - BOARD 模块命名规范修复
- **feat(platform)**: 完成 openlark-platform Transport API 迁移（102 个 API）
- **feat(ai)**: 完成 openlark-ai 模块 27 个 API 实现
- **feat**: 实现缺失的 bizTag API（100% 覆盖）
- **feat(examples)**: 新增长连接 WebSocket Echo 示例并补充测试
- **feat(core)**: 新增测试基础设施模块（testing）
  - `test_runtime()` - 安全的测试运行时
  - `assert_res_ok!`, `assert_err_contains!` 等断言宏
- **feat(client)**: 新增 LazyService 延迟初始化工具
- **docs**: 添加 AGENTS.md 项目知识库

### 🔄 变更

- **refactor(docs)**: 简化 API 入口设计，删除 Service 层，统一 Request 模式
- **refactor(docs)**: 将 glob 重导出转换为显式导出（258 → 7 处）
- **perf(docs)**: 优化 Config 传递，使用 `Arc<Config>` 替代 `Config`
- **refactor(meeting)**: 统一 Request 模式，删除冗余 RequestBuilder
- **refactor(hr)**: 统一架构并添加 feature gating 支持
- **refactor(core)**: 为 testing 模块添加 feature gate
- **refactor(core)**: 清理未使用的空 features，将测试依赖移动到 dev-dependencies
- **refactor**: 实现显式导出系统，消除 251+ 个通配符导出
- **style(security)**: 修复命名规范异常，替换硬编码 URL，统一代码风格

### 🐛 修复

- **fix(core)**: 统一 validate_required 语义，支持字符串 trim
- **fix(docs)**: 修复 sheets_v2 数据读取 API 路径
- **fix(docs)**: 修复 Arc<Config> 类型不匹配错误
- **fix(docs)**: 修复 explorer v2 模块编译错误和导出问题
- **fix(hr)**: 添加 CoreHR 缺失的 API 端点定义并修复语法错误
- **fix**: 修复 no-default-features 编译错误
- **fix**: 修复多个 crate 的代码风格和导出
- **fix(examples)**: 修复 examples 编译错误
- **fix(ci)**: 修复 Coverage 工作流覆盖率收集问题（多次迭代修复）

### 🧪 测试

- 大幅提升测试覆盖率至 ~47%
- 为所有主要模块添加测试：docs、workflow、platform、cardkit、hr、meeting、auth、core
- 为 workflow v1/v2 模块添加完整测试套件
- 迁移 44 个测试文件到新框架，消除 144 处 `Runtime::new().unwrap()`

## [0.15.1] - 2025-11-20

### 🔄 架构优化 - openlark-docs 链式调用支持与 API 覆盖率更新

#### ✅ 核心能力增强

- **🔗 openlark-docs 完整链式调用支持** - 通过 openlark-client 提供流畅的链式调用体验
  - **DocsClient 集成** - Client 结构体包含 DocsClient 字段，启用 `docs` feature 即可使用
  - **完整链式调用路径** - `client.docs.ccm.drive.v1()`, `client.docs.ccm.sheets.v3()`, `client.docs.base.bitable()` 等
  - **类型安全** - 编译时验证所有链式调用路径

#### 📊 openlark-docs API 覆盖率验证

- **✅ 254 个 API，100% 完成** - 全面验证 openlark-docs 的 API 实现情况
  - **✅ 零未完成标记** - 无 TODO/FIXME/unimplemented! 标记
  - **✅ 代码质量优异** - 所有实现文件经过验证，零编译警告

#### 📈 模块实现状态详情

| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |
|------|---------|--------|--------|--------|
| **CCM** | 174 | 174 | 0 | 100% |
| **BASE** | 49 | 49 | 0 | 100% |
| **BAIKE** | 27 | 27 | 0 | 100% |
| **MINUTES** | 4 | 4 | 0 | 100% |

#### 🏗️ 架构优化

- **链式调用架构** - DocsClient 通过字段链式访问所有子服务
  - **模块化设计** - ccm, base, baike, minutes 清晰的功能分层
  - **类型安全接口** - 提供服务访问器方法，如 `client.docs.ccm.drive.v1()`
  - **配置透传** - 支持从 DocsClient 获取底层服务

#### 📋 文档与示例

- **链式调用文档** - 详细的链式调用路径和使用示例
- **API 验证报告** - `docs/API_COVERAGE_REPORT.md` 提供详细的实现状态
- **模块级文档** - 每个 AGENTS.md 提供模块特定的使用指南

#### 🔧 技术债务清理

- **零未完成标记** - 扫描所有 API 文件，无 TODO/FIXME/unimplemented! 标记
- **代码质量优秀** - 所有实现文件通过编译和 lint 检查
- **架构清晰** - 严格的模块划分和命名规范

#### 📊 性能与质量

- **编译性能** - 默认功能 0.6s，全功能验证 0.37s
- **零警告构建** - 所有模块通过 clippy 检查
- **测试覆盖** - 核心功能完整测试覆盖
