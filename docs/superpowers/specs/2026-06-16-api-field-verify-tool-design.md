# API 字段核对工具设计

**日期**: 2026-06-16
**状态**: 已实现并投入使用（`tools/verify_api_fields.py` + Official Document Evidence）
**关联技能**: `openlark-api-field-verify`（手动单接口流程）、`openlark-api-validation`（覆盖率）
**关联 Issue**: #618–#624（2026-08 字段核对流程加固）

## 背景与动机

OpenLark 仓库现有 **1593 个已实现 API**，分布在 15 个 crate。这些 API 的请求体/响应体字段有一部分是**参照同族接口推断**而来，而非直接来自飞书官方文档（详见 approval v4 用户级接口的修正过程：11 个接口中 8 个有字段偏差）。

现有工具只解决两个问题：
- `validate_apis.py`：检查 API 文件**在不在**（覆盖率）
- `field-verify` 技能：**手动逐个**核对单接口字段

缺失的是**批量、自动化的字段正确性核对工具**——能扫描全仓代码字段，对比飞书文档真实字段，输出差异报告。

## 目标

产出自动化工具 `tools/verify_api_fields.py`，用于定期/分批执行字段正确性核对。

### 非目标

- 不修改任何 API 实现（核对与修正分离）
- 不替代 `validate_apis.py`（覆盖率 vs 正确性，互补）
- 不做运行时反射式验证（需要为每个 API 写触发代码，违背自动化）

## 方案选择

经评估三方案，选定 **方案 B：正则 + 结构匹配**：

| 方案 | 准确度 | 成本 | 选择 |
|------|--------|------|------|
| A. syn 语法解析 | 最高（识别所有 serde 属性） | 高（独立 Rust 工具，编译重） | ✗ |
| **B. 正则结构匹配** | 够用（仓库字段定义高度规整） | 低（Python，复用现有扫描逻辑） | **✓** |
| C. 运行时反射 | 100% 真实 | 极高（每个 API 要写序列化触发） | ✗ |

方案 B 的可靠性依据：仓库 `Body`/`Response` struct 写法高度统一（`pub field: Type`，serde 属性独立成行），正则可稳定提取。能发现的主要问题——多余字段、缺字段、字段名错、类型/必填不一致——正是实际发生过的偏差类型。

## 架构

三段式流水线：

```
api_list_export.csv ──┐
                      ├─→ 1. 路径解析（每个 API → .rs 文件路径）
                      │
crates/**/*.rs ───────┤
                      ├─→ 2. 代码字段提取（正则扫描 Body/Response struct）
                      │
飞书文档页面 ─────────┤   （仅完整模式；经 Official Document Evidence）
                      ├─→ 3. 文档字段抓取（structured detail → rendered fallback）
                      │
                      └─→ 4. 对比 → reports/api_field_verify/<crate>.md
```

### 双运行模式

**快速模式（默认）**：只做代码字段自检 + 可疑模式检测，**不抓文档**，秒级完成全仓。适合定期扫描发现明显问题。

**完整模式（`--fetch-docs`）**：经 Official Document Evidence 抓飞书文档对比字段，慢但彻底（1 个文档约 8 秒）。适合分批跑单个 crate 或手动触发。

## 组件设计

### 1. 路径解析

复用 `tools/api_coverage.toml` 的 crate→bizTag 映射和 `validate_apis.py` 的路径推断逻辑：
- CSV 的 `meta.Project/Version/Resource/Name` → 推断 `.rs` 文件路径
- `meta.Resource` 的 `.` → `/`，`meta.Name` 的 `:` → `_`

### 2. 代码字段提取（核心）

**提取对象**：每个 API 文件的 `XxxBody`（请求体）和 `XxxResponse`（响应体）struct。

**提取规则**：

```rust
// 输入：仓库规整的 struct 定义
pub struct PassTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,                    // → instance_code, String, 必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,                     // → form, String, 选填
    pub cc_user_ids: Vec<String>,                 // → cc_user_ids, String, 必填(数组)
}
```

