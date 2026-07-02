## 1. 勘探 + pilot
- [x] 1.1 勘探 application 91 文件占位分布（按 version v1/v5/v6/v7 + sub-domain 分组）。
- [x] 1.2 Pilot 1 文件验证 recipe + 位置修正。

## 2. 批量回补（subagent-driven 按组）
- [ ] 2.1 按 version/domain 组回补 578 占位为真 doc + 修正 `///` 位置到 `#[derive]` 前。
- [ ] 2.2 逐组自验 `cargo doc -p openlark-application` 该组文件无 warning。

## 3. 守门 + 验证
- [ ] 3.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/` 为空；位置守门（`#[derive]` 后不紧跟 `///`）。
- [ ] 3.2 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint`；application 现有测试不破。
