# ADR: 导航壳重设计（per-layer 5 项职责判定 + per-crate 差异化产出）

- **状态**: Accepted（2026-07-06 全部落地，含 platform inception #371；见文末「执行记录」）
- **日期**: 2026-07-05（决策）/ 2026-07-06（执行完成）
- **决策者**: 首席架构师裁决（深导航 / 砍导航 / 混合 三选一）
- **相关 issue**: #347（多层 Arc<Config>-only 纯转发壳）；执行跟踪见文末「执行记录」
- **breaking 窗口**: v0.18（项目已有 7 个 v0.18 deprecated 归档先例，CHANGELOG 迁移表流程成熟）

## 背景

OpenLark 11 个业务 crate 的导航壳（`Service → Vx → Resource → Leaf builder`）存在系统性 shallow module 反模式。已核实的事实分三档：

**重灾区（指控完全属实）**：

- **bot crate** —— 4 层纯转发壳（`BotService → Bot → V4 → BotResource`）包裹唯一 1 个 API（search）。用户调用链 `service.bot().v4().bot().search()` 中段名 `bot` 出现两次，4 跳才到 builder。已核实：4 个壳文件 `service.rs(37) + bot/mod.rs(24) + v4/mod.rs(24) + v4/bot/mod.rs(24) = 109 行`，全部仅持 `Arc<Config>` 单字段 + 1 accessor，零校验/零默认值/零日志；全仓 grep 确认该长链零外部调用者。`lib.rs:1` 用 `#![allow(clippy::module_inception)]` 抑制 `bot::bot` 重复段名。
- **meeting crate** —— `common/chain.rs` 9 个 Client struct 中 8 个是空壳（已核实：每个仅 `new(Config) + config()`，无业务方法），且是 `Config` 深拷贝链而非 `Arc`。唯一桥接真实 API 的 `VcNoteResourceClient` 证明 chain 与真实模块树（`crate::vc::vc::v1::*`）不对齐（字段链少一个 vc 段）。最严重：官方文档 `openlark-client/docs/meta-api-style.md:99` 与 `openlark-client/src/client.rs:97` 承诺的 `client.meeting.vc.v1.room.create()` 走不通——`VcRoomResourceClient` 是空壳无 `create()`，文档示例无法编译。
- **mail/helpdesk crate** —— `Mail`（mail/mail/mod.rs）、`Helpdesk`（helpdesk/helpdesk/mod.rs）域层是 1:1 单子转发壳（仅 `.v1()`），crate 当前仅 v1 单版本；service 提供了 `mailgroup()`/`ticket()` 快捷 accessor 绕过它们，lib.rs 文档也只演示快捷通道。`Mail`/`Helpdesk` 层全仓零调用者，是事实死代码。

**反例（指控显然不属实）**：

- **workflow crate** —— `service.rs` 648 行（已核实），是真实深度 facade：含 `WorkflowTaskListQuery`(L50) / `WorkflowTaskMutation`(L109) / `ApprovalTaskQuery`(L172) 三个 DTO + `list_tasks_all`(L334，自动分页 + `MAX_PAGE_SIZE` 默认注入) + `query_approval_tasks`(L437，本地 retain 筛选) + `approve_task`(L479) 等 helper。`v2::task::Task` 是 19 builder 工厂 + 类型 re-export hub。纯壳仅 `TaskV1`/`TaskV2` 两层，且 `service.task()` 与 `service.v2().task()` 形成等价双入口。

**部分属实（路径参数层是真职责）**：

- **platform crate** —— `ApaasV1.application(namespace).workspace(ws_id).table(id).records_post()` 路径参数绑定层（`apaas/v1/mod.rs:68/73/122` 实证）是真实成本；`SparkAppService.patch(app_id)/get_app_visibility(app_id)` 绑定路径参数；`DirectoryUserService.id_convert()` 注入默认 enum。但 `spark/spark`、`admin/admin`、`directory/directory`、`app_engine/apaas` 4 个 3 行空跳 mod.rs（`lib.rs:39` `#![allow(clippy::module_inception)]` 抑制）是纯样板；门面缺口（`lib.rs` 暴露 `mdm`/`tenant`/`trust_party` 但 `PlatformService` 无对应 accessor）。

**已事实上采用砍导航（指控方向相反）**：

- **analytics crate** —— 已核实 `service.rs:42` `#[deprecated(`，门面 `search()` 自标「导航死胡同」，`SearchV2` 是 dead struct；实际可用 API（`data_source`/`app`/`doc_wiki`）已走直路径 `GetXxxRequest::new(Arc<Config>, ...)`。
- **docs crate** —— 已核实 `base/service.rs:1` `// 此文件已废弃，Service 层已删除`；`DocsClient` 顶层聚合 ~15 个真实 async helper（`search_bitable_records_all` 分页循环、`find_wiki_node_by_path` 逐级导航、`folder_children_pager` 分页状态机、`upload_drive_file` 注入 `parent_type="explorer"` + size 推断默认值），定义 6 个 typed helper 结构。

