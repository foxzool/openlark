## Why

CI 的 lint 门禁（`.github/workflows/ci.yml` `lint` job）所有 clippy 调用都用 `--lib`，不编译 test target，导致 `#[cfg(test)]` 模块的 lint 回归（如 #248：11 处未用 `use super::*`）能溜过 CI、只在本地 `just lint`（`--all-targets`）暴露。需要让 CI 覆盖 test target，从源头拦住此类回归。

## What Changes

- `.github/workflows/ci.yml` `lint` job 的两处 clippy 由 `--lib` 改为 `--all-targets`：
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`

## Capabilities

### New Capabilities
<!-- 无：纯 CI 门禁调整，不引入产品 capability -->

### Modified Capabilities
<!-- 无：不改变任何已有 spec 的验收行为 -->

## Impact

- `.github/workflows/ci.yml`：`lint` job 两行命令调整（`--lib` → `--all-targets`）
- 无源码、无 API、无依赖变化
- 前置已验证：main 上 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0（#249 修绿后），主修项无阻塞可立即落地
- `feature-combinations` 矩阵的 clippy 仍用 `--lib`，改 `--all-targets` 需逐组合先验绿，留作后续（不在本 change 范围）
