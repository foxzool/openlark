# Tasks — unify-auth-request-naming（#271 pilot）

> issue #271 命名统一 pilot：openlark-auth 13 个请求类型 `XxxBuilder` → `XxxRequest` + `#[deprecated]` type alias 软迁移。
> 验证模式后，其余 crate（platform/ai/hr/cardkit）按批次另开 change。

## 1. 精确化范围

- [ ] 1.1 核实 13 个 Builder 的可见性（pub 与否）+ auth prelude/re-export 链 + 根 crate `openlark::` 是否再导出；确定哪些需 `#[deprecated]` alias（公开的）、哪些可直接改名（内部的）

## 2. 重命名 + alias

- [ ] 2.1 将 13 个请求类型 struct `XxxBuilder` 重命名为 `XxxRequest`（`TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`，去 Builder 避免双 Request）；同步 struct 定义、impl 块、new() 返回类型
- [ ] 2.2 为 13 个旧名添加 `#[deprecated(note = "renamed to XxxRequest, will be removed in v1.0 (#271)")] pub type XxxBuilder = XxxRequest;`
- [ ] 2.3 同步 auth 内部所有引用（同 crate 内 `use`/类型标注）到新名；prelude/re-export 同时导出新名 + 旧名 alias

## 3. examples / docs / tests

- [ ] 3.1 更新 auth 相关 examples、docs、tests 到新名（旧名 alias 保留兼容，但新代码用新名）；为 alias 的 deprecation warning 行为加针对性测试

## 4. CHANGELOG

- [ ] 4.1 CHANGELOG v0.18 Breaking Changes 段记录 13 个重命名 + `#[deprecated]` alias + v1.0 移除指引 + 迁移说明（`XxxBuilder` → `XxxRequest`）

## 5. 验证

- [ ] 5.1 `cargo build --workspace --all-features` exit 0
- [ ] 5.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 5.3 `cargo test -p openlark-auth` 全部通过（0 failed）
- [ ] 5.4 实证 alias：用旧名 `AppAccessTokenBuilder` 写临时代码 build → 成功 + deprecation warning；用新名 → 成功无 warning；还原
- [ ] 5.5 grep 确认：13 个 `XxxRequest` struct 存在、13 个 `XxxBuilder` type alias 带 `#[deprecated]`、`AuthorizationUrlBuilder` 未被改动
