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
