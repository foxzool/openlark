## ADDED Requirements

### Requirement: 公开项 MUST 有有意义 doc（禁止占位符）

OpenLark 公开项（struct/enum/fn/method/field 等）的 doc 注释 MUST 是有意义的、描述该项的内容，MUST NOT 是 `/// 待补充文档。`、`/// 公开项说明。` 等占位符。占位符 doc 虽能让 missing_docs lint 通过，但无实际文档价值，违反 doc 质量治理。doc 注释 MUST 放在 `#[derive]`/属性**之前**（标准 Rust 规范）。

#### Scenario: docs crate 无占位符 doc

- **WHEN** 运行 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/`
- **THEN** 输出 MUST 为空（docs crate 的 144 行 `公开项说明` 占位已替换为有意义 doc）

#### Scenario: 占位符 doc 不在 #[derive] 后

- **WHEN** 检查 docs crate 的 doc 注释位置
- **THEN** `///` MUST 在 `#[derive(...)]`/属性之前（修正 legacy 把 `///` 放 `#[derive]` 后的非标准位置）