**轻动区**：

- **application** —— 已自带反壳立场（`application/application/mod.rs` 无 struct 仅 feature 门控，`WorkplaceV1` 源码注释「不引入单方法 resource 中间层」），仅 `v1/app/mod.rs` 声明了未接线的端点。
- **user** —— `UserService::new` 返回 `SDKResult` 却函数体永远 `Ok(...)` 是误导签名；`PersonalSettingsResource` 纯转发。
- **cardkit** —— 门面链（`common/chain.rs`）+ strict-path 死树（`cardkit/cardkit/v1/*`）双导航树并存，且同名 `CardElementResource` 命名碰撞（一个持 `Arc<Config>` async，一个持 `Config` 值 builder）。

issue #347 的指控非均匀——bot/meeting/mail/helpdesk 完全属实、workflow 显然不属实、platform 部分属实、analytics/docs 方向相反。一刀切（深导航全深化 / 砍导航全砍）都会强行拟合不属于它的 crate。

## 决策

采用 **混合方案（per-layer 5 项职责判定 + per-crate 差异化产出）**。一刀切是错的。

### 5 项保留判定（命中任一则留层，全不命中则砍）

| # | 职责 | 命中示例 |
|---|------|---------|
| 1 | **feature 门控层** —— `#[cfg(feature)]` 编译期可达性裁剪 | `BotService::v4` 标 `#[cfg(feature = "v4")]` |
| 2 | **版本路由层** —— 同域 ≥2 版本共存（v1/v2 选择） | `TaskV2`（workflow 有 v1/v2 共存） |
| 3 | **路径参数/状态绑定层** —— 接受参数并下传 | `ApaasV1.application(namespace).object(obj)` |
| 4 | **扇出分组层** —— ≥3 下属资源 + 类型 re-export hub | `MailV1`（5 资源）、`HelpdeskV1`（11 资源） |
| 5 | **真实 helper 层** —— 校验/分页/默认值/日志/DTO | `WorkflowService`、`v2::task::Task`（19 builder） |

### per-crate 判定结果（同一规则产出相反结果，证明非一刀切）

| crate | 中间层判定 | 产出链 |
|-------|-----------|--------|
| **bot** | Bot / V4 / BotResource 全 0 命中 → 砍；BotService 命中(1) → 留 accessor 直达 leaf | `service.search_bot().query().execute()` |
| **workflow** | TaskV2 命中(2)+(4) → 留；Task 命中(5) 19 builder → 留；`service.task()` 捷径冗余 → 删 | `service.v2().task().create()` |
| **platform** | ApaasV1 命中(3) 路径参数 → 留；spark/spark 等空跳 mod.rs 0 命中 → 砍；补 mdm/tenant/trust_party 门面缺口 | `service.app_engine().v1().application(ns).object(obj).record().create()` |
| **analytics** | Search/SearchV2 deprecated 死链 → 收口为扁平 | `GetDataSourceRequest::new(Arc<Config>, id).execute()` 直路径 |
| **docs** | 已是扁平范本 → 仅清 5 个纯 config 子客户端 | `DocsClient` 顶层 ~15 个 helper 全保留 |
| **mail/helpdesk** | Mail/Helpdesk 域层 0 命中 → 砍；MailV1/HelpdeskV1 命中(4) 扇出 → 留 | `service.mailgroup().create()`（统一快捷 accessor） |
| **meeting** | 8 空 ResourceClient 0 命中 → 砍；VcNoteResourceClient 命中(5) → 留并上提 | 重写 chain.rs + 修文档谎言 |
| **cardkit** | 双导航树合并（死树 0 外部引用 → 删）+ 解决 CardElementResource 命名碰撞 | `service.v1().card.create(body)` |
| **application** | 已是反壳范本 → 仅补 v1/app 接线缺口 | `service.v1().app().get()` |
| **user** | UserService 误导签名 → 改为非 Result | `service.system_status().list()` |

### 三条硬约束

1. **leaf builder API 100% 冻结** —— `SearchBotRequest::new(Arc<Config>)`、`CreateXxxRequest` 等叶子（含 `validate_required!` / 序列化 / `Transport::request` / `Response` 提取）全部不动。走严格路径或 crate 直构造的用户零影响。blast radius 仅限「走 service 深链入口」的调用方。
2. **per-crate 独立 PR、灰度推进** —— 每个 crate 一个独立 PR（bot ~1-2h、meeting ~1d 最重、cardkit ~半天-1d 最易出错）；不必 11 crate 统一上线，v0.18 窗口内按 crate 灰度。每 PR 跑 `just check-all`。
3. **leaf URL 解耦单独成议题** —— 深导航的 version meta 思路是好方向但属另一议题（AGENTS.md「不要硬编码 URL」），不在本 ADR 范围；本 ADR 只裁壳、不动 leaf。

