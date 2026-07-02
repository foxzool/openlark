## 1. 勘探与确认

- [x] 1.1 确认 codegen_render 的 pub 项全集：已勘探仅 struct 字段（经 `_field_lines`）需 fallback；无 `pub enum`/`pub type`/`pub const`（endpoint const 在 endpoints/，[MANUAL] 不自动生成）。
- [x] 1.2 检查 `tools/api_contracts/test_codegen_render.py` 是否有 `_field_lines` / `render_api_file` 的 golden 断言需随 fallback 同步（无 description 字段的预期输出）。

## 2. D2：`_field_lines` fallback doc

- [x] 2.1 修改 `tools/api_contracts/codegen_render.py` 的 `_field_lines`：`description` 缺失时生成 `/// {field.rust_name}`（字段名兜底），确保每个 pub 字段都有 doc。
- [x] 2.2 自验：构造一个无 `description` 的 FieldDef，确认 `_field_lines` 输出含 `/// <field_name>`。

## 3. D1：移除 `-A missing_docs`

- [x] 3.1 修改 `tools/codegen.py:185` `run_closed_loop` 的 clippy 命令：移除 `-A missing_docs`（`-- -Dwarnings` 结尾）。

## 4. 测试同步

- [x] 4.1 若 1.2 发现 golden 断言，更新 `test_codegen_render.py` 对应预期输出（含 fallback doc）。
- [x] 4.2 跑 `python3 -m pytest tools/api_contracts/test_codegen_render.py tools/api_contracts/test_codegen_ir.py tools/api_contracts/test_mod_tree.py`（或等价）通过。

## 5. 验证

- [x] 5.1 **实跑生成闭环**：在测试夹具或临时目标上跑 codegen 生成一个 API（含无 description 字段）+ `cargo clippy -p <crate> --all-targets --all-features -- -Dwarnings`（无 `-A`）exit 0；**不覆盖真实业务 crate 文件**（生成后 `git checkout` 还原或用临时目录）。
- [x] 5.2 `cargo fmt --check` + `just lint`（codegen 改动属 tools/，确认 workspace lint 不破）通过。
- [x] 5.3 确认 `api-contracts` CI 相关本地可跑的验证（`python3 tools/validate_api_contracts.py` 等）不破。
- [x] 5.4 占位符守门：fallback doc 是字段真实名（非 `待补充文档` 等占位符）。
