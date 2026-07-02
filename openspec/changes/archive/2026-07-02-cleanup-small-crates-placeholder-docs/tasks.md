## 1. 勘探 + pilot
- [x] 1.1 勘探 5 crate（mail/workflow/meeting/user/hr）占位文件分布。
- [x] 1.2 Pilot mail 1 文件验证 recipe + 位置修正。

## 2. 按 crate 回补（subagent-driven，5 组）
- [x] 2.1 mail（104）→ workflow（78）→ meeting（65）→ user（47）→ hr（41）逐 crate 回补占位为真 doc + 修正 `///` 位置。
- [x] 2.2 逐 crate 自验 `cargo doc -p <crate>` 无 warning。

## 3. 守门 + 验证
- [x] 3.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-{mail,workflow,meeting,user,hr}/src/` 为空；位置守门。
- [x] 3.2 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint`；5 crate 现有测试不破。
