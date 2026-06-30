# Brainstorm Summary
- Change: unify-platform-directory-request-naming
- Date: 2026-06-30
## 确认的技术方案
21 个请求 builder → RequestBuilder + #[deprecated] alias。5 批验证模式直接应用。
## 关键取舍
CollaborationTenantListBuilder 不撞名（不同模块路径）。alias 放 #[cfg(test)] 前。
## 测试策略
build/clippy×3/test/fmt/grep。
## Spec Patch
无。
