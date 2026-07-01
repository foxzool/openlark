# Comet Design Handoff

- Change: enforce-bare-urls
- Phase: design
- Mode: compact
- Context hash: 6ee02570ed6c6a26244e19db252e70282d1a23f3fea48090a222e2be69cc677f

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/enforce-bare-urls/proposal.md

- Source: openspec/changes/enforce-bare-urls/proposal.md
- Lines: 1-32
- SHA256: 7c9c034c57ee9843a1bb8435e9bb6505d1731f7c86665a66c9cc83bf205dd634

```md
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
```

## openspec/changes/enforce-bare-urls/design.md

- Source: openspec/changes/enforce-bare-urls/design.md
- Lines: 1-68
- SHA256: 8d1b1a0680cbb75b558cb31a1b4e1fbe9ffd0eeb2ced53276a262876ec552962

```md
## Context

OpenLark workspace 的 `rustdoc::bare_urls` lint 当前处于「CI 压制 + workspace 未治理」状态：

| 层 | 现状 |
|----|------|
| `[workspace.lints]`（根 Cargo.toml:62-71） | 有 `[workspace.lints.rust]` / `[.clippy]`，**无 `[.rustdoc]` 段**——bare_urls 未治理 |
| CI doc job（ci.yml:40-57） | `RUSTDOCFLAGS="-D warnings -A rustdoc::bare_urls"`——显式 allow 压制 |
| 实测 warning | **1578 条**（cargo doc --workspace --all-features），16 crate 分布，被 CI 静默放过 |
| 形态 | 94.4% codegen 注入的 `//! docPath: <裸URL>`；4.8% 手写 `//! 文档: <裸URL>`；全仓 `<URL>` 规范化的 URL = 0 |

约束：codegen 渲染器（`tools/api_contracts/codegen_render.py`）的 `_module_doc()` 等函数负责发射 `//! docPath:` 行；`tools/codegen.py` 闭环用 `-A missing_docs`（不在本范围）；20 crate 全 `[lints] workspace = true` 继承。

issue #273 Part A 现象 B 要求清零 bare_urls 并 workspace 级统一（deny 锁死）。Part A1（missing_docs/一致性治理）另开 change。

## Goals / Non-Goals

**Goals:**

- 全 workspace bare_urls warning **1578 → 0**（`cargo doc --workspace --all-features` 实测零警告）
- `[workspace.lints.rustdoc] bare_urls = "deny"` 落盘，全 crate 继承
- CI doc job 移除 `-A rustdoc::bare_urls` 压制，真正执行 deny
- codegen 渲染器发射 `<URL>`，未来 `--write` 不再生裸 URL
- 落盘 `rustdoc-bare-urls` capability spec（判定准则 + codegen 发射规范）

**Non-Goals:**

- 不动 missing_docs 策略（A1，另开 change）
- 不动源码级 `#![...]` outlier（security deny / analytics allow / client 注释，A1）
- 不修 `just lint -A missing_docs` 与 CI 的不一致（A1）
- 不动 codegen 的 `-A missing_docs` 闭环（`tools/codegen.py:185`，A1/另案）
- 不清理 1057 行占位 doc（独立债务）
- 不改 doc comment 的文本内容（仅包裹 URL）

## Decisions

### D1: 批量修复用脚本 + `cargo doc` 闭环验证

1578 处裸 URL 形态极统一（`//! <label>: <bare-url>`），用脚本（sed 或小 Python）把行内 `https://...` 包成 `<https://...>`。脚本须处理：行尾 URL、同一 `//!` 行多 URL、`docPath:` / `文档:` 两类标签。修完跑 `cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` 必须 = 0 闭环验证。具体脚本实现（sed vs Python）+ 边界抽样在 design 阶段定。

### D2: codegen 渲染器同步改造（防复发必需）

定位 `tools/api_contracts/codegen_render.py` 中发射 `docPath:` 的函数（调研指向 `_module_doc()` :69-76），把裸 URL 改为 `<URL>`。这样未来 codegen `--write` 生成的新 API doc 不含裸 URL。**若不改 codegen，workspace deny 后任何 codegen --write 会立刻 CI 红**——故 codegen 改造是 deny 锁死的前置必要条件。

### D3: workspace `[workspace.lints.rustdoc] bare_urls = "deny"`

根 Cargo.toml 新增 `[workspace.lints.rustdoc]` 段（当前不存在），声明 `bare_urls = "deny"`。全 crate 已 `[lints] workspace = true`，自动继承。选 `deny`（而非 warn）以锁死、防复发——前提是 D1 已清零存量、D2 已防新增。

### D4: CI doc job 移除 `-A rustdoc::bare_urls`

`ci.yml:44` 的 `RUSTDOCFLAGS` 去掉 `-A rustdoc::bare_urls`，让 deny 生效。移除后 CI doc job 会因任何残留 bare_urls 失败——由 D1 的零警告保证通过。

### D5: 落盘 `rustdoc-bare-urls` capability

