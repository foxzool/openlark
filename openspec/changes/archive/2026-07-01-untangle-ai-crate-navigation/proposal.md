## Why

#275 暴露的 4 个 ai v1 入口 struct（`DocumentAiV1` / `Image` / `Speech` / `Text`）无访问器只是表层症状。实地核查后发现 openlark-ai 存在 **3 套并行导航范式互相打架**：

- **范式 A**（`common/chain.rs`，pub field 直达）：唯一活的链，但与顶层 `AiClient` 脱节，零外部引用
- **范式 B**（`ai::AiService`+`v1::V1` 聚合 + `service.rs` 的 4 个 `*Client`）：4 条链全部导向零引用孤儿 struct，死路
- **范式 C**（SparkV1 accessor 方法，#274 已在 platform 落地）：ai crate 尚未采用

附带症状：同名 `DocumentAiClient` 存在两份（`service.rs` vs `chain.rs`）、Speech 实现重复两套、OCR/Speech/Translation 的已实现 API 根本没接进 canonical 入口 `AiClient`。直接给孤儿装访问器而不收敛范式，只会加深技术债。v0.18 breaking 窗口正开启（死链全部零外部引用），是收敛到单一 canonical 范式、对齐 #274 的时机。

## What Changes

- 确立 `AiClient` + SparkV1 accessor 方法为 openlark-ai 唯一 canonical 导航范式
- 4 个能力（DocumentAi / OCR / Speech / Translation）的全部已实现 API 接进 `AiClient` 链：每级入口通过 `pub fn` 访问器返回下一级 resource/service 类型，链式到达叶子 RequestBuilder（范式对齐 SparkV1）
- **BREAKING**：退役范式 A —— 将 `common/chain.rs` 的 `RecognizeResource` 能力迁移进 `AiClient.document_ai().v1()` 链后，删除 `common/chain.rs` 及 lib.rs:72 的顶层 `DocumentAiClient` re-export（零外部引用，迁移无外部成本）
- **BREAKING**：删除范式 B 死聚合 —— `ai::AiService` + `ai::v1::V1`（4 孤儿聚合体），全仓零外部引用
- 合并 Speech 重复实现（`ai/speech_to_text/v1/speech/` 2 个 vs 顶层 `speech_to_text/speech_to_text/v1/recognize/` 3 个）为单套
- 4 个孤儿 struct 的 `_config` 字段恢复为 `config` 并被新增访问器消费（清除 #267 的 reserved 注释）
- 消除同名两份 `DocumentAiClient`

## Capabilities

### New Capabilities

（无）

### Modified Capabilities

- `v1-sub-api-accessors`：扩展范式覆盖范围到 openlark-ai 的 4 个 v1 入口（`DocumentAiV1` / `OcrV1` / `SpeechToTextV1` / `TranslationV1`），要求其通过 `AiClient` 链式 accessor 暴露叶子 builder，对齐 SparkV1。新增 ai crate 特有 requirement：单一 canonical 导航入口（`AiClient`），不允许并行导航范式并存；零外部引用的死链（范式 A/B）SHALL 收敛删除而非保留。原 Requirement「非破坏性补全」针对 platform 纯加法场景，ai crate 因范式收敛需要 breaking 删除死链，需在 delta 中明确该差异。

## Impact

- **crates/openlark-ai**（主要改动）：
  - `src/service.rs`：`AiClient` 的 4 个 `*Client.v1()` 链改为返回带 accessor 的真实 resource/service，而非孤儿
  - `src/common/chain.rs`：删除（能力迁入 `AiClient` 链）
  - `src/common/mod.rs`：移除 chain re-export
  - `src/ai/mod.rs`、`src/ai/v1/mod.rs`：删除 `AiService` + `V1` 死聚合，或按 design 决策重构
  - `src/ai/{document_ai,optical_char_recognition,speech_to_text,translation}/v1/`：孤儿 struct 装访问器、`_config`→`config`
  - `src/document_ai/`、`src/speech_to_text/`：Speech 重复实现合并
  - `src/lib.rs`：移除 `common::chain::DocumentAiClient` 顶层 re-export
- **crates/openlark-client**：`client.rs:105` 注册的 `AiClient` 类型不变，但需确认 prelude（`lib.rs:342/490`）导出与新导航链一致
- **公开 API（v0.18 breaking）**：移除 `openlark_ai::DocumentAiClient`（chain.rs 版）顶层导出、移除 `openlark_ai::ai::AiService`、移除 `ai::v1::V1` 聚合体；4 个入口 struct 字段 `_config`→`config`
- **依赖**：无新增
