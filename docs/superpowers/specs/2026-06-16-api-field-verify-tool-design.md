# API 字段核对工具设计

**日期**: 2026-06-16
**状态**: 已批准，待实现
**关联技能**: `openlark-api-field-verify`（手动单接口流程）、`openlark-api-validation`（覆盖率）

## 背景与动机

OpenLark 仓库现有 **1593 个已实现 API**，分布在 15 个 crate。这些 API 的请求体/响应体字段有一部分是**参照同族接口推断**而来，而非直接来自飞书官方文档（详见 approval v4 用户级接口的修正过程：11 个接口中 8 个有字段偏差）。

现有工具只解决两个问题：
- `validate_apis.py`：检查 API 文件**在不在**（覆盖率）
- `field-verify` 技能：**手动逐个**核对单接口字段

缺失的是**批量、自动化的字段正确性核对工具**——能扫描全仓代码字段，对比飞书文档真实字段，输出差异报告。

## 目标

产出自动化工具 `tools/verify_api_fields.py`，**本次只建工具不执行全量核对**，留作后续定期/分批执行。

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

方案 B 的可靠性依据：仓库 `Body`/`Response` struct 写法高度统一（`pub field: Type`，serde 属性独立成行），正则可稳定提取。能发现的主要问题——多余字段、缺字段、字段名错、上限不符——正是实际发生过的偏差类型。

## 架构

三段式流水线：

```
api_list_export.csv ──┐
                      ├─→ 1. 路径解析（每个 API → .rs 文件路径）
                      │
crates/**/*.rs ───────┤
                      ├─→ 2. 代码字段提取（正则扫描 Body/Response struct）
                      │
飞书文档页面 ─────────┤   （仅完整模式）
                      ├─→ 3. 文档字段抓取（复用 fetch_doc.js + 解析）
                      │
                      └─→ 4. 对比 → reports/api_field_verify/<crate>.md
```

### 双运行模式

**快速模式（默认）**：只做代码字段自检 + 可疑模式检测，**不抓文档**，秒级完成全仓。适合定期扫描发现明显问题。

**完整模式（`--fetch-docs`）**：抓飞书文档对比字段，慢但彻底（1 个文档约 8 秒，全量约 3.5 小时）。适合分批跑单个 crate 或手动触发。

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
    pub cc_user_ids: Vec<String>,                 // → cc_user_ids, Vec<String>, 必填(数组)
}
```

**三步提取**：
1. 定位 struct 块：正则 `pub struct (\w+(?:Body|Response)\w*)\s*\{` 匹配到 `}`
2. 提取字段行：块内匹配 `pub (\w+):\s*(.+?),?$`，跳过注释行和 `#[serde(...)]`/`#[cfg(...)]` 属性行
3. 解析类型与可选性：
   - `Option<T>` → 选填
   - `Vec<T>` → 数组必填
   - 裸 `String`/`i32`/`bool` → 必填
   - 记录 `#[serde(rename = "xxx")]`（对比时用 rename 后的名字）

**已知局限**（文档化，不阻塞）：
- 复杂泛型（`HashMap<K,V>`、嵌套 `Vec<Vec<>>`）只取首层类型名
- `#[serde(flatten)]` 字段无法展开（仓库极少）

### 3. 可疑模式检测（快速模式核心）

不抓文档就能发现的问题，三类红旗：

| 红旗 | 检测逻辑 | 严重度 |
|------|---------|--------|
| 用户级接口含 `user_id`/`approval_code` | CSV 的 `fullPath` 含 `/reference/`（新版用户级文档路径），且其 Body 含这些字段。注：判定依据是 `fullPath` 而非 API url，因为 reference 是文档路径标识 | 🟡 警告 |
| Vec 字段缺 `validate_required_list!` | Body 有 `Vec<String>` 字段，但同文件 `execute_with_options` 无对应校验 | 🟡 警告 |
| GET 查询接口的 Response 为空 `{}` | CSV 的 `url` 以 `GET:` 开头，且 Response struct 无字段 | 🟢 提示 |

### 4. 文档字段抓取与对比（完整模式）

**抓取**：复用 `.agents/skills/openlark-api-field-verify/scripts/fetch_doc.js`，URL 从 CSV 的 `fullPath` 拼接（不自己拼，避免路径格式错）。

