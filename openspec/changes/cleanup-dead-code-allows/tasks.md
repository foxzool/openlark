# Tasks — cleanup-dead-code-allows

> 范围：全 392 处 `#[allow(dead_code)]` = 389 cruft 删除 + 3 真死字段修正。关联 issue [#267](https://github.com/foxzool/openlark/issues/267)。

## 1. 调研（build 前）

- [ ] 1.1 读 `crates/openlark-platform/src/admin/admin/v1/mod.rs`、`app_engine/apaas/v1/mod.rs`、`directory/directory/v1/mod.rs`，判明 `config` 字段是**缺陷**（子 API 未收到 config）还是**冗余**（子 API 已有 config）→ 定 D2 修法
- [ ] 1.2 确认移除 389 cruft 后三组 feature（default/full/no-default）均 0 warning（已 workspace 实证 3 真死字段外全 clean）

## 2. 移除 389 处 cruft（机械）

- [ ] 2.1 批量移除 `crates/`+`src/` 下全部 `#[allow(dead_code)]` 行（381 文件）
- [ ] 2.2 保留 3 个 platform v1 真死字段的处理给 Task 3（先不删它们的 allow，或删后由 Task 3 修复字段）

## 3. 修正 3 个真死字段（platform v1 入口 struct config）

- [ ] 3.1 按 D2 修法修正 `admin/admin/v1/mod.rs` 的 `config` 字段（读取传递 / 移除 / `_` 前缀+注释）
- [ ] 3.2 同上修正 `app_engine/apaas/v1/mod.rs`
- [ ] 3.3 同上修正 `directory/directory/v1/mod.rs`
- [ ] 3.4 若移除字段导致 `XxxV1::new(config)` 签名变化，同步 `AdminService::v1()` 等调用点

## 4. 验证

- [ ] 4.1 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [ ] 4.3 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [ ] 4.4 `cargo test --workspace` 通过

## 5. 防复发与收尾

- [ ] 5.1 D3 决策：是否加防复发约束（CI grep 检查 `#[allow(dead_code)]` / 约定文档）——若加则落地
- [ ] 5.2 CHANGELOG `[Unreleased] > Fixed/Changed` 记录
- [ ] 5.3 关闭 issue #267（归档后）
