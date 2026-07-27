# Typed API 覆盖率发布准入标准

本文档定义 OpenLark 在**新正式版发布**前如何使用 typed API 覆盖率报告做发布结论判断。

对应的机器可读策略文件位于：

- `tools/typed_coverage_release.toml`

## 1. 输入物

发布前至少需要重新生成以下报告：

```bash
python3 tools/validate_apis.py --all-crates
```

生成后，发布核查应以这两份报告为准：

- `reports/api_validation/summary.json`
- `reports/api_validation/dashboards/core_business.json`

其中：

- `summary.json` 负责全仓总体完成度判断
- `core_business.json` 负责核心业务 crate 的集中视图

## 2. 结论状态

发布前对 typed coverage 的结论只允许落在以下三种状态之一：

### PASS

所有 hard gate 满足，且没有触发 waiver gate。

### WAIVER REQUIRED

hard gate 全部满足，但触发了需要书面例外的条件。

典型例子：

- 核心业务 dashboard 仍存在 `P0` 缺口，但其范围、影响和补偿方案已被明确记录。

### BLOCKED

任一 hard gate 不满足，或触发 waiver gate 但没有有效 waiver 记录。

## 3. Hard Gate

当前稳定版 hard gate 定义见 `tools/typed_coverage_release.toml`：

- 全仓 `summary.completion_rate >= 93.0%`
- 核心业务 dashboard `completion_rate >= 92.0%`
- 任一核心业务 crate `completion_rate >= 80.0%`

解释：

- 第一条保证整体 typed 覆盖率不能退化到不可控状态。
- 第二条保证主业务域的覆盖率不能被尾部 crate 的高完成度“稀释”。
- 第三条防止单个核心业务 crate 明显掉队。

## 4. Waiver Gate

以下条件不会直接放行发布，但可以进入 waiver 流程：

- `core_business` dashboard 中仍存在 `P0` 缺口

换言之：

- **有 P0 缺口 ≠ 自动允许发布**
- **有 P0 缺口 + 有效 waiver = 可进入例外放行讨论**

## 5. Waiver 规则

waiver 至少需要满足：

- 审批人：
  - `maintainer`
  - 对应业务域 `domain-owner`
- 必填字段：
  - `reason`
  - `affected_crates`
  - `affected_apis`
  - `compensating_controls`
  - `owner`
  - `target_release`
- 有效期：
  - 最多跨 **1 个发布周期**

允许的 waiver reason 目前限定为：

- `upstream-contract-gap`
- `dependency-blocker`
- `release-window-constraint`
- `low-risk-tail-gap`

## 6. 建议执行顺序

1. 运行 `python3 tools/validate_apis.py --all-crates`
2. 阅读 `summary.md` 与 `dashboards/core_business.md`
3. 依据 `tools/typed_coverage_release.toml` 判断当前状态属于 `PASS / WAIVER REQUIRED / BLOCKED`
4. 若需要 waiver，补齐记录并附到 release checklist / release notes 审核上下文
5. 只有在 hard gate 满足且 waiver 有效时，才允许继续发布流程

## 7. 非目标

本文档和 `tools/typed_coverage_release.toml` 当前只定义**准入规则**，不直接改动：

- GitHub Actions release workflow
- `just release`
- 自动阻断脚本

这些自动化接入可以在后续发布流程 issue 中完成，但应直接复用这里的输入与门槛。

### 7.1 阈值纪律（#586 institutionalization）

Hard gate 阈值是 0.20 trust 程序的一部分：**不得下调**（must not lower）阈值来
换取 CI / 发布 PASS。具体：

- `summary_completion_rate_min` / `core_business_completion_rate_min` /
  `core_crate_completion_rate_min` 的制度化地板由
  `tools/tests/test_typed_coverage_release_policy.py` 钉死；
- 任何阈值变更必须是独立 policy PR，且只允许升高或维持，禁止降低；
- 与 contract 侧「禁止 monorepo-wide field strict、一域一 PR」规则一起，保证
  “CI green” 继续表示可调用准确性，而不是门槛被游戏。

分类纪律（path_noise vs true_gap）同样不得为了刷完成度而删除：噪音匹配必须
保留 evidence，真缺口继续计入 `true_missing`。详见
`docs/typed-api-coverage.md` 与 `docs/api-contract-validation.md` §1.6。
