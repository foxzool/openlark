# API Contract Validation

本文档说明如何验证 OpenLark 的 typed API 实现是否和飞书开放平台官方接口契约一致。

这套校验补充 `tools/validate_apis.py` 的覆盖率口径。覆盖率只回答“接口文件是否存在”，contract validation 进一步回答“实现里的 HTTP endpoint、request 字段和 response 字段是否和官方文档一致”。

## 1. 校验层级

### 1.1 Endpoint 离线校验

默认推荐入口：

```bash
just api-contracts
```

等价命令：

```bash
python3 tools/validate_api_contracts.py --all-crates --strict endpoint
```

该模式不访问网络，使用仓库根目录的 `api_list_export.csv` 作为官方快照，检查：

- Rust 实现文件是否存在；
- `ApiRequest::get/post/put/patch/delete(...)` 方法是否匹配 CSV 中的 HTTP method；
- Rust endpoint 常量或简单 `format!` 路径是否匹配 CSV 中的 `/open-apis/...` path。

报告输出：

- `reports/api_contracts/summary.md`
- `reports/api_contracts/summary.json`
- `reports/api_contracts/crates/<crate>.md`
- `reports/api_contracts/crates/<crate>.json`

CI 启用的 strict gate（`api-contracts` job）：

- 全仓离线 endpoint strict（`--all-crates --strict endpoint`）
- token strict：security/auth 各一个显式 Trusted API inventory（见 §1.4）
- field live monitor：attendance 与 docs；field strict inventory 暂为空
- live endpoint monitor：`openlark-ai --live-endpoints`，无 strict

### 1.2 Endpoint live 校验

需要确认当前官网详情页是否和 checked-in CSV 快照一致时，使用：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-ai \
  --live-endpoints \
  --strict endpoint \
  --report-dir /tmp/openlark-api-contracts-live-endpoints
```

该模式通过统一的 Official Document Evidence `collect` seam 获取 Endpoint Evidence，再和 Rust 实现比较。CI monitor 使用 Structured-only composition，不安装 Playwright；需要 rendered fallback 但 adapter 不可用时会明确记录 non-passing Evidence 与既有 finding code。全仓 endpoint strict 继续使用 checked-in CSV，避免官方页面暂时不健康导致永久红灯；人工命令可组合 `--live-endpoints --strict endpoint`，同时保留既有 strict 退出码用途。

### 1.3 Field live 校验

字段级校验必须显式打开 live 模式：

```bash
just api-contract-fields openlark-ai 5
```

等价命令：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-ai \
  --fields \
  --live-fields \
  --max-field-apis 5 \
  --report-dir reports/api_contract_fields
```

该模式通过同一个 `collect` seam 获取 Request Fields 与 Response Fields Evidence，并和 Rust 结构体的可序列化字段比较。当前 Rust comparison 只消费 Evidence 中的顶层字段：

- request body 顶层 `FieldObservation`
- response body 顶层 `FieldObservation`

request 字段解析支持：

- `#[serde(rename = "...")]`
- `#[serde(rename_all = "camelCase")]`
- `Option<T>` optionality
- required request field missing

如果需要让字段漂移直接返回非零退出码，使用 strict 入口：

```bash
just api-contract-fields-strict openlark-ai 5
```

或：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-ai \
  --fields \
  --live-fields \
  --max-field-apis 5 \
  --strict fields
```

### 1.4 Token 类型 live 校验

核对 Rust 实现里 `.with_supported_access_token_types(...)` 声明的 token 类型，是否被
飞书官方文档「请求头 → Authorization」接受。这是 [#511](https://github.com/foxzool/openlark/issues/511)
acs / security_and_compliance 批量误配（误设 `App`/`app_access_token`）的防回归手段。

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-security \
  --api-id 7321978105899122716 \
  --tokens \
  --strict tokens \
  --report-dir /tmp/openlark-api-contracts-tokens
```

Token oracle 同样来自 Official Document Evidence `collect` seam。Structured Detail 无法产生 Trusted Tokens Evidence 时，manual/full composition 可逐维度回退到 live Rendered Document；CI composition 不安装 Playwright，fallback 不可用会明确返回 `Unavailable`，不会静默跳过或把 Recorded Snapshot 冒充 fresh evidence。

