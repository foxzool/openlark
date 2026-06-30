## 修复方案

升级 anyhow 1.0.102 → 1.0.103（修复 RUSTSEC-2026-0190 的 patch 版本）。

- `cargo update -p anyhow` → Cargo.lock 更新
- 同步 `.github/msrv/Cargo.lock`（block 感知移除旧 anyhow、加 1.0.103，不动版本——msrv lock pin 的其他版本原样保留）
- deny.toml 不改（`ignore=[]` 保持；advisory 由升级消除，非 ignore 掩盖）

patch 版本升级，无 breaking，无代码逻辑变更。