## 理由

1. **issue #347 的指控在 11 crate 间严重不均匀**。深导航强行深化 analytics/docs 是开倒车（两者已事实上采用砍导航）；砍导航强行砍 workflow/platform 会丢真深度。混合方案承认 11 crate 形态本就不同。

2. **5 项判定可证伪、可复现**，不靠口味。bot 三壳全 0 命中→砍；workflow TaskV2 命中(2)+(4)→留；platform ApaasV1 命中(3)→留——同一规则产出相反结果，证明非伪装的一刀切。

3. **leaf 100% 保留 = 最小破坏面**。blast radius 仅限 service 深链调用方；全仓 grep 确认 bot/mail/helpdesk 长链零调用者，且 v0.18 窗口已开。

4. **与项目既有反壳立场一致**。application 已跳过域 struct、docs `base/service.rs` 标注「Service 层已删除」、analytics 门面 `#[deprecated]` 自标「导航死胡同」——混合方案把这四个孤立反例系统化为全仓规则。

5. **拒绝深导航**：对 1-API 的 bot 是过度工程（CLAUDE.md §3「为未来版本预留是猜未来，通常错」）；meeting 8 空 ResourceClient「深化」等于从零重写整个资源层。

6. **拒绝砍导航一刀切**：失去版本命名空间前瞻性；platform path-param 聚合层是真成本，扁平化后 namespace+object_id 全堆到叶子签名变长易传错位。

## 后果

### 正面

- 消除/扁平化纯转发样板约 **1200-1400 行**（占测绘的 ~2000 行导航壳 60-70%）。
- 消除别扭链：bot 段名重复（`bot().v4().bot()`）消失；meeting 字段链与真实模块树对齐。
- **消除文档谎言**：meeting `client.meeting.vc.v1.room.create()` 走不通——重写后不再撒谎。
- 消除命名碰撞：cardkit 双导航树合并后同名 `CardElementResource` 不再存在。
- 修误导签名：`UserService::new` 不再返永不 Err 的 `SDKResult`。
- 与项目既有反壳事实对齐。

### 负面

- **全 crate breaking**（被砍 pub struct 都是 SemVer-major 删除），必须搭 v0.18 窗口；CHANGELOG 迁移表需逐 crate 记录。
- **跨 crate 形态不一致是 by design**——bot 1 层、workflow 3 层、platform 4 层。**缓解**：本 ADR + 5 项判定规则进 `docs/CLIENT_NAMING_CONVENTION.md`，作为唯一权威解释。
- **判定阈值有灰色区**——「扇出 ≥3」是拍脑袋；未来新 crate 落灰色区时需 ADR 修订。
- **只改 service 入口、不改模块树**——`crate::bot::bot::v4::bot::search::SearchBotRequest` 这种 4 段模块路径仍存在（leaf 保留的代价）；module 重组作为后续独立议题。

## 迁移路径（分阶段，v0.18 breaking 窗口内灰度）

### 阶段 1：范式验证（最高价值 + 最严重，2 crate）

1. **openlark-meeting**（最重，~1 天）：删 `chain.rs` 8 空 `*ResourceClient` + 重写文档示例为可达路径 + 修 `Config` 深拷贝链为 `Arc`。`VcNoteResourceClient` 唯一真桥接上提到门面。同步修 `openlark-client/docs/meta-api-style.md:99` 与 `client.rs:97` 的会议谎言。
2. **openlark-bot**（~1-2 小时）：删 `Bot` / `V4` / `BotResource` 3 文件，`BotService` 加 `search_bot()` 直达 leaf（保留 `feature=v4` 门控）。issue #347 点名、最快验证 5 项判定规则。

### 阶段 2：死壳清理（机械删除为主，5 crate）

3. **openlark-mail**（~半天）：删 `Mail` 域层死壳，`MailV1` 扇出 5 留；统一快捷 accessor 策略。
4. **openlark-helpdesk**（~半天）：删 `Helpdesk` 域层死壳，`HelpdeskV1` 扇出 11 留。
5. **openlark-analytics**（~2 小时）：删 deprecated `Search` / `SearchV2` 死链，正式收口为扁平。
6. **openlark-user**（~2 小时）：删 `UserService` / `PersonalSettingsResource`，修 `UserService::new` 误导签名。
7. **openlark-platform**（~半天）：删 4 个 module_inception 空跳 mod.rs，补 `mdm` / `tenant` / `trust_party` 门面缺口；路径参数绑定层 100% 保留。

### 阶段 3：双树合并 + 范本清理（2 crate）

