## 1. workspace.dependencies 声明 bytes

- [x] 1.1 根 `Cargo.toml` 的 `[workspace.dependencies]` 段新增 `bytes = "1.6"`（prost 已存在，不动）

## 2. openlark-protocol 改消费 workspace

- [ ] 2.1 `crates/openlark-protocol/Cargo.toml`：`bytes = "1.6.0"` → `bytes = { workspace = true }`
- [ ] 2.2 `crates/openlark-protocol/Cargo.toml`：`prost = "0.13.1"` → `prost = { workspace = true }`

## 3. lockfile 与 MSRV 同步

- [ ] 3.1 本地 `cargo update -p bytes -p prost --workspace` 后看 `Cargo.lock` diff，确认 resolved 版本是否变化
- [ ] 3.2 若变化：同步 `.github/msrv/Cargo.lock` 对应 `[package]` 条目（按 Q1 决策：cp 覆盖或手动精准改），保证 msrv `--locked` 可过

## 4. 验证

- [ ] 4.1 `cargo fmt --check` 过
- [ ] 4.2 `just lint` 过（CI 双模式：`--all-features` 与 `--no-default-features` 均过）
- [ ] 4.3 `cargo build --workspace --all-features` 过
- [ ] 4.4 msrv 验证：用 `.github/msrv/Cargo.lock` 跑 `--locked` 过（docker rust:1.88 或本地）
- [ ] 4.5 `cargo deny check` 过（依赖图无新冲突）+ `cargo tree -d`（无 bytes/prost 多版本）