对齐 `workspace-dependency-policy` / `feature-naming-convention` 范式，新建 capability spec 记录「doc comment URL MUST `<...>` 包裹 + workspace deny 强制 + CI 不压制 + codegen 发射规范」，供 code-review/design-review 与 codegen 引用防复发。

## Risks / Trade-offs

- **[脚本误伤非 URL 文本]** → Mitigation：脚本只匹配 `//!` 行内的 `https://` / `http://` token 并包裹；修完 `git diff` 抽样人工核对 + `cargo doc` 闭环零警告双保险。
- **[77 处手写 `文档:` URL 边界]** → Mitigation：design 阶段抽样核实是否有非 URL 尾随文本；脚本对这类行单独处理。
- **[deny 后 codegen --write 破坏 CI]** → Mitigation：D2 同步改 codegen 渲染器；验证阶段跑一次 codegen 渲染（若可行）确认输出含 `<URL>`。
- **[codegen 当前未 --write 落盘]** → 现状 codegen 产物未大规模入库（风格 A 6 PR 完成但未 --write），故 D2 是「面向未来」的预防性改造，不阻塞当前清零。

## Open Questions

- **Q1（codegen 发射点精确位置）**：`codegen_render.py` 中发射 `docPath: <URL>` 的具体函数与行（调研指向 `_module_doc:69-76`，design 阶段读源确认）。是否还有 `_field_lines` / 其他函数也发射 URL？
- **Q2（脚本实现：sed vs Python）**：1578 处用单行 sed 还是写小 Python 脚本（更易处理多 URL/边界）？design 阶段看样本后定。
- **Q3（CI doc job 移除后是否需要 rustdoc 其他 lint 配置）**：移除 `-A rustdoc::bare_urls` 后，`-D warnings` 会不会暴露其他 rustdoc lint（如 broken_intra_doc_links）？design 阶段实测 `cargo doc` 全量 warning 种类确认。
```

## openspec/changes/enforce-bare-urls/tasks.md

- Source: openspec/changes/enforce-bare-urls/tasks.md
- Lines: 1-31
- SHA256: 56d612d2dc03fceeac1829d02ab120c128996cb61482c591d6b2a25ecbb3aaa1

```md
## 1. codegen 渲染器改造（D2，防复发前置）

- [ ] 1.1 定位 `tools/api_contracts/codegen_render.py` 中发射 `docPath:` / URL 的函数（调研指向 `_module_doc:69-76`，读源确认所有 URL 发射点）
- [ ] 1.2 改发射 `<URL>` 而非裸 URL（如 `//! docPath: <https://...>`）
- [ ] 1.3 抽样/单元验证 codegen 渲染输出含 `<URL>`（若 codegen 可离线跑渲染）

## 2. 批量修复现存 1578 裸 URL（D1）

- [ ] 2.1 写脚本（sed 或小 Python）包裹 `//!` 行内的裸 `https://`/`http://` token 为 `<...>`，处理：行尾 URL、同一行多 URL、`docPath:` 与 `文档:` 两类标签
- [ ] 2.2 跑脚本修全 workspace（16 crate）
- [ ] 2.3 闭环验证：`cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` = **0**
- [ ] 2.4 `git diff` 抽样人工核对（重点：77 处手写 `文档:` URL、多 URL 行、非 URL 尾随文本未被误伤）

## 3. workspace 级 bare_urls deny（D3）

- [ ] 3.1 根 `Cargo.toml` 新增 `[workspace.lints.rustdoc]` 段，声明 `bare_urls = "deny"`
- [ ] 3.2 确认 20 业务 crate + 根包经 `[lints] workspace = true` 继承（应已全部继承，sanity check）

## 4. CI doc job 移除压制（D4）

- [ ] 4.1 `.github/workflows/ci.yml:44` 的 doc job `RUSTDOCFLAGS` 移除 `-A rustdoc::bare_urls`
- [ ] 4.2 实测移除后 `cargo doc --workspace --all-features` 是否暴露其他 rustdoc lint（如 broken_intra_doc_links），若有则记录并在 design 阶段决定处理

## 5. 验证（D1-D4 全过）

- [ ] 5.1 `cargo fmt --check` 过
- [ ] 5.2 `cargo doc --workspace --all-features` 过（deny 下 0 warning，exit 0）
- [ ] 5.3 `just lint` 过（clippy `--all-features` 双模式）+ 补跑 `--no-default-features` clippy（CI 同款）
- [ ] 5.4 `cargo build --workspace --all-features` 过
- [ ] 5.5 codegen 渲染器验证：渲染含 URL 的模块 doc，输出 MUST 为 `<URL>` 形态（Spec Req 3）
- [ ] 5.6 msrv `--locked` 验证（`.github/msrv/Cargo.lock`，rustup +1.88），确认无回归
```

## openspec/changes/enforce-bare-urls/specs/rustdoc-bare-urls/spec.md

- Source: openspec/changes/enforce-bare-urls/specs/rustdoc-bare-urls/spec.md
- Lines: 1-38
- SHA256: 8b996e2d1f8aef7661d50007abe87628dca810abcd7bba3bd1ef87a0d5c95135

```md
## ADDED Requirements

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
```

