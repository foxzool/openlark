# 验证报告 — remove-unused-deprecated

- Change: remove-unused-deprecated
- Date: 2026-06-28
- 分支: feature/20260628/remove-unused-deprecated
- base-ref: 0156f201975953f58ba37eea9b28e2668770d41c
- verify_mode: full（6 文件）

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | tasks.md 全完成；delta spec `no-unused-deprecated` 需求（修正后）覆盖 |
| Correctness | D+C 满足；spec req1 已按实际修正（G 保留） |
| Coherence | design doc 加 Implementation Divergence 记录 G→D+C scope 收窄 |

## Fresh 验证证据（本阶段重跑）

| 检查 | 命令 | 结果 |
|------|------|------|
| default clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Finished，0 warning |
| all-features clippy | `--all-features` | Finished，0 warning |
| no-default clippy | `--no-default-features` | Finished，0 warning |
| test | `cargo test --workspace` | 0 failed |
| to_value 删除 | grep | 0 |
| 宏 new 删除 | grep | 0 |

## 需求对照（delta spec `no-unused-deprecated`，修正后）

1. **auth legacy 方法保留（functional）** ✓ — build 核实 G 是 functional two-step flow（execute 读取 + test 验证），**保留不删**；req1 已从「移除」修正为「保留」。移除 legacy flow 留 #278。
2. **docs 不暴露 to_value + 宏 new** ✓ — 两者删除；空 impl 移除；`json!` import 改 `#[cfg(test)]`。
3. **构建/examples 不破坏** ✓ — 三组 clippy 0 warning；test 0 failed；examples/tests 不引用已移除项。

## Implementation Divergence（已在 design doc 记录）

proposal/design/spec 原述 G+D+C(5)。**build 核实发现 G（auth app_id/secret/ticket）是 functional legacy two-step flow**（非 unused）→ G 还原保留，实际只做 **D+C(2)**。delta spec req1 已修正为「保留」；design doc 加 Implementation Divergence 节。这是基于 build 实证的 scope 收窄（spec 原基于错误的「G unused」假设），方向不变。

## Coherence 检查

- D（to_value）+ C（宏 new）确实是零调用/dead，删除干净（连带空 impl + json import test-gate）。
- G 还原完整（auth vs base 无净变化，legacy flow + test 完好）。
- code review 通过（Nit 1 宏文档残留已修；Nit 2 风格接受）。

## 问题

- CRITICAL：无
- WARNING：无
- SUGGESTION：无

## 最终评估

**全部检查通过，Ready for archive。** 移除 D+C（2 个零调用/dead deprecated），BREAKING。G 经核实是 functional legacy flow，保留并修正 spec。三组 clippy/test 全绿，review 通过。B/F 留 #278。
