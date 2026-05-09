# Typed API 覆盖率报告说明

本文档定义 `tools/validate_apis.py` 的覆盖率统计口径与缺失 API 优先级模型，用于支持按 crate 的缺口跟踪、里程碑规划与发布节奏管理。

若需要验证已实现 API 的 HTTP endpoint 或 request body 字段是否和飞书官网一致，使用 [`docs/api-contract-validation.md`](api-contract-validation.md) 中的 contract validation 工作流。

## 1. 统计口径

- 数据源：仓库根目录 `api_list_export.csv`。
- 实现源：`tools/api_coverage.toml` 中配置的 crate 源码目录与 bizTag 映射。
- 默认排除 `meta.Version=old`（脚本默认开启 `--skip-old`）。
- 报告默认不写入时间戳，确保同输入下可稳定复现（可通过 `--with-timestamp` 打开时间戳）。
- 覆盖率定义：`已实现 / API 总数 * 100%`。

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
- 企业信息、席位信息、跨组织可见范围
- CoreHR 流程发起、流程模板、时间轴查询
- 安全合规迁移、多地域查询
- 只读查询类接口的低复杂度倾斜

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
