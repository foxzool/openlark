## Why

openlark-platform 的四个 service（admin、app_engine、directory、spark）在 `default` 和 `full` feature 构建下是**空壳**：service 结构体只有 `new()` / `config()`，唯一的 API 入口 `pub fn v1()` 及其实现子树（`admin/admin`、`app_engine/apaas`、`directory/directory`、`spark/spark`，共 ~163 文件、96 个 API）全部挂在 `#[cfg(feature = "v1")]` 后面，而 `v1` 既不在 `default` 也不在 `full`（`full` 只含 `v4`，且 `v4` 不门控任何代码）。根因是 commit `995066e5a` 为通过 clippy 给 facade 加门控时的**意外后果**——这让平台 96 个真实 API 对用户彻底不可达。需要让这些 API 在标准 feature 组合下可达。

## What Changes

- **修正 platform facade 的 v1 门控**：让 admin / app_engine / directory / spark 四个 service 的 API 实现在标准 feature 组合下可达（具体修法——移除门控 vs 把 `v1` 纳入 `default`/`full`——在 design 阶段 brainstorm 确定）。
- 处置 `full` 中不门控任何代码的空 `v4` feature，并在 design 阶段明确 `v1`/`v2`/`v3`/`v4` 的语义。
- **保留** `v1`/`v2`/`v3`/`v4` feature flag 本身（测试 clippy 门控仍依赖，见 `docs/superpowers/plans/2026-06-25-test-feature-gating.md`），只动 facade 上的门控点。

## Capabilities

### New Capabilities
- `platform-service-access`: openlark-platform 的 admin / app_engine / directory / spark 四个 service SHALL 在标准 feature 组合（至少 `full`，目标含 `default`）下暴露其完整 API 实现，不再是无 API 的空壳。

### Modified Capabilities
<!-- 无现有 platform spec 需要修改 -->

## Impact

- **openlark-platform**: `src/admin.rs`、`src/app_engine.rs`、`src/directory/mod.rs`、`src/spark/mod.rs` 四处 facade 的 `#[cfg(feature = "v1")]` 门控点；`Cargo.toml` `[features]` 的 `default` / `full` / `v1` / `v2` / `v3` / `v4` 定义。
- **编译影响**: 若把 API 纳入 `default`，默认构建会多编译 ~163 文件（构建时间上升）；若仅纳入 `full`，影响仅限 full 构建。取舍在 design 阶段定。
- **测试约束**: 必须保持 `cargo clippy --workspace --all-targets --no-default-features -D warnings` 与 `--all-features` 均 exit 0（不破坏现有 test-gating），4 个 service 现有单元测试仍通过。
- **非目标**: 不重构双层目录（`admin/admin` 等）；不改其他 crate 的 feature 方案；不新增或删除 API。
- **兼容性**: 让原本不可达的 API 变可达，不移除任何公开符号——视为行为补全，非 breaking。
