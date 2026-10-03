# 飞书 API 文档方案准确性审计

**日期**：2026-07-23
**核对来源**：飞书开放平台官方文档 `open.feishu.cn`（一手，仅采信官方页，不采二手博客）
**核对方式**：原计划派两个后台 agent 并行核对，但 subagent 派发即失败（`API Error 400 [1210]`，底层模型 API 参数错误，非任务问题），改由主 agent 直接抓取飞书官方文档的 `.md` 源逐一核对。

## 审计范围

被审计的四份「API 实现规范」文档：

- `.agents/skills/openlark-api/SKILL.md`（API 实现规范 skill）
- `docs/API_DESIGN_SPECIFICATION.md`
- `docs/api-implementation-template.md`
- `docs/api-contract-validation.md`

> **重要前提**：这四份文档**绝大部分是仓库内部 Rust 代码范式约定**（Builder / execute / execute_with_options / Transport / validate_required / RequestOption 透传 / mod.rs 导出等），属于本仓自有规范，不涉及飞书官网，无需也无法「对照飞书官网核对」。
>
> 本报告**只审计其中对飞书开放平台 API 形态的外部断言**——共提炼出 6 类（A1 / A2 / B / C / D / E）。

## TL;DR

| # | 断言 | 结论 |
|---|------|------|
| A1 | 响应统一 `{code, msg, data}`，`code == 0` 成功 | ✅ 正确 |
| A2 | 响应有 `Data` / `Raw` / `Items` 三种**包装形态** | ⚠️ 措辞误导（实为 SDK 对 `data` 内容的建模，非飞书的响应包装形态） |
| **B** | **「接口文档要求 `tenant_access_token` → 用 App token」** | ❌ **错误**（混淆 `tenant_access_token` 与 `app_access_token`；OpenLark 代码已证实 `App = app_access_token`） |
| C | `RequestOption` 语义：`user_access_token` / `tenant_key` / `app_ticket` / `request_id` | ⚠️ 前三项准确；`request_id` 非飞书官方约定 |
| D | 飞书文档 schema 路径 `apiSchema.httpMethod/path/...` | ⚠️ 未直接核实，仓库自带运行实证佐证有效 |
| E | `/open-apis/` 前缀、域名 `open.feishu.cn`、`Bearer` 前缀 | ✅ 正确 |

**最关键发现**：断言 B 是真 bug（详见下文）。`.agents/skills/openlark-api/SKILL.md` 核心契约 4 的「判断方法」与飞书官方文档**和 OpenLark 自身代码**双重矛盾。

---

## 逐条核对

### A1. 响应包装结构 —— ✅ 正确

**规范原文**（多处）：飞书 API 响应体为 `ApiResponse<R> = {code, msg, data: R, ...}`，业务数据在 `data` 下；`code == 0` 表示成功。

**核对发现**：飞书官方「调用 API」页原文：

> 绝大多数 API 的响应体结构包括 `code`、`msg`、`data` 三个部分：
> - `code`：错误码。如果是成功响应，`code` 取值为 0。
> - `msg`：错误信息。
> - `data`：API 的调用结果。`data` 在一些操作类 API 的返回中可能不存在。
> - 请不要依据 `msg` 来判定一个请求是否失败。

成功示例 `{"code":0,"msg":"success","data":{...}}`，失败示例 `{"code":40004,"msg":"no dept authority error"}`。

**补充（规范未覆盖）**：失败响应还可能携带 `error` 对象，含 `field_violations`（参数错误）、`permission_violations`（权限错误）、`helps`、`logid`、`troubleshooter` 等排查字段。规范的错误处理描述可补充这一点。

**来源**：
- [调用 API — 响应结果](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-)（`.md` 源）

---

### A2. 三种「响应包装形态」Data/Raw/Items —— ⚠️ 措辞误导

**规范原文**（`docs/api-implementation-template.md` §8.4）：

| 场景 | ResponseFormat |
|------|---------------|
| 标准 data 包装响应 | `ResponseFormat::Data` |
| 原始响应（无包装） | `ResponseFormat::Raw` |
| items 数组响应 | `ResponseFormat::Items` |

