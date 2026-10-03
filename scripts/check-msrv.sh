#!/usr/bin/env bash
set -euo pipefail

# 本地复现 CI 的 MSRV 检查：
# 1) 使用仓库维护的锁定依赖图（避免依赖漂移）
# 2) 使用 Rust 1.88 toolchain 执行全量 feature 检查

cp .github/msrv/Cargo.lock Cargo.lock
cargo +1.88.0 check --workspace --all-features --locked
