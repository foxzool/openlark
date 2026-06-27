# Brainstorm Summary

- Change: cleanup-dead-code-allows
- Date: 2026-06-27

## 确认的技术方案

**拆分（已用户确认）**：本 change 做"清理 dead_code allows"，最小修 3 个真死字段；platform v1 导航补全（A-full）另开 issue #274。

- **389 cruft**：批量移除 `#[allow(dead_code)]`，机械、已验证 0 warning。
- **D2（3 个 platform v1 config 真死字段）→ 方案 C**：字段改名 `config` → `_config` + 注释「reserved：待装访问器（见 #274）」，前向兼容（#274 闭环时改回 `config` + 装访问器）。更新对应 3 个 `#[cfg(test)]` 测试（`api.config` → `api._config`）。
- **D3（防复发）**：加 CI grep 检查，禁止非测试代码出现 `#[allow(dead_code)]`。

## 关键取舍与风险

- **取舍**：为何不在此 change 做 A-full（补访问器）？因为 admin/apaas/directory 子模块**无 service 入口类型**（与 spark 不同，spark 有 SparkAppService 等），A-full 需先建 24 个 service 类型 → 超出"清理"范畴，故拆至 #274。
- **风险①**：389 批量删除需 final clippy 三组 feature（default/full/no-default）复核。
- **风险②**：`_config` 是有意为之的未用字段，需注释说明，避免被误清理。

## 测试策略

- `cargo clippy --workspace --all-targets` 三组（default / --all-features / --no-default-features）+ `-D warnings` 全 exit 0。
- `cargo test --workspace` 通过（389 删除无行为影响；3 个 `_config` 字段测试同步改名）。
- CI grep 检查作为新测试：确认非测试代码无 `#[allow(dead_code)]`。

## Spec Patch

无。delta spec `dead-code-lint-hygiene` 的需求与方案一致（移除 392、3 真死字段修正、lint 信号有效）。D2 选 C（`_config`）属于"修正"的一种合规形式（`_` 前缀 + 注释），符合 spec「真死字段必须修正或显式 `_` 前缀 + 注释」。

## 关联

- 本 change 关联 issue #267（dead_code allows 原始）。
- 导航缺口拆至 issue #274（platform v1 访问器补全）。
