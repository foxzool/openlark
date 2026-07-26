# API 一致性复查指南

本文是「复查 OpenLark 实现的 API 是否与飞书官网一致」的**方法论总纲**：把一致性问题拆成若干维度，给出每个维度的核对工具、官方数据来源（oracle）、覆盖范围与盲区，以及一份从快到慢的复查决策流。

它不重复命令细节——具体参数与 finding code 见 [`api-contract-validation.md`](api-contract-validation.md)（契约校验工具手册）；发布流程视角见 [`api-compatibility-release-checklist.md`](api-compatibility-release-checklist.md)。本文回答的是：**面对一个一致性复查任务，该查什么、按什么顺序查、查的时候别踩哪些坑。**

> 所有事实均对照源码核对（2026-07-24）：工具入口见 `justfile:43-66`，CI gate 见 `.github/workflows/ci.yml:54-93`。

---

## 1. 「一致性」的五个维度

API 实现与官网一致，至少要同时满足下表五件事。每一项都有专门的工具和 oracle，覆盖强度不同。

| 维度 | 问什么 | 工具 / 入口 | 官方数据来源（oracle） | 覆盖强度 | 主要盲区 |
|------|--------|------------|----------------------|---------|---------|
| **① 覆盖率** | 官方有的 API，SDK 是否都落了盘？ | `tools/validate_apis.py`（`just api-coverage`） | `api_list_export.csv` | 按路径约定判文件存在/缺失 | 只看**路径**，不看内容；路径合规 ≠ URL 正确（见 §6.1） |
| **② Endpoint** | HTTP method + path 是否一致？ | `tools/validate_api_contracts.py`（`just api-contracts`） | 离线=CSV `url` 列；live=详情接口 `apiSchema.httpMethod/path` | 离线全仓进 CI；复杂 `to_url()` enum 解析不了报 WARN 不失败 | 动态拼接路径可能 `W_ENDPOINT_UNRESOLVED` |
| **③ Request 字段** | 请求体字段名/必填是否一致？ | `tools/validate_api_contracts.py --fields --live-fields` | 详情接口 `apiSchema.requestBody...properties` | 仅顶层字段；需 live（抓网络） | 嵌套对象、query/path 参数未纳入 |
| **④ Response 字段** | 响应 `data` 字段是否齐全？ | 同上（`--fields --live-fields`） | 详情接口 `apiSchema.responses.200...data.properties` | 仅 `data` 顶层；缺失只 WARN 不阻塞 | 折叠的子字段拿不到（见 §6.4） |
| **⑤ Token 类型** | 声明的鉴权 token 官网是否接受？ | `tools/validate_api_contracts.py --tokens` | `apiSchema.security.supportedAccessToken`，缺标注回退 `.md` 源 Authorization 行 | CI 仅 scope 到 `openlark-security`+`openlark-auth` | 全仓 live 抓取量太大，其他 crate 靠人工抽样 |

**维度之间的关系**：①是前置（文件不存在谈不上后面），②③④⑤逐层深入。①过了不代表②对（文件在、URL 可能写错）；②过了不代表③④对（URL 对、字段可能漂移）。所以一次完整的复查要**纵向走完五层**，不能只看某一层绿灯。

---

## 2. 官方数据从哪来（两个 oracle）

所有核对最终都在和飞书的「真相」比对。项目里有两条获取真相的路径，**别混用**：

### 2.1 离线快照：`api_list_export.csv`（仓库根）

飞书开放平台导出的全量 API 清单，checked-in。每行字段（见 `tools/api_contracts/official.py:65-95`）：

- `url` —— `METHOD:/open-apis/...`，endpoint 维度的离线 oracle
- `fullPath` —— 文档路径（拼到 `https://open.feishu.cn` 后是文档页 URL），**取它做抓取入口，不要自己拼**（见 §6.3）
- `docPath` / `meta.{Project,Version,Resource,Name}` / `bizTag` / `id` / `name`

优点：快、离线、可 diff、进 CI。缺点：是**导出时刻的快照**，飞书当天改了字段它不知道——所以字段级核对必须走 live。

### 2.2 Live 结构化 schema：飞书详情接口

`https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=...`（`official.py:107-130`）返回机器可读 JSON，真相在 `data.schema.apiSchema` 下：`httpMethod` / `path` / `requestBody` / `responses` / `security.supportedAccessToken`。

字段级、token 级核对都走它。缺点：每个 API 一次网络抓取，全仓约 1500+ 次，CI 跑不完 → 只能抽样或 scope。

### 2.3 SPA 文档正文的 `.md` 源（回退手段）

