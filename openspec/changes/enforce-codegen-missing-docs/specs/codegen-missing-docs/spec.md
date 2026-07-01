## ADDED Requirements

### Requirement: codegen 生成后验证闭环 MUST NOT 绕过 missing_docs

`tools/codegen.py` 的生成后验证闭环（`run_closed_loop`，对刚生成的代码跑 fmt + clippy）MUST 与 workspace/CI 的 missing_docs 治理一致——其 clippy 调用 MUST NOT 含 `-A missing_docs`（或任何放过 missing_docs 的 `-A` 标志）。这消除 codegen 工具维度「生成时绕过、仓库级强制」的执行不一致，确保 codegen 持续产出符合 workspace missing_docs 标准的代码，不让 `-A` 兜底掩盖生成代码的文档缺口。

#### Scenario: run_closed_loop clippy 不含 -A missing_docs

- **WHEN** 检查 `tools/codegen.py` 的 `run_closed_loop` 函数中 clippy 命令的参数
- **THEN** MUST NOT 出现 `-A missing_docs`（命令以 `-- -Dwarnings` 结尾，不额外放过 missing_docs）

#### Scenario: 生成的代码在无 -A 的 clippy 下通过

- **WHEN** codegen 生成一个 API 文件后，对目标 crate 跑 `cargo clippy -p <crate> --all-targets --all-features -- -Dwarnings`（无 `-A missing_docs`）
- **THEN** MUST exit 0（生成代码 doc-complete，不依赖 `-A` 兜底）

### Requirement: codegen 生成的 pub 字段 MUST 有 doc（含 fallback）

`tools/api_contracts/codegen_render.py` 渲染 struct 字段时，每个 pub 字段 MUST 带 `///` doc。当 schema 字段有 `description` 时用其作为 doc；当 `description` 缺失时 MUST 生成 fallback doc（`/// {field.rust_name}`，引用字段真实名），不得留空。此约束保证移除 `-A missing_docs` 后生成代码仍 doc-complete，从源头杜绝 #273 治理项 #1 那样的累积缺口。

#### Scenario: 有 description 的字段生成语义 doc

- **WHEN** codegen 渲染一个 schema 中带 `description` 的 pub 字段
- **THEN** 生成的 `.rs` 中该字段 MUST 有 `/// {_oneliner(description)}` 形式的 doc

#### Scenario: 无 description 的字段生成 fallback doc

- **WHEN** codegen 渲染一个 schema 中 `description` 为空的 pub 字段（`field.rust_name` = `user_id`）
- **THEN** 生成的 `.rs` 中该字段 MUST 有 `/// user_id` 形式的 fallback doc（引用字段真实名），MUST NOT 无 doc
