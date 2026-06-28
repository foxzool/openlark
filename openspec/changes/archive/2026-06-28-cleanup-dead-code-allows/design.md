## Context

392 个 `#[allow(dead_code)]` 散布全 workspace，淹没 dead_code lint 信号。实证：389 cruft（字段已读）+ 3 真死字段（platform v1 入口 struct `config`）。

```
392 #[allow(dead_code)]
 ├─ 389 cruft（字段在 execute() 中已读）→ 直接删除，0 风险
 └─ 3 真死字段（platform v1 入口 struct 的 config）
     ├─ admin/admin/v1/mod.rs:28      AdminV1.config
     ├─ app_engine/apaas/v1/mod.rs:28 ApaasV1.config
     └─ directory/directory/v1/mod.rs:29 DirectoryV1.config
     （fix-platform-v1-feature-gating ungate 后新暴露）
```

389 个删除是机械操作（已验证移除后 0 warning）。真正的设计点是 3 个 platform v1 `config` 字段如何处理。

## Goals / Non-Goals

**Goals:**
- 移除全部 392 个 `#[allow(dead_code)]`（389 删除 + 3 修正）。
- 恢复 dead_code lint 信号。
- 修正 3 个真死字段（platform v1 `config`），不破坏 v1 API 行为。

**Non-Goals:**
- 不改 codegen 工具（`codegen.py` 不 emit dead_code；HR 的 allows 是历史脚手架遗留，无活跃 codegen 复发）。
- 不重构 request builder / v1 入口 struct 结构。
- 不引入新 API。

## Decisions

> 核心选型留 design 阶段 brainstorming 确认。

**D1（389 cruft 删除）—— 已确定**：机械移除，已验证安全（0 warning）。

**D2（3 个 platform v1 `config` 真死字段，待 brainstorm）**——需先读 `v1/mod.rs` 确认 `config` 为何未被读：
- 候选 A：入口 struct 应把 `config` 传给子 API 访问器（如 `AdminV1::badge()` 应 `BadgeApi::new(self.config.clone())`）——若子 API 当前没收到 config，这是**潜在功能缺陷**，修法是真正读取 config 并传递。
- 候选 B：子 API 已通过其他途径获得 config，入口 struct 的 `config` 是冗余 → 移除字段（连同 `new(config)` 签名调整，但 `AdminService::v1()` 调用 `AdminV1::new(config)`，需同步）。
- 候选 C：保留字段但 `_config` 前缀 + 注释说明（最小改动，但未真正修复）。

倾向先读代码判明 A vs B（是缺陷还是冗余），再定修法。

**D3（防复发，待 brainstorm）**：是否加 workspace 级约束（clippy custom / 约定文档 / CI grep 检查）防止 `#[allow(dead_code)]` 再度泛滥。

## Risks / Trade-offs

- **[3 字段修正可能触及 v1 公共签名]** 若 D2 选 B（移除 config），`AdminV1::new` 签名变化，但 `AdminV1` 本身刚 ungate（上个 change），外部依赖极少 → 低风险。
- **[mass-deletion 误删]** 389 删除已验证 0 warning，但需 final clippy 复核 default+full+no-default 三组 feature。
- **[其他 crate 31 处]** 已纳入范围（实证 0 真死字段），一并清理。

## Migration Plan

1. 移除 389 cruft（机械）。
2. 按 D2 修法处理 3 个 platform v1 config 字段。
3. `cargo clippy --workspace` 三组 feature（default/full/no-default）`-D warnings` 全过。
4. 测试通过。
5. 回滚：纯删除/小改，git revert 即可。

## Open Questions

- D2：3 个 `config` 字段是缺陷（应传递）还是冗余（应移除）？
- D3：是否加防复发约束？
