## 1. just lint 对齐 CI（D1）

- [x] 1.1 `justfile:14` 的 `just lint` recipe 移除 `-A missing_docs`（改为 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`）
- [x] 1.2 跑 `just lint` 确认仍通过（missing_docs 现状 0 警告，移除 -A 不破坏）

## 2. 源码 outlier 清理（D2 + D3）

- [x] 2.1 删除 `crates/openlark-security/src/lib.rs:88` 的 `#![deny(missing_docs)]`
- [x] 2.2 删除 `crates/openlark-client/src/lib.rs:238` 的死注释 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译`
- [x] 2.3 确认 `crates/openlark-protocol/src/lib.rs:9` 的 `#[allow(missing_docs)]`（item 级例外）保留不动

## 3. 验证

- [x] 3.1 `cargo fmt --check` 过
- [x] 3.2 `cargo doc --workspace --all-features` 过（missing_docs warning = 0，deny/warn 不变）
- [x] 3.3 `cargo clippy -p openlark-security --all-features -- -Dwarnings` 过（移除 deny 后 security 仍 0 警告）
- [x] 3.4 `just lint` 过（移除 -A missing_docs 后与 CI 一致）
- [x] 3.5 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` 过（CI 同款）
- [x] 3.6 `cargo build --workspace --all-features` 过
- [x] 3.7 msrv `--locked` 验证（`.github/msrv/Cargo.lock`，rustup +1.88）无回归
- [x] 3.8 确认 outlier 清理结果：`grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client` 无命中
