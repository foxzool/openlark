# 验证报告 — remove-deprecated-wiki-params

- Date: 2026-06-28 | 分支: feature/20260628/remove-deprecated-wiki-params | base-ref: 402f3b66d
- verify_mode: full

## Fresh 证据
- default/all-features/no-default clippy `-D warnings`：三组 Finished
- `cargo test --workspace`：0 failures
- 4 个 Params struct grep = 0

## 结论
4 个 deprecated wiki Params（SearchWikiParams/ListWikiSpacesParams/CreateWikiSpaceParams/MoveDocsToWikiParams）移除。实证：无生产用法（仅 test_deprecated_params 兼容测试，一并删除）。mod.rs re-export 清理。BREAKING。

**Ready for archive。** 关联 #268（B）；#278（F im 别名仍 open）。