飞书 `open.feishu.cn` 文档页是 SPA，直接抓 HTML 只拿到外壳。但每页在 `<link rel="alternate" type="text/markdown">` 提供 `.md` 原始版本，URL 形如 `https://open.feishu.cn/document/<path>.md`（`official.py:195-214`）。token 维度在详情接口缺 `supportedAccessToken` 标注时回退到它解析 Authorization 行；字段维度的 playwright 渲染失败时也可作为补充正文来源。

---

## 3. 复查决策流（从快到慢）

接到「复查某 crate / 某 API 一致性」的任务，按下面分层走，**前一层发现可疑才深入下一层**，避免一上来就全量 live 抓取。

### 第 0 层：先看 CI 已经守护了什么（日常不必手动跑）

CI 的 `api-contracts` job（`ci.yml:54-93`）已经在每次推送时跑：

- 全仓离线 endpoint strict gate（`--all-crates --strict endpoint`）
- `openlark-security` + `openlark-auth` 的 token strict gate（`--strict tokens`）
- 三个合约测试模块（`tools.tests.test_validate_api_contracts_{official,rust_source,compare}`）

→ 这意味着 **method/path 漂移和这两个 crate 的 token 漂移，CI 会自动挡**。手动复查的重点应放在 CI 没覆盖的：字段级、其他 crate 的 token、以及路径噪音掩盖的真 bug。

### 第 1 层：覆盖率（秒级，离线）

```bash
just api-coverage                          # 全仓汇总
python3 tools/validate_apis.py --crate openlark-workflow   # 单 crate 详细缺失清单
```

看缺失 API 清单。但记住：报「缺失」要按 §6.1 三类鉴别，**不要直接当成真缺口**。

### 第 2 层：Endpoint 离线（秒级，离线，CI 同款）

```bash
just api-contracts                         # 全仓（CI 跑的就是这个）
```

看 `reports/api_contracts/crates/<crate>.md`。重点盯 `E_ENDPOINT_*`（ERROR）。`W_ENDPOINT_UNRESOLVED` 不阻塞，但意味着该 API 的 URL 没被机器验证过——标记为待人工核（见第 4 层）。

> 若 `validate_api_contracts` 对某个复杂 enum 路径报 `W_ENDPOINT_UNRESOLVED`，可用独立的 `tools/check_api_urls.py`（`python3 tools/check_api_urls.py --crate <name>`，输出 `reports/api_url_validation/`）深挖：它的 `ExprResolver` 对 `format!`/`replace`/字符串拼接/`to_url()` enum/变量赋值的展开比契约工具更激进，能解析更多动态路径。该工具无 just recipe、不进 CI，是一份独立的离线 URL 全量核对报告。

### 第 3 层：字段启发式（秒级，离线，不抓文档）

```bash
python3 tools/verify_api_fields.py --crate openlark-workflow            # 快速模式
python3 tools/verify_api_fields.py --api-id 7642253323628383198         # 单 API 调试
```

`verify_api_fields.py` 的快速模式不抓文档，靠三类**红旗**嗅探可疑实现（`detect_suspicious_patterns`）：

1. 用户级接口（`/reference/` 路径）的 Body 含 `user_id`/`approval_code`（可能误抄应用级同族接口）
2. 必填 `Vec` 字段缺非空校验（不认 `validate_required_list!` 也不认 `is_empty()`）
3. GET 查询接口 Response 为空（可能漏建响应体）

这是「不抓文档就能发现字段问题」的廉价初筛，用来圈定需要第 4 层 live 核对的可疑 API。

### 第 4 层：字段 / Token live 抽样（约 8 秒/API，抓网络）

对第 2、3 层圈出的可疑 crate / API，抓飞书详情接口逐字段比对：

```bash
just api-contract-fields openlark-ai 5            # 抽前 5 个 API 核 request/response 字段
just api-contract-tokens openlark-communication   # 核 token 类型（CI 未覆盖的 crate）
```

这是目前**唯一能机器发现字段漂移**的手段（`api-contract-validation.md` §4 有 `openlark-ai` 验出真实漂移的实证）。注意只覆盖 request/response **顶层**字段。

### 第 5 层：playwright 渲染逐字段（最慢，最后手段）

当 live schema 也拿不全（如 response `data` 子字段在折叠区、或新接口 schema 未结构化），用真实浏览器渲染文档页提取 `innerText`：

```bash
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  "https://open.feishu.cn/document/<fullPath>" /tmp/doc.txt
```

