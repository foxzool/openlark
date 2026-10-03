---
name: openlark-api-field-verify
description: OpenLark API 字段核对技能。用于新增/重构飞书 API 后，核对 Rust 实现的请求体/响应体字段是否与飞书官方文档一致。通过 playwright 渲染飞书 SPA 文档页面，提取真实的请求/响应字段定义，对比代码实现找出不符项。触发关键词：字段核对、字段验证、字段不符、文档核对、核对请求字段、核对响应字段、飞书文档字段、推断字段、user 级接口、用户级接口字段
allowed-tools: Bash, Read, Edit, Write, Grep, Glob
---

# OpenLark API 字段核对技能

## 🧭 技能路由指南

**本技能适用场景：**
- **实现前/后**读取飞书官方文档字段（本仓库唯一可靠的文档抓取入口）
- 新增/重构飞书 API 后，核对请求体/响应体字段是否与官方文档一致
- 怀疑某个 API 的字段是"推断"而非来自真实文档（如参照同族接口复制）
- 用户级（user_access_token）接口的字段核对（这类接口字段常与应用级不同）
- `fetch_docpath.py` 在线抓取失败（返回占位文本），需要替代方案

**其他技能：**
- 添加/重构 API 的实现规范 → `Skill(openlark-api)`（读文档后回此技能核对，或实现时先来此抓取）
- 统计 API 覆盖率/缺失清单 → `Skill(openlark-api-validation)`
- 代码规范、风格一致性 → `Skill(openlark-code-standards)`

### 关键词触发映射

- 字段核对、字段验证、字段不符、文档核对、核对请求字段、核对响应字段、飞书文档、playwright 抓文档 → `openlark-api-field-verify`
- 新增 API、重构 API、Builder、Request/Response → `openlark-api`
- 覆盖率、缺失 API、CSV 对比 → `openlark-api-validation`

### 双向跳转规则

- **`openlark-api` 需要读文档时必须来本技能**（勿用 `fetch_docpath.py` 在线抓取）
- 若核对发现字段不符需要修正实现，转 `openlark-api` 落地修正
- 若核对发现是 API 尚未实现，转 `openlark-api` 补齐
- 若核对根源是覆盖率脚本误报，转 `openlark-api-validation`

## 🎯 技能用途

飞书开放平台文档是 **SPA（单页应用）**，内容靠 JS 动态渲染。常见的两种抓取方式各有局限：

| 方式 | 问题 |
|------|------|
| `fetch_docpath.py`（项目 skill 脚本） | 对**新接口**常返回占位文本，抓不到字段表 |
| 直接 HTTP 请求文档 URL | 只拿到 SPA 外壳，正文为空 |
| web reader / 搜索引擎 | 新接口搜不到，SPA 抓不到 |

**本技能用 playwright 真实渲染页面**，等待 JS 执行后提取 `innerText`，拿到完整的字段表。这是目前唯一可靠的方式。

## 📋 核心工作流

### 第 0 步：判断是否需要核对

以下情况**必须核对**（字段易错）：
- ✅ 用户级（user_access_token）接口 —— 字段常与应用级不同（无 user_id，从 token 推断）
- ✅ 参照"同族接口"复制的实现 —— 字段名/结构可能已变
- ✅ 新上的飞书接口 —— 文档可能尚未被旧脚本收录
- ✅ 请求体有数组/嵌套对象的接口 —— 上限、子字段易漏

以下情况可跳过：
- ⏭️ 直接照抄飞书官方 JSON 示例实现的（已有真实样本）
- ⏭️ 仅改端点 URL、字段未动的重构

### 第 1 步：找到正确的文档 URL

**这是最易错的一步。** URL **唯一权威源**是 CSV 的 `fullPath`：

```text
canonical_url = "https://open.feishu.cn" + fullPath
```

| 来源 | 是否可用 | 说明 |
|------|---------|------|
| `fullPath` | ✅ 唯一权威 | 原样拼接，不要改路径格式 |
| `docPath` | ❌ 默认勿用 | 常与 `fullPath` 不一致（大量 server-docs vs 实际路径） |
| 手拼 `/reference/...` 或 `/server-docs/...` | ❌ 禁止 | 易 404："The documentation could not be found." |

```bash
# 从 CSV 用 api id 或 url 反查 fullPath
python3 -c "
import csv
with open('api_list_export.csv', encoding='utf-8-sig') as f:
    for row in csv.DictReader(f):
        if row['id'] == '7642253323628383198' or 'approval/v4/tasks/pass' in row['url']:
            print(row['fullPath'])
"
```

> ⚠️ 若用错路径，页面会显示 "The documentation could not be found."，**这不是抓取失败，是 URL 错了**。

### 第 2 步：用 playwright 渲染抓取

#### 环境准备

```bash
# 确认 playwright + chromium 已装（agent-browser 自带的版本可能不匹配）
node -e "require('playwright')" 2>/dev/null && echo "playwright ok" || npm i -g playwright
npx playwright install chromium  # 装匹配版本（约 170MB）
```

