# OpenLark 文档索引

面向贡献者与维护者的导航。用户快速上手请先看仓库根 [`README.md`](../README.md)；Agent/开发约定见 [`AGENTS.md`](../AGENTS.md)。

**当前版本：** workspace `0.20.0` · **MSRV：** Rust 1.88 · **许可：** Apache-2.0

**目录 API 数量：** `python3 tools/validate_apis.py --all-crates`（排除 old）于 2026-09-15 测得 **1,640 / 1,640**。按 crate / bizTag 见 [`../crates.md`](../crates.md)；口径说明见 [`typed-api-coverage.md`](typed-api-coverage.md)。[`../ARCHITECTURE.md`](../ARCHITECTURE.md) 是历史叙事，不要用它对数。

## 入门与贡献

| 文档 | 说明 |
|------|------|
| [`../README.md`](../README.md) | 安装、feature 组合、快速示例 |
| [`../CONTRIBUTING.md`](../CONTRIBUTING.md) | 贡献流程与规范 |
| [`../AGENTS.md`](../AGENTS.md) | 仓库结构、命令、Agent skills |
| [`../CONTEXT.md`](../CONTEXT.md) | API 合同领域用语（证据/目录条目等） |
| [`../TESTING.md`](../TESTING.md) | 测试策略总览 |
| [`../SECURITY.md`](../SECURITY.md) | 安全披露 |
| [`../examples/README.md`](../examples/README.md) | 可编译公开示例清单 |
| [`../crates.md`](../crates.md) | crate ↔ bizTag 对照 |

## 公开 API 与兼容性

| 文档 | 说明 |
|------|------|
| [`PUBLIC_REEXPORT_POLICY.md`](PUBLIC_REEXPORT_POLICY.md) | 根 crate / client / 领域 crate 入口规则 |
| [`PUBLIC_API_STABILITY_POLICY.md`](PUBLIC_API_STABILITY_POLICY.md) | 公开 API 稳定性 |
| [`DEPRECATED_API_SUPPORT_POLICY.md`](DEPRECATED_API_SUPPORT_POLICY.md) | 弃用 API 支持策略 |
| [`HELPER_SEMVER_RULES.md`](HELPER_SEMVER_RULES.md) / [`TYPED_API_SEMVER_RULES.md`](TYPED_API_SEMVER_RULES.md) | Helper / 强类型 API 的 semver |
| [`migration-guide.md`](migration-guide.md) | 跨版本迁移 |
| [`changelog-compatibility-categories.md`](changelog-compatibility-categories.md) | CHANGELOG 兼容性分类 |
| [`CLIENT_NAMING_CONVENTION.md`](CLIENT_NAMING_CONVENTION.md) | Client 命名 |
| [`FEATURE_NAMING_CONVENTION.md`](FEATURE_NAMING_CONVENTION.md) | Feature 命名 |
| [`FEATURE_MATRIX.md`](FEATURE_MATRIX.md) | Feature 组合回归矩阵 |

## API 实现与校验

| 文档 | 说明 |
|------|------|
| [`API_DESIGN_SPECIFICATION.md`](API_DESIGN_SPECIFICATION.md) | API 设计规范 |
| [`api-implementation-template.md`](api-implementation-template.md) | 实现模板 |
| [`api-consistency-review.md`](api-consistency-review.md) | 与官网一致性复查方法论 |
| [`api-contract-validation.md`](api-contract-validation.md) | 合同校验流程 |
| [`typed-api-coverage.md`](typed-api-coverage.md) | 强类型覆盖率口径 |
| [`typed-coverage-release-criteria.md`](typed-coverage-release-criteria.md) | 覆盖率发布门槛 |
| [`typed-coverage-priorities/`](typed-coverage-priorities/) | 分域优先级 |

> 机器报告默认写到 gitignored 的 `reports/`（如 `reports/api_validation/`），不入库。

## 架构决策（ADR）

见 [`adr/`](adr/)。Agent 配置（issue tracker / triage / domain）见 [`agents/`](agents/)。

## 测试与质量

| 文档 | 说明 |
|------|------|
| [`TEST_ARCHITECTURE_SUMMARY.md`](TEST_ARCHITECTURE_SUMMARY.md) | 测试架构 |
| [`API_E2E_TEST_STRATEGY.md`](API_E2E_TEST_STRATEGY.md) | E2E 策略 |
| [`hr-testing-guide.md`](hr-testing-guide.md) | HR 测试 |
| [`CI_TEST_TARGET_COVERAGE.md`](CI_TEST_TARGET_COVERAGE.md) | CI 目标覆盖 |
| [`error-context-policy.md`](error-context-policy.md) | 错误上下文策略 |

## 发布与历史归档

当前发布窗口说明：[`../RELEASE_NOTES.md`](../RELEASE_NOTES.md)。

下列 cut/signoff/checklist 为**已发布版本的历史记录**（保留在本目录以便 CI 路径与既有链接稳定；新读者可跳过）：

- [`0.20.0_RELEASE_CUT.md`](0.20.0_RELEASE_CUT.md) / [`0.20.0_RELEASE_SIGNOFF.md`](0.20.0_RELEASE_SIGNOFF.md)
- [`0.19.0_RELEASE_CUT.md`](0.19.0_RELEASE_CUT.md) / [`0.19.0_RELEASE_SIGNOFF.md`](0.19.0_RELEASE_SIGNOFF.md)
- [`0.15.0_RELEASE_CHECKLIST.md`](0.15.0_RELEASE_CHECKLIST.md)

兼容性发布清单与模板：[`api-compatibility-release-checklist.md`](api-compatibility-release-checklist.md)、[`pre-release-compatibility-workflow.md`](pre-release-compatibility-workflow.md)、[`api-compatibility-note-template.md`](api-compatibility-note-template.md)。

## Agent / 一次性计划（内部）

[`superpowers/`](superpowers/) 存放历史 design/plan 记录，优先看已落地的规范文档（上表），而不是计划稿本身。

仓库内 Agent skills：`.agents/skills/`（API 实现/字段核对/覆盖率等）、`.cursor/skills/verify-openlark/`（验证流程）。
