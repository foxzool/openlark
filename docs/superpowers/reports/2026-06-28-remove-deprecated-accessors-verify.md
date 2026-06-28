# 验证报告 — remove-deprecated-accessors

- Change: remove-deprecated-accessors
- Date: 2026-06-28
- 分支: feature/20260628/remove-deprecated-accessors
- base-ref: e9c4b0267bd69ffdbf400524fd440c9a2b755b31
- verify_mode: full（6 文件）

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | tasks.md 11/11 完成；delta spec `no-deprecated-compat-accessors` 3 需求全覆盖 |
| Correctness | 3 需求满足；spec scenarios 由 fresh 证据覆盖 |
| Coherence | 与 Design Doc（D1 删除 / D2 保留类型）一致 |

## Fresh 验证证据（本阶段重跑）

| 检查 | 命令 | 结果 |
|------|------|------|
| default clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Finished，0 warning |
| all-features clippy | `--all-features` | Finished，0 warning |
| test | `cargo test --workspace` | 0 failed |
| HR 目标 deprecated | grep 8 个 fn | 0（全删） |
| analytics 目标 | grep query/user | 0（全删） |
| examples 引用 | grep `.attendance()`/`.query()` | 0 |

## 需求对照（delta spec `no-deprecated-compat-accessors`）

1. **HR 不暴露 deprecated 访问器** ✓ — HR lib.rs 的 attendance/corehr/.../ehr 8 个 fn 删除；字段仍 `pub`（字段访问替代，test 已迁移）。
2. **analytics SearchV2 不暴露 deprecated 存根** ✓ — query()/user() 删除；`QueryApi`/`UserSearchApi` 类型 + `v2/query.rs`/`v2/user.rs` 模块保留（D2）。
3. **移除不破坏构建与测试** ✓ — 三组 feature clippy 0 warning；`cargo test --workspace` 0 failed；examples 不引用。

## Coherence 检查

- Design Doc D1（删除 10 fn）/ D2（保留 QueryApi/UserSearchApi 类型）逐项落实。
- **build 发现并修复**：HR lib test 用了 `client.attendance()`（迁移为 `.attendance.clone()`）；analytics 移除存根后 `SearchV2.config` 变 dead（改 `_config`）。两者均在 D1/D2 精神内（移除 deprecated + 处理连带 dead code），无 spec 矛盾、无方案变更。
- code review 通过（Minor #1 注释错误 #275 引用已修；#2 CHANGELOG user() 说明已增强；#3 _config 保留 acceptable）。

## 问题

- CRITICAL：无
- WARNING：无
- SUGGESTION：review Minor #3（SearchV2._config 字段当前无用）—— 若 #275/#276 不补访问器，未来可简化为 `pub struct SearchV2;`。当前保留 + 注释合理。

## 最终评估

**全部检查通过，Ready for archive。** 移除 10 个 deprecated 兼容访问器（HR 8 → 字段访问；analytics 2 存根移除 + config→_config），BREAKING，CHANGELOG 附迁移映射。三组 clippy/test 全绿，review 通过。B/C/D/F/G 10 项拆至 #278。
