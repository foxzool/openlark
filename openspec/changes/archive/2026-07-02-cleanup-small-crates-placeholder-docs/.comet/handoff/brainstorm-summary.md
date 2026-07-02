# Brainstorm Summary

- Change: cleanup-small-crates-placeholder-docs
- Date: 2026-07-02

## 探勘事实（并行 workflow 5 crate 全量确认）

| crate | 占位 | 文件 | fn | struct | field | module | other | struct 位置 bug | data 字段 | 命名字段 |
|-------|------|------|-----|--------|-------|--------|-------|----------------|----------|----------|
| mail | 104 | 15 | 45 | 35 | 22 | 0 | 0 | 35 | 15 | 7 |
| workflow | 78 | 29 | 48 | 7 | 15 | 8 | 0 | 7 | 0 | 11 |
| meeting | 65 | 41 | 50 | 2 | 13 | 0 | 0 | 2 | 0 | 11 |
| user | 47 | 7 | 21 | 15 | 8 | 0 | 0 | 15 | 7 | 1 |
| hr | 41 | 3 | 9 | 15 | 8 | 0 | 6 | 4 | 0 | 8 |
| **合计** | **335** | **95** | **173** | **74** | **66** | **8** | **6** | **63** | **22** | **38** |

**关键确认**：
- **enum_variant = 0** → 5 crate 全机械模式（同 application），非语义（非 docs 的 74 variant）。recipe 已被 application（578）实证。
- **multi_attr_boundary = 0** → 63 处 struct 位置 bug 全是单 `#[derive(...)]` 行的 trivial 交换（上方无 `#[serde]` 叠加），与 application 190 处同构。
- **recipe_match = mechanical**（5 crate 全部）。
- fn outlier：builder setter 14 去重名（mail/workflow/user/hr）；mod factory = 0；module 8（全 workflow v4/mod.rs + 子模块）；**impl 块 6（全 hr）= recipe 新增角色**。

## 确认的技术方案（用户已确认）

1. **Recipe** = application 的 patched 10 行 + 1 新行（impl 块 → `<API>请求构建器实现。`）。全机械，0 enum variant。
2. **分组** = 按 crate 5 组（mail 104 / workflow 78 / meeting 65 / user 47 / hr 41）。mail 104 < application G5=138，**无需拆分**。每组 = 一个 crate（独立编译单元，`cargo doc -p <crate>` 自验）。
3. **位置修正** = 63 处 struct `///` 从 `#[derive]` 后移到前，全 trivial 单行交换。

## 关键取舍与风险

- [跨 crate 同名字段翻译漂移] → recipe 附共享翻译表（33 命名字段 + 14 builder setter），implementer 按表 + 飞书常识（如 user_ids 在 user/workflow 两处一致）。
- [impl 块 doc（hr 6 处）是新 recipe 角色] → 加 1 行。
- [规模 335 诱发偷懒] → recipe 强制引用真实 API 名 + 占位 grep 守门 + mail pilot 先行（1 文件验证 recipe + 位置变换）。

## 测试策略

逐 crate `cargo doc -p <crate>` 无 warning；占位 grep 双守门（占位空 + 位置空）；workspace `cargo doc --all-features` missing_docs=0；`cargo fmt --check` + `just lint` 双路径 exit 0；5 crate 现有测试不破。

## Spec Patch

无。delta spec（`specs/missing-docs-quality/spec.md`）open 阶段已写 small-crates 2 scenarios；主 spec missing-docs-quality 现 2 requirements（docs + application）。本 change 仅 ADDED small-crates 场景（已存在）。