8. **openlark-cardkit**（~半天-1 天，最易出错）：合并门面链与 strict-path 死树，解决 `CardElementResource` 命名碰撞。
9. **openlark-docs**（~2 小时）：删 5 个纯 config-holder 子客户端，`DocsClient` 600 行真 helper 全保留。

### 阶段 4：反壳范本微调 + 深范本清理（2 crate）

10. **openlark-application**（~半天）：已是反壳范本，仅补 `v1/app/mod.rs` 声明了未接线的端点。
11. **openlark-workflow**（~1 小时）：已是深范本，仅删 `service.task()` 与 `service.v2().task()` 等价双入口二选一。

### 阶段 5：统一收口（~1 天）

- 统一改 `docs/CLIENT_NAMING_CONVENTION.md`（5 项判定规则）+ `openlark-client` facade 转发层 + CHANGELOG。
- 全仓 `just check-all` 验证。

## 遵循

- **AGENTS.md 反模式**：「不要硬编码 URL」——本 ADR 不在范围（leaf URL 解耦单独议题）。
- **CLAUDE.md §3 简洁性**：「为未来版本预留是猜未来，通常错」——拒绝深导航对单版本域的版本层前瞻性命名。
- **CLAUDE.md §4 外科手术式修改**：leaf builder API 100% 冻结，per-crate 独立 PR，不 reformat、不碰无关代码。
- **CHANGELOG 0.18 迁移表**：项目已有 v0.18 deprecated 清理先例（7 个 change 归档），流程成熟，本 ADR 沿用。
- **`docs/CLIENT_NAMING_CONVENTION.md`**：5 项判定规则落此文档，作为全仓权威解释。
- **MSRV 1.88+ / CI msrv pinned lockfile**：删/改依赖的 change 须同步 `.github/msrv/Cargo.lock`。
- **CI lint 门控**：每 PR 显式跑 `cargo fmt --check` + `cargo clippy --workspace --all-targets --no-default-features`。

## 执行记录（2026-07-06 完成）

ADR 按阶段灰度落地，每 crate 独立 PR，全部 CI 绿（fmt/clippy×2/test/doc/machete/msrv）。

| 阶段 | crate | PR | 产出 |
|------|-------|-----|------|
| 1 | bot | #354 | 删 `Bot`/`V4`/`BotResource` 3 壳 + `search_bot()` 直达 leaf |
| 1 | meeting | #353 | 砍 chain.rs 7 空壳 + 修文档谎言（`note.get()`） |
| 2 | mail | #357 | 砍 `Mail` 域层壳 + 统一 `v1()` accessor（5 资源扇出） |
| 2 | helpdesk | #358 | 砍 `Helpdesk` 域层壳 + 统一 `v1()` accessor（11 资源扇出） |
| 2 | analytics | #359 | 删 deprecated `Search`/`SearchV2`/`search()` 死链，扁平收口 |
| 2 | user | #360 | 砍 `PersonalSettingsResource` 中间层 + 修 `UserService::new` 误导签名（#350 P9） |
| 2 | platform facade | #361 | `mdm`/`tenant`/`trust_party` 宣布 flat-by-design（不加 shell） |
| 3 | docs | #365 | 砍 5 个 config-holder 子客户端，`DocsClient` ~15 真 helper 全保留 |
| 3 | cardkit | #366 | 合并双导航树（砍死 strict 树）+ 解决 `CardElementResource` 命名碰撞（6-agent 勘察 workflow） |
| 4 | application | #362 | v1/app 补齐 4 个声明却未接线的端点（create/delete/list/patch） |
| 4 | workflow | #364 | 删 `service.task()`/`tasklist()` 冗余双入口 |
| 后续 | platform inception | #371 | 折叠 spark/admin/directory 3 个同名 inception + 删 `#![allow(clippy::module_inception)]`（8-agent 勘察 workflow；app_engine/apaas 异名排除） |

### 决策痕迹：platform inception（先延后做）

- **platform inception 折叠**（line 120「删 module_inception 空跳」）原与硬约束 line 105「不改模块树…module 重组作为后续独立议题」字面冲突。**2026-07-06 #368 执行期初裁：守硬约束，延后到 #367**；**2026-07-06 #371 落地完成**——折叠 spark/admin/directory 3 个同名 inception（实测 app_engine/apaas 异名非 inception，排除）+ 删 `#![allow(clippy::module_inception)]`。至此 line 120 全部兑现，硬约束 line 105 的「后续独立议题」收口。

### 阶段 5（统一收口）

- `docs/CLIENT_NAMING_CONVENTION.md` 落 5 项判定规则、`openlark-client` facade 转发层对齐、CHANGELOG 迁移表——随各 PR 同步落地（facade 转发层在每 PR 连带改）。全仓 `just check-all` 等价验证（每 PR 跑 CI 三元组 + workspace doc）。

