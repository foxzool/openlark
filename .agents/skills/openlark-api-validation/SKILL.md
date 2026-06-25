---
name: openlark-api-validation
description: OpenLark API 覆盖率验证技能。用于验证各 crate 的 API 实现数量与覆盖率，基于 tools/validate_apis.py 脚本和 api_list_export.csv 对比实际代码实现。触发关键词：API 验证、API 覆盖率、验证 API 数量、检查 API 实现、API 统计
argument-hint: "[crate-name|path|bizTag]"
allowed-tools: Bash, Read, Edit, Write
---

# OpenLark API 覆盖率验证技能

## 🧭 技能路由指南

**本技能适用场景：**
- 需要统计某个 crate/bizTag 的 API 覆盖率
- 需要输出缺失 API 清单与完成率报告
- 需要对比 `api_list_export.csv` 与实际落盘实现

**其他技能：**
- 项目级规范体检（架构/API/导出/校验一体）→ `Skill(openlark-code-standards)`
- 新增/重构具体 API → `Skill(openlark-api)`
- 审查整体架构与公共 API 收敛 → `Skill(openlark-design-review)`

### 关键词触发映射

- 覆盖率、缺失 API、实现数量、CSV 对比、验证脚本、报告 → `openlark-api-validation`
- 新增 API、重构 API、Builder、Request/Response、mod.rs 导出 → `openlark-api`
- 代码规范、规范检查、风格一致性、体检 → `openlark-code-standards`
- 架构设计、public API、收敛方案、feature gating、兼容策略 → `openlark-design-review`
- validate、必填校验、validate_required、空白字符串、校验聚合 → `openlark-validation-style`

### 双向跳转规则

- 若发现缺失 API 的根因是架构分层/范式混乱，转 `openlark-design-review`。
- 若发现问题是具体 API 尚未实现，转 `openlark-api` 落地实现。
- 若需要把覆盖率问题归因到全仓规范一致性，转 `openlark-code-standards`。

## 🎯 技能用途

本技能用于验证 OpenLark 项目中各 crate 的 API 实现覆盖率，通过对比 `api_list_export.csv` 中的 API 定义与实际代码实现，生成详细的覆盖率报告。

## 📋 快速工作流

### 1. 验证单个 crate 的 API 覆盖率

```bash
# 验证 openlark-docs crate
python3 tools/validate_apis.py --crate openlark-docs

# 验证 openlark-communication crate
python3 tools/validate_apis.py --crate openlark-communication

# 验证 openlark-meeting crate
python3 tools/validate_apis.py --crate openlark-meeting
```

**输出位置：** `reports/api_validation/{crate}.md`

### 2. 列出所有可用的 crate 映射

```bash
# 查看所有 crate → bizTag 映射
python3 tools/validate_apis.py --list-crates
```

**示例输出（以实际 `--list-crates` 输出为准）：**
```
📄 映射文件: tools/api_coverage.toml

- openlark-analytics: src=crates/openlark-analytics/src biz_tags=[search, report]
- openlark-application: src=crates/openlark-application/src biz_tags=[application, workplace]
- openlark-auth: src=crates/openlark-auth/src biz_tags=[auth, passport, verification_information, human_authentication]
...
```

> 映射共 **15 个 crate**（核对自 `tools/api_coverage.toml`）。新增 crate 时，在 `tools/api_coverage.toml` 追加 `[crates.<name>]` 段（`src` + `biz_tags`，可选 `dashboard_groups`）即自动纳入 `--crate`/`--all-crates`/`--list-crates`，无需改脚本。

### 3. 自定义验证范围

```bash
# 指定源码目录和业务标签
python3 tools/validate_apis.py \
  --csv api_list_export.csv \
  --src crates/openlark-docs/src \
  --filter ccm base baike \
  --output custom_report.md

# 包含旧版本 API
python3 tools/validate_apis.py --crate openlark-docs --include-old
```

### 4. 验证所有 crates（批量）

```bash
# 一条命令验证映射里的全部 crate，并生成汇总报告 + 仪表盘
python3 tools/validate_apis.py --all-crates

# 等价的日常快捷命令（just recipe）
just api-coverage
```

