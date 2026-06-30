---
comet_change: unify-platform-directory-request-naming
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-30-unify-platform-directory-request-naming
status: final
---
# Design — unify-platform-directory-request-naming（#271 directory 批）
> 技术 HOW。delta spec 为 canonical。
## 1. 背景
#271 directory 批。模式 5 批验证。21 个全最简（无 re-export/service/trait impl）。
## 2. 实现
每类型：struct+impl→RequestBuilder；#[deprecated] alias（#[cfg(test)] 前）；测试同步。
## 3. 测试
build/clippy×3/test/fmt/grep。
