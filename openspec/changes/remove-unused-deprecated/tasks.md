# Tasks — remove-unused-deprecated

> 范围：移除 5 个零调用/dead 的 deprecated 项（G auth ×3 + D docs to_value ×1 + C docs 宏 new ×1）。BREAKING。关联 #278（G+D+C 子集）；B（wiki Params）+ F（im 别名）留在 #278。

## 1. 移除 G：auth 3 个 deprecated 方法

- [ ] 1.1 删除 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 的 `app_id()`/`app_secret()`/`app_ticket()` 3 个 `#[deprecated] pub fn`（含 deprecation 注释）

## 2. 移除 D：docs to_value

- [ ] 2.1 删除 `crates/openlark-docs/src/base/bitable/v1/field_types.rs` 的 `to_value()` 方法（含 `#[deprecated]`）

## 3. 移除 C：docs 宏 new()

- [ ] 3.1 从 `crates/openlark-docs/src/common/request_builder.rs` 的 `impl_required_builder!` 宏删除 `new()` 生成块（含 `#[deprecated]`+`#[expect(dead_code)]`）；确认唯一调用者 TestRequest 不受影响（用 builder()）

## 4. 验证

- [ ] 4.1 目标 deprecated 已删：auth 3 + docs to_value + 宏 new = 5
- [ ] 4.2 examples/tests 不引用已移除项（`.to_value()` / `tenant_access_token().app_id` 等 = 0）
- [ ] 4.3 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] 4.4 `cargo test --workspace` 通过

## 5. 文档与收尾

- [ ] 5.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射表
- [ ] 5.2 评论 #278：本 change 处理 G+D+C（5 干净项）；B（wiki Params ~16 用法）+ F（im 别名 47 文件）仍 open
