# Verification Report — clarify-transport-reqwest-boundary (#270)

- Date: 2026-06-29
- verify_mode: full（10 tasks / 1 capability / 20 实现文件）
- 分支: feature/20260629/clarify-transport-reqwest-boundary
- base-ref: ee2a7a8b8 → HEAD

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | 10/10 tasks ✅；5/5 Requirements 实现；14/14 Scenarios 覆盖 |
| Correctness | 5/5 Requirements 实现匹配意图；无偏离 |
| Coherence | Design 4 项决策全部遵循；守卫脚本风格对齐既有 hygiene 先例 |

**最终评估：All checks passed. Ready for archive.**（0 CRITICAL / 0 WARNING / 1 SUGGESTION）

---

## Completeness

**任务完成**：tasks.md 0 未勾选 / 10 已勾选；plan 33 步骤全勾选。

**Spec 覆盖**（delta spec 5 Requirement / 14 Scenario → 实现证据）：

| Requirement / Scenario | 证据 |
|---|---|
| R1 业务 crate 不直接依赖 reqwest |  |
| - 12 crate Cargo.toml 不含 reqwest | grep `^[[:space:]]*reqwest[[:space:]]*=` 全 0 ✅ |
| - 业务 crate 源码不出现 reqwest 类型 | grep 12 crate `src/` 全 0 ✅ |
| - 三个例外 crate 仍可声明 reqwest | core/client/webhook Cargo.toml 各保留 1 行 ✅ |
| R2 Transport 边界显式文档化 |  |
| - ARCHITECTURE.md 含边界约定 | 「Transport HTTP 边界」小节存在 ✅ |
| R3 webhook by-design 例外文档化 |  |
| - ARCHITECTURE.md 记录 webhook 例外 | 「webhook by-design 例外」段存在 ✅ |
| - webhook crate 文档注释说明例外 | send.rs `shared_client` + client.rs `WebhookClient` 注释补 ARCHITECTURE.md 交叉引用 ✅ |
| R4 Transport 边界由守卫脚本机器检验 |  |
| - 守卫脚本存在 | `tools/check_reqwest_boundary.sh` 可执行 + `set -euo pipefail` + 白名单 ✅ |
| - 清理后守卫通过 | `bash tools/check_reqwest_boundary.sh` exit 0 ✅ |
| - 守卫能抓回归 | Task 2 验证：注入 mail → exit 1 列违规 ✅ |
| - 守卫接入 CI 与 justfile | justfile `reqwest-boundary` recipe + ci.yml lint job 步存在 ✅ |
| R5 清理不破坏构建/lint/测试 |  |
| - 全 feature 构建通过 | `cargo build --workspace --all-features` exit 0（verify 阶段新鲜证据）✅ |
| - 三组 feature clippy 通过 | Task 7：default/all/no-default + `-D warnings` 全 exit 0 ✅ |
| - 测试通过 | Task 7：`cargo test --workspace` 0 failed ✅ |

## Correctness

- **R1 实现匹配意图**：12 业务 crate 的 reqwest 依赖声明 + cargo-machete ignored 列表 + auth oauth feature（`["reqwest","url"]`→`["url"]`）全部清理，core/client/webhook 例外保留。auth 的 url optional dep 仍声明，oauth feature 无悬空引用。
- **R4 守卫脚本正确**：白名单 {core,client,webhook} 精确枚举；`continue 2` 正确跳过白名单；grep 模式 `^[[:space:]]*reqwest[[:space:]]*=` POSIX 可移植、不误匹配注释/数组项；nullglob 硬化防空 glob 噪音；exit 1/0 语义正确（code review 已验证 happy + regression + 白名单绕过）。
- **R5 非破坏性**：业务 crate 无 re-export reqwest，删依赖不改公开 API。Cargo.lock 同步移除 12 处 crate 依赖列表的 reqwest 条目。

## Coherence

**Design 4 项决策全部遵循**：
1. 选 C（清理+文档化）非 A（trait+中间件）✅
2. webhook 例外保留并文档化（#214）✅
3. 中间件层标注 future 不实现 ✅
4. 边界写入 ARCHITECTURE.md ✅

**Code Pattern 一致性**：`tools/check_reqwest_boundary.sh` 风格对齐既有 `tools/check_no_dead_code_allows.sh`（shebang + set -euo pipefail + 头注释 + grep + 白名单 + ✅/❌）；justfile recipe 与 ci.yml step 平行于 `no-dead-code-allows`。

## Issues

### CRITICAL
无。

### WARNING
无。

### SUGGESTION
1. **delta spec 可补 auth oauth + cargo-machete ignore 的 Scenario**：这两项是 R1「Cargo.toml 不含 reqwest」的实现细节（已在 tasks/plan/design 记录），但 delta spec 未给独立验收 Scenario。当前 R1 的 grep Scenario 已覆盖（`oauth = ["reqwest",...]` 也会被 `reqwest` 字符串 grep 命中）。归档时可选择性补充，非阻塞。

## 验证命令清单（新鲜证据）

- `cargo build --workspace --all-features` → exit 0（verify 阶段）
- `bash tools/check_reqwest_boundary.sh` → exit 0 + ✅（verify 阶段）
- grep 双重确认（A/B/C/D/E/F 六组）→ 全 ✅（verify 阶段）
- `cargo clippy --workspace --all-targets` × 3 feature（`-D warnings`）→ 全 exit 0（Task 7，同源码状态）
- `cargo test --workspace` → 0 failed（Task 7，同源码状态）

注：verify 阶段无 .rs 业务逻辑改动（仅守卫脚本 nullglob + Cargo.lock + 文档/状态），Task 7 的 clippy/test 在相同源码提交运行，结果有效。
