## 1. workspace.dependencies 声明 bytes

- [x] 1.1 根 `Cargo.toml` 的 `[workspace.dependencies]` 段新增 `bytes = "1.6"`（prost 已存在，不动）

## 2. openlark-protocol 改消费 workspace

- [x] 2.1 `crates/openlark-protocol/Cargo.toml`：`bytes = "1.6.0"` → `bytes = { workspace = true }`
- [x] 2.2 `crates/openlark-protocol/Cargo.toml`：`prost = "0.13.1"` → `prost = { workspace = true }`

## 3. lockfile 与 MSRV 同步

- [x] 3.1 本地 `cargo update -p bytes -p prost --workspace` 后看 `Cargo.lock` diff，确认 resolved 版本是否变化
  - **执行订正**：`cargo update` 自身会 bump 版本（实测 bytes 1.11.1→1.12.0），是错误验证手段；改用 `cargo build --locked` 证明既有 lockfile 满足新 Cargo.toml。详见 plan Task 3 Step 1 订正段
- [x] 3.2 若变化：同步 `.github/msrv/Cargo.lock` 对应 `[package]` 条目（按 Q1 决策：cp 覆盖或手动精准改），保证 msrv `--locked` 可过

## 4. 验证

- [x] 4.1 `cargo fmt --check` 过
- [x] 4.2 `just lint` 过（CI 双模式：`--all-features` 与 `--no-default-features` 均过）
- [x] 4.3 `cargo build --workspace --all-features` 过
- [x] 4.4 msrv 验证：用 `.github/msrv/Cargo.lock` 跑 `--locked` 过（docker rust:1.88 或本地）
- [x] 4.5 `cargo deny check` 过（依赖图无新冲突）+ `cargo tree -d` 对比 baseline（**不引入** bytes/prost 新多版本；prost 0.12/0.13 既存 split 由 vendored prost-build 引入，不计入新增）