`--all-crates` 会遍历 `tools/api_coverage.toml` 中映射的 15 个 crate，逐个生成 `reports/api_validation/<crate>.md`，再产出：

- `reports/api_validation/summary.md` / `summary.json` —— 全仓汇总
- `reports/api_validation/dashboards/<group>.{md,json}` —— 按 `dashboard_groups` 分组（如 `core_business`）

## 📊 报告解读

### 报告结构

生成的 Markdown 报告包含以下部分：

#### 一、总体统计
- **API 总数**：CSV 中定义的 API 数量
- **已实现**：已实现的 API 数量
- **未实现**：缺失的 API 数量
- **完成率**：实现百分比
- **额外文件**：代码中存在但 CSV 中未定义的文件

#### 二、模块统计
按 bizTag 分组的统计信息，展示各业务域的完成率。

#### 三、未实现的 API
详细列出所有未实现的 API，包括：
- API ID
- 预期文件路径
- API URL
- 文档链接

#### 四、额外的实现文件
列出不匹配 CSV 定义的额外文件（可能是辅助文件或需要更新 CSV）。

#### 五、已实现的 API
按模块分组列出所有已实现的 API。

### 示例报告片段

> 以下数字仅为格式示意，**以实际生成的报告为准**，请勿当作覆盖率基线。

```markdown
## 一、总体统计

| 指标 | 数量 |
|------|------|
| **API 总数** | 254 |
| **已实现** | 240 |
| **未实现** | 14 |
| **完成率** | 94.5% |
| **额外文件** | 3 |

## 二、模块统计

| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |
|------|---------|--------|--------|--------|
| BASE | 49 | 48 | 1 | 98.0% |
| BAIKE | 27 | 27 | 0 | 100.0% |
| CCM | 174 | 160 | 14 | 92.0% |
| MINUTES | 4 | 4 | 0 | 100.0% |
```

## 🔧 配置文件

### tools/api_coverage.toml

定义 crate → bizTag 映射关系，用于自动补全验证参数。

**格式：**
```toml
[crates.{crate_name}]
src = "crates/{crate_name}/src"
biz_tags = ["bizTag1", "bizTag2", ...]
```

**添加新 crate 映射：**
1. 编辑 `tools/api_coverage.toml`
2. 追加 `[crates.<name>]` 段（`src` + `biz_tags`，可选 `dashboard_groups`）
3. 运行 `--list-crates` 验证配置

### tools/api_priority.toml

缺失 API 的**业务优先级模型**，把“有多少 API 没实现”升级为“缺口按价值怎么排序”。

- **综合分公式**（核对自 `tools/validate_apis.py` 的 `priority_formula()`）：

  `综合分 = 业务价值×0.50 + 高频场景×0.30 + (6 - 实现复杂度)×0.20`

  三个维度均取 1–5（`[defaults]` 默认各为 3），实现复杂度在综合分里**反向计入**（越难分越低）。
- **评分来源**：`[defaults]` 基线 + `[[rules]]` 覆盖（按声明顺序匹配，后面的更具体规则覆盖前面的；可按 `biz_tags` / `expected_file_prefixes` / `methods` / `name_prefixes` 命中）。
- **分层**：`[[priority_tiers]]` 定义 P0/P1/P2/P3 阈值（如 `min_score = 4.4` → P0）。
- **覆盖路径**：脚本默认 `--priority-config tools/api_priority.toml`（见 `tools/validate_apis.py` 参数定义）；如需切换模型可用该参数指向别的 toml。
- **输出条件**：仅当该 crate **存在缺失 API** 时，才在报告里写「缺失 API 优先级清单」段（含综合分、P 级、判定规则）；无缺口则不输出。

## 🚨 常见问题

### 1. CSV 文件不存在

**错误：** `❌ 错误: CSV 文件不存在: api_list_export.csv`

**解决：**
- 确保 `api_list_export.csv` 在项目根目录
- 或使用 `--csv` 参数指定路径

