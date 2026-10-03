# Verification Report — unify-auth-request-naming（#271 pilot）

- Date: 2026-06-29
- verify_mode: full（9 tasks / 1 capability / 25 实现文件）
- 分支: feature/20260629/unify-auth-request-naming
- base-ref: 6557def91 → HEAD

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | tasks.md 全勾选；plan 全勾选；1 capability 实现 |
| Correctness | 3 Requirement / 9 Scenario 全覆盖；alias 机制实证 |
| Coherence | 方向 pivot（Request→RequestBuilder）已记 design/spec/proposal/plan/CHANGELOG |

**最终评估：Ready for archive.**（0 CRITICAL / 0 WARNING / 0 SUGGESTION）

## Completeness

tasks.md 0 未勾选；plan 全勾选。delta spec `auth-request-naming` 3 Requirement 实现。

## Correctness（spec scenario → 新鲜证据，verify 阶段当轮重跑）

| Requirement / Scenario | 证据（当轮新鲜） |
|---|---|
| R1 统一 RequestBuilder 后缀 |  |
| - 12 重命名为 RequestBuilder（+1 已有） | grep `^pub struct XxxRequestBuilder` = **13** ✅ |
| - 不再有裸 Builder 请求类型 | grep 旧 `XxxBuilder` struct 残留 = **0** ✅ |
| - AuthorizationUrlBuilder 不动 | 仍是 `pub struct`（1 命中）✅ |
| R2 旧名 #[deprecated] alias |  |
| - 12 alias 存在且标 deprecated | grep `^pub type XxxBuilder = XxxRequestBuilder;` = **12** ✅ |
| - 旧名调用产生 warning | `test_app_access_token_legacy_alias_still_callable` 用 `#[allow(deprecated)]`（证明 alias 触发 deprecated lint）+ /tmp spike 实证 ✅ |
| - 新名无 warning | `test_app_access_token_new_name_no_deprecation` 通过 ✅ |
| R3 不破坏 build/clippy/test |  |
| - 全 feature 构建 | `cargo build --workspace --all-features` Finished exit 0 ✅ |
| - 三组 clippy | default/all-features/no-default + `-D warnings` 均 Finished exit 0 ✅ |
| - auth 测试 | `cargo test -p openlark-auth` → 120+16+10 passed, **0 failed** ✅ |

## Coherence

**方向 pivot 已全产物同步**：build 阶段发现 5/13 目标 `XxxRequest` 撞 `crate::models` body 模型（E0255），用户确认 Request→RequestBuilder。pivot 已记入：proposal（What Changes note）、design.md（决策 1）、Design Doc（顶部 pivot note）、delta spec（方向变更说明）、plan（PIVOT 权威 note）、CHANGELOG（方向说明）。无 spec/design 矛盾。

## Issues

### CRITICAL / WARNING / SUGGESTION
无。

### 代码审查（review_mode: standard）
Ready to merge: Yes（0 Critical/Important，3 Minor 接受：alias note v1.0 保留因 issue 明示 + 活跃重命名宜长窗口；2 处注释措辞 cosmetic）。

## 验证命令（新鲜证据，verify 阶段当轮）

- `cargo build --workspace --all-features` → Finished exit 0
- `cargo clippy --workspace --all-targets` × 3 feature（`-D warnings`）→ 均 Finished exit 0
- `cargo test -p openlark-auth` → 120+16+10 passed, 0 failed
- grep：13 RequestBuilder struct / 12 deprecated alias / AuthorizationUrlBuilder 未动 / 0 旧 struct 残留
