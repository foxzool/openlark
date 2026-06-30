---
change: unify-platform-small-request-naming
design-doc: docs/superpowers/specs/2026-06-30-unify-platform-small-request-naming-design.md
base-ref: 015ef54d0e7a76586ed82cd0567fe65a93ab313c
---

# unify-platform-small-request-naming 实施计划（#271 platform 第 1 批小批）

> #271 既定模式（auth/application/docs 已验证）。12 类型，无 re-export/trait impl，最简。

**Goal:** platform trust_party/mdm/tenant/spark 12 个请求 builder → XxxRequestBuilder + #[deprecated] alias。

## Global Constraints

- 仅改名 + alias；alias 放 `#[cfg(test)]` 前；push 前跑 `cargo fmt --check`。
- 不动 body 模型、其他 platform 子系统、非请求 builder。

## 12 类型 → 新名（均 +RequestBuilder）

AssignInfoListQuery / CollaborationDepartmentGet / CollaborationTenantGet / CollaborationTenantList / CollaborationUserGet / CountryRegionBatchGet / CountryRegionList / DirectoryUserIdConvert / TenantQuery / UserAuthDataRelationBind / UserAuthDataRelationUnbind / VisibleOrganization（各 Builder → RequestBuilder）

## Task 1：12 类型重命名 + alias

- [ ] **Step 1:** 12 个定义文件：struct+impl+测试 `XxxBuilder` → `XxxRequestBuilder`；`#[cfg(test)]` 前加 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`
- [ ] **Step 2:** `cargo build -p openlark-platform` exit 0
- [ ] **Step 3:** commit

## Task 2：全量验证

- [ ] **Step 1:** `cargo build --workspace --all-features` exit 0
- [ ] **Step 2:** 三组 clippy（-D warnings）exit 0
- [ ] **Step 3:** `cargo test -p openlark-platform` 0 failed
- [ ] **Step 4:** **`cargo fmt --all -- --check` exit 0**
- [ ] **Step 5:** grep 12 RequestBuilder struct + 12 alias + 0 残留
- [ ] **Step 6:** CHANGELOG v0.18 breaking 段记录

## Task 3：commit + 完成

- [ ] **Step 1:** 提交 CHANGELOG + tasks 勾选
