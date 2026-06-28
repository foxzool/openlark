# Tasks — remove-deprecated-accessors

> 已完成。移除 10 个 deprecated 兼容访问器（HR 8 + analytics 2），BREAKING。build 发现：HR lib test 用了 `.attendance()`（已迁移字段访问）；analytics 移除 query()/user() 后 SearchV2.config 变 dead（已改 _config，#275）。关联 #268（A+E）；B/C/D/F/G 拆至 #278。

## 1. 移除 HR 8 个 deprecated 访问器

- [x] 1.1 删除 `crates/openlark-hr/src/lib.rs` 的 attendance/corehr/compensation/payroll/performance/okr/hire/ehr 8 个 `#[deprecated] pub fn`（含 cfg 门控 + 注释）
- [x] 1.2 保留 Hr 的 pub 字段（attendance/corehr/...）；lib test `client.attendance()` → `client.attendance.clone()`（字段访问）

## 2. 移除 analytics 2 个 deprecated 存根

- [x] 2.1 删除 `SearchV2::query()`/`user()`；config → `_config`（存根移除后无读取者，#275）
- [x] 2.2 保留 `v2/query.rs`/`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型（D2）

## 3. 验证

- [x] 3.1 HR+analytics 目标 deprecated = 0
- [x] 3.2 examples 不引用已移除方法（= 0）
- [x] 3.3 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [x] 3.4 `cargo test --workspace` 通过

## 4. 文档与收尾

- [x] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射表
- [x] 4.2 开 follow-up issue #278（B/C/D/F/G 10 个异构 deprecated）
- [x] 4.3 评论 #268：A+E 已处理，B/C/D/F/G 拆至 #278；#268 待 #278 完成后关闭
