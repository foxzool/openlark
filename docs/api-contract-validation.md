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

CI 当前只启用这一层 strict gate。

### 1.2 Endpoint live 校验

需要确认当前官网详情页是否和 checked-in CSV 快照一致时，使用：

```bash
python3 tools/validate_api_contracts.py \
  --crate openlark-ai \
  --live-endpoints \
  --strict endpoint \
  --report-dir /tmp/openlark-api-contracts-live-endpoints
```

该模式会访问飞书文档详情接口，读取 `schema.apiSchema.httpMethod` 和 `schema.apiSchema.path`，再和 Rust 实现比较。它适合人工抽样或 API 变动排查，不适合作为默认本地快检。

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

该模式会读取当前官网详情页中的结构化 schema，并和 Rust 结构体的可序列化字段比较。当前覆盖：

- request body 顶层字段：`schema.apiSchema.requestBody.content.*.schema.properties`
- response data 顶层字段：`schema.apiSchema.responses.200.content.application/json.schema.properties.data.properties`

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
| `U_OFFICIAL_DETAIL_FETCH_FAILED` | live 模式无法获取官网详情 payload |

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

- 字段级校验目前只覆盖 request body 顶层字段和 response `data` 顶层字段。
- 嵌套字段、query/path 参数还未纳入 strict 比较。
- endpoint 解析对复杂 `to_url()` enum 或动态拼接会给出 `W_ENDPOINT_UNRESOLVED`，不会在 endpoint strict 模式下失败。
- live 模式依赖飞书官网详情接口，适合抽样和排查，不应替代离线快检。
