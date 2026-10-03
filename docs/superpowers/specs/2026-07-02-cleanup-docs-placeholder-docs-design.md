---
comet_change: cleanup-docs-placeholder-docs
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-02-cleanup-docs-placeholder-docs
status: final
---

# Design: cleanup-docs-placeholder-docs

> #273 missing_docs 深度治理子项 #3（批量拆分第 1 个：docs crate）。承接 #1（PR #293 recipe）+ #4（PR #294 codegen）+ #2（PR #295 测试）。本 change 清理 docs crate 的 144 行 `/// 公开项说明。` 占位。
>
> Canonical spec：`openspec/changes/cleanup-docs-placeholder-docs/specs/missing-docs-quality/spec.md`（delta，新 capability）。

## 1. Context

`openlark-docs` crate 有 **144 行 `/// 公开项说明。` 占位**（14 文件），legacy 产物撑着 0 missing_docs 但无文档价值。

**关键勘探发现（推翻 open 阶段假设）**：docs crate 与 #1 analytics 模式根本不同——

| 占位项类型 | 数量 | 占比 |
|-----------|------|------|
| **enum variant**（如 `OpenId`/`UnionId`） | 74 | 52% |
| **struct field**（如 `pub app_id: String`） | 58 | 40% |
| pub struct | 4 | 3% |
| pub const / other / pub type | 8 | 5% |

**92% 是 enum variant + struct field**——需要**逐项语义 doc**（理解每个变体/字段代表什么），非 #1 的 `<//!标题>+<角色>` 机械模板（那是 Request/Response struct 模式）。`OpenId` 不是"标题+角色"，是枚举值，需 `/// 开放平台用户 ID` 这样的语义描述。

**4/14 文件无 `//!` 头**（models.rs/content/get.rs/docx/models.rs/field_types.rs）→ doc 从 item 上下文派生，非文件标题。

**位置模式非统一**：仅 4/144 是 `#[derive]` 后置 `///`（struct）；其余是 variant/field（无 derive 问题）。

**对比**：application（578 占位在 `new`/`execute`/struct）+ small-crates 是 #1 机械模式；docs 是异类（variant/field 语义）。

## 2. 目标 / 非目标

**目标**：替换 docs 144 占位为**逐项语义 doc**；修正 4 处 `#[derive]` 后置；建立 `missing-docs-quality` capability。

**非目标**：不改逻辑/签名；不动 application/small-crates（各自 change）；不引入机械名称堆砌（选语义而非 fallback）。

## 3. 方案

### 核心方法：逐项语义 doc

每个占位项读其**名称 + 所在 enum/struct 上下文 + Feishu 常识**，写有意义中文描述：

| item 类型 | doc 来源 | 示例 |
|-----------|---------|------|
| enum variant | 变体名 + enum 职责 + Feishu 语义 | `UserIdType::OpenId` → `/// 开放平台用户 ID` |
| struct field | 字段名 + struct 职责 | `pub app_id: String` → `/// 应用 ID` |
| pub struct | struct 名 + 所在 API 上下文 | `UpdateEntityReq` → `/// 更新实体请求体。` |
| pub const / type | 名 + 用途 | 按职责描述 |

**无 `//!` 头的文件**（4 个）：doc 从 item 所在 enum/struct 的现有 doc 或名称派生（不依赖文件标题）。

### 位置修正（4 处）

4 个 struct 的 `#[derive(...)]` 后紧跟 `///` → 移 `///` 到 `#[derive]` 前（标准 + 对齐 #1/communication 规范）。其余 140 处 variant/field 无 derive 问题。

### 执行（subagent-driven）

14 文件，按 baike/ccm/common/base 等域分组（或按文件）。每 subagent：
1. 读文件，定位每个 `/// 公开项说明。` 占位
2. 读占位项的名称 + 所在 enum/struct 上下文 + 必要 Feishu 常识
3. 写语义 doc（中文，有意义，非名称堆砌）
4. 处理 4 处 `#[derive]` 后置（移前）
5. 自验：`grep 公开项说明 该文件` 空 + `cargo doc -p openlark-docs` 该文件 0 警告

### 验证

- **grep 守门**：`grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/` 为空
- **编译**：`cargo doc -p openlark-docs --all-features` 0 警告；workspace 0
- **位置守门**：`#[derive]` 后不紧跟 `///`
- **语义质量**：final code review 抽查 variant/field doc 是否有意义（非占位、非纯名称堆砌）
- **回归**：`cargo fmt --check` + `just lint`；docs 现有测试不破

## 4. 决策与替代

| 决策 | 选择 | 否决的替代 |
|------|------|-----------|
| doc 方法 | 逐项语义 | 机械 recipe（不适用 variant/field）/ 名称堆砌（值低，spec"有意义"边界） |
| 位置修正 | 仅 4 处 derive 后置 | 全量位置检查（多数无 derive 问题） |
| 执行 | subagent-driven 按域 | direct 主会话（14 文件语义工作，并行 + 隔离上下文更优） |

## 5. 风险与缓解

- **[语义 doc 质量]** → 名称+上下文+Feishu 常识派生；final review 抽查；多数是常见标识（OpenId/UnionId/app_id）语义清晰。
- **[逐项工作量大]** → 144 项多为常见 Feishu 字段/变体，判断成本低；subagent 并行。
- **[4 无 //! 文件]** → doc 从 item 上下文派生，不依赖文件标题。

## 6. 迁移与回滚

纯 doc 改动。回滚 = revert。顺序：按域组逐文件回补 → grep/位置守门 → cargo doc 0 → review。

## 7. Open Questions / Build 阶段决策

- build_mode：subagent-driven（14 文件语义工作，并行优）→ build plan-ready 暂停由用户选定。
- isolation: branch。tdd_mode: direct（doc 写作）。
- recipe 验证：docs 是批量第 1 个，语义方法验证后供 application/small-crates 参考（但那两个是机械模式，用 #1 recipe）。
