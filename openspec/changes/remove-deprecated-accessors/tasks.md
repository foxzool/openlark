# Tasks — remove-deprecated-accessors

> 范围：移除 10 个 deprecated 兼容访问器（HR 8 + analytics 2），均 0 内部调用、干净移除。BREAKING。关联 #268（A+E 子集）；B/C/D/F/G 另开 issue。

## 1. 移除 HR 8 个 deprecated 访问器

- [ ] 1.1 删除 `crates/openlark-hr/src/lib.rs` 的 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` 8 个 `#[deprecated] pub fn`（含其 `#[cfg(feature)]` 门控与文档注释）
- [ ] 1.2 确认 `Hr` 的对应字段（`attendance`/`corehr`/...）保留且 `pub`（字段访问替代）

## 2. 移除 analytics 2 个 deprecated 存根

- [ ] 2.1 删除 `crates/openlark-analytics/src/search/search/v2.rs` 的 `query()`/`user()` 2 个 `#[deprecated] pub fn`
- [ ] 2.2 确认 `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型保留 `pub`（仅移除便捷存根方法）

## 3. 验证

- [ ] 3.1 `grep '#\[deprecated' crates/openlark-hr/ crates/openlark-analytics/` 确认目标 10 个已移除（剩余的 B/C/D/F/G 不在本范围）
- [ ] 3.2 examples 不引用已移除方法：`grep '\.attendance()|\.corehr()|\.query()' examples/` = 0
- [ ] 3.3 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [ ] 3.4 `--all-features` exit 0
- [ ] 3.5 `--no-default-features` exit 0
- [ ] 3.6 `cargo test --workspace` 通过

## 4. 文档与收尾

- [ ] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加迁移条目 + 迁移映射表（方法→字段 / 其他 surface）
- [ ] 4.2 开 follow-up issue：B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）共 10 个 deprecated 异构项的迁移清理
- [ ] 4.3 评论 #268：本 change 处理 A+E（10），B/C/D/F/G 拆至 follow-up；#268 待 follow-up 完成后关闭
