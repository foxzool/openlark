# Tasks — ci-clippy-all-targets（#250）

- [x] 1. 改 `.github/workflows/ci.yml` `lint` job 的 **all-features** clippy：`--lib` → `--all-targets`（line 107）。注：no-default-features 行（line 116）**不改**——`--no-default-features --all-targets` 触发 hr/helpdesk/analytics 集成测试未门控的历史遗留编译错误（E0433/missing_docs），见 #251。
- [x] 2. 验证 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0（line 116 维持 `--lib` 不变）
- [x] 3. 提交（commit message: `tweak: ci lint 改用 --all-targets 覆盖 test target (#250)`）；另开 #251 追踪 test feature-gating 清理（line 116 + 矩阵升级的前置）
