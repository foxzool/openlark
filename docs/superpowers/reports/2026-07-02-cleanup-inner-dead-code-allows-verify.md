# 验证报告：cleanup-inner-dead-code-allows (#277)

- **Date**: 2026-07-02
- **Change**: cleanup-inner-dead-code-allows（phase=verify, verify_mode=full, workflow=full）
- **Branch**: feature/20260702/cleanup-inner-dead-code-allows
- **base-ref**: 8aedb2de3 → HEAD: 1845583a1（12 commits）
- **规模**: 26 tasks / 1 delta capability（dead-code-lint-hygiene MODIFIED）/ 35 文件，+1294/−3131

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | tasks.md 26/26 ✓，plan 44/44 step ✓；1 capability 1 requirement 4 scenario |
| Correctness | 4/4 scenario 被 fresh 证据满足；0 CRITICAL/WARNING |
| Coherence | design.md D2 / Design Doc §2 已记录 build 实测修正，无 spec 漂移 |

## Fresh 验证证据（本阶段实跑，非复述）

| # | 检查 | 命令 | 结果 |
|---|------|------|------|
| 1 | 格式 | `cargo fmt --all --check` | exit 0，无 diff ✓ |
| 2 | lint default | `cargo clippy --workspace --all-targets` | 0 dead_code，0 warning ✓ |
| 3 | lint --all-features | `cargo clippy --workspace --all-targets --all-features` | 0 dead_code，0 warning ✓ |
| 4 | lint --no-default-features | `cargo clippy --workspace --all-targets --no-default-features` | 0 dead_code，0 warning ✓ |
| 5 | 测试 | `cargo test --workspace` | **6241 passed / 0 failed** ✓ |
| 6 | CI 守卫 | `bash tools/check_no_dead_code_allows.sh` | exit 0；KNOWN_INNER_DEBT 空 ✓ |
| 7 | lockfile 一致 | `cargo check --workspace --locked` | exit 0（msrv lockfile 与 manifest 精确匹配）✓ |
| 8 | inner allow 残留 | grep `#!\[allow(dead_code)\]` 非测试 | 0 残留 ✓ |

## Scenario 覆盖映射（delta spec `dead-code-lint-hygiene`）

1. **HR crate 内外层均无残留** → 证据 #8（workspace grep 0，含 hr）✓
2. **全 workspace 内外层均无 cruft 残留** → 证据 #8（0 残留；`User.config`/`service.config` 为 `#[expect]]`/`cfg_attr` 显式标注项，符合 scenario「仅保留带显式注释的 expect 项」）✓
3. **CI 死代码守卫无人为开口** → 证据 #6（KNOWN_INNER_DEBT 清空）✓
4. **废弃模块与 0 引用脚手架被删除而非抑制** → 证据 #2-4（clippy×3 0 dead_code）+ 提交实证（hr endpoints 整模块删、observability 死项删留 ResponseTracker、query_params 整文件删、add_headers 删）✓

## Coherence：build 期 design 修正（已回写，非漂移）

build Task 2 实测发现 design D2 两处误判，已在 build 阶段同步回写：
- **observability.rs 非全文件死** → `response_handler` 用 `ResponseTracker`。重写 observability.rs 仅留 ResponseTracker + 4 测试，删死 tracker/trace 函数/5 宏/feature 门控块。已回写 design.md D2 + Design Doc §2。
- **根 Cargo.toml 有 `otel=["openlark-core/otel"]` 转发 feature**（Task 1 `crates/`-only grep 漏检）→ 补删。已回写 tasks.md 1.1 注记。
- **`testing` feature 解耦保留**（hr/docs 测试大量用 `pub mod testing`）→ `testing = []`。design 已记录。
- **mail/bot `service.rs::config` nodef 条件死代码**（Task 5/6 删 blanket allow 后暴露）→ `cfg_attr(not(feature), expect(dead_code))` 标注。

delta spec 范围未变（仍是 dead-code-lint-hygiene 单 capability MODIFIED）；scenario 4 措辞在 verify 阶段精化（反映 observability 留 ResponseTracker、条件死字段用 expect）以保归档后 main spec 准确。

## 代码审查（review_mode=standard，build 阶段已执行）

- 1 Important（`User.config` 无条件 `#[expect]]` 疑在 default 组 unfulfilled）→ 经 default 组 fresh clippy 实证驳回（User 无 accessor，config 恒死，expect fulfilled，0 warning）。
- 3 Minor/Recommendation（tasks 补勾 3 项、Task 1.1 grep 注记、CHANGELOG 补根 otel feature）→ 已修。

## 最终评估

**0 CRITICAL / 0 WARNING / 0 SUGGESTION 待处理。**

所有验收场景被 fresh 证据满足；design build 修正已回写无漂移；CI 守卫、fmt、clippy×3、test、build×2、--locked、CI script 全绿。**Ready for archive.**

## 后续：CI 实跑补丁（verify 后发现）

PR #299 推送后 GitHub Actions 实跑暴露一项本地验证漏检：`feature-combinations (otel)` job fail——移除 `otel` feature 后，CI matrix 仍含 `- "otel"` 项（`cargo build --features otel` → unknown feature）。本地 clippy×3 未覆盖 CI feature-matrix 组合，故漏。

已修（commit + push 到 PR #299）：
- `.github/workflows/feature-matrix.yml`：matrix 删 `- "otel"` 项
- `justfile`：`test-features` 的 `--exclude-features websocket,otel` → `websocket`

**教训（记入 MEMORY 候选）**：移除 Cargo feature 须同步清所有引用面——CI workflow matrix、justfile `--exclude-features`、任何 `--features` 脚本——不只 Cargo.toml。验证时除 clippy×3 外，应 grep CI 配置里的 feature 名。

修后重跑 CI 确认 `feature-combinations (otel)` 项消失、其余全绿（见下 CI watch 结果）。

