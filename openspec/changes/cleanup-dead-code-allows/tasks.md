# Tasks — cleanup-dead-code-allows

> 范围：移除全 392 处 `#[allow(dead_code)]` = 389 cruft 删除 + 3 真死字段最小修（`_config`）。关联 issue #267；导航补全拆至 #274。D2/D3 已 design 确认。

## 1. 调研（build 前）

- [x] 1.1 读 3 个 platform v1 `mod.rs` 判明 config 字段无访问器（子模块无 service 类型）→ D2 定方案 C
- [x] 1.2 确认移除 389 cruft 后三组 feature均 0 warning（workspace 实证：仅 3 真死字段）

## 2. 移除 389 处 cruft（机械）

- [ ] 2.1 批量移除 `crates/`+`src/` 下全部 `#[allow(dead_code)]` 行（381 文件，389 处）
- [ ] 2.2 保留 3 个 platform v1 字段的处理给 Task 3（它们的 allow 删除后由 `_config` 改名消除 warning）

## 3. 修正 3 个真死字段（方案 C：`_config` + 注释）

- [ ] 3.1 `admin/admin/v1/mod.rs`：`config` → `_config` + 注释「reserved：待装访问器（见 #274）」；测试 `api.config` → `api._config`
- [ ] 3.2 `app_engine/apaas/v1/mod.rs`：同上
- [ ] 3.3 `directory/directory/v1/mod.rs`：同上

## 4. 验证

- [ ] 4.1 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [ ] 4.3 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [ ] 4.4 `cargo test --workspace` 通过

## 5. 防复发与收尾

- [ ] 5.1 D3：加 CI grep 检查（`.github/workflows` 或 justfile）——禁止非测试代码出现 `#[allow(dead_code)]`
- [ ] 5.2 CHANGELOG `[Unreleased] > Changed/Fixed` 记录
- [ ] 5.3 关闭 issue #267（归档后）；#274（导航补全）保持 open 待后续 change
