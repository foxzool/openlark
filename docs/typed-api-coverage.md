# Typed API 覆盖率报告说明

本文档定义 `tools/validate_apis.py` 的覆盖率统计口径与缺失 API 优先级模型，用于支持按 crate 的缺口跟踪、里程碑规划与发布节奏管理。

若需要验证已实现 API 的 HTTP endpoint 或 request body 字段是否和飞书官网一致，使用 [`docs/api-contract-validation.md`](api-contract-validation.md) 中的 contract validation 工作流。

## 1. 统计口径

- 数据源：仓库根目录 `api_list_export.csv`。
- 实现源：`tools/api_coverage.toml` 中配置的 crate 源码目录与 bizTag 映射。
- 默认排除 `meta.Version=old`（脚本默认开启 `--skip-old`）。
- 报告默认不写入时间戳，确保同输入下可稳定复现（可通过 `--with-timestamp` 打开时间戳）。
- 覆盖率定义：`已实现 / API 总数 * 100%`（含 layout denoise 后的路径噪音匹配）。

### 1.1 路径公式与 layout 候选

Canonical（nested）公式：

```text
src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource}/{meta.Name}.rs
```

其中 `meta.Resource` 的 `.`、`meta.Name` 的 `:`/`#` 会按脚本规则规范化。

在 strict 路径之外，比较阶段还会尝试下列 **layout 候选**（命中即计为已实现，并写入 evidence）：

| match_kind | 含义 | 典型 crate |
|------------|------|------------|
| `strict` | 与 canonical 公式完全一致 | docs / communication / hr 多数 |
| `flat_project` | `biz/biz/version/...` → 磁盘 `biz/version/...`（省略重复 project） | platform（admin/directory/tenant/…） |
| `rust_keyword` | 目录段为 Rust 关键字时用 `{kw}_mod`（如 `enum` → `enum_mod`） | platform app_engine workspace |
| `rewrite` / `alias` | `tools/api_coverage.toml` 显式登记的 legacy 映射 | workflow / security |
| `typo_correction` | 已知 CSV 拼写错误修正（如 `collboration` → `collaboration`） | platform directory |

候选按 strict → 配置 alias/rewrite → 在已有候选上展开 flat/keyword/typo 的顺序生成；**先命中者优先**，因此 nested 与 flat 同时存在时计为 `strict`。

### 1.2 分类字段（人读 + 机器可读）

crate 报告与 `summary.json` 将结果拆成四类，避免把 layout 噪音当成「实现 API」工单：

| 分类 | JSON 字段 | 是否计入「已实现」 |
|------|-----------|-------------------|
| strict 匹配 | `classification.strict_matched` | 是 |
| 路径噪音匹配 | `classification.path_noise_matched` + `path_noise_matches[]` | 是（必须带 `expected_file` / `implementation_file` / `match_kind` / `match_reason`） |
| 真缺口 | `classification.true_missing` + `true_missing_apis[]` / `prioritized_missing_apis[]` | 否 |
| 额外实现文件 | `classification.extra_files` + `extra_file_list[]` | 否（不在 CSV） |

约定：

- `missing` **仅等于** 真缺口数量，不再混入 path noise。
- 路径噪音重分类必须可追溯 evidence；禁止静默丢弃真缺口。
- 发布 hard gate 阈值不因 denoise 下调；denoise 只修正路径匹配真实性。

## 2. 缺失 API 优先级模型

缺失 API 不再只按 raw count 推进，而是同时参考以下三个维度：

- `business_value`：对企业级集成闭环的业务价值，1-5 分。
- `usage_frequency`：在高频使用场景中的出现概率，1-5 分。
- `implementation_effort`：实现复杂度，1-5 分；分数越高表示越难。

综合分公式如下：

```text
business_value × 0.50
+ usage_frequency × 0.30
+ (6 - implementation_effort) × 0.20
```

说明：

- 实现复杂度在综合分中是反向计入，意味着“价值高且更容易落地”的缺口会更靠前。
- 优先级规则定义在 `tools/api_priority.toml`。
- 规则按声明顺序匹配，后面的更具体规则可以覆盖前面的通用规则。

