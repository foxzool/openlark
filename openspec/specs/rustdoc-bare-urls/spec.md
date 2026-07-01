# rustdoc-bare-urls Specification

## Purpose
TBD - created by archiving change enforce-bare-urls. Update Purpose after archive.
## Requirements
### Requirement: doc comment 中的 URL MUST 以超链接格式书写

OpenLark workspace 内所有 Rust doc comment（`//!` 模块级与 `///` 项级）中出现的 URL MUST 以 `<...>` 包裹的超链接格式书写（如 `//! 文档: <https://open.feishu.cn/document/...>`），不得出现裸 URL（如 `https://...` 未包裹）。此约束消除 `rustdoc::bare_urls` warning，保证 rustdoc 输出可点击、可被文档工具正确解析。

#### Scenario: 现存裸 URL 全部包裹

- **WHEN** 运行 `cargo doc --workspace --all-features 2>&1 | grep -c 'bare_urls'`
- **THEN** 输出 MUST 为 `0`（迁移前实测 1578，迁移后清零）

#### Scenario: 新增裸 URL 即时暴露

- **WHEN** 开发者在任意 `//!` / `///` doc comment 中新增未包裹的裸 URL
- **THEN** `cargo doc --workspace --all-features` MUST 报 `rustdoc::bare_urls` 否定级错误（deny）

### Requirement: bare_urls lint 在 workspace 级 deny 强制，CI 不压制

`rustdoc::bare_urls` lint MUST 在根 `Cargo.toml` 的 `[workspace.lints.rustdoc]` 段声明为 `deny`，全 crate 经 `[lints] workspace = true` 继承。CI doc job MUST NOT 用 `-A rustdoc::bare_urls` 压制该 lint（即 CI 真正执行 deny，任何残留裸 URL 导致 CI 失败）。

#### Scenario: workspace.rustdoc 段声明 deny

- **WHEN** 检查根 `Cargo.toml`
- **THEN** MUST 存在 `[workspace.lints.rustdoc]` 段含 `bare_urls = "deny"`；20 个业务 crate + 根包 MUST 经 `[lints] workspace = true` 继承

#### Scenario: CI doc job 不压制 bare_urls

- **WHEN** 检查 `.github/workflows/ci.yml` 的 doc job `RUSTDOCFLAGS`
- **THEN** MUST NOT 含 `-A rustdoc::bare_urls`（移除迁移前的压制）；doc job 在零 bare_urls 存量下 MUST 通过

### Requirement: codegen 渲染器 MUST 发射超链接格式的 URL

`tools/api_contracts/codegen_render.py`（及任何生成 `//! doc comment` 的 codegen 模块）MUST 把 URL 以 `<...>` 包裹格式发射，不得发射裸 URL。此约束保证未来 codegen `--write` 生成的新 API doc 不重新引入 bare_urls，与 workspace deny 一致。

#### Scenario: codegen 输出含包裹 URL

- **WHEN** codegen 渲染器生成含 URL 的模块级 doc（如 `docPath:` 行）
- **THEN** 输出 MUST 为 `//! docPath: <https://...>` 形态（URL 被 `<>` 包裹），MUST NOT 为裸 `//! docPath: https://...`

