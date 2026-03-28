# OpenLark API 覆盖率报告

本文档汇总 `0.15.0-rc.2` 阶段的 API 覆盖率口径与统计结果，作为发布说明与实现状态的配套材料。

## 统计口径

- 数据源：`api_list_export.csv`
- 映射来源：`crates.md` 与 `tools/api_coverage.toml`
- 统计规则：按 `bizTag` 分组，仅统计 `meta.Version != old` 的 API 作为“有效 API”

## 总览

根据当前仓库统计：

- **有效 API 总数**：1456
- **API 总数**：1572
- **old 版本 API 数**：116

这意味着当前仓库的实现与验证工作主要围绕“非 old 版本 API”展开，RC.2 发布判断也以这部分作为主要覆盖面。

## crate 维度覆盖范围

| crate | 对应 bizTag |
|---|---|
| `openlark-docs` | `ccm`, `base`, `baike`, `minutes` |
| `openlark-meeting` | `calendar`, `vc`, `meeting_room` |
| `openlark-communication` | `im`, `contact`, `moments`, `aily`, `event` |
| `openlark-cardkit` | `cardkit` |
| `openlark-hr` | `hire`, `feishu_people`, `attendance`, `compensation_management`, `performance`, `payroll`, `okr`, `ehr` |
| `openlark-security` | `acs`, `security_and_compliance` |
| `openlark-ai` | `ai` |
| `openlark-auth` | `auth`, `passport` |
| `openlark-platform` | `admin`, `directory`, `app_engine` |
| `openlark-workflow` | `task`, `approval`, `board` |
| `openlark-analytics` | `search`, `report` |
| `openlark-user` | `personal_settings` |
| `openlark-mail` | `mail` |
| `openlark-helpdesk` | `helpdesk` |
| `openlark-application` | `application` |

## 重点 bizTag 统计

| bizTag | 有效 API | API 总数 | old 数量 |
|---|---:|---:|---:|
| `feishu_people` | 257 | 257 | 0 |
| `hire` | 182 | 182 | 0 |
| `ccm` | 122 | 174 | 52 |
| `task` | 75 | 75 | 0 |
| `im` | 71 | 75 | 4 |
| `contact` | 70 | 77 | 7 |
| `mail` | 67 | 67 | 0 |
| `vc` | 56 | 56 | 0 |
| `helpdesk` | 50 | 50 | 0 |
| `base` | 49 | 49 | 0 |
| `app_engine` | 48 | 48 | 0 |
| `calendar` | 44 | 44 | 0 |

完整明细请参考仓库根目录 `crates.md`。

## 与发布相关的解释

- RC.2 的重点不是继续扩张 API 数量，而是让根 crate 入口、feature 模型、文档与发布链路先稳定下来。
- API 覆盖率材料在本阶段主要用于说明“当前覆盖到哪些业务域”，而不是承诺所有历史 `old` 接口继续作为正式覆盖目标。
- 正式版发布前，可以继续基于 `python3 tools/validate_apis.py --crate <crate-name>` 输出更细粒度的 crate 级报告。

## 复核命令

```bash
python3 tools/validate_apis.py --list-crates
python3 tools/validate_apis.py --crate openlark-docs
python3 tools/validate_apis.py --crate openlark-communication
python3 tools/validate_apis.py --crate openlark-hr
```