### 2.1 当前优先级关注点

`tools/api_priority.toml` 当前已覆盖以下高价值缺口类型：

- Task v2 的任务、清单、评论、自定义字段、分组和附件能力
- 联系人基础查询类入口
- 企业信息、席位信息、跨组织可见范围（见 §2.2：0.20 已 clear-or-disprove，不再是真缺口）
- CoreHR 流程发起、流程模板、时间轴查询
- 安全合规迁移、多地域查询
- 只读查询类接口的低复杂度倾斜

### 2.2 0.20 P1 clear-or-disprove（#570）

0.19 签核时 workspace 真缺口优先级分布含 **P1=7**，全部落在 `openlark-platform` 的 tenant / trust_party / directory 跨组织只读接口。#567 完成路径 denoise 后，这七项均已在磁盘找到 typed 实现，终端结论一律为 **path noise**（非真缺口、不延期）：

| # | API | 预期文件（nested / CSV） | 实际实现文件 | match_kind | 终端结论 |
|---|-----|--------------------------|--------------|------------|----------|
| 1 | 获取关联组织双方共享成员范围 | `directory/directory/v1/collboration_share_entity/list.rs` | `directory/v1/collaboration_share_entity/list.rs` | `typo_correction` | noise |
| 2 | 获取企业席位信息接口 | `tenant/tenant/v2/tenant/product_assign_info/query.rs` | `tenant/v2/tenant/product_assign_info/query.rs` | `flat_project` | noise |
| 3 | 获取企业信息 | `tenant/tenant/v2/tenant/query.rs` | `tenant/v2/tenant/query.rs` | `flat_project` | noise |
| 4 | 获取关联组织部门详情 | `trust_party/trust_party/v1/collaboration_tenant/collaboration_department/get.rs` | `trust_party/v1/collaboration_tenant/collaboration_department/get.rs` | `flat_project` | noise |
| 5 | 获取关联组织成员详情 | `trust_party/trust_party/v1/collaboration_tenant/collaboration_user/get.rs` | `trust_party/v1/collaboration_tenant/collaboration_user/get.rs` | `flat_project` | noise |
| 6 | 获取关联组织详情 | `trust_party/trust_party/v1/collaboration_tenant/get.rs` | `trust_party/v1/collaboration_tenant/get.rs` | `flat_project` | noise |
| 7 | 获取可见关联组织的列表 | `trust_party/trust_party/v1/collaboration_tenant/list.rs` | `trust_party/v1/collaboration_tenant/list.rs` | `flat_project` | noise |

说明：

- **实现形态**：均为 `*RequestBuilder` + `async fn execute` + `Transport::request_typed` 的可调用 typed API。`tenant` / `trust_party` 按 ADR-0001 **flat-by-design** 直路径访问（`crate::tenant::v2::*` / `crate::trust_party::v1::*`），`PlatformService` 故意不暴露 shell accessor；directory 侧经 `PlatformService::directory().v1().collaboration_share_entity()` 链可达。
- **证据复现**：`python3 tools/validate_apis.py --crate openlark-platform` 后，platform `true_missing=0`、`priority_counts` 无 P1。
- **硬门禁**：未下调 `tools/typed_coverage_release.toml` 阈值；core-business P0 仍为 0。

### 2.3 0.20 selective P2 slice（#571）

0.19 签核时 workspace 真缺口含 **P2=89** 量级。#567 denoise 后，helpdesk / mail 尾部与 hr OKR v2 的「未实现」行被证明是 **path noise**（磁盘已有可调用 typed leaf），而非业务缺口。#571 只清这一小片（28 行），**不**以「清光全部 P2」为目标，也不下调 hard gate。

| 范围 | 行数 | 机制 | 配置 |
|------|-----:|------|------|
| helpdesk FAQ 图像 | 1 | `alias`：`faq/faq_image.rs` → `faq/image.rs` | `[crates.openlark-helpdesk.implementation_path_aliases]` |
| mail 撤回进度 / 撤回 | 2 | `alias`：`sent_message/*` → `message/recall/*` | `[crates.openlark-mail.implementation_path_aliases]` |
| hr OKR v2 全套 | 25 | `rewrite`：`okr/okr/v2/okr/` → `okr/okr/v2/`（CSV resource `okr.*` 相对 project 冗余） | `[crates.openlark-hr] implementation_path_rewrites` |

