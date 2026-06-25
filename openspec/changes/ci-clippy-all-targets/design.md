# Design — ci-clippy-all-targets（#250）

## 背景

CI lint job 用 `--lib` 只检查 lib target，跳过 test/bench。本地 `just lint` 用 `--all-targets` 才覆盖 test 模块。两者差异导致 test-target lint 回归不被 CI 拦截（#248 即此：CI 全绿、本地红）。

## 方案

把 `lint` job 的 **all-features** clippy 的 `--lib` 换成 `--all-targets`，使 CI（all-features 维度）与本地 `just lint` 的 target 覆盖对齐。改动 1 行。

| 位置 | 现 | 改 |
|------|----|----|
| all-features（line 107） | `cargo clippy --workspace --lib --all-features -- -D warnings` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| no-default-features（line 116） | `cargo clippy --workspace --lib --no-default-features -- -D warnings` | **不改**（维持 `--lib`） |

## 范围调整（build 阶段发现）

原计划两行都改。build 验证发现 `--no-default-features --all-targets` 失败（exit 101）：hr/helpdesk/analytics 的 `tests/` 集成测试引用 feature-gated 模块（如 `compensation_management`，`openlark-hr/src/lib.rs:45` 为 `#[cfg(feature = "compensation")]`）但自身未门控 → E0433 / missing_docs。属历史遗留、与本改动无关、被 CI 的 `--lib` 长期掩盖。

决策（用户确认）：**仅改 line 107**（all-features `--all-targets` 实测 exit 0，达成 #250 目标、能抓 #248 类回归）；line 116 + 矩阵的 `--all-targets` 升级作为 **#251** 的后续（修好测试门控后再做）。

## 风险与边界

- **编译时间**：`--all-targets` 额外编译 test/bench target，lint job 耗时上升（CI 60min timeout，可接受）。
- **既有 test-target 问题**：#249 已清理 `--all-features` 下 11 处；line 107 改后 CI 在 all-features 维度覆盖 test target（实测绿）。no-default-features 维度的 test-target 覆盖由 #251 解锁。
- **范围外**：line 116 + feature-combinations 矩阵 clippy 的 `--all-targets` 升级 = #251 后续。
