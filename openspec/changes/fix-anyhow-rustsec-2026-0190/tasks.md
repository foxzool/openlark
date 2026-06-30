# Tasks — fix-anyhow-rustsec-2026-0190（hotfix）

> 升级 anyhow 1.0.102→1.0.103 修复 RUSTSEC-2026-0190（security-audit CI 失败）。

## 1. 升级 + 同步

- [x] 1.1 `cargo update -p anyhow`（Cargo.lock 1.0.102→1.0.103）
- [x] 1.2 同步 `.github/msrv/Cargo.lock`（block 感知：移除旧 anyhow 行、加 1.0.103，不动其他版本）
- [x] 1.3 CHANGELOG v0.18 security 段记录 anyhow 升级 + RUSTSEC-2026-0190 修复

## 2. 验证

- [x] 2.1 cargo-deny check 通过（advisory 消失）——若本地无 cargo-deny，靠 CI security-audit
- [x] 2.2 `cargo build --workspace --all-features` exit 0
- [x] 2.3 `cargo test --workspace` 0 failed
- [x] 2.4 `cargo fmt --all -- --check` exit 0（lockfile 改动通常不触发，但跑一遍）
- [x] 2.5 grep 确认两个 lockfile 都 anyhow 1.0.103