或直接用 `verify_api_fields.py --fetch-docs` 的完整模式（自动调上面的脚本 + 缓存 + 对比）。详见 `openlark-api-field-verify` skill。这是字段核对的兜底，准确但慢、依赖 chromium。

---

## 4. 单个 API 的端到端核对 checklist

收到「核对某个具体 API」时，按此顺序：

1. **定位文件**：从 CSV 的 `bizTag/meta.*` 推 `src/{bizTag}/{project}/{version}/{resource}/{name}.rs`，`find crates/<crate>/src -name '<name>.rs'` 确认存在（不存在 = ③类真缺口）。
2. **取官方文档 URL**：从 CSV `fullPath` 字段取（**勿自己拼**，见 §6.3），拼到 `https://open.feishu.cn`。
3. **endpoint**：`python3 tools/validate_api_contracts.py --crate <crate> --strict endpoint`，或直接对照 CSV `url` 列 vs 代码里 `ApiRequest::post("/open-apis/...")`。
4. **字段**：`just api-contract-fields <crate> 1` 限定到该 API，或单 API `python3 tools/verify_api_fields.py --api-id <id> --fetch-docs`。
5. **token**：确认 `.with_supported_access_token_types(...)` 声明与官方 Authorization 一致（用户级接口尤其注意，见 §6.5）。
6. **URL 是否真被测试覆盖**：看测试是否真正调用了生产 endpoint 构造，而非 Potemkin（见 §6.2）。

---

## 5. 判定标准：怎么读报告

`validate_api_contracts.py` 的 finding 分三档严重度（详见 `api-contract-validation.md` §3）：

| Severity | 含义 | 处置 |
|----------|------|------|
| `ERROR` | 已确认 contract drift | 必须修；strict 模式会让 CI/命令失败 |
| `WARN` | 实现缺失/解析不到/低风险 | 需人工判断是否真问题 |
| `UNVERIFIED` | 官方数据不足，机器无法核对 | **不是没问题**，要人工查文档（如 §6.6 的孤儿 docPath） |

关键认知：**全绿 ≠ 一致**。`UNVERIFIED` 太多时，机器其实没核对多少，得靠人工补。字段维度默认不进 strict（只 `--strict fields` 才阻塞），所以 `just api-contracts` 绿灯只保证 endpoint，不保证字段。

---

## 6. 已知陷阱与误报（必读）

这些是反复踩过的坑，复查时务必带着它们看报告，否则会被假绿灯或假缺失误导。

### 6.1 覆盖率是「路径制」，文件存在 ≠ URL 正确

`validate_apis.py` 按路径约定判文件在不在，并在比较阶段对常见 layout 做 denoise（flat_project / rust_keyword / rewrite / alias / typo_correction）。报告已将结果拆成 **strict 匹配 / 路径噪音匹配 / 真缺口 / 额外文件**（见 `docs/typed-api-coverage.md` §1.1–1.2）。仍须人工鉴别三类语义：

- **① 路径命名噪音**：文件在、URL 也对，只是落盘路径偏离 canonical nested 公式。工具会尽量自动记入 `path_noise_matches`（计入已实现，附 evidence）；未覆盖到的变体仍可能出现在真缺口里，需对照 `extra_file_list`。
- **② 文件在但 endpoint URL 写错**（真 bug，调不通）：曾发生在 workflow task v2 的 `section`/`custom_field`——文件、结构体、测试齐全，但 URL 错用了 tasklist 作用域前缀，**调用必 404**。路径覆盖率检查发现不了这一类。
- **③ 真未实现**：`classification=true_gap`，磁盘上无对应叶子实现。

→ 鉴别两步：(1) 先看报告分类与 `path_noise_matches` evidence；(2) **文件存在时必须再核对 endpoint URL** 对照 CSV `url` 列——文件存在不等于 URL 正确，这是 ②类 bug 的发现方式。

### 6.2 Potemkin URL 测试：测试名有 URL ≠ URL 被测

部分叶子的 `test_url_path` 测试形如 `let _req = Request::new(config)...; assert_eq!(format!("/open-apis/..."), "...");`——`_req` 建完丢弃（前缀 `_`），assert 只比 `format!` 宏与硬编码串，**从不调生产 `execute()`**。生产端点 URL 的真实测试覆盖因此 ≈ 0，改 URL 时这些测试抓不到回归。

→ 审计「URL 是否被测试覆盖」不能只看测试名/测试里有无 URL 字符串，要看测试是否真正观测了生产输出。enum 的 `to_url()` 孤立断言才是真 URL 测试。

### 6.3 CSV `fullPath` 两种格式，不能混用

