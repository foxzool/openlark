# 验证报告 — ci-clippy-all-targets（#250）

- 日期: 2026-06-25
- 变更: ci-clippy-all-targets（tweak preset）
- 范围: `.github/workflows/ci.yml` lint job all-features clippy 由 `--lib` 改 `--all-targets`（1 行）
- 分支: ci/clippy-all-targets-250，commit 8ebf7b3b1，base_ref 1b39f383
- verify_mode: light（实际实现仅 1 文件；scale 误报 full 系 OpenSpec 规划工件被计入文件数，已按覆盖机制改回 light）

## 6 项轻量验证（全 PASS）

| # | 检查 | 证据 | 结果 |
|---|------|------|------|
| 1 | tasks.md 全部完成 | unchecked count = 0 | PASS |
| 2 | 改动文件与 tasks 一致 | ci.yml +1/−1 | PASS |
| 3 | fmt | `cargo fmt --all --check` exit 0 | PASS |
| 4 | 新 CI 命令（含 test target） | `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0 | PASS |
| 5 | 构建 | `cargo build --workspace --all-features` exit 0 | PASS |
| 6 | 安全 | diff 无 secrets / 无新增 unsafe | PASS |

代码审查：`review_mode=off`，跳过自动 code review（CI 配置 1 行 flag 调整，无逻辑/边界风险）。

## 范围调整记录（build 阶段）

原计划改 2 行（line 107 + line 116）。验证发现 `--no-default-features --all-targets` 触发 hr/helpdesk/analytics 集成测试未按 feature 门控的历史遗留编译错误（E0433/missing_docs），属预存问题、与本改动无关、被 CI 的 `--lib` 长期掩盖。用户确认：仅改 line 107（安全、达成 #250 目标），line 116 + 矩阵升级作为 #251 后续。

## 结论

**PASS** — 无 CRITICAL/IMPORTANT 问题。#250 目标达成：CI 在 all-features 维度覆盖 test target，可抓 #248 类（`#[cfg(test)]` 未用导入）回归。
