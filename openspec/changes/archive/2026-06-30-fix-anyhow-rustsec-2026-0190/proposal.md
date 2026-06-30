## Why

CI `security-audit` job（cargo-deny）失败：**RUSTSEC-2026-0190**——`anyhow 1.0.102` 的 `Error::downcast_mut()` 在 `Error::context` 后调用时违反借用规则（UB）。anyhow 是 workspace 直接依赖（9 个业务 crate + core 用）。deny.toml `ignore=[]`（不忽略 advisory），故 security-audit 红。影响所有当前 PR（仓库级）。

## 根因

anyhow 1.0.102 有未修复的 unsoundness advisory；workspace lockfile 锁定 1.0.102。

## 修复目标

升级 anyhow 到修复版本 **1.0.103**（`cargo update -p anyhow` 确认可达；workspace 约束 `anyhow="1.0.86"` 已允许）。同步 `.github/msrv/Cargo.lock`（CI msrv job 用 pinned lockfile）。验证 cargo-deny 通过（advisory 消失）。

## What Changes

- `Cargo.lock`：anyhow 1.0.102 → 1.0.103
- `.github/msrv/Cargo.lock`：同步 anyhow → 1.0.103（msrv pinned lockfile，dep 变更必须同步）
- 无 Cargo.toml 版本约束变更（`anyhow="1.0.86"` 已覆盖 1.0.103）
- 无代码逻辑变更（纯 patch 版本升级）

## Impact

- 安全：消除 RUSTSEC-2026-0190，security-audit CI 恢复绿。
- 兼容：anyhow 1.0.102→1.0.103 是 patch 版本（semver 兼容），无 breaking。
- 验证：cargo-deny check 通过 + build/clippy/test/fmt。