### 2. 源码目录不存在

**错误：** `❌ 错误: 源码目录不存在: crates/xxx/src`

**解决：**
- 检查 crate 名称是否正确（使用 `--list-crates` 查看）
- 或使用 `--src` 参数手动指定路径

### 3. 完成率异常

**现象：** 完成率超过 100% 或有大量"额外文件"

**可能原因：**
- 命名规范不匹配（文件命名与 CSV 定义不一致）
- 存在辅助文件（service.rs、models.rs 等）
- CSV 定义过时

**解决：**
- 检查命名规范：`src/{bizTag}/{project}/{version}/{resource}/{name}.rs`
- 更新 CSV 文件
- 检查是否需要更新 `tools/api_coverage.toml` 映射

### 4. 「额外文件」白名单（预期，非缺陷）

**现象：** 报告「额外的实现文件」段里反复出现一批非 API 文件，完成率看着偏低。

**说明：** 以下文件是 crate 的组织骨架/聚合层/测试，**本就不对应任何 CSV 中的 API**，不计入 API 覆盖，出现属预期：

- `prelude.rs`、`mod.rs` —— 模块聚合 / 导出预导出
- `versions.rs` —— 版本聚合入口
- `models.rs`、`models/` —— Serde 模型集中存放
- `services.rs`、`service.rs` —— Service/Client 注册表
- `tests.rs`、`tests/` —— 测试

只有当「额外文件」里出现**疑似真实 API 落盘但命名不匹配 CSV**（例如拼错 bizTag / version / resource）时，才需要按命名规范排查。

## 📝 命名规范

API 文件路径严格遵循以下规范：

```
src/{bizTag}/{project}/{version}/{resource}/{name}.rs
```

**规则：**
- `meta.resource` 中的 `.` 转换为 `/` 作为子目录
- `meta.name` 中的 `/` 转换为 `/` 作为子目录
- `meta.name` 中的 `:` 替换为 `_`（路径参数）
- 使用 snake_case 命名

**示例：**
| API | 文件路径 |
|-----|----------|
| `bizTag=ccm, project=drive, version=v1, resource=file, name=create` | `src/ccm/drive/v1/file/create.rs` |
| `bizTag=base, project=bitable, version=v1, resource=app.table, name=record/create` | `src/base/bitable/v1/app/table/record/create.rs` |

## 🔗 相关技能

- **添加新 API**：`Skill(openlark-api)`
- **设计审查**：`Skill(openlark-design-review)`
- **校验风格**：`Skill(openlark-validation-style)`

## 📚 工作流集成

### CI/CD 接线（已存在，勿重复造）

仓库**没有** `.github/workflows/api-validation.yml`，也**没有** pre-commit hook 跑覆盖率。真实接线分布在两个 workflow：

- **release.yml**（`.github/workflows/release.yml:70`）—— 打 tag 发版时，`github-release` job 执行 `python3 tools/validate_apis.py --all-crates`，生成全仓覆盖率汇总。
- **pre-release-compatibility.yml**（`.github/workflows/pre-release-compatibility.yml`）—— 当 `tools/validate_apis.py`（以及 release.yml、相关 docs/scripts、`src/lib.rs`）改动时触发；它调用 `scripts/check-pre-release-compatibility.sh`，并上传 `reports/api_validation/{summary.json,summary.md,dashboards/core_business.json,dashboards/core_business.md}` 作为 artifact（`pre-release-compatibility-report`）。

### 日常本地

```bash
just api-coverage   # 等价 python3 tools/validate_apis.py --all-crates
```

> 仓库未配置 pre-commit hook，本地验证靠 `just api-coverage` 或直接跑脚本。

## 🎓 最佳实践

1. **定期验证**：每次添加新 API 后运行验证
2. **保持同步**：确保 CSV 文件与飞书官方文档同步
3. **更新映射**：添加新 crate 时及时更新 `api_coverage.toml`
4. **审查报告**：关注"额外文件"，可能需要更新 CSV 或重构代码
5. **100% 目标**：确保核心 API 实现率达到 100%
