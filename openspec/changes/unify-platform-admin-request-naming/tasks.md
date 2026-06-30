# Tasks — unify-platform-admin-request-naming（#271 platform admin 批）

## 1. 重命名 + alias
- [x] 1.1 14 个定义文件：struct+impl+测试 `XxxBuilder`→`XxxRequestBuilder`；`#[cfg(test)]` 前加 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`

## 2. 验证
- [x] 2.1 `cargo build --workspace --all-features` exit 0
- [x] 2.2 三组 clippy（-D warnings）exit 0
- [x] 2.3 `cargo test -p openlark-platform` 0 failed
- [x] 2.4 `cargo fmt --all -- --check` exit 0
- [x] 2.5 grep 14 RequestBuilder struct + 14 alias + 0 残留

## 3. CHANGELOG
- [x] 3.1 CHANGELOG v0.18 breaking 段记录

## 代码审查（review_mode: standard）
Ready to merge: Yes（#271 既定模式第 5 批，14 类型最简，0 Critical/Important）
