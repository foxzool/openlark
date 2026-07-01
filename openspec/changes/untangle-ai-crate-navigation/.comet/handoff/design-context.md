# Comet Design Handoff

- Change: untangle-ai-crate-navigation
- Phase: design
- Mode: compact
- Context hash: c8ec6a2871378879744888a16afd5ac22b4e43cf0d7b539f885d70450298e262

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/untangle-ai-crate-navigation/proposal.md

- Source: openspec/changes/untangle-ai-crate-navigation/proposal.md
- Lines: 1-43
- SHA256: 5976e8441847cae91bec4c2abeef54d23751a705408b421791c56ea5b3a27fa3

```md
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
```

## openspec/changes/untangle-ai-crate-navigation/design.md

- Source: openspec/changes/untangle-ai-crate-navigation/design.md
- Lines: 1-75
- SHA256: a7512efabd48d1494d0267153d20f783930d9613ba343d0376cadb10de4832ba

```md
## Context

openlark-ai crate 当前存在 3 套并行导航范式（详见 proposal.md 的 Why）：

```
范式 A (chain.rs, pub field)   : DocumentAiClient.v1.recognize.resume_parse()  ← 唯一活链，但脱节、零引用
范式 B (ai+service, 孤儿聚合)   : AiClient.document_ai().v1() → 孤儿 struct     ← 4 条全死
范式 C (SparkV1 accessor, #274): platform.spark().v1().app().create()          ← 目标范式，ai 未采用
```

约束：
- `AiClient`（service.rs）是 openlark-client ServiceRegistry 注册的 canonical 入口（client.rs:105），不可删除
- 范式 A（`common::chain::DocumentAiClient`）与范式 B（`ai::AiService`/`v1::V1`）grep 全仓零外部引用
- v0.18 breaking 窗口开启（workspace=0.17.0，CHANGELOG 已有 v0.17→v0.18 迁移表）
- 4 个孤儿 struct 的 `_config` 字段为 #267 临时 reserved，本 change 需恢复

## Goals / Non-Goals

**Goals:**
- 收敛 ai crate 到单一 canonical 导航范式（SparkV1 accessor），消除 3 套范式打架
- 让 4 个能力（DocumentAi/OCR/Speech/Translation）的全部已实现 API 从 `AiClient` 链式可达
- 消除同名两份 `DocumentAiClient`、Speech 重复实现、死聚合代码
- 对齐 `v1-sub-api-accessors` spec（#274 建立的范式契约）

**Non-Goals:**
- 不新增飞书 AI API 实现（只接线已有 RequestBuilder）
- 不重构已有 API 实现内部（`document_ai/document_ai/v1/recognize/*` 等）
- 不动 openlark-client ServiceRegistry 机制
- 不触碰其他业务 crate

## Decisions

### D1: canonical 范式 = 范式 C（SparkV1 accessor 方法）
**选择**：每级入口通过 `pub fn` 访问器返回下一级 resource/service 类型，链式到达叶子 builder。
**理由**：#274 已在 platform 落地范式 C 并建立 `v1-sub-api-accessors` spec；ai 与 platform 同属 v1 子 API 访问器范式，跨 crate 统一。
**替代（否决）**：保留范式 A 的 pub field 风格——与 SparkV1 不一致，范式分裂；保留范式 B——死代码。

### D2: canonical 入口 = `AiClient`（service.rs）
**选择**：以 `AiClient` 为唯一顶层入口，4 能力挂在其访问器链下。
**理由**：`AiClient` 是 openlark-client 唯一注册认可的入口（client.rs:105 + prelude lib.rs:342/490）；chain.rs 的 `DocumentAiClient` 虽活但与 `AiClient` 脱节且零引用。
**替代（否决）**：以 chain.rs 的 `DocumentAiClient` 为入口——未注册进 ServiceRegistry，非 canonical。

### D3: 死链处置 = 迁移后删除（breaking，v0.18）
**选择**：
- 范式 A：`RecognizeResource` 的 5 个 API 方法迁移进 `AiClient.document_ai().v1()` 链后，删除 `common/chain.rs` + lib.rs:72 顶层 re-export
- 范式 B：`ai::AiService` + `ai::v1::V1` 聚合体直接删除
**理由**：grep 确认零外部引用；v0.18 breaking 窗口允许；保留双入口/死聚合与「单 canonical」目标矛盾。
**替代（否决）**：保留为便捷别名——长期认知负担，范式不清。

### D4: 4 能力接线 = 孤儿 struct 装 accessor + `_config`→`config`
**选择**：`DocumentAiV1`/`OcrV1`(+`Image`)/`SpeechToTextV1`(+`Speech`)/`TranslationV1`(+`Text`) 各加 accessor 指向已有 RequestBuilder；字段 `_config` 恢复为 `config` 并被 accessor 消费。
**理由**：对齐 #274 的「入口 config 字段恢复并被访问器消费」requirement；清除 #267 reserved。

## Risks / Trade-offs

- **[BREAKING 公开 API]** 移除 `openlark_ai::DocumentAiClient`(chain)、`ai::AiService`、`ai::v1::V1`、字段改名 → 缓解：v0.18 窗口允许；CHANGELOG 写迁移表；零外部引用故实际影响面为零
- **[Speech 实现合并选错套]** `ai/.../speech/`(2 个) vs 顶层 `speech_to_text/.../recognize/`(3 个) → 缓解：design 阶段核对两套 endpoint URL 与字段完整性，留对的一套
- **[openlark-client 注册受影响]** → 缓解：`AiClient` 类型签名与 `new(Config)` 构造不变，仅内部导航变化；verify 阶段确认 client.rs:105 编译
- **[层级设计偏差导致二次重构]** → 缓解：层级决策延后到 brainstorming，对照飞书 API URL 路径层级定

## Migration Plan

- v0.18 breaking 项：移除 chain 版 `DocumentAiClient` 顶层导出、`AiService`、`V1` 聚合、4 入口字段 `_config`→`config`
- 用户迁移路径：`DocumentAiClient::new(cfg).v1.recognize.resume_parse()` → `AiClient::new(cfg).document_ai().v1().<...>().resume_parse()`（最终层级见 Open Questions）
- CHANGELOG 按 v0.17→v0.18 迁移表风格补充 ai 导航迁移条目

## Open Questions

（留 comet-design brainstorming 深挖，open 阶段不定）

1. **导航层级形态**：`.v1().recognize().resume_parse()`（保留 recognize 中间层）还是 `.v1().resume_parse()`（拍平）？SparkV1 是 `.v1().app()`（v1 下直接 service，无额外层），但 ai 的 DocumentAi 已有 recognize 子层。需对照飞书 API URL 层级（如 `/open-apis/document_ai/v1/{resume_parse|id_card_recognize}`）决定。
2. **OCR `Image` / Translation `Text` 中间孤儿层去留**：飞书 URL `/optical_char_recognition/v1/image/basic_recognize`、`/translation/v1/text/translate` 暗示 image/text 是路径层——可能需保留以对齐 URL 语义。
3. **Speech 重复实现合并方向**：留顶层 `speech_to_text/speech_to_text/v1/recognize/`（3 个，含 speech_recognize，更全）还是 `ai/.../speech/`（2 个）？需核对 endpoint URL 与字段完整性。
4. **accessor feature 门控**：v1 feature 已存在（`default=["v1"]`），是否对齐 SparkV1 的 `#[cfg(feature="...")]` 统一门控模式。
5. **实现文件归位**：DocumentAi 实现在顶层 `document_ai/`，OCR/Translation 在 `ai/` 下——收敛后是否统一到单一位置。
```

## openspec/changes/untangle-ai-crate-navigation/tasks.md

- Source: openspec/changes/untangle-ai-crate-navigation/tasks.md
- Lines: 1-27
- SHA256: 33d9edac5efdef7209fb4d6b8da81ead0bd27bd3d03084770eb1e94ab7593f1d

```md
## 1. 死代码清理（范式 B 聚合体）

- [ ] 1.1 删除 `ai::AiService`（`src/ai/mod.rs`）与 `ai::v1::V1` 聚合体（`src/ai/v1/mod.rs`），保留 4 个入口 struct（`DocumentAiV1`/`OcrV1`/`SpeechToTextV1`/`TranslationV1`，因 `service.rs` 的 `*Client.v1()` 依赖它们）。验证：`cargo build -p openlark-ai` 通过，`grep -r "AiService\|v1::V1" crates/openlark-ai/src` 无残留聚合定义。

## 2. 4 能力 accessor 接线（核心）

- [ ] 2.1 `DocumentAiV1`：按 Design Doc 的层级决策装 accessor，链式指向 `document_ai/document_ai/v1/recognize/*` 的 5 个叶子 RequestBuilder（resume_parse/id_card_recognize/bank_card_recognize/business_license_recognize/vat_invoice_recognize）。验证：`ai_client.document_ai().v1().<...>.resume_parse()` 返回 `ResumeParseRequestBuilder`。
- [ ] 2.2 `OcrV1`（+ `Image` 按 Design 决定中间层去留）：装 accessor 指向 `basic_recognize` 叶子 builder。验证：`ai_client.ocr().v1().<...>.basic_recognize()` 可达。
- [ ] 2.3 `SpeechToTextV1`（+ `Speech`）：按 Design 决策合并 `ai/.../speech/`(2 个) 与顶层 `speech_to_text/.../recognize/`(3 个) 重复实现，留单套并装 accessor。验证：file_recognize/stream_recognize/speech_recognize 经 `AiClient` 链可达且无重复定义。
- [ ] 2.4 `TranslationV1`（+ `Text` 按 Design 决定中间层去留）：装 accessor 指向 `translate`/`detect` 叶子 builder。验证：`ai_client.translation().v1().<...>.translate()` 可达。
- [ ] 2.5 4 个入口 struct 的 `_config` 字段恢复为 `config`，并被新增 accessor 消费（清除 #267 reserved 注释）。验证：4 文件无 `_config` 前缀字段、无 dead_code 例外。

## 3. 范式 A（chain.rs）退役

- [ ] 3.1 删除 `src/common/chain.rs`、`src/common/mod.rs` 的 chain re-export、`src/lib.rs:72` 的顶层 `DocumentAiClient` re-export（其 API 能力已由任务 2.1 迁入 `AiClient` 链）。验证：`grep -r "chain::DocumentAiClient\|common::chain" crates/openlark-ai/src` 空，同名两份 `DocumentAiClient` 消除。

## 4. lib.rs 与 openlark-client 收尾

- [ ] 4.1 清理 `src/lib.rs` 导出：移除 chain 版 `DocumentAiClient`，确认 `AiClient`（line 66）与 endpoints 导出完整、模块文档示例（line 26 `client.document_ai().v1()...`）与新导航链一致。验证：`cargo doc -p openlark-ai --all-features` 无文档断链。
- [ ] 4.2 确认 `crates/openlark-client/src/client.rs:105` 注册的 `AiClient` 类型与 `lib.rs:342/490` prelude 导出在新导航链下编译通过。验证：`cargo build -p openlark-client` 通过。

## 5. 质量验证

- [ ] 5.1 `cargo fmt --check`（全仓）通过。
- [ ] 5.2 `cargo clippy --all-features --workspace` 通过，openlark-ai 无 dead_code 告警。
- [ ] 5.3 `cargo test --all-features -p openlark-ai` 通过（含访问器链可达性测试）。
- [ ] 5.4 终检：同名 `DocumentAiClient` 消除、Speech 实现单套、范式 A/B 死链清零、4 入口字段均为 `config`。
```

## openspec/changes/untangle-ai-crate-navigation/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/untangle-ai-crate-navigation/specs/v1-sub-api-accessors/spec.md
- Lines: 1-67
- SHA256: 4e544d94a33f0fb46ad4977dee197539ab9d89c1dbdc54c1c233a5018df7b621

```md
## ADDED Requirements

### Requirement: openlark-ai v1 入口经 AiClient 暴露链式子 API 访问器

openlark-ai 的 4 个 v1 入口（`DocumentAiV1` / `OcrV1` / `SpeechToTextV1` / `TranslationV1`）SHALL 通过顶层 `AiClient` 的链式 `pub fn` 访问器暴露其每一级子 API，链式导航一路到达叶子请求 builder，范式对齐 `SparkV1`（与 platform 的 AdminV1/ApaasV1/DirectoryV1 一致）。每一级入口与中间 service SHALL 持有 config 并通过 accessor 向下传递，叶子节点 SHALL 返回已存在的 RequestBuilder 构造方法。4 个入口 struct 的临时 `_config` 字段 SHALL 恢复为 `config` 且被新增访问器消费（清除 #267 reserved 注释，不再有下划线前缀或 dead_code 例外）。

#### Scenario: DocumentAiV1 链式访问叶子 builder

- **WHEN** 调用 `ai_client.document_ai().v1().<recognize 层>().resume_parse()` 后 execute
- **THEN** 返回 `ResumeParseRequestBuilder` 并可完成请求构建，链式导航从 `AiClient` 一路到达叶子 builder

#### Scenario: OCR / Speech / Translation 叶子 builder 可达

- **WHEN** 分别调用 `ai_client.ocr().v1().<...>().basic_recognize()`、`ai_client.speech_to_text().v1().<...>().file_recognize()`、`ai_client.translation().v1().<...>().translate()`
- **THEN** 各自返回对应的已存在 RequestBuilder，链式导航可用（具体中间层级由 design 决策定）

#### Scenario: 4 入口字段 _config 恢复为 config

- **WHEN** 变更后检查 4 个 ai v1 入口 struct 的字段命名
- **THEN** 不存在 `_config` 前缀字段，`config` 被访问器读取使用，无 dead_code 例外注释

### Requirement: openlark-ai 单一 canonical 导航入口

openlark-ai SHALL 以 `AiClient`（`service.rs`，由 openlark-client ServiceRegistry 注册）为唯一顶层导航入口，不允许多套并行导航范式并存。此前并存的范式 A（`common::chain::DocumentAiClient` pub-field 链）与范式 B（`ai::AiService` + `ai::v1::V1` 孤儿聚合）SHALL 收敛：范式 A 的 API 能力迁移进 `AiClient` 链后删除 `common/chain.rs` 及其顶层 re-export，范式 B 死聚合 SHALL 直接删除。本 requirement 构成对 `v1-sub-api-accessors` 现有「非破坏性补全」requirement 的 ai crate 专属例外：ai 因范式收敛需要 breaking 删除零引用死链，而非 platform 的纯加法。

#### Scenario: AiClient 为唯一顶层入口

- **WHEN** 变更后检查 openlark-ai 的顶层导出
- **THEN** 不存在 `common::chain::DocumentAiClient` 顶层 re-export，不存在 `ai::AiService` / `ai::v1::V1` 聚合类型，`AiClient` 是唯一顶层导航入口

#### Scenario: 死链移除不产生 dead_code

- **WHEN** 运行 `cargo clippy -W dead_code` 于 openlark-ai
- **THEN** 不存在因范式 A/B 残留引发的 dead_code 告警，同名两份 `DocumentAiClient` 冲突消除

#### Scenario: Speech 实现收敛为 csv 官方单套

- **WHEN** 变更后检查 speech_to_text 的 RequestBuilder 实现
- **THEN** 仅存在 file_recognize 与 stream_recognize（对齐 `api_list_export.csv`），位置在 `ai/speech_to_text/v1/speech/`，B 套顶层实现与多余的 speech_recognize 已删除，经 `AiClient` 链可达

### Requirement: openlark-ai 导航链层级对齐飞书 API URL

openlark-ai 的 v1 入口 accessor 链 SHALL 对齐飞书 API URL 路径层级：URL 形如 `/v1/{resource}/{action}` 时，导航链 SHALL 为 `.v1().{resource}().{action}()`，每一中间层 service 对应一个 URL 路径段。不得将不同路径段拍平为单一方法名。

#### Scenario: DocumentAi 三层导航对齐 URL

- **WHEN** 调用 `ai_client.document_ai().v1().id_card().recognize()`
- **THEN** 链式层级对齐飞书 URL `/open-apis/document_ai/v1/id_card/recognize`，每层 service 对应一个路径段

#### Scenario: OCR / Translation 两层导航对齐 URL

- **WHEN** 调用 `ai_client.ocr().v1().image().basic_recognize()` 或 `ai_client.translation().v1().text().translate()`
- **THEN** 链式层级对齐 `/open-apis/optical_char_recognition/v1/image/basic_recognize` 与 `/open-apis/translation/v1/text/translate`

### Requirement: openlark-ai endpoint URL 正确性

openlark-ai 的 endpoint 常量 SHALL 与 `api_list_export.csv`（飞书官方 API 清单）一致。历史上并存的错误 URL（如 `/speech_to_text/v1/file/recognize`、`/optical_char_recognition/v1/basic_recognize`）SHALL 在收敛中删除，不得保留。

#### Scenario: Speech endpoint 为 csv 官方 URL

- **WHEN** 变更后检查 speech_to_text endpoint 常量
- **THEN** file_recognize 为 `/open-apis/speech_to_text/v1/speech/file_recognize`、stream_recognize 为 `/open-apis/speech_to_text/v1/speech/stream_recognize`（对齐 csv），不存在 `/speech_to_text/v1/file/recognize` 等错误 URL

#### Scenario: 4 能力 endpoint 与 csv 全量对照

- **WHEN** 运行 validator 工具对照 `api_list_export.csv`
- **THEN** DocumentAi(18) / OCR(1) / Speech(2) / Translation(2) 的 endpoint URL 全部与 csv 一致，无错误 URL 残留
```