**三步提取**：
1. 定位 struct 块：正则 `pub struct (\w+(?:Body|Response)\w*)\s*\{` 匹配到 `}`
2. 提取字段行：块内匹配 `pub (\w+):\s*(.+?),?$`，跳过注释行和 `#[serde(...)]`/`#[cfg(...)]` 属性行
3. 解析类型与可选性：
   - `Option<T>` → 选填
   - `Vec<T>` → 数组必填（`is_array=true`）
   - 裸 `String`/`i32`/`bool` → 必填
   - 记录 `#[serde(rename = "xxx")]`（对比时用 rename 后的名字）

**已知局限**（文档化，不阻塞）：
- 复杂泛型（`HashMap<K,V>`、嵌套 `Vec<Vec<>>`）只取首层类型名
- `#[serde(flatten)]` 字段无法展开（仓库极少）
- 嵌套子对象的独立 struct（非 `*Body`）不参与对比——见技能「工具核对边界」

### 3. 可疑模式检测（快速模式核心）

不抓文档就能发现的问题，三类红旗：

| 红旗 | 检测逻辑 | 严重度 |
|------|---------|--------|
| 用户级接口含 `user_id`/`approval_code` | CSV 的 `fullPath` 含 `/reference/`（新版用户级文档路径），且其 Body 含这些字段。注：判定依据是 `fullPath` 而非 API url，因为 reference 是文档路径标识 | 🟢 提示（需人工判断） |
| Vec 字段缺非空校验 | Body 有必填数组字段，但同文件无 `validate_required_list!` / `is_empty()` | 🟡 警告 |
| GET 查询接口的 Response 为空 `{}` | CSV 的 `url` 以 `GET:` 开头，且 Response struct 无字段 | 🟢 提示 |

### 4. 文档字段抓取与对比（完整模式）

**抓取**：经 `tools/api_contracts/official_evidence`（structured detail 优先，rendered innerText fallback）。手动单页可用 `.agents/skills/openlark-api-field-verify/scripts/fetch_doc.js`（固定 `en-US` locale）。

**文档字段解析**：

| 接口类型 | 解析段 | 提取 |
|---------|--------|------|
| POST | Request body（第2次出现）→ Request example | 参数名 + 必填 + 类型 |
| GET | Query parameters → Request example | 同上 |
| 所有 | Response body example JSON | 响应字段名集合（正则 `"([a-z][a-z0-9_]*)"\s*:`，含数字字符） |

**对比逻辑**（`compare_fields`）：

```
请求体：
  代码有 ∩ 文档无  → 多余字段（warning）
  代码无 ∩ 文档有  → 缺失字段（error）
  两边都有         → 继续比必填性与类型
    文档 Yes + 代码 Option → required_mismatch（error）
    文档 No  + 代码非 Option → required_mismatch（warning）
    文档类型可映射且与 Rust 核心类型不符 → type_mismatch（warning）
    文档 type 为空 / 未建模类型（object 等）→ 跳过类型对比

响应体：
  文档示例字段 - 代码 Response 字段 = 缺失的响应字段（info，弱保证）
```

**差异分级**：
- 🔴 硬错误：必填字段缺失、文档必填但代码为 Option
- 🟡 警告：多余字段、类型不一致、文档选填但代码更严、证据不完整
- 🟢 提示：响应字段可能缺失、用户级启发式红旗

**缓存与刷新**（Official Evidence 快照，目录 `reports/api_field_verify/official_evidence/`）：
- 批量 `--fetch-docs`：默认 `PreferSnapshotPolicy(max_age_days=30)`——有未超龄快照则复用，否则重抓
- `--force-refresh`：`FreshOfficialPolicy`，忽略快照强制重抓
- `--max-age N`：批量模式快照最大年龄（天），默认 30
- 单 API 门禁（`--api-id` + `--fetch-docs`）：默认 `FreshOfficialPolicy`（单页约 8 秒，对齐官网）