- 旧版/server-docs：`/document/server-docs/{project}-{version}/{resource}-{name}`，用 `-` 连接
- 新版/reference：`/document/uAjLw4CM/.../reference/{project}/{version}/{resource}/{name}`，用 `/` 分层

用错格式页面显示 "The documentation could not be found."，这不是抓取失败是 URL 错。**永远从 CSV `fullPath` 取真实路径**，不要自己拼。

### 6.4 Response `data` 子字段在折叠区

文档 Response body 段常只显示外层 `code/msg/data`，`data` 的子字段在 "Show sublists" 折叠区，innerText / 结构化 schema 都可能拿不到。

→ 改从 **Response body example 的 JSON** 提取字段名（`grep -oE '"[a-z_]+"\s*:'`），示例里出现的字段就是真实字段。live schema 的 `responses.200...data.properties` 也只到顶层。

### 6.5 用户级 vs 应用级字段混淆

用户级接口（`user_access_token`）的请求体**不含** `user_id`/`approval_code`——操作者身份从 token 推断。若参照了应用级同族接口复制字段，会多出这些字段，序列化发出可能被服务端拒绝。

→ 核对用户级接口时优先排除这类多余字段。`verify_api_fields.py` 红旗 1 会提示（info 级，因 `/reference/` 也含管理员级接口，需人工判断）。

### 6.6 孤儿 API：docPath 404 无法核对

少数 `.rs` 实现不在 `api_list_export.csv` 里，或其 `docPath`/`fullPath` 在飞书文档站 404（文档不存在）。这类**无法按飞书文档核对**，工具会报 `UNVERIFIED` 或根本不纳入校验。遇到时标注「无官方文档可核对」，不要硬凑字段。

### 6.7 字段级只覆盖顶层

契约工具的 field 维度只比 request body 顶层 + response `data` 顶层。嵌套对象内部字段、query 参数、path 参数都**未纳入**机器比对。怀疑嵌套字段有问题时，只能靠第 5 层 playwright 渲染人工核对。

---

## 7. 持续一致性：谁在长期盯着

一次性复查之外，项目有三道长期防线：

1. **CI strict gate**（`ci.yml`）：endpoint 全仓 + token(security/auth) 每次推送自动跑。method/path 和这两个 crate 的 token 漂移会被自动挡。
2. **飞书 API 变动检测 issue**：周期性 bot issue 汇总飞书侧新增/变更。新增 API → 补 SDK 实现 + 同步 CSV；字段元数据变化（计费/显示名）通常 out of scope（不影响请求/响应结构）。
3. **发布前兼容性 checklist**（`api-compatibility-release-checklist.md`）：打 tag 前的人工 + 自动复查，release.yml 跑覆盖率产出 artifact。

日常开发只需保证 CI 绿；深度字段核对（第 3-5 层）在新增/重构 API、或怀疑漂移时按需做。

---

## 8. 工具速查表

| 工具 | 路径 | 维度 | 模式 | 入口 |
|------|------|------|------|------|
| 覆盖率 | `tools/validate_apis.py` | ① | 离线 | `just api-coverage` |
| 契约-endpoint | `tools/validate_api_contracts.py` | ② | 离线/live | `just api-contracts` |
| 契约-fields | 同上 `--fields --live-fields` | ③④ | live | `just api-contract-fields <crate> <N>` |
| 契约-tokens | 同上 `--tokens` | ⑤ | live | `just api-contract-tokens <crate>` |
| 字段启发式 | `tools/verify_api_fields.py` | ③④ | 离线快速/live 完整 | `python3 tools/verify_api_fields.py --crate <name> [--fetch-docs]` |
| URL 全量解析 | `tools/check_api_urls.py` | ② | 离线（更激进展开） | `python3 tools/check_api_urls.py --crate <name>` |
| 文档渲染 | `.agents/skills/openlark-api-field-verify/scripts/fetch_doc.js` | 字段人工核对 | live(playwright) | `node fetch_doc.js <url> <out>` |

## 9. 相关文档

- [`api-contract-validation.md`](api-contract-validation.md) —— 契约校验三层（endpoint/field/token）的工具手册与完整 finding code
- [`api-compatibility-release-checklist.md`](api-compatibility-release-checklist.md) —— 发布前兼容性复查清单
- [`api-spec-accuracy-audit.md`](api-spec-accuracy-audit.md) —— 一次规范文档准确性审计的实证（含 `.md` 源抓取方法）
- `.agents/skills/openlark-api-field-verify/SKILL.md` —— playwright 渲染字段核对的完整工作流
- `.agents/skills/openlark-api-validation/SKILL.md` —— 覆盖率验证技能
