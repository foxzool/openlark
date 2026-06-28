# Tasks — remove-unused-deprecated

> 已完成（scope 调整）。原计划 G+D+C（5），**build 核实发现 G（auth app_id/secret/ticket）是 functional legacy two-step flow（execute 读取 + test 验证），非 unused** → G 不删，留 #278。本 change 实际做 **D+C（2 项零调用/dead）**。关联 #278。

## 1. ~~移除 G：auth 3 个 deprecated 方法~~ → N/A（G 是 functional legacy flow，非 unused，不删）

- [x] 1.1 核实：G 的 app_id/secret/ticket 喂 legacy two-step flow（execute L121-151 读取 legacy_app_id/secret/ticket），test_execute_legacy_chain 验证 → 非 unused，保留

## 2. 移除 D：docs to_value ✓

- [x] 2.1 删除 `RecordFieldValue::to_value()` + 连带空 `impl RecordFieldValue {}` 移除
- [x] 2.2 `json!` import 改 `#[cfg(test)]`（lib 不再用，tests 用）

## 3. 移除 C：docs 宏 new ✓

- [x] 3.1 从 `impl_required_builder!` 宏删除 `new()` 生成块（唯一调用者 TestRequest 用 builder()）

## 4. 验证 ✓

- [x] 4.1 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [x] 4.2 `cargo test --workspace` 0 failed
- [x] 4.3 to_value=0、macro new=0；G auth 保留（functional）

## 5. 文档与收尾

- [x] 5.1 CHANGELOG `[Unreleased] > Breaking Changes` 加 D+C 条目 + 迁移映射
- [x] 5.2 评论 #278：本 change 处理 D+C（2）；G（functional legacy flow）+ B（wiki Params）+ F（im 别名）仍 open
