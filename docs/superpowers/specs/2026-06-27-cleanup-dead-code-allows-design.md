---
comet_change: cleanup-dead-code-allows
role: technical-design
canonical_spec: openspec
---

# Design — cleanup-dead-code-allows

> OpenSpec delta spec `dead-code-lint-hygiene` 是事实源（canonical）。本文档为技术设计。

## 背景与根因

全 workspace 392 个 `#[allow(dead_code)]` 淹没 dead_code lint 信号。实证（移除全部后 `cargo check --workspace`）：**389 cruft**（字段在 `execute()` 中已读）+ **3 真死字段**（platform v1 入口 struct `config`）。

3 真死字段根因：`AdminV1`/`ApaasV1`/`DirectoryV1` 存 `config` 但**无访问器**——其子模块（`badge`/`approval_task`/`department`…）只是操作集合，**无 service 入口类型**（与 `SparkV1` 不同，spark 子模块有 `SparkAppService` 等）。故 config 无消费者，仅 `#[cfg(test)]` 测试读取。

## 决策

**D1（389 cruft）**：批量移除 `#[allow(dead_code)]`，已验证 0 warning。机械、安全。

**D2（3 真死字段）→ 方案 C（拆分）**：
- 本 change：3 个字段 `config` → `_config` + 注释「reserved：待装访问器（见 #274）」；同步 3 个测试（`api.config` → `api._config`）。
- 为何不 A-full：admin/apaas/directory 子模块无 service 类型，补访问器需先建 24 个 service（中-大工作量，超出"清理"范畴）→ 拆至 issue #274。
- C 的合理性：`_` 前缀是 Rust 惯用的"有意未用"标记，符合 spec「显式 `_` 前缀 + 注释」；前向兼容（#274 改回 `config` + 装访问器）。

**D3（防复发）**：加 CI grep 检查——禁止非测试代码出现 `#[allow(dead_code)]`（在 `.github/workflows` 或 justfile 加一步 grep，命中即失败）。

## 改动清单

| 范围 | 动作 |
|------|------|
| `crates/`+`src/` 381 文件 | 移除 389 处 `#[allow(dead_code)]`（cruft） |
| `admin/admin/v1/mod.rs`、`app_engine/apaas/v1/mod.rs`、`directory/directory/v1/mod.rs` | `config` → `_config` + 注释（3 处）；同步 3 个测试 |
| `.github/workflows` 或 justfile | 加 CI grep 检查防复发（D3） |
| `CHANGELOG.md` | `[Unreleased]` 记录 |

不动：codegen 工具（不 emit dead_code）、request builder 结构、public API（389 删除零影响；3 处 `_config` 是私有字段改名，非 API 变化）。

## 风险与缓解

- **[批量删除误删]** → final clippy 三组 feature（default/full/no-default）`-D warnings` 复核。
- **[`_config` 被误清理]** → 注释明确标注 reserved + 关联 #274。
- **[CI grep 误报]** → grep 范围排除 `#[cfg(test)]` 测试代码与 `tests/` 目录。

## 测试策略

1. `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
3. `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
4. `cargo test --workspace` 通过
5. CI grep 检查：非测试代码 `#[allow(dead_code)]` 命中数 = 0

## 迁移与回滚

纯删除 + 私有字段改名 + CI 脚本，无 API/数据迁移；`git revert` 即可。389 删除无行为影响；3 处 `_config` 是私有字段（不改对外签名）。

## 关联 issue

- #267（dead_code allows 原始，本 change 闭环）
- #274（platform v1 导航补全，A-full 拆分至此）