**并发**：当前**串行**抓取（避免限流）。`--max-workers N` **未实现**，列为后续可选优化。

**失败处理**：失败的 API 记入 `failed.json` 不阻塞整体扫描；完整模式汇总 error/warning 时非 0 退出。

## 输出格式

### Markdown 报告（`reports/api_field_verify/<crate>.md`）

```markdown
# API 字段核对报告：openlark-workflow

## 一、总体统计
| 指标 | 数量 |
|------|------|
| 核对 API 数 | 118 |
| 文件存在 | 118 |
| 有问题 | 23 |

## 二、问题详情（按严重度）
### 🔴 硬错误（N）
### 🟡 警告（N）
### 🟢 提示（N）
```

### JSON 汇总（`reports/api_field_verify/summary.json`）

结构对齐现有 `api_validation/summary.json`，含每个 API 的字段级差异与 evidence provenance，便于趋势对比和 CI 集成。

## 命令行接口

```bash
# 快速模式（默认）：全仓代码自检，秒级（无参数 = 全仓，无 --all-crates 旗标）
python3 tools/verify_api_fields.py

# 快速模式：单个 crate
python3 tools/verify_api_fields.py --crate openlark-workflow

# 完整模式：抓文档对比（慢；批量默认复用未超龄快照）
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs

# 完整模式：强制重抓 / 自定义超龄
python3 tools/verify_api_fields.py --crate openlark-docs --fetch-docs --force-refresh
python3 tools/verify_api_fields.py --crate openlark-docs --fetch-docs --max-age 7

# 单个 API（门禁；默认 Fresh 重抓）
python3 tools/verify_api_fields.py --api-id 7642253323628383198 --fetch-docs
```

> 历史设计中的 `--all-crates` / `--resume` / `--max-workers` **不是**当前 CLI 旗标。
> 等价行为：裸跑 = 全仓；快照复用 + `--max-age`/`--force-refresh` 替代旧「文件存在即跳过」；并发为后续项。

## 测试与集成

### 回归测试（`tools/tests/test_verify_api_fields.py`）

- 字段提取（含 Vec `is_array`、serde rename）
- 可疑模式检测
- `compare_fields` 名字 / 必填 / 类型差异
- 响应示例含数字字段名（`i18n_name` 等）
- 单 API CLI evidence 契约与多 crate 路径告警
- Official Evidence 快照 `max_age` 过期重抓

### CI 集成

- **快速模式**：每周定时 workflow（`.github/workflows/api-field-verify-weekly.yml`），全仓自检；发现 error/warning 时开或更新 tracking issue，**不因 findings 失败**（工具崩溃除外）
- **完整模式**：`workflow_dispatch`（`.github/workflows/api-field-verify-full.yml`），可选 crate，上传 `reports/api_field_verify/` artifact

## 与现有工具的关系

| 工具 | 解决问题 | 关系 |
|------|---------|------|
| `validate_apis.py` | 文件在不在（覆盖率） | 互补，新工具检查正确性 |
| `field-verify` 技能 | 手动单接口核对 | 新工具是其批量化版本 |
| `compare_api_catalogs.py` | API 清单增删变 | 不同层面（清单 vs 字段） |
| `official_evidence` | 官方文档证据采集 | 完整模式的文档来源 |

## 开放问题

- **飞书限流**：完整模式批量抓 100+ 文档可能触发限流；当前串行，后续可评估受控并发（`--max-workers`）
- **reference vs server-docs 路径**：部分老接口 fullPath 是 server-docs 格式，抓取逻辑需兼容两种（fetch_doc.js / rendered worker 已处理，但解析段位置可能不同）
- **嵌套结构与响应体弱保证**：见技能「工具核对边界」；不在本工具自动门禁范围内