判定规则（被测对象 = Rust 声明的有效 token 集合，未显式声明时取默认 `[User, Tenant]`）：

- Rust 集合与官方集合**不相交** → `ERROR`（运行时注入的 token 必被飞书拒绝）。
- 官方未标注 → `UNVERIFIED`（无法核对，不阻塞）。
- 实现文件缺失 → `WARN`。
- 存在交集（SDK 至少能选出一种官方接受的 token）→ 无 finding。
- 声明 `None`（自行管理鉴权、bypass token cache）但源码手动注入
  `Authorization: Bearer <self.token_field>` 的端点（如 OIDC `authen/v1/user_info/get`），
  按实际注入的 token 类型核对，而非 `none_access_token`——避免把「手动注入 user token」
  误判为 disjoint `ERROR`。真正无鉴权（声明 `None` 且无手动注入）对要求 token 的文档仍报 `ERROR`。

CI 的 token strict gate 只包含已证明 Structured Tokens Evidence 为 `Trusted` 的显式
API inventory：`openlark-security/7321978105899122716` 与
`openlark-auth/7277403063290724380`。两个 crate 的全量 token Evidence 仍以
non-strict monitor 采集并上传报告；新增 strict API 必须先满足 §1.6 admission。

### 1.5 Field Evidence monitor 与 strict 准入

Structured-only composition 当前无法让 attendance/docs 的 Request Fields 与 Response
Fields 全部成为 `Trusted`；CI 不安装 Playwright，因此这两个域不能继续伪装成 strict
pass。完整 live 采集保留为 monitor，field strict inventory 暂为空。

| 域 | 范围 | 当前状态 | Issue |
|---|---|---|---|
| attendance | `openlark-hr --biz-tag attendance`（~39 API） | live monitor；未进入 strict inventory | #526 / #533 / #534 / #540 |
| docs | `openlark-docs`（ccm/base/baike/minutes，~214 API） | live monitor；未进入 strict inventory | #569 |
| *(next field-strict domain slot)* | 待选（**一域一 PR**） | 未 flip | 见 §1.6 admission |

本地复现 docs monitor：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-docs \
  --fields \
  --live-fields \
  --report-dir /tmp/openlark-api-contracts-docs-fields
