## ADDED Requirements

### Requirement: application crate 公开项 MUST 无占位符 doc

`openlark-application` crate 的公开项 MUST 有有意义 doc，MUST NOT 含 `/// 待补充文档。` 占位符。doc 注释 MUST 在 `#[derive]`/属性之前。

#### Scenario: application crate 无占位符 doc

- **WHEN** 运行 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/`
- **THEN** 输出 MUST 为空（578 行 `待补充文档` 占位已替换为有意义 doc）

#### Scenario: application doc 不在 #[derive] 后

- **WHEN** 检查 application crate 的 doc 注释位置
- **THEN** `///` MUST 在 `#[derive(...)]`/属性之前
