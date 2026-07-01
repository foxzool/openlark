## 1. 规范文档落盘

- [x] 1.1 创建 `docs/FEATURE_NAMING_CONVENTION.md`：写明「模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外」原则；附三套方案现状清单（A 单版本门控 / B 纯模块门控 / C 混合）、6 个单版本 crate 合法例外清单，并显式纠正 issue #272 对 `openlark-bot` v4 的误判（v4 门控 live 代码）
- [x] 1.2 在 `AGENTS.md` 的 CONVENTIONS 段补一行指向 `docs/FEATURE_NAMING_CONVENTION.md`

## 2. 移除 5 个混合 crate 的死版本链

- [x] 2.1 `openlark-platform/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [x] 2.2 `openlark-analytics/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [x] 2.3 `openlark-security/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [x] 2.4 `openlark-docs/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [x] 2.5 `openlark-user/Cargo.toml`：移除 `v2`/`v3`/`v4` 定义（**保留 live 的 `v1`**），`full` 去掉 `"v4"`

## 3. 同步下游 docs feature 引用

- [x] 3.1 `crates/openlark-client/Cargo.toml`：`docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` 引用
- [x] 3.2 根 `Cargo.toml`：workspace `docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"`；按 Q2 决策处理 `docs-sheets-v2`/`docs-sheets-v3`（退化为 `docs-ccm` 别名或移除）

## 4. 验证

- [x] 4.1 `just fmt` + `cargo fmt --check` 通过
- [x] 4.2 `just lint` 通过（CI 双模式：`--no-default-features` 与 `--all-features` 均过，确认无悬空 feature 引用）
- [x] 4.3 `just build` + `just test` 通过
- [x] 4.4 核实 `docs/FEATURE_MATRIX.md` 是否需同步更新移除的 feature（Q3）

## 5. src vestigial 代码清理（实现中发现，Spec Patch）

> build 验证发现 spec 前提「5 crate v-feature 全门控 0 行」部分有误：窄 grep `cfg(feature="v1")` 漏了 `cfg(all/any(...))` 形态。实情：docs v1/v2/v3 门控孤立的 `versions` 模块（vestigial，硬编码版本号，0 外部引用），platform v1 门控 2 个测试（其 `.v1()` 方法本身是无条件 pub fn）。补清理这些 vestigial 代码/测试门控，以消除 `--all-targets` 下的 `unexpected_cfgs` warning。

- [x] 5.1 `crates/openlark-docs/src/lib.rs`：移除 `// API版本模块` + `#[cfg(any(...))] pub mod versions;` 块；删除 `crates/openlark-docs/src/versions.rs`（vestigial 未用模块，BREAKING 但 0 外部引用）
- [x] 5.2 `crates/openlark-platform/src/lib.rs:161`：`#[cfg(all(feature = "spark", feature = "v1"))]` → `#[cfg(feature = "spark")]`（测试测的是无条件 `.v1()` 方法，去死 v1 条件）
- [x] 5.3 `crates/openlark-platform/tests/platform_contract_models.rs`：`#![cfg(all(...))]` 去 `feature = "v1"` 行
- [x] 5.4 根 `src/lib.rs`：两处 `#[cfg(any(...))]`（L53-64 re-export + L168-179 测试）移除 `feature = "docs-sheets-v2"`、`feature = "docs-sheets-v3"` 两行
