# 验证报告 — cleanup-dead-code-allows

- Change: cleanup-dead-code-allows
- Date: 2026-06-28
- 分支: feature/20260627/cleanup-dead-code-allows
- base-ref: e4cfc63748279a4178bf14d33f83f539cff4681b
- verify_mode: full（388 文件）

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | tasks.md 17/17 完成；delta spec `dead-code-lint-hygiene` 需求全覆盖 |
| Correctness | 4 需求全部满足；spec scenarios 由 fresh 证据覆盖 |
| Coherence | 与 Design Doc（D2=`_config`、D3=CI grep）一致；范围漂移见下方「实现偏差」 |

## Fresh 验证证据（本阶段重跑）

| 检查 | 命令 | 结果 |
|------|------|------|
| default clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Finished，0 warning |
| all-features clippy | `--all-features` | Finished，0 warning |
| no-default clippy | `--no-default-features` | Finished，0 warning |
| test | `cargo test --workspace` | 0 failed |
| 外层 allow grep | `grep '#\[allow(dead_code)\]' crates/ src/` | 0 命中 |
| 防复发脚本 | `bash tools/check_no_dead_code_allows.sh` | ✅（7 inner 已登记 #277） |

## 需求对照（delta spec `dead-code-lint-hygiene`）

1. **不用 allow 掩盖** ✓ — 全 workspace 外层 `#[allow(dead_code)]` = 0。7 处 inner-attribute `#![allow(dead_code)]`（掩盖 ~109 处死代码）未在本 change 清理，已登记为 CI 脚本 KNOWN_INNER_DEBT 并拆至 #277（code review I1）。
2. **真死字段必须修正** ✓ — ~16 个死 `config` 字段改 `_config` + reserved 注释（关联 #274/#275/#276）；2 个 pub 死函数 + docs 测试 helper 用 `#[expect(dead_code)]`。
3. **lint 信号保持有效** ✓ — 三组 feature clippy `-D warnings` 全 exit 0；CI 脚本 catch 外层+新内层 allow。
4. **测试不回归** ✓ — `cargo test --workspace` 0 failed。

## 实现偏差（Implementation Divergence）

Design Doc D2 原述「3 个 platform v1 config 字段 → `_config`」。实证后实际处理 **~24 处**死代码（跨 platform v1 / ai / analytics / user / helpdesk / docs / application），原因：移除 allow 后不同 feature 组合（default / --all-features / --no-default-features）暴露的死字段远超 design 阶段（仅 default）所测的 3 个。处理方式与 D2 一致（`_config` + 注释 / `#[expect]]`），偏差仅为「数量与覆盖 crate 更多」，方向与方案不变。完整范围见 tasks.md 与各 commit。

## Coherence 检查

- Design Doc D2（`_config`）/ D3（CI grep）逐项落实。
- 非 breaking 论证成立（code review 确认 `_config` 均为私有字段）。
- code review I1（inner-attribute 正则盲区）已修；M1–M3 minor 接受（M2 已在脚本注释，M1/M3 非必需）。

## 问题

- CRITICAL：无
- WARNING：无
- SUGGESTION：M1（application/helpdesk 可改 `cfg_attr(not(feature="v1"), expect)`）留作后续优化

## 最终评估

**全部检查通过，Ready for archive。** 388 文件改动：376 cruft 机械删除 + ~16 `_config` + 2 `#[expect]]` + CI 防复发脚本（catch inner+outer）。三组 clippy/test 全绿，零外层 allow。补全工作拆至 #274 / #275 / #276 / #277。
