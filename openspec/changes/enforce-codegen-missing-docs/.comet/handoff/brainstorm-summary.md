# Brainstorm Summary

- Change: enforce-codegen-missing-docs
- Date: 2026-07-02

## 确认的技术方案

**D1 移除 -A**：`tools/codegen.py:185` `run_closed_loop` 的 clippy 命令去掉 `-A missing_docs`（`-- -Dwarnings` 结尾，1 行）。

**D2 fallback doc**：`tools/api_contracts/codegen_render.py` 的 `_field_lines`——当 `field.description` 为空时生成 `/// {field.rust_name}`（字段名兜底），确保每个 pub 字段都有 doc。改动 ~2 行：
```python
doc = _oneliner(field.description) if field.description else field.rust_name
out.append(f"    /// {doc}")
```

## 关键取舍与风险

- **D1 验证用「对 communication 跑无 -A clippy」而非实跑 codegen 生成**：避免覆盖真实 crate 文件（codegen marker 警示手工修改会被覆盖）；直接证明移除 -A 对已提交产出安全（communication 已 0 missing_docs）。
- **D2 由单测覆盖**：test_codegen_render.py 新增空 description FieldDef 断言；现有 inline 断言（fixture 字段有 description）不破坏。
- **[test_codegen_render.py 是 inline assertIn，非 golden file]** → fallback 不影响现有断言；只增不删。
- **[无其它 pub 项]**：`_tests` 是 `#[cfg(test)]` 私有 mod、`_render_api_response_trait` 是 trait impl（doc 继承）、无 enum 渲染 → D2 是唯一 doc 缺口。
- **[fallback doc 质量]** → 字段名兜底（`/// user_id`），机械不误导，与 #1「引用真实名」原则一致。语义 doc 依赖 schema description 质量提升，属另案。

## 测试策略

- D2 单测：test_codegen_render.py 构造空 description FieldDef → 断言 `_field_lines` 输出 `/// {field_name}`。
- D1 安全验证：移除 -A 后对 communication 跑 `cargo clippy -p openlark-analytics-style`... 实际是 `cargo clippy -p openlark-communication --all-targets --all-features -- -Dwarnings`（无 -A）exit 0。
- 回归：现有 test_codegen_render/ir/mod_tree + api-contracts CI 验证通过；`cargo fmt --check` + workspace lint 不破。

## Spec Patch

无。delta spec（codegen-missing-docs：ADDED 闭环不绕过 missing_docs + ADDED pub 字段必有 fallback doc，各含 scenario）已覆盖本设计。
