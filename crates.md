# Feature Crate → bizTag 对照表

本项目的 API 代码按 **bizTag** 组织（对应 `api_list_export.csv` 的 `bizTag` 列，且决定 API 文件路径第一层目录）。

为了让工具链能从「crate/feature」快速定位到对应的 bizTag（例如 API 覆盖率统计、`tools/validate_apis.py --crate` 自动补齐 `--src/--filter`），仓库维护了一份 **单一真相映射表**：

- `tools/api_coverage.toml`

## crate（feature-crate）→ bizTag

> 下面表格与 `tools/api_coverage.toml` 保持一致，新增/调整映射请优先修改该 TOML。

| crate（workspace package） | 负责的 bizTag（CSV 原值） |
|---|---|
| `openlark-docs` | `ccm`, `base`, `baike`, `minutes` |
| `openlark-meeting` | `calendar`, `vc`, `meeting_room` |
| `openlark-communication` | `im`, `contact`, `moments`, `aily`, `event` |
| `openlark-cardkit` | `cardkit` |
| `openlark-hr` | `hire`, `feishu_people`, `attendance`, `compensation_management`, `performance`, `payroll`, `okr`, `ehr` |
| `openlark-security` | `acs`, `security_and_compliance` |
| `openlark-ai` | `ai` |
| `openlark-auth` | `auth`, `passport`, `verification_information`, `human_authentication` |
| `openlark-platform` | `admin`, `directory`, `app_engine`, `trust_party`, `tenant`, `mdm`, `spark` |
| `openlark-workflow` | `task`, `approval`, `board` |
| `openlark-analytics` | `search`, `report` |
| `openlark-user` | `personal_settings` |
| `openlark-mail` | `mail` |
| `openlark-helpdesk` | `helpdesk` |
| `openlark-application` | `application`, `workplace` |

## bizTag API 数量（排除 meta.Version=old）

> 数据来源：`api_list_export.csv`。统计口径：按 `bizTag` 分组计数，仅统计 `meta.Version != old` 的行（old 版本不计入“有效 API 数”）。

| bizTag | API 数量（排除 old） | API 总数 | old 数量 |
|---|---:|---:|---:|
| `feishu_people` | 262 | 262 | 0 |
| `hire` | 182 | 182 | 0 |
| `ccm` | 122 | 171 | 49 |
| `task` | 75 | 75 | 0 |
| `contact` | 71 | 78 | 7 |
| `im` | 71 | 75 | 4 |
| `mail` | 67 | 67 | 0 |
| `vc` | 56 | 56 | 0 |
| `base` | 52 | 52 | 0 |
| `helpdesk` | 50 | 50 | 0 |
| `app_engine` | 48 | 48 | 0 |
| `calendar` | 44 | 44 | 0 |
| `attendance` | 39 | 39 | 0 |
| `approval` | 31 | 54 | 23 |
| `application` | 27 | 32 | 5 |
| `baike` | 27 | 27 | 0 |
| `ai` | 23 | 23 | 0 |
| `aily` | 21 | 21 | 0 |
| `compensation_management` | 21 | 21 | 0 |
| `directory` | 21 | 21 | 0 |
| `performance` | 21 | 21 | 0 |
| `search` | 15 | 15 | 0 |
| `acs` | 14 | 14 | 0 |
| `admin` | 14 | 14 | 0 |
| `security_and_compliance` | 13 | 13 | 0 |
| `okr` | 12 | 12 | 0 |
| `payroll` | 12 | 12 | 0 |
| `auth` | 10 | 11 | 1 |
| `cardkit` | 10 | 10 | 0 |
| `board` | 6 | 6 | 0 |
| `personal_settings` | 6 | 6 | 0 |
| `trust_party` | 5 | 5 | 0 |
| `mdm` | 4 | 4 | 0 |
| `minutes` | 4 | 4 | 0 |
| `report` | 3 | 3 | 0 |
| `workplace` | 3 | 3 | 0 |
| `ehr` | 2 | 2 | 0 |
| `passport` | 2 | 2 | 0 |
| `tenant` | 2 | 2 | 0 |
| `event` | 1 | 1 | 0 |
| `human_authentication` | 1 | 4 | 3 |
| `moments` | 1 | 1 | 0 |
| `spark` | 1 | 1 | 0 |
| `verification_information` | 1 | 1 | 0 |
| `meeting_room` | 0 | 17 | 17 |
| **合计** | 1473 | 1582 | 109 |

## 工具用法

- 列出所有 crate 与 bizTag：`python3 tools/validate_apis.py --list-crates`
- 以 crate 为入口验证实现：`python3 tools/validate_apis.py --crate openlark-docs`

## open-lark（根 crate）feature → crate → bizTag

当你通过 `open-lark` 这个统一包启用 feature 时，可以按下表理解它最终覆盖的 bizTag 范围：

| `open-lark` feature | 依赖的 workspace crate | 对应 bizTag |
|---|---|---|
| `auth` | `openlark-auth` | `auth`, `passport`, `verification_information`, `human_authentication` |
| `communication` | `openlark-communication` | `im`, `contact`, `moments`, `aily`, `event` |
| `cardkit` | `openlark-cardkit` | `cardkit` |
| `docs` / `base` / `bitable` | `openlark-docs` | `ccm`, `base`, `baike`, `minutes` |
| `hr` | `openlark-hr` | `hire`, `feishu_people`, `attendance`, `compensation_management`, `performance`, `payroll`, `okr`, `ehr` |
| `security` | `openlark-security` | `acs`, `security_and_compliance` |
| `meeting` | `openlark-meeting` | `calendar`, `vc`, `meeting_room` |
| `workflow` | `openlark-workflow` | `task`, `approval`, `board` |
| `platform` | `openlark-platform` | `admin`, `directory`, `app_engine`, `trust_party`, `tenant`, `mdm`, `spark` |
| `analytics` | `openlark-analytics` | `search`, `report` |
| `user` | `openlark-user` | `personal_settings` |
| `helpdesk` | `openlark-helpdesk` | `helpdesk` |
| `mail` | `openlark-mail` | `mail` |
| `application` | `openlark-application` | `application`, `workplace` |