> ⚠️ `agent-browser` CLI 绑定的 playwright 版本可能与系统全局版不一致，导致 "Executable doesn't exist"。**优先用本技能自带的 `scripts/fetch_doc.js`**，它自动用匹配的 playwright。

#### 抓取单页

```bash
# 推荐：按 CSV api-id（脚本内用 fullPath 拼 URL）
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  --from-csv 7642253323628383198 \
  --out /tmp/doc_pass.txt

# 或直接传完整 URL / fullPath
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass" \
  /tmp/doc_pass.txt
```

脚本会 `waitUntil: 'networkidle'` + 多次延时 + 滚动触发懒加载，导出完整 `innerText`。正常应抓到 5000-8000 字符；若 < 500 字符，说明 URL 错或页面没渲染（回第 1 步检查 URL）。**实现前抓取失败不得继续写字段。**

#### 批量抓取

传入 **完整 fullPath 或完整 URL**（勿再传「去掉前缀的短 path」——旧用法会错误拼到 `/reference/` 下）：

```bash
node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js \
  --batch \
  /document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/add_cc \
  /document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass \
  --out-dir /tmp/docs
```

#### 批量自动化（推荐用于多接口/全 crate 核对）

对于多接口或全 crate 核对，使用自动化工具而非手动逐个：

```bash
# 快速模式：全仓代码自检（秒级，不抓文档）
# 无参数裸跑 = 扫描整个 crates，生成 reports/api_field_verify/all.md
python3 tools/verify_api_fields.py

# 快速模式：单个 crate
python3 tools/verify_api_fields.py --crate openlark-workflow

# 完整模式：抓飞书文档对比字段（慢，约 8 秒/API）
# 批量模式默认复用未超龄 Official Evidence 快照（--max-age 天，默认 30）
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs

# 强制忽略快照重抓 / 自定义超龄阈值
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs --force-refresh
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs --max-age 7

# 单个 API 核对门禁（实现后必做）：默认 Fresh 重抓（单页约 8 秒）
# 抓取失败 / error / warning → 非 0 退出（禁止假绿）
python3 tools/verify_api_fields.py --api-id 7642253323628383198 --fetch-docs
```

工具自动完成路径解析、字段提取、文档抓取、差异对比，输出 `reports/api_field_verify/` 报告。
`--fetch-docs` 模式下：文档抓取失败、内容过少/404、字段 error/warning 均记入报告并以非 0 退出；info 不阻断。
设计文档见 `docs/superpowers/specs/2026-06-16-api-field-verify-tool-design.md`。

### 工具核对边界

门禁通过 ≠ 字段完全正确。自动化覆盖有限，以下边界需知情：

1. **嵌套结构盲区**：请求体只对比「第一个名字含 Body 的 struct」；嵌套子对象的独立 struct（通常不叫 `*Body`）不参与对比。文档 innerText 拍平后嵌套子字段可能被当成顶层参数，误报/漏报方向不定。响应侧只做「示例字段名集合差」，字段在错误层级也算存在。
2. **响应体是弱保证**：`missing_response_field` 仅 info 不阻断；代码多余的响应字段完全不检测。响应体完整性需人工比对 Response body example。
3. **人工核对项**（门禁不覆盖）：
   - 数组上限（如 `cc_user_ids` ≤20）——无法从 innerText 结构化解析
   - 嵌套子结构字段与层级（见上）
   - 响应体字段完整性与类型（见上）

> 类型与必填性已纳入 `compare_fields()` 自动对比（文档 Yes+代码 Option → error；类型映射不匹配 → warning）；不再需要人工逐项核对这两类。

### 第 3 步：解析字段

抓取到的 `innerText` 是拍平的表格（参数名、类型、必填、描述交错成行）。解析规则：

**POST 接口的 Request body**（最常见）：
```
Request body（第二次出现）... Request example 之间
每段：参数名行 → 类型行(string/int/string[]/-) → 必填行(Yes/No) → 描述
```

**GET 接口的 Query parameters**：
```
Query parameters ... Request example 之间
结构同上
```

**Response body 的 data 子字段**：
- 外层只有 `code/msg/data`
- `data` 的子字段在折叠的 "Show sublists" 里，innerText 拿不到
- **改从 Response body example 的 JSON 提取字段名**：`grep -oE '"[a-z][a-z0-9_]*"\s*:' doc.txt | sort -u`

#### 解析辅助命令

```bash
# 提取 POST 请求体字段（参数名/类型/必填）
awk '/^Request body$/{c++; if(c==2){p=1; next}} /^Request example$/{p=0} p' doc_xxx.txt \
  | grep -E "^[a-z_]+$|^(Yes|No)$|^(string|int|boolean|string\[\]|object|-)$" \
  | grep -vE "^(parameter|type|required|description)$"

# 提取响应示例里的所有字段名（用于完整建模响应体；含 i18n_name/md5/s3_key 等数字字符）
awk '/^Response body example$/{p=1} /^Error code$/{p=0} p' doc_xxx.txt \
  | grep -oE '"[a-z][a-z0-9_]*"\s*:' | tr -d '":' | sort -u

# 确认 POST 响应的 data 是否空对象（决定 Response struct 是否留空）
awk '/^Response body example$/{p=1} /^Error code$/{p=0} p' doc_xxx.txt \
  | grep -oE '"data".{0,30}' | head -1
```

