## Why

issue #273 Part A 现象 B：`rustdoc::bare_urls` warning 系统性存在。全 workspace 实测 **1578 条**（openlark-hr 542、communication 352、meeting 215、platform 149、workflow 67、docs 56…），被 CI doc job 用 `-A rustdoc::bare_urls` 静默压制（`.github/workflows/ci.yml:44`）——既不 fail 也不被修，是「已知债务被 allow 隐藏」的典型。形态极统一：94.4%（1515 处）是 codegen 注入的 `//! docPath: <裸URL>`，4.8%（77 处）是手写 `文档:` 标签。workspace.lints 当前无 `rustdoc` 段，无统一治理点。

现在做是因为：改动机械化（裸 URL 包 `<...>` 即可）、收益确定（1578→0）、可借 codegen 改造一并杜绝复发，趁 v0.18 周期补齐。

## What Changes

- **批量修复** 1578 处裸 URL → `<https://...>`（覆盖 codegen 注入的 `docPath:` 1515 处 + 手写 `文档:` 77 处 + 其余边界；脚本执行 + `cargo doc` 闭环验证）
- **修改 codegen 渲染器**（`tools/api_contracts/codegen_render.py`，发射 `docPath:` 的函数）输出 `<URL>` 而非裸 URL，杜绝未来 codegen `--write` 再生裸 URL
- **新增** 根 `Cargo.toml` 的 `[workspace.lints.rustdoc] bare_urls = "deny"`，全 crate 经 `[lints] workspace = true` 继承
- **移除** CI doc job 的 `-A rustdoc::bare_urls` 压制（`ci.yml:44`），让 CI 真正执行 deny
- **保留** 所有现有 `//!` 文档文本内容，仅把裸 URL 包裹成 `<...>` 超链接

## Capabilities

### New Capabilities

- `rustdoc-bare-urls`: workspace 级 rustdoc bare_urls 治理策略——doc comment 中的 URL MUST 以 `<...>` 超链接格式书写；lint 在 workspace 级 `deny` 强制，CI 不再压制。含判定准则（`cargo doc --workspace --all-features` 零 bare_urls warning）与 codegen 发射规范。

### Modified Capabilities

（无——本次仅 bare_urls 治理，不影响其他 lint 或 missing_docs 策略，后者属 issue #273 Part A1 另开 change。）

## Impact

- **代码**：~1200 个 `//!` doc comment 行跨 16 crate（脚本批量包裹 URL，文本内容不变）；codegen 渲染器 1-2 处函数改造
- **配置**：根 `Cargo.toml` 新增 `[workspace.lints.rustdoc]` 段；`.github/workflows/ci.yml` doc job 移除 `-A rustdoc::bare_urls`
- **公共 API**：无变化（仅 doc comment 文本格式）
- **CI**：doc job 从「压制 bare_urls（-A）」转为「执行 deny」；feature-combinations job 不受影响（不跑 doc）
- **codegen**：渲染器发射 `<URL>`，未来生成的 API doc 不再含裸 URL（codegen 闭环 `tools/codegen.py` 的 `-A missing_docs` 不在本范围，属 A1）
- **性能**：无影响
