# Brainstorm Summary

- Change: clarify-transport-reqwest-boundary
- Date: 2026-06-29

## 确认的技术方案

policy 方向 C（open 阶段确认）：清理 12 个未用 reqwest 依赖 + 文档化 `Transport<T>` 边界；webhook 例外保留并文档化；中间件层推迟为 future change。

design 阶段追加确认（brainstorming）：**新增防回归守卫脚本**，匹配项目 hygiene 不变量既定模式（`check_no_dead_code_allows.sh` 先例）。完整方案 6 部分：

1. **依赖清理**：12 个业务 crate（analytics/auth/bot/application/communication/mail/hr/docs/helpdesk/platform/user/workflow）的 `[dependencies]` 各删 1 行 `reqwest = { workspace = true }`。已核实全部在 `[dependencies]`，无 dev/build 残留。保留 core/client/webhook 例外。
2. **防回归守卫（新增）**：`tools/check_reqwest_boundary.sh`——遍历 `crates/openlark-*/`，白名单 {core,client,webhook}，业务 crate Cargo.toml 出现 reqwest 则 exit 1 + 列违规项；`set -euo pipefail` + ✅/❌ 风格对齐先例。justfile 加 `reqwest-boundary` recipe（对齐 justfile:17-19）；ci.yml lint job 加一步（对齐 ci.yml:115-116）。
3. **ARCHITECTURE.md 边界文档化**：`## 模块详细设计` 下新增「Transport HTTP 边界」小节——边界定义 + 调用路径（`*Request::execute() → Transport::request() → ReqTranslator → reqwest`）+ webhook by-design 例外（#214）+ 中间件/熔断/重试层标注「规划中 future、当前未实现」。
4. **webhook 文档注释强化**：`crates/openlark-webhook/src/robot/v1/` 的 shared_client/client 注释指向边界约定（不改源码逻辑）。
5. **CHANGELOG**：v0.18 hygiene 条目（删 12 未用 reqwest 依赖 + 新增边界守卫，非 breaking）。
6. **Spec Patch（回写 delta spec）**：新增 Requirement「Transport 边界由守卫脚本机器检验」+ scenarios（脚本存在 / 清理后通过 / 能抓回归）。

## 关键取舍与风险

- **取舍**：加守卫脚本 = 多 1 文件 + 2 处接线（justfile/ci.yml），换来边界可机器检验、防回归。审计 finding 本身证明该边界易被误解（看 Cargo.toml 误判）。匹配项目 hygiene 既定模式，非 scope 蔓延。
- **备选拒绝**：A（trait + 中间件层）——为不存在的问题引入抽象，YAGNI；B（仅一句文档不删依赖）——审计 finding 未闭环；cleanup-only（不加守卫）——无机器防护，可能被未来 PR 重新引入。
- **风险**：
  - 删依赖致 build 失败 → 已核实 12 crate 源码 0 处 reqwest 引用、tests/examples/build.rs 均无；workspace dep features 集中管理（root Cargo.toml:90）。build 阶段 `cargo build --workspace --all-features` 实证兜底。
  - 守卫脚本误报 → 白名单精确（仅 core/client/webhook 三个例外）。
  - 下游隐式依赖业务 crate 传递出的 reqwest → 业务 crate 无 re-export reqwest，属不支持用法；CHANGELOG 记录兜底。

## 测试策略

- `cargo build --workspace --all-features` exit 0
- 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- `cargo test --workspace` 全部通过（0 failed）
- 守卫脚本自洽：清理后 `bash tools/check_reqwest_boundary.sh` 必须 PASS
- grep 双重确认：12 crate 的 Cargo.toml + src `reqwest` 命中均 0；core/client/webhook Cargo.toml 仍保留 reqwest

## Spec Patch

delta spec `specs/transport-reqwest-boundary/spec.md` 新增：

- **Requirement: Transport 边界由守卫脚本机器检验**
  - Scenario: 守卫脚本存在并可执行（`tools/check_reqwest_boundary.sh` 存在）
  - Scenario: 清理后守卫通过（业务 crate Cargo.toml 无 reqwest → 脚本 exit 0）
  - Scenario: 守卫能抓回归（业务 crate 重新声明 reqwest → 脚本 exit 1 并列出违规项）
