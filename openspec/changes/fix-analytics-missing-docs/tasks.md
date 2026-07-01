## 1. 基线与准备

- [x] 1.1 记录移除 allow 前的 missing_docs 基线：122 项（68 struct / 36 method / 18 associated fn），分布在 search/v2 与 report/v1 的 API 文件（实测 `cargo doc -p openlark-analytics --all-features` 输出）。
- [x] 1.2 确认 analytics 文件级 `//!` + Feishu `docPath` 已覆盖（17 文件）；标记 `search/v2/doc_wiki/search.rs:2` 空 docPath 待补。

## 2. 回补 analytics 公开项文档（122 项，全在 search/ + report/ 叶子 API 文件）

> 每个文件按 D1 模板回补：Request/Response struct + named field + `new`/`execute`/关联函数各一行有意义中文，对齐 `communication/contact/.../task/get.rs` 规范。文件级 `//!`+docPath 已在，不重写。

- [x] 2.1 回补 `search/v2/data_source/*`：`create/get/patch/delete/list` + `item/{create,get,delete}`。
- [x] 2.2 回补 `search/v2/schema/*`：`create/get/patch/delete`。
- [x] 2.3 回补 `search/v2/{app/create, message/create, doc_wiki/search, user, query}`；**补全 `doc_wiki/search.rs:2` 空 docPath**。
- [x] 2.4 回补 `report/v1/*`：`rule/query`、`rule/view/remove`、`task/query`。
- [x] 2.5 逐文件自验：每补完一组运行 `cargo doc -p openlark-analytics --all-features 2>&1 | grep <该组文件>` 无新增 warning。

## 3. 移除 crate 级抑制

- [ ] 3.1 移除 `crates/openlark-analytics/src/lib.rs:36` 的 `#![allow(missing_docs)]`（**保留** line 34 `#![allow(clippy::module_inception)]`，不在范围）。
- [ ] 3.2 验证 `cargo doc -p openlark-analytics --all-features` 0 missing_docs 警告。

## 4. 占位符守门（D2）

- [ ] 4.1 `grep -rnE '待补充文档|公开项说明' crates/openlark-analytics/src/` 输出为空（回补未引入占位符）。

## 5. CI 接线（D3）

- [ ] 5.1 在 `.github/workflows/ci.yml` 现有 `python3 -m unittest tools.tests.test_check_mod_reachability`（ci.yml:113）旁，加 `python3 -m unittest tools.tests.test_workspace_missing_docs`。
- [ ] 5.2 本地复现 CI 步骤：运行该模块，确认 3 个测试（`test_workspace_has_no_missing_docs_warnings` / `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions` / `test_workspace_item_level_missing_docs_exception_is_protocol_generated_module_only`）全绿。

## 6. 全局验证

- [ ] 6.1 `cargo doc --workspace --all-features` 无 missing_docs 警告（workspace 整体仍 0）。
- [ ] 6.2 `cargo fmt --check` 通过 + `just lint`（已无 `-A missing_docs`，会强制 missing_docs）通过。
- [ ] 6.3 `cargo test -p openlark-analytics` 现有测试不破。
