# Brainstorm Summary
- Change: unify-platform-admin-request-naming
- Date: 2026-06-30

## 确认的技术方案
#271 platform admin 14 个请求 builder → XxxRequestBuilder + #[deprecated] alias。直接应用 4 批验证模式。14 个全无 trait impl/re-export/service → 最简。

## 关键取舍与风险
全 →RequestBuilder。alias 放 #[cfg(test)] 前。push 前 cargo fmt --check。

## 测试策略
build/clippy×3/test/fmt/grep。

## Spec Patch
无。
