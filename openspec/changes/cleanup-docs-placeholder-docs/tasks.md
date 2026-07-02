## 1. 回补 docs crate 占位 doc（144 项 / 14 文件）
- [ ] 1.1 勘探 docs crate 14 文件的占位项分布（按 sub-domain 分组）。
- [ ] 1.2 按 sub-domain 组回补：替换 `/// 公开项说明。` 为 `<//! 标题>+<item 角色>` 真 doc；修正 `///` 位置到 `#[derive]` 前。
- [ ] 1.3 逐组自验：`cargo doc -p openlark-docs --all-features` 该组文件无 warning。

## 2. 守门
- [ ] 2.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/` 为空。
- [ ] 2.2 doc 位置守门：`#[derive]` 后不紧跟 `///`。

## 3. 验证
- [ ] 3.1 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint` 通过。
- [ ] 3.2 docs crate 现有测试不破。