### 第 4 步：对比实现，列出差异

把真实字段与代码实现逐项对比，常见差异类型：

| 差异类型 | 例子 | 危害 |
|---------|------|------|
| **多余字段** | 用户级接口不该有 `user_id`（从 token 推断） | 序列化发出多余字段，可能被服务端拒绝 |
| **缺字段** | `remind` 漏了 `task_ids[]` | 功能不完整 |
| **字段名错** | `transfer_to_user_id` → `transfer_user_id` | 调用必失败 |
| **上限错** | `cc_user_ids` 上限 20 而非 1000 | 校验过松 |
| **类型错** | `add_sign_type` 是 int 不是 string | 序列化类型不符 |
| **响应字段缺失** | detail 响应有 10+ 字段，只建了 3 个 | 用户拿不到数据 |

**重点核对用户级接口**：请求体**不含** `user_id`/`approval_code`（这些是应用级接口的字段，用户级从 token 推断）。

### 第 5 步：修正实现

按 `openlark-api` 技能的实现规范修正：
- 请求体字段：严格对齐真实文档（必填校验依赖它）
- 响应体字段：按真实示例完整建模（用 `#[serde(default)]` 容忍未列出的可选字段）
- 修正后跑 `just fmt && just lint && just test` 验证

## 🔧 配套脚本

### scripts/fetch_doc.js

playwright 渲染抓取脚本（本仓库读飞书文档的**唯一在线入口**）：

- 单页：`node fetch_doc.js <完整URL|fullPath> <out.txt>`
- 按 CSV：`node fetch_doc.js --from-csv <api_id> --out <out.txt>`
- 批量：`node fetch_doc.js --batch <fullPath|URL>... --out-dir <dir>`

URL 解析规则：以 `http` 开头原样使用；以 `/` 开头则拼 `https://open.feishu.cn`；**禁止**手拼 `/reference/` 或 `/server-docs/` 前缀。

> 依赖：`playwright` npm 包 + chromium。首次用前跑 `npx playwright install chromium`。

## 🚨 常见陷阱

### 1. URL 路径错误（最高频）

症状：抓到的内容 < 500 字符，含 "The documentation could not be found."

原因：用手拼了 `server-docs` / `reference` 前缀，或误用了 CSV `docPath`（常与 `fullPath` 不一致）。

解决：**永远用 CSV `fullPath` 拼 `https://open.feishu.cn` + fullPath**，或 `--from-csv <api_id>`。

### 2. 响应 data 子字段在折叠区

症状：Response body 段只显示外层 `code/msg/data`，看不到 data 内部字段。

原因：data 的子字段在 "Show sublists" 折叠区，innerText 拿不到。

解决：从 **Response body example 的 JSON** 提取字段名（`grep -oE '"[a-z][a-z0-9_]*"\s*:'`），示例里出现的字段就是真实字段。

### 3. playwright 版本不匹配

症状：`Executable doesn't exist at .../chromium_headless_shell-XXXX`。

原因：`agent-browser` CLI 绑定的 playwright 版本与已装的 chromium build 号不一致。

解决：在本技能脚本所在目录跑 `npx playwright install chromium`，让它装匹配版本；或直接用 `scripts/fetch_doc.js`（它会用全局匹配的 playwright）。

### 4. 用户级 vs 应用级字段混淆

症状：用户级接口的请求体多了 `user_id`、`approval_code`。

原因：参照了应用级同族接口（如 `approve.rs`）复制字段。

解决：用户级接口（需 `user_access_token`）的请求体**不含** `user_id`——操作者身份从 token 推断。核对时优先排除这类字段。

## 📝 核对报告模板

核对完成后，输出对比清单供决策：

```markdown
## 字段核对结果：<接口名>

### 请求体差异
| 字段 | 真实文档 | 当前实现 | 问题 |
|------|---------|---------|------|
| user_id | ❌ 不存在 | ✅ 有 | 多余（用户级从 token 推断） |
| task_ids | ✅ 必填 string[] | ❌ 缺失 | 缺字段 |

### 响应体差异
- 真实字段：definition_name, start_time, status, form, tasks[]（10+ 字段）
- 当前实现：仅 3 字段
- 建议：完整建模

### 修正建议
- [ ] 删除 user_id/approval_code
- [ ] 补 task_ids 字段
- [ ] 响应体完整建模
```

## 🔗 相关技能

- **添加/重构 API 实现**：`Skill(openlark-api)` —— 实现前先用本技能抓文档；核对差异后回它落地修正；其 checklist 含本技能核对门禁
- **覆盖率验证**：`Skill(openlark-api-validation)` —— 核对文件落盘是否完整（不管字段正确性）
- **校验风格**：`Skill(openlark-validation-style)` —— `validate_required` vs `validate_required_list` 用法