**核对发现**：飞书**所有响应统一是 `{code, msg, data}` 包装**，不存在「无包装的原始响应」或「顶层 items 数组响应」这两种**包装形态**：

- 即使是操作类 API（如 acs `device_bind`）成功返回的也是 `{"code":0,"msg":"success","data":{}}`——仍有 `code/msg/data` 包装，`data` 只是空对象，而非「无包装」。
- 列表类接口（如通讯录、消息列表）的响应是 `{code, msg, data: {items: [...], page_token, has_more, ...}}`——`items` 数组**嵌在 `data` 内部**，不是顶层。

**结论**：`ResponseFormat::Data/Raw/Items` 是 OpenLark **SDK 层对 `data` 字段内容形态的建模**（data 是结构体对象 / data 为空或透传原始值 / data 内含 items 数组），**不是飞书的三种响应包装形态**。文档措辞「原始响应（无包装）」「items 数组响应」容易让读者误以为飞书存在非标准包装，应改为「`data` 内容形态」之类的表述。

**来源**：
- [调用 API — 响应结果](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-)
- [acs 设备绑定权限组 — 响应体](https://open.feishu.cn/document/acs-v1/rule_external/device_bind)（`data: {}` 实例）

---

### B. token 类型映射 —— ❌ 错误（重点）

**规范原文**（`.agents/skills/openlark-api/SKILL.md` 核心契约 4）：

> Token 类型必须显式声明（应用级接口尤其重要）：`ApiRequest` 默认 `supported_access_token_types = [User, Tenant]`。应用级接口（如 acs）**必须** `.with_supported_access_token_types(vec![AccessTokenType::App])`，否则 Transport 解析到 Tenant/User token 被飞书拒绝。**判断方法：接口文档要求 `tenant_access_token` 的 → App token**。

#### 核对 1：飞书的 token 体系

飞书官方「获取访问凭证」明确三种访问凭证，**互不相同**：

| 凭证 | 代表身份 | 值前缀 | 官方说明 |
|------|---------|--------|---------|
| `tenant_access_token` | **应用**身份（租户内） | `t-` | 以应用身份调用 API，最常用 |
| `user_access_token` | **用户**身份 | `u-`（旧）/ JWT（新） | 以用户身份调用 API |
| `app_access_token` | **应用**身份（短期令牌） | `a-` 或 `t-` | 「使用场景比较少（一般用于商店应用获取 tenant_access_token），平台正在逐步统一 `app_access_token` 和 `tenant_access_token` 两个凭证」 |

#### 核对 2：OpenLark 代码的映射（自证）

`crates/openlark-core/src/constants.rs:47-66` 中 `AccessTokenType` 的 `Display` 实现：

```rust
AccessTokenType::None    => "none_access_token",
AccessTokenType::App     => "app_access_token",   // ← App 就是 app_access_token
AccessTokenType::Tenant  => "tenant_access_token",
AccessTokenType::User    => "user_access_token",
```

即 SDK 的 `App` ↔ 飞书 `app_access_token`，`Tenant` ↔ `tenant_access_token`，二者**是不同的 token**。

#### 结论：规范「判断方法」完全错误

规范写「接口文档要求 `tenant_access_token` 的 → App token」，但：

- 代码里 `App = app_access_token` ≠ `tenant_access_token`。
- 飞书官方：接口文档要求哪种 token 就发哪种。要求 `tenant_access_token` 的接口应发 `tenant_access_token`（SDK 的 `Tenant`），而不是 `app_access_token`（SDK 的 `App`）。
- 官方且明确 `app_access_token` 场景极少，正在被 `tenant_access_token` 收敛。

**正确判断方法应是**：以**每个接口官方文档「请求头 → Authorization」字段标注的凭证类型**为准逐接口选择，没有「应用级 → App」这种通则。

#### 附：举例也站不住

规范举的「acs 接口要用 App token」与实际冲突——acs 接口 `device_bind`（`acs/v1/rule_external/device_bind`）官方文档明确：

> Authorization | string | 是 | `user_access_token`，示例值 `Bearer u-7f1bcd13fc57d46bac21793a18e560`

即该接口要的是 `user_access_token`（`u-` 前缀）。若按规范对其设 `[App]`（app_access_token），调用会被飞书拒绝。不同 acs 接口鉴权方式各异，不能一刀切。

> 说明：规范「默认 `[User, Tenant]`」本身合理（飞书大量接口确实支持 tenant 或 user 两种），问题仅在「判断方法」那句及其推导。

**来源**：
- [获取访问凭证（token 体系总览）](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-access-token)（`.md` 源）
- [如何选择使用不同类型的 Token](https://open.feishu.cn/document/faq/trouble-shooting/how-to-choose-which-type-of-token-to-use)
- [自建应用获取 tenant_access_token](https://open.feishu.cn/document/server-docs/authentication-management/access-token/tenant_access_token_internal)
- [自建应用获取 app_access_token](https://open.feishu.cn/document/server-docs/authentication-management/access-token/app_access_token_internal)
- [acs 设备绑定权限组（鉴权方式）](https://open.feishu.cn/document/acs-v1/rule_external/device_bind)
- 代码：`crates/openlark-core/src/constants.rs:47-66`、`crates/openlark-core/src/http.rs:222-297`、`crates/openlark-auth/src/token_provider.rs:158-184`

#### 实际影响（跟踪于 [#511](https://github.com/foxzool/openlark/issues/511)）

> **状态（2026-07-23）**：下列误配的**代码**已由 #515（PR #517）批量修正——`openlark-security` 下 `AccessTokenType::App` 命中数已降为 0；本节描述的是修正前的根因影响，规范层面「判断方法」的纠正见 #513。

规范的错误「判断方法」已误导批量实现。全仓约 43 处 `AccessTokenType::App`，运行时均注入 `app_access_token`：

- `openlark-security/acs/v1/` 约 28 个接口几乎全错 — 实测 `user/get`、`user/list` 要 `tenant_access_token`；`device_bind` 要 `user_access_token`。
- `openlark-security/security_and_compliance/v1,v2/` 约 12 个接口同模式误配 — 实测 `user_migrations/get` 要 `tenant_access_token` 或 `user_access_token`。
- `openlark-auth/authen/v1/` 4 个 OIDC token 端点用 App **正确** ✅（token 端点本就用 app 凭证）。

即按当前实现调用 acs / security_and_compliance 接口会因 token 类型不符被飞书拒 —— 这是运行时鉴权 bug，非纯文档问题。

---

### C. RequestOption 语义 —— ⚠️ 大体准确，`request_id` 待澄清

**规范原文**：`RequestOption` 用于传递 `user_access_token`（用户态）、`tenant_key`（商店应用）、`app_ticket`（商店应用）、`request_id` / 自定义 header（链路追踪）。

**核对发现**：

| 字段 | 结论 | 官方依据 |
|------|------|---------|
| `user_access_token` | ✅ | 以用户身份调用 API |
| `app_ticket` | ✅ | 飞书每隔 1 小时向商店应用事件订阅地址推送 `app_ticket`，商店应用凭 `app_id + app_secret + app_ticket` 换取 `app_access_token` |
| `tenant_key` | ✅ | 「租户在飞书上的唯一标识，用来换取对应的 `tenant_access_token`」，商店应用凭 `app_access_token + tenant_key` 换 `tenant_access_token` |
| `request_id` / 自定义 header | ⚠️ | 飞书官方**响应头**提供 `x-tt-logid` 用于排查；**未发现**官方约定支持请求方自定义 `request_id` header。此处的 `request_id` 更像是 SDK 自定义透传字段，规范未说明这一点，易被误读为飞书官方能力 |

**来源**：
- [获取访问凭证 — 商店应用获取 app_access_token / tenant_access_token](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-access-token)
- [调用 API — `x-tt-logid`](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-)

---

### D. 飞书文档 schema 路径 —— ⚠️ 未直接核实，有运行实证

**规范原文**（`docs/api-contract-validation.md` §1.2 / §1.3）：飞书文档详情接口返回的结构化 schema，JSON 路径为：

- `schema.apiSchema.httpMethod`
- `schema.apiSchema.path`
- `schema.apiSchema.requestBody.content.*.schema.properties`
- `schema.apiSchema.responses.200.content.application/json.schema.properties.data.properties`

**核对发现**：该路径是 OpenLark 内部校验工具（`tools/validate_api_contracts.py`）解析飞书文档详情接口所用的结构，**无法从公开官方文档页直接核实**（属内部接口契约）。

但 `api-contract-validation.md` §4 自带运行实证：`openlark-ai` 字段 live 校验曾检出真实漂移（官网 request 要求 `multipart/form-data` 的 `file`，Rust 实现却是 `file_token, is_async`；官网 response `data` 下有 `bank_card`，Rust 响应模型却是 `parsing_result`）——说明上述 JSON 路径在实践中**能取到结构化 schema 且有效**。

**结论**：未直接核实，但仓库自带实证佐证其有效。建议在该文档处补一条「已由 ai crate live 校验验证」的指针，避免读者误以为是臆测。

**来源**：
- `docs/api-contract-validation.md` §4（仓库内置实证）

---

### E. URL / 路径约定 —— ✅ 正确

**规范原文**：API 路径带 `/open-apis/` 前缀（如 `/open-apis/im/v1/messages`）；基础域名 `open.feishu.cn`（中）/ `open.larksuite.com`（国际）；文档 URL 形如 `open.feishu.cn/document/...`。

**核对发现**：全部准确。官方 cURL 示例佐证：

- `https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal`
- `https://open.feishu.cn/open-apis/im/v1/messages`
- `https://open.feishu.cn/open-apis/contact/v3/users/{user_id}`
- `https://open.feishu.cn/open-apis/acs/v1/rule_external/device_bind`

Authorization 头需 `Bearer<空格>` 前缀（官方明确）。

**来源**：
- [获取访问凭证 — cURL 示例](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-access-token)
- [调用 API — cURL 示例](https://open.feishu.cn/document/server-docs/api-call-guide/calling-process/get-)

---

## 需修正项清单（按优先级）

### P0 — 事实性错误，必须改

1. **`.agents/skills/openlark-api/SKILL.md` 核心契约 4**：删除或改正「判断方法：接口文档要求 `tenant_access_token` 的 → App token」。
   - 正确表述：以每个接口官方文档「请求头 → Authorization」标注的凭证类型为准——要求 `tenant_access_token` 用 `AccessTokenType::Tenant`；要求 `app_access_token` 用 `App`；要求 `user_access_token` 用 `User`。不存在「应用级接口 → App」的通则。
   - 风险：按现行规范描述，开发者会对要求 `tenant_access_token` 的接口误设 `App`，导致飞书拒绝（鉴权失败）。

2. **核心契约 4 的 acs 举例**：`device_bind` 实际要求 `user_access_token`，与「acs 用 App token」矛盾。改举例或删除该归纳。

### P1 — 措辞误导

3. **`docs/api-implementation-template.md` §8.4**：将 `ResponseFormat::Data/Raw/Items` 的描述从「响应包装形态」改为「`data` 字段内容形态」，删去「原始响应（无包装）」「items 数组响应」等易被误解为飞书非标准包装的措辞。

### P2 — 补充澄清

4. **RequestOption 的 `request_id`**：注明其为 SDK 透传的自定义 header，非飞书官方约定；飞书官方排查用响应头 `x-tt-logid`。
5. **A1 错误结构**：补充失败响应可能携带的 `error` 对象（`field_violations` / `permission_violations` / `logid` / `troubleshooter`）。

### P3 — 文档自证

6. **`docs/api-contract-validation.md` §1.2/1.3**：为 `apiSchema.*` JSON 路径补一条「已由 ai crate live 校验实证（见 §4）」的说明。

---

## 核对方法与局限

- **方法**：飞书文档站是 SPA，直接抓 HTML 仅得外壳；但其每个文档页提供 `.md` 原始版本（页面 metadata 的 `alternate` 字段，形如 `.../document/<path>.md`）。本审计一律抓取 `.md` 源作为一手依据。
- **仅采信官方**：排除所有 CSDN / 第三方博客结果。
- **局限**：
  - 断言 D（飞书文档详情接口的 `apiSchema.*` 路径）属内部接口契约，未能从公开页直接核实，仅以仓库内置实证佐证。
  - 未逐个核对 acs 全部接口的鉴权方式分布（仅抽样 `device_bind` 一例证伪规范归纳）。
  - 国际站 `open.larksuite.com` 未单独验证，结论以中文站为准。
