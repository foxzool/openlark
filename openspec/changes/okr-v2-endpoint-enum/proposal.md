## Why

`openlark-hr` okr/v2 的 25 个叶子端点 URL 构造方式**不一致**，`OkrApiV2` enum（`crates/openlark-hr/src/common/api_endpoints.rs:1941`）25 variant 齐备却**在生产代码中零调用**：

- **23 叶** inline `format!("/open-apis/okr/v2/...")` 硬编码 ❌
- **2 叶** inline 字符串字面量：`category/list` 的 `ApiRequest::get("/open-apis/okr/v2/categories")` 与 `cycle/list` 生产代码的 `ApiRequest::get("/open-apis/okr/v2/cycles")` ❌（`cycle/list` 仅其**测试**用了 enum，生产未用）

这违 `openlark-api` 核心契约 5（端点用 enum）+ AGENTS.md ANTI-PATTERNS「不要硬编码 URL」。`OkrApiV2` enum 25 个 variant **已齐备且 `to_url()` 实现正确**，但生产代码零调用——enum 形同 Potemkin，single source of truth 失效。

这是 **#328（type-okr-v2-responses）刻意 carve-out 的遗留**：该 change 的 D4 决策保留 URL 构造不动以避免混入 typed 化主线，其 spec 显式声明「端点 enum 一致性不在本 requirement 范围」。本 change 填补这块 carve-out，是 #328 code-review follow-up 的第三支（#336 消重、#337 CHANGELOG 之外的 URL 收敛）。

issue #338 原称「12 叶 inline format!」是**低估**：仅匹配单行 `format!("/open-apis/okr/v2`，漏数 12 个多行 `format!(` 与 2 个字符串字面量（`category/list`、`cycle/list` 生产）。实测 **25 叶全部待迁移**（enum 生产零调用）。

## What Changes

- 把 okr/v2 **25 个叶子**的端点 URL 构造从 inline `format!`（23 叶）/ inline 字符串字面量（`category/list`、`cycle/list` 2 叶）统一改为 `OkrApiV2::*().to_url()`
- `OkrApiV2` enum 已含全部 25 variant，**无需新增 variant**——纯调用点迁移
- URL 路径字符串**零变化**（enum 的 `to_url()` 已产生与现有 inline 串完全相同的路径）
- **补 `OkrApiV2` enum `to_url()` 测试**：`api_endpoints.rs::test_okr_api_urls` 当前只测 `OkrApiV1`，补全 25 variant 断言，锁定 variant→URL 映射（迁移的安全网，抓潜伏休眠 bug）
- 不动 typed Response（#328 已定）、不动导航链、不动请求/响应模型、不动叶子现有测试（含 Potemkin `test_url_path`，另案）

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `v1-sub-api-accessors`：新增 okr/v2 端点 URL 构造一致性 requirement。现有 requirement 覆盖「版本节点 SHALL 暴露链式 accessor」「HR 零 accessor 死节点 SHALL 删除」（#327）与「okr/v2 navigable 链叶子 SHALL 返回 typed Response」（#328，显式把端点 enum 排除在外）；本 change 补第四条——**okr/v2 叶子端点 SHALL 经 `OkrApiV2` enum 的 `.to_url()` 构造**，不得 inline `format!` 或字符串字面量硬编码。属非破坏性内部一致性收敛（URL 串不变，纯构造方式迁移）。

## Impact

- **crates/openlark-hr**（全部改动集中于此）：
  - `src/okr/okr/v2/<resource>/...` 25 个叶子的 `execute_with_options()`：URL 构造行 `format!(...)` / 字符串字面量 → `OkrApiV2::Variant(self.id).to_url()`
  - 资源批次：alignment(2) / category(1) / cycle 含 objective 子树(6，含 cycle/list) / indicator(1) / key_result 含 indicator/progress(5) / objective 含 alignment/indicator/key_result/progress(10)
  - `src/common/api_endpoints.rs` `OkrApiV2`：enum 本身不动（variant 齐、`to_url()` 正确）；**仅补 `test_okr_api_urls` 的 25 variant to_url 断言**
- **公开 API**：无变化（URL 串不变，`execute()` 签名不变）——非 breaking
- **依赖**：无新增
- **测试**：叶子现有 `test_url_path` 是 Potemkin（不调生产），**不动**（修需架构改动，另案）；迁移安全网靠新增的 enum `to_url()` 25 variant 测试 + 全量 build/clippy/fmt/test + 验收 grep
