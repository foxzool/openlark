# CI Test-Target Coverage（#228 审核链路闭环记录）

> 日期：2026-06-25 ｜ 相关 issue：#228 #246 #248 #250 #251 ｜ 相关 PR：#247 #249 #252 #253 #254

## 背景

对 `#228`（event 模块是否应经统一 client re-export）的设计审查，陆续牵出三类问题：feature gate 缺失、测试门控系统性缺口、CI lint 覆盖漏洞。逐一查实并修复后形成本记录。

## 决策

1. **event 不经统一 client re-export**（#228）：event 在项目优先级模型（`tools/api_priority.toml`）中为 **P2**（非 P0 核心业务），仅在 `openlark-communication` 暴露是正确的定位；按**基础设施类**对待（与 WebSocket `LarkWsClient` 同类，不进 `declare_client!` 资源客户端注册表）。

2. **CI clippy 统一用 `--all-targets`**（#250 / #254）：`all-features`、`no-default-features`、各 feature 组合三个维度的 clippy 都覆盖 test/bench target。这样 `#[cfg(test)]` 类 lint 回归（如 #248 的未用 `use super::*`）会被 CI 直接拦住，不再只依赖本地 `just lint`。

3. **测试 / 示例 feature 门控约定**（#251）：
   - `tests/*.rs` 引用 feature-gated 模块时，文件顶部加 `#![cfg(feature = "...")]`，且 **`//!` 文档必须在 `#![cfg]` 之上**（顺序反了会触发 clippy `missing_docs`）。
   - `examples/*.rs` 用 `Cargo.toml` 的 `[[example]] required-features`（**不要**用 `#![cfg]`——会把 example 清空成无 `main`，报 `E0601`）。

## 修复落地

| Issue | PR | 内容 |
|-------|----|------|
| #246 | #247 | event 模块补 feature gate（修复无条件编译不一致） |
| #248 | #249 | 删 11 处 `#[cfg(test)]` 未用 `use super::*`（codegen 旧模板遗留） |
| #250 | #252 | CI lint job（all-features）clippy 升 `--all-targets` |
| #251 | #253 | 27 个 test/example feature 门控修复（解锁 #250 deferred） |
| — | #254 | CI no-default-features + feature-combinations 矩阵 clippy 升 `--all-targets`（#250 deferred 收尾） |

## 结果

- CI 在 **all-features / no-default-features / 各 feature 组合** 三个维度都覆盖 test target。
- `#228` 审核衍生的全部 issue 已关闭。
- codegen 现行测试模板（`tools/api_contracts/codegen_render.py:_tests()`）已 lint-safe（`#[allow(unused_imports)]` + `//!` 在 cfg 之上），重生不会复发 #248 类问题。
