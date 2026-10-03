---
comet_change: clarify-transport-reqwest-boundary
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-29-clarify-transport-reqwest-boundary
status: final
---

# Design — clarify-transport-reqwest-boundary

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/clarify-transport-reqwest-boundary/specs/transport-reqwest-boundary/spec.md` 为 canonical。

## 1. 背景与目标

issue #270（2026-06 架构审计）原判「14 个业务 crate 直接依赖 reqwest，存在分层泄漏与配置不一致风险」。open 阶段实证核实**推翻该前提**：业务 crate 早已经 core `Transport<T>` 收口、源码层不碰 reqwest。真正遗留是 12 个未用依赖声明 + webhook 例外未文档化 + 中间件层是未实现草案。

**目标**：闭环 #270——清理未用依赖 + 把已存在且在工作的 `Transport<T>` 边界显式文档化 + 加防回归守卫（匹配项目 hygiene 既定模式）。

**非目标**：不升级 `Transport` 为 trait、不实现中间件链、不改造 webhook、不改 core 实现、不改任何公开 API。

## 2. 关键技术验证（design 阶段实证）

| 断言 | 验证方式 | 结论 |
|------|---------|------|
| 业务 crate 源码不碰 reqwest | 全仓 grep `reqwest`（src/，逐 crate 计数） | ✅ core 62 / webhook 10（有意）/ client 3；12 业务 crate 全 0 |
| 调用路径经 core Transport | 读 `communication/.../role/list.rs` | ✅ `use openlark_core::http::Transport` → `Transport::request(req,&config,option)` |
| 12 依赖全部在 `[dependencies]` | 逐 crate 查 reqwest 行 + 上方最近 section 头 | ✅ 全部 `[dependencies]`，无 dev/build 残留 → 每 crate 删 1 行 |
| tests/examples/build.rs 不依赖 | grep 业务 crate 的 tests/examples + find build.rs | ✅ 全无 reqwest 引用 |
| 项目有 hygiene 守卫先例 | `ls tools/` + 读 `check_no_dead_code_allows.sh` | ✅ `check_no_dead_code_allows.sh`（shell + set -euo + grep + allowlist + ✅/❌）|
| 守卫接线点 | grep justfile + ci.yml | ✅ justfile:17-19 `no-dead-code-allows` recipe；ci.yml:115-116 lint job 一步 |
| workspace reqwest features 集中管理 | root Cargo.toml:90 | ✅ features 在 root 声明，删业务 crate 声明不影响 feature 合并 |
| cargo-machete 假阴性 | `cargo machete` vs 实证 | ⚠️ machete 报"无未用依赖"（不遍历 crates/）；实证 12 crate 确为未用 |

## 3. 实现步骤

### 3.1 依赖清理（12 个 Cargo.toml）
对 `analytics / auth / bot / application / communication / mail / hr / docs / helpdesk / platform / user / workflow` 各删 `[dependencies]` 下的 `reqwest = { workspace = true }` 一行。保留 `core / client / webhook`。

### 3.2 防回归守卫脚本
新增 `tools/check_reqwest_boundary.sh`（风格对齐 `check_no_dead_code_allows.sh`）：

```bash
#!/usr/bin/env bash
# 检查业务 crate 的 Cargo.toml 不直接声明 reqwest 依赖（issue #270 边界防复发）。
# 业务 crate 须经 core Transport<T> 发请求。例外白名单：core/client/webhook。
# 被 justfile (just reqwest-boundary) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail

ALLOW=(openlark-core openlark-client openlark-webhook)
hits=""
for toml in crates/*/Cargo.toml; do
  crate=$(basename "$(dirname "$toml")")
  # 跳过白名单
  for a in "${ALLOW[@]}"; do [ "$crate" = "$a" ] && continue 2; done
  # 业务 crate 的 [dependencies]/[dev-dependencies] 出现 reqwest 即违规
  if grep -qE '^\s*reqwest\s*=' "$toml"; then
    hits="$hits\n$toml"
  fi
done

if [ -n "$hits" ]; then
  echo "❌ 业务 crate 直接声明了 reqwest 依赖（须经 core Transport 发请求）：" >&2
  echo -e "$hits" >&2
  exit 1
fi
echo "✅ 业务 crate Cargo.toml 无 reqwest 直接依赖（边界由 core Transport 收口）"
```

### 3.3 守卫接线
- **justfile**：在 `no-dead-code-allows`（line 17-19）后加平行 recipe `reqwest-boundary` → `@bash tools/check_reqwest_boundary.sh`。
- **ci.yml**：在 lint job「Check no #[allow(dead_code)]」步（line 115-116）后加一步「Check Transport/reqwest boundary」→ `bash tools/check_reqwest_boundary.sh`。

### 3.4 ARCHITECTURE.md 边界文档化
在 `## 模块详细设计`（line 106）下新增 `### Transport HTTP 边界` 小节：
- **边界定义**：`openlark_core::http::Transport<T>` 是唯一 HTTP 出口。
- **调用路径**：`*Request::execute() → Transport::request(req,&config,option) → ReqTranslator → reqwest::RequestBuilder`（仅 core 碰 reqwest）。
- **约定**：业务 crate 不在 Cargo.toml 声明 reqwest、不在源码用 reqwest 类型；由 `tools/check_reqwest_boundary.sh` 机器检验。
- **webhook 例外**：无鉴权推送器，进程级共享 `reqwest::Client` 复用连接池（#214），by-design。
- **中间件层**：熔断/重试中间件草案标为「规划中 future、当前未实现」（实际为 RetryPolicy 配置模式）。

### 3.5 webhook 文档注释强化
`crates/openlark-webhook/src/robot/v1/send.rs` 的 `shared_client()` 与 `client.rs` 的相关注释补一句「直接 reqwest 是 Transport 边界的 by-design 例外，见 ARCHITECTURE.md」（不改源码逻辑）。

### 3.6 CHANGELOG
v0.18 段加 hygiene 条目：「移除 12 个业务 crate 未用的 reqwest 依赖声明；新增 `tools/check_reqwest_boundary.sh` 边界守卫（非 breaking）」。

## 4. 测试策略

- `cargo build --workspace --all-features` exit 0
- 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- `cargo test --workspace` 全部通过（0 failed）
- `bash tools/check_reqwest_boundary.sh` exit 0（清理后通过）
- grep 双重确认：12 crate 的 Cargo.toml + src `reqwest` 命中均 0；core/client/webhook Cargo.toml 仍保留

## 5. 风险与回滚

- **删依赖致 build 失败** → 已核实 0 引用 + workspace features 集中管理；build 实证兜底。回滚 = revert。
- **守卫误报** → 白名单精确仅 core/client/webhook。
- **下游隐式依赖传递 reqwest** → 业务 crate 无 re-export；CHANGELOG 记录兜底。
- 非破坏性：不改公开 API。部署 = 合并；回滚 = revert 单 commit。