```

monitor 的 `Incomplete` / `Unavailable` / `Rejected` 会保留在 JSON/Markdown 报告中。
只有域内 requested dimensions 全部为 `Trusted`，且 comparison 为 0 `ERROR`，才允许
加入 `--strict fields` inventory。

### 1.6 Trust gate inventory + strict admission（#586 / #616）

CI 必须明确区分 hard strict gate 与 live monitor；monitor 不得通过命名、参数或
`continue-on-error` 伪装成 strict pass。清单由 `.github/workflows/ci.yml` 与
`tools/tests/test_validate_api_contracts_ci_gates.py` 双改钉死。

#### 1.6.1 Gate inventory（当前 pinned 清单）

| 层 | 范围 | CI 模式 | Inventory pin |
|---|---|---|---|
| Endpoint strict | monorepo 全仓离线 | `--all-crates --strict endpoint` | `test_endpoint_strict_covers_all_crates_offline` |
| Endpoint monitor | `openlark-ai` live Structured | `--live-endpoints`，无 strict | `test_live_endpoint_monitor_uses_structured_only_cli_mode` |
| Token strict | security/auth 各一个 Trusted API | `--api-id … --strict tokens` | `test_token_strict_uses_explicit_trusted_api_inventory` |
| Token monitor | `openlark-security` + `openlark-auth` 全量 | `--tokens`，无 strict | `test_full_token_domains_remain_non_strict_monitors` |
| Field strict | 空 inventory | 无 `--strict fields` | `test_field_strict_inventory_is_empty_until_trusted` |
| Field monitor | attendance + docs | `--fields --live-fields`，无 strict | `test_field_monitor_inventory_is_exactly_attendance_and_docs` |

与 coverage 侧硬门禁的关系（**不在本 job 内执行，但同属 0.20 trust 程序**）：

| 锁 | 位置 | 不得弱化 |
|---|---|---|
| Typed-coverage hard gates | `tools/typed_coverage_release.toml` + `docs/typed-coverage-release-criteria.md` | 阈值不得下调（见 #586 非目标） |
| path_noise vs true_gap 分类 | `tools/validate_apis.py` 报告 + denoise 回归测试 | 分类保留；不得把噪音当「实现完成」删掉真相 |
| Core-business P0 missing = 0 | release gate / `core_business` dashboard | 不得靠降低门槛伪装 PASS |
| Platform P1 clear-or-disprove | `tools/tests/test_p1_platform_clear_or_disprove.py` | 锁仍绿 |
| Selective P2 path_noise | `tools/tests/test_p2_selective_slice.py` | 锁仍绿 |

本地复核 inventory（离线、秒级；不跑 live monitors）：

```bash
python3 -m unittest tools.tests.test_validate_api_contracts_ci_gates -v
python3 -m unittest tools.tests.test_typed_coverage_release_policy -v
```

#### 1.6.2 Strict Evidence admission（下一项准入）

每次只接纳一个显式 API inventory 项或一个完整 field 域。全部满足才允许 flip：

1. **Fresh Structured baseline**：CI composition 不安装 Playwright，也不得用 Recorded
   Snapshot 冒充 fresh evidence。
2. **Only Trusted**：候选范围内每个 requested Evidence Dimension 必须全部为
   `Trusted`；`Incomplete`、`Unavailable`、`Rejected` 任一出现都不得进入 strict。
3. **Comparison 0 ERROR**：Evidence Trusted 后，Rust comparison 必须为 0 `ERROR`。
4. **完整范围**：field 域 admission 必须覆盖整个声明域，不得用 `--max-field-apis`
   或单个 `--api-id` 冒充域级 strict。Token 可按显式 API inventory 逐项推进。
5. **Dual-edit（双改）**：同一变更必须同时更新 workflow、gate inventory test 与本节
   表格；只改一处即审查拒绝。
6. **域范围显式**：step 必须带 `--crate`，必要时带 `--biz-tag` / `--api-id`，并使用
   独立 report-dir。

推荐本地基线命令（以假设候选 `openlark-communication` 为例，**非**已 flip 域）：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-communication \
  --fields \
  --live-fields \
  --report-dir /tmp/openlark-api-contracts-candidate-fields
# 仅当 requested Evidence 全 Trusted 且 0 ERROR，才可 dual-edit 加入 strict inventory
```

#### 1.6.3 非目标（Explicit non-goals）

- **禁止 monorepo-wide field strict**：不得用 `--all-crates --strict fields`
  （或等价「一次打开全仓 fields」）替代域批推进。
- **禁止 hard-gate 阈值下调**：不得为了让 typed-coverage / contract CI 变绿而降低
  `tools/typed_coverage_release.toml` 中的 hard gate 阈值；阈值变更必须是独立
  policy PR，且只能升高或维持，不得降低。
- **本票不 march 新域**：#616 恢复 only-Trusted verdict 并重建诚实 inventory，
  不把任何尚有 non-Trusted Evidence 的 field 域写回 strict。

## 2. 单 crate 使用

只验证一个 crate 的 endpoint：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-docs \
  --strict endpoint
```

输出默认写到 `reports/api_contracts/`。如需避免污染本地报告目录：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-docs \
  --strict endpoint \
  --report-dir /tmp/openlark-api-contracts-docs
```

## 3. 结果解读

报告中的 severity：

| Severity | 含义 |
|---|---|
| `ERROR` | 已确认 contract drift；strict 模式会失败 |
| `WARN` | 实现缺失、解析不到或低噪声风险；endpoint strict 当前不因 warning 失败 |
| `UNVERIFIED` | 官方详情或实现形态无法机器确认，需要人工判断 |

Evidence 报告中，`Incomplete`、`Unavailable`、`Rejected` 均明确标记为 non-passing，并映射到既有 finding code。只要命令请求了 `--strict endpoint|fields|tokens`，其实际采集的 requested Evidence Dimensions 必须全部为 `Trusted`；任一 non-Trusted 状态都返回非零。live strict 范围经 `--biz-tag` / `--api-id` 等过滤后若没有采集到 requested Evidence，同样 fail closed，防止空 inventory 误绿。未请求 live Evidence 的离线 endpoint strict 继续按既有 `ERROR` verdict 工作。

