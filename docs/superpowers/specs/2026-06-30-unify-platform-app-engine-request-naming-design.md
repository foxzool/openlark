---
comet_change: unify-platform-app-engine-request-naming
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-30-unify-platform-app-engine-request-naming
status: final
---
# Design — unify-platform-app-engine-request-naming（#271 app_engine 批，最后一批）
> 技术 HOW。delta spec 为 canonical。
## 1. 背景
#271 最后一批。模式 6 批验证。51 个全最简（无 re-export/service/trait impl）。全在 apaas 子目录。
## 2. 实现
每类型：struct+impl→RequestBuilder；#[deprecated] alias（#[cfg(test)] 前）；测试同步。脚本统一处理 51 个。
## 3. 测试
build/clippy×3/test/fmt/grep。
