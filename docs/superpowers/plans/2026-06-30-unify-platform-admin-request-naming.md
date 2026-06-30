---
change: unify-platform-admin-request-naming
design-doc: docs/superpowers/specs/2026-06-30-unify-platform-admin-request-naming-design.md
base-ref: PLACEHOLDER
---

# unify-platform-admin-request-naming（#271 platform admin）

> 14 类型，最简模式（无 re-export/service/trait impl），#271 既定模式。

## Task 1: 14 重命名 + alias
- [ ] **Step 1:** 14 定义文件 struct+impl+测试 XxxBuilder→XxxRequestBuilder；#[cfg(test)] 前加 #[deprecated] pub type alias
- [ ] **Step 2:** build + commit

## Task 2: 验证
- [ ] **Step 1:** build --all-features + clippy×3 + test + fmt + grep
- [ ] **Step 2:** CHANGELOG

## Task 3: commit
- [ ] **Step 1:** CHANGELOG + tasks