**文档字段解析**：

| 接口类型 | 解析段 | 提取 |
|---------|--------|------|
| POST | Request body（第2次出现）→ Request example | 参数名 + 必填 + 类型 |
| GET | Query parameters → Request example | 同上 |
| 所有 | Response body example JSON | 响应字段名集合 |

**对比逻辑**：

```
请求体：
  代码有 ∩ 文档无  → 多余字段
  代码无 ∩ 文档有  → 缺失字段
  两边都有         → 一致

响应体：
  文档示例字段 - 代码 Response 字段 = 缺失的响应字段
```

**差异分级**：
- 🔴 硬错误：必填字段缺失、字段名错（rename 不匹配）
- 🟡 警告：多余字段、Vec 上限不符、响应字段漏建
- 🟢 提示：可选字段差异

**性能控制**：
- 默认串行抓（避免限流），`--max-workers N` 控制并发
- 失败的 API 记入 `failed.json` 不阻塞整体
- `--resume` 跳过已抓取的（按文件 mtime 判断）

## 输出格式

### Markdown 报告（`reports/api_field_verify/<crate>.md`）

```markdown
# API 字段核对报告：openlark-workflow

## 一、总体统计
| 指标 | 数量 |
|------|------|
| 核对 API 数 | 118 |
| 完全一致 | 95 |
| 有差异 | 23 |
| 抓取失败 | 0 |

## 二、差异详情（按严重度分组）
### 🔴 硬错误（N）
| API | 文件 | 问题 | 详情 |
### 🟡 警告（N）
### 🟢 提示（N）

## 三、按 API 明细
### POST /open-apis/approval/v4/tasks/pass
- 文件: task/pass.rs
- 请求体: ✅ 一致
- 响应体: ✅ 一致
- 模式检测: 🟢 通过
```

### JSON 汇总（`reports/api_field_verify/summary.json`）

结构对齐现有 `api_validation/summary.json`，含每个 API 的字段级差异，便于趋势对比和 CI 集成。

## 命令行接口

```bash
# 快速模式（默认）：全仓代码自检，秒级
python3 tools/verify_api_fields.py --all-crates

# 快速模式：单个 crate
python3 tools/verify_api_fields.py --crate openlark-workflow

# 完整模式：抓文档对比（慢）
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs

# 完整模式 + 并发 + 断点续跑
python3 tools/verify_api_fields.py --crate openlark-docs --fetch-docs --max-workers 4 --resume

# 单个 API（调试用）
python3 tools/verify_api_fields.py --api-id 7642253323628383198 --fetch-docs
```

## 测试与集成

### 回归测试（`tools/tests/test_verify_api_fields.py`）

用几个**已知正确**的 API（如刚修正的 approval v4 用户级接口）做黄金样本：
- 验证字段提取逻辑不回归
- 验证可疑模式检测能正确识别用户级接口的 `user_id` 红旗
- 复用现有 `tools/tests/` 的 unittest 框架

### CI 集成（可选，后续）

- 快速模式：每周定时跑，发现可疑模式开 issue（复用 `feishu-api-catalog-watch.yml` 模式）
- 完整模式：`workflow_dispatch` 手动触发（耗时长，不适合定时）

## 与现有工具的关系

| 工具 | 解决问题 | 关系 |
|------|---------|------|
| `validate_apis.py` | 文件在不在（覆盖率） | 互补，新工具检查正确性 |
| `field-verify` 技能 | 手动单接口核对 | 新工具是其批量化版本 |
| `compare_api_catalogs.py` | API 清单增删变 | 不同层面（清单 vs 字段） |

## 实现顺序建议

1. 路径解析 + 代码字段提取（复用 validate_apis.py 扫描）
2. 可疑模式检测（快速模式即可用）
3. 报告生成（Markdown + JSON）
4. 文档抓取对接（完整模式，复用 fetch_doc.js）
5. 回归测试
6. 文档/技能更新（更新 field-verify 技能指向新工具）

## 开放问题

- **飞书限流**：完整模式批量抓 100+ 文档可能触发限流，需观察后调整并发数和重试策略
- **reference vs server-docs 路径**：部分老接口 fullPath 是 server-docs 格式，抓取逻辑需兼容两种（fetch_doc.js 已处理，但解析段位置可能不同）