终端结论（复现命令）：

```bash
python3 tools/validate_apis.py --crate openlark-helpdesk  # true_missing=0, path_noise=1
python3 tools/validate_apis.py --crate openlark-mail      # true_missing=0, path_noise=2
python3 tools/validate_apis.py --crate openlark-hr        # true_missing=0, path_noise=25（OKR v2）
```

- **选型清单**：写在 issue #571 评论（coding 前），本表为仓库内 SSOT 摘要。
- **硬门禁**：未改 `tools/typed_coverage_release.toml` 阈值；core-business P0 仍为 0。
- **非目标**：platform/admin/acs 等其余 P2 尾、OKR 新功能实现（本片仅 reclassify 已存在 leaf）。

## 3. 报告产物

执行批量模式后，会生成以下文件：

- `reports/api_validation/summary.md`：面向人读的汇总看板（总览 + 各 crate 指标 + 高价值缺失 API backlog）。
- `reports/api_validation/summary.json`：机器可读汇总（便于 CI/看板系统消费）。
- `reports/api_validation/dashboards/<group>.md`：按 crate 分组的专题 dashboard（例如 `core_business`）。
- `reports/api_validation/dashboards/<group>.json`：专题 dashboard 的机器可读输出。
- `reports/api_validation/crates/<crate>.md`：每个 crate 的详细报告（含缺失 API 排序清单和按模块展开详情）。

每个 crate 的报告至少包含：

- API 总数
- 已实现数量
- 未实现数量
- 完成率
- 缺失 API 的优先级表（含维度分数、综合分、判定规则）

专题 dashboard 至少包含：

- 分组内 crate 的集中状态视图
- 每个 crate 的重点缺口
- 分组级的优先级分布与重点 backlog

## 4. 使用方式

### 4.1 生成全量覆盖率与优先级 backlog（推荐）

```bash
just api-coverage
```

等价命令：

```bash
python3 tools/validate_apis.py --all-crates
```

### 4.2 单 crate 验证

```bash
python3 tools/validate_apis.py --crate openlark-workflow
```

### 4.3 包含 old 版本 API

```bash
python3 tools/validate_apis.py --all-crates --include-old
```

### 4.4 指定自定义优先级模型

```bash
python3 tools/validate_apis.py \
  --all-crates \
  --priority-config tools/api_priority.toml
```

### 4.5 生成核心业务 crate dashboard

批量模式会自动读取 `tools/api_coverage.toml` 中的 `dashboard_groups` 元数据。

当前默认的 `core_business` 分组对齐 `Cargo.toml` 中的 `essential + enterprise`
业务 crate（排除基础设施性质的 `auth`），并输出：

- `reports/api_validation/dashboards/core_business.md`
- `reports/api_validation/dashboards/core_business.json`

## 5. 规划建议

- 以 `summary.md` 中的 `高价值缺失 API Backlog` 作为季度实现清单入口，而不是只看 `未实现` 总数。
- 以 `dashboards/core_business.md` 作为核心业务域的周度/发布前复盘入口。
- 先处理 `P0/P1` 缺口，再回到尾部模块做补齐。
- 当某个业务域出现大批量缺口时，优先补其只读查询与主闭环写操作，避免只做边角接口。
- 每次调整优先级规则后重新生成报告，保证计划依据与仓库现状同步。

## 6. 发布准入

typed coverage 的稳定版发布准入规则定义在：

- `docs/typed-coverage-release-criteria.md`
- `tools/typed_coverage_release.toml`

推荐流程：

1. 运行 `python3 tools/validate_apis.py --all-crates`
2. 阅读 `summary.md` 与 `dashboards/core_business.md`
3. 按 `tools/typed_coverage_release.toml` 判断 `PASS / WAIVER REQUIRED / BLOCKED`
4. 若需要 waiver，在发布 checklist 中补齐审批记录
