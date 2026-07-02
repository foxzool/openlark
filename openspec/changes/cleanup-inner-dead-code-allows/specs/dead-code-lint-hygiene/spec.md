## MODIFIED Requirements

### Requirement: 不用 #[allow(dead_code)] 掩盖可修复的死字段
openlark 公开源代码 SHALL 不使用外层 `#[allow(dead_code)]` **或内层 `#![allow(dead_code)]`**（crate/mod 级）抑制可修复的 dead_code 警告。废弃模块、0 引用的 `pub(crate)`/私有脚手架 SHALL 直接删除；真死字段 SHALL 修正（读取/移除）或显式处理（`_` 前缀 + 注释 / `#[expect(dead_code)]`）。CI 死代码守卫脚本 SHALL 不保留 `KNOWN_INNER_DEBT` 类人为开口（inner-attribute 例外清单必须为空）。历次清理（#267 外层 392 处 + 本 change 内层 7 处/104 项）后，dead_code lint 信号 SHALL 对未来真死字段保持有效。

#### Scenario: HR crate 内外层均无残留
- **WHEN** 在 `crates/openlark-hr/` 中 grep `#!?\[allow\(dead_code\)\]`
- **THEN** 命中数为 0（原 361 外层 cruft + 已删除的废弃 `endpoints/` 模块 84 内层常量全清）

#### Scenario: 全 workspace 内外层均无 cruft 残留
- **WHEN** 在 `crates/` + `src/` 中 grep `#!?\[allow\(dead_code\)\]`（排除 `#[cfg(test)]` 测试代码）
- **THEN** 命中数为 0，或仅保留带显式注释说明的 `_` 前缀字段 / `#[expect(dead_code)]` 项

#### Scenario: CI 死代码守卫无人为开口
- **WHEN** 运行 `just no-dead-code-allows`（即 `tools/check_no_dead_code_allows.sh`）
- **THEN** `KNOWN_INNER_DEBT` 例外清单为空，inner-attribute 不再享受豁免

#### Scenario: 废弃模块与 0 引用脚手架被删除而非抑制
- **WHEN** 移除全部 `#![allow(dead_code)]` 后运行 `cargo clippy --workspace --all-targets`
- **THEN** 0 dead_code 警告，且承载原死代码的废弃模块（hr `endpoints/`）/ 0 引用脚手架（core `observability.rs` 等）已从源码删除，而非以 `#[expect]]` 保留