常见 finding code：

| Code | 含义 |
|---|---|
| `E_ENDPOINT_METHOD_MISMATCH` | Rust `ApiRequest::*` 方法和官方 method 不一致 |
| `E_ENDPOINT_PATH_MISMATCH` | Rust endpoint path 和官方 path 不一致 |
| `W_ENDPOINT_UNRESOLVED` | validator 暂时无法解析实现里的 endpoint 表达式 |
| `W_IMPLEMENTATION_FILE_MISSING` | CSV 期望的 API 文件不存在 |
| `E_REQUIRED_REQUEST_FIELD_MISSING` | 官网 required request field 在 Rust 请求结构体中缺失 |
| `W_OPTIONAL_REQUEST_FIELD_MISSING` | 官网 optional request field 在 Rust 请求结构体中缺失 |
| `W_REQUIRED_REQUEST_FIELD_OPTIONAL` | 官网 required 字段在 Rust 中建模为 `Option<T>` |
| `W_RESPONSE_FIELD_MISSING` | 官网 response `data` 字段在 Rust 响应模型中缺失 |
| `E_ACCESS_TOKEN_TYPE_MISMATCH` | Rust 声明的 token 类型全被官方文档拒绝（不相交，鉴权必失败） |
| `U_ACCESS_TOKEN_UNANNOTATED` | 官方文档未标注 `supportedAccessToken`/Authorization，token 类型无法核对 |
| `U_OFFICIAL_DETAIL_FETCH_FAILED` | live 模式无法获取官网详情 payload |


JSON 顶层字段保持兼容，并追加 `evidence`。每个维度包含 `status`、`selected_source`、provenance、diagnostics 与 `acquisition_trail`；Markdown 报告同步追加逐维度 Evidence 表。

## 4. 当前已知验证证据

`openlark-ai` 的字段 live 烟测能发现真实漂移：

```bash
just api-contract-fields openlark-ai 1
```

当前报告会指出两个真实漂移：

- 官网 request body 要求 `multipart/form-data` 字段 `file`，而 Rust 实现是 `file_token, is_async`。
- 官网 response `data` 下有 `bank_card`，而 Rust 响应模型中是 `parsing_result` 及其派生字段。

这说明字段级 validator 已经能用官网当前结构化 schema 验出 request 和 response 的实现不一致。

## 5. 与覆盖率校验的关系

- `just api-coverage`：验证 API 文件覆盖率和缺失 API backlog。
- `just api-contracts`：验证已实现 API 的 endpoint contract。
- `just api-contract-fields`：抽样验证 request/response 字段是否和当前官网一致。

推荐日常顺序：

1. `just api-coverage`
2. `just api-contracts`
3. 对可疑 crate 跑 `just api-contract-fields <crate> <N>`

## 6. 当前限制

- 字段级 Rust comparison 目前只消费 request/response 顶层 `FieldObservation`。
- 嵌套字段、query/path 参数尚未纳入 strict comparison。
- endpoint 解析对**无法识别**的复杂动态拼接仍会给出 `W_ENDPOINT_UNRESOLVED`，不会单独触发 endpoint strict 失败。
- live acquisition 统一位于 Official Document Evidence 模块；manual/full 可用 Playwright fallback，CI 为 Structured-only 且 fail closed。

### 6.1 Docs CatalogEndpoint 解析（#568）

`openlark-docs` 的 typed API 几乎全部通过 `CatalogEndpoint::to_request()` 构造请求（而不是
`ApiRequest::get/post(...)` 字面量）。contract validator 现已支持：

- `api_endpoints.rs` **以及** `api_endpoints/**/*.rs` 子模块中的 enum `to_url` / `method`
- `Enum::Variant(...).to_request()` / `.to_request::<T>()` / `var.to_request()`
- `pub use Target as Alias` 与独立 baike/lingo catalog（路径前缀不可混用）

因此 docs 域的 path/method drift 会以 `E_ENDPOINT_*` **ERROR** 出现在报告中（不再被
`W_ENDPOINT_UNRESOLVED` 掩盖）。`just api-contracts` / CI `--strict endpoint` 对
`openlark-docs` 与其他 crate 使用同一 strict 规则。
