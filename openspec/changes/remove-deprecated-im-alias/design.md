## Context

`im/mod.rs` 的 `pub mod im { pub use super::project::{v1,v2}; }` 是 deprecated 别名（since 0.15.0），造成 47 文件用冗余 `im::im::` 路径。新路径 `im::v1`/`im::v2` 已存在（`pub use project::{v1,v2}`）。迁移是**路径缩短**，机械、无语义变化。

```
im::im::v1::x  →  im::v1::x      （去掉冗余的 im:: 别名层）
im::im::v2::x  →  im::v2::x
```

## Goals / Non-Goals

**Goals:** 47 文件 `im::im::` → `im::` 迁移；删除 `pub mod im` 别名块。

**Non-Goals:** 不动 B（wiki Params）；不改 im v1/v2 实现；不重命名模块；不动 `mod project`（实际模块）。

## Decisions

**D1（迁移方式）**：`sed 's/im::im::/im::/g'` 替换 47 文件中的导入路径（机械、无语义变化），然后删除 `pub mod im { ... }` 别名块（im/mod.rs:17-20）。`mod project`（实际模块，`#[path="im/mod.rs"]`）保留。

**D2（外部 breaking）**：`openlark_communication::im::im::v1` 外部路径移除。CHANGELOG 指引改 `im::v1`。

## Risks

- **[Breaking]** `im::im` 路径移除 → 外部编译失败；缓解：CHANGELOG 迁移（`im::im::` → `im::`，drop-in）。
- **[sed 误替换]** `im::im::` 模式可能匹配非导入场景？— 实证 grep 仅命中 `use ... im::im::` 导入，无歧义。build 阶段 clippy 验证。

## Migration Plan

1. `sed -i '' 's/im::im::/im::/g'` 替换 47 文件。
2. 删除 `im/mod.rs` 的 `pub mod im { ... }` 别名块 + `#[allow(module_inception)]`。
3. 三组 clippy + test。
4. CHANGELOG breaking + 迁移。
5. 回滚：git revert。

## Open Questions

- 无（路径缩短是确定的；grep 确认无歧义）。
