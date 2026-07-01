---
comet_change: untangle-ai-crate-navigation
role: technical-design
canonical_spec: openspec
---

# Design: untangle-ai-crate-navigation

## Context

openlark-ai crate 表面问题是 4 个 v1 入口 struct（`DocumentAiV1`/`Image`/`Speech`/`Text`）无访问器（#275）。深查后发现根因是**两代实现并存 + 3 套导航范式打架**：

- **第一代**（顶层 `document_ai/`、`speech_to_text/`）：层级拍平、URL 部分错误、覆盖不全（DocumentAi 5/18）
- **第二代**（`ai/` 下）：目录结构精确对齐飞书 API URL、覆盖完整（DocumentAi 18/18）、实现生产级，但入口孤儿未接进 `AiClient`
- **chain.rs**（范式 A，pub field）：唯一活链，但引用第一代 DocumentAi 实现（5 个）
- **ai::AiService + v1::V1**（范式 B，孤儿聚合）：全死，零外部引用
- **AiClient**（范式 C 候选）：openlark-client 注册的 canonical 入口，但 4 条链导向孤儿

事实基准来自 `api_list_export.csv`（飞书官方 API 清单），核对出 Speech B 套、OCR `/v1/basic_recognize` 等 URL 是错的（印证 [[validator-coverage-is-path-based]] 的 P0 URL bug 类）。

约束：v0.18 breaking 窗口开启；死链全部零外部引用；4 入口 struct 的 `_config` 为 #267 临时 reserved。

## Goals / Non-Goals

**Goals:**
- 收敛到单一 canonical 范式（AiClient + SparkV1 accessor），消除 3 套范式与同名 `DocumentAiClient` 冲突
- 4 能力（DocumentAi 18 + OCR 1 + Speech 2 + Translation 2 = 23 个 API）经 `AiClient` 链式可达
- 导航链层级对齐飞书 API URL 路径层级
- 废弃第一代实现 + 死链，修复 Speech/OCR 的错误 URL

**Non-Goals:**
- 不新增飞书 AI API（只给第二代已有实现装导航入口）
- 不重构已有 RequestBuilder 内部
- 不动 openlark-client ServiceRegistry 机制
- 不触碰其他业务 crate

## Decisions

### D1: 导航链层级对齐 URL（不拍平）
飞书 URL 本身有 `{resource}/{action}` 层级（如 `/document_ai/v1/id_card/recognize`、`/translation/v1/text/translate`）。导航链 SHALL 保留对应中间层：
- DocumentAi 三层：`.v1().{doc_type}().{action}()`
- OCR/Speech/Translation 两层：`.v1().{resource}().{action}()`

**理由**：第二代 `ai/` 下目录结构已如此组织；对齐 URL 让导航链可从 API 路径反推；与 SparkV1 深嵌套范式（如 platform 的 `.v1().application().object().record()`）一致。
**替代（否决）**：拍平为 `.v1().id_card_recognize()`——与第一代 chain.rs 拍平风格相同（正是要淘汰的），且 doc_type 多时方法名爆炸。

### D2: DocumentAi 18 doc_type 各建 service struct
DocumentAiV1 装 18 个 doc_type accessor（`.resume()`/`.id_card()`/...），每个返回一个 doc_type service（如 `IdCardService`），service 装 action accessor（`.recognize()`/`.parse()`）返回叶子 RequestBuilder。

**理由**：实现已存在（`ai/document_ai/v1/{doc_type}/{action}.rs` 18 个生产级文件），只装导航壳；对齐 URL 三层；doc_type 间 action 名冲突（多个 recognize）必须有中间层。
**工作量**：18 个 service struct 是大头，但机械工作可批量。

### D3: config 流转对齐 SparkV1
各级 service（入口、中间）持 `Arc<Config>`，叶子 accessor 解引用 clone 为 owned `Config` 喂 `RequestBuilder::new(config: Config)`。4 入口 struct 的 `_config` 恢复为 `config`。

**理由**：所有第二代 RequestBuilder 的 `new` 都是 owned `Config`；与 `v1-sub-api-accessors` 的「访问器 config 流转对齐 SparkV1」requirement 一致。

### D4: Speech = B 套完整实现 + A 套正确 URL
A 套（`ai/speech_to_text/v1/speech/`）URL 正确（`/v1/speech/{file,stream}_recognize`，csv 确认）但实现简陋（无 Builder、Response 用 `serde_json::Value` 未类型化）；B 套（顶层 `speech_to_text/speech_to_text/`）实现完整（Builder + 类型化 Result + free function + 测试）但 URL 错。

**决策**：把 B 套完整实现迁移到 A 套位置 + A 套 URL，删 B 套顶层目录 + 错误 URL 常量。csv 无 `speech_recognize`，B 套第 3 个直接删。

### D5: 死链与第一代全部删除（breaking，v0.18）
- 范式 B 死聚合：`ai::AiService` + `ai::v1::V1`
- 范式 A：`common/chain.rs` + `lib.rs:72` 顶层 `DocumentAiClient` re-export + `common/mod.rs` chain re-export
- 第一代 DocumentAi：顶层 `document_ai/document_ai/v1/recognize/`（5 个）
- 第一代 Speech B 套：顶层 `speech_to_text/speech_to_text/`（实现迁走后删）
- 错误 endpoint 常量：`/v1/basic_recognize`、`/v1/file/recognize`、`/v1/stream/recognize`、`/v1/speech/recognize`

**理由**：grep 全仓零外部引用；v0.18 breaking 窗口允许；保留与「单 canonical」目标矛盾。

### D6: feature 门控保持现状
`*Client.v1()` 已 `#[cfg(feature="v1")]`（`v1` feature 存在，`default=["v1"]`）。新增 accessor 沿用同一门控，不改。

## 导航链最终形态

```
AiClient（canonical，openlark-client ServiceRegistry 注册）
├─ .document_ai() → DocumentAiClient → .v1() → DocumentAiV1
│   ├─ .resume()    → ResumeService      → .parse()     → ResumeParseRequestBuilder
│   ├─ .id_card()   → IdCardService      → .recognize() → IdCardRecognizeRequestBuilder
│   ├─ .bank_card() → BankCardService    → .recognize() → ...
│   └─ ...（18 doc_type，对齐 /document_ai/v1/{doc_type}/{action}）
├─ .ocr() → OcrClient → .v1() → OcrV1 → .image() → Image → .basic_recognize() → BasicRecognizeRequestBuilder
├─ .speech_to_text() → SpeechToTextClient → .v1() → SpeechToTextV1 → .speech() → Speech
│   ├─ .file_recognize()  → FileRecognizeRequestBuilder（B 套完整实现）
│   └─ .stream_recognize() → StreamRecognizeRequestBuilder
└─ .translation() → TranslationClient → .v1() → TranslationV1 → .text() → Text
    ├─ .translate() → TextTranslateRequestBuilder
    └─ .detect()    → TextDetectRequestBuilder
```

OCR/Translation 的中间 accessor（`.image()`/`.text()`）已存在于 OcrV1/TranslationV1；DocumentAi/Speech 需补装。

## 实现工作清单（每能力待装 accessor）

| 能力 | v1 入口 | 中间层 | 叶子 |
|------|---------|--------|------|
| DocumentAi | DocumentAiV1：`_config`→`config` + 装 18 doc_type accessor | 建 18 个 doc_type service struct + action accessor | 已存在（18 个生产级实现） |
| OCR | OcrV1：`.image()` 已有 | Image：`_config`→`config` + 装 `.basic_recognize()` | 已存在 |
| Speech | SpeechToTextV1：装 `.speech()`（若无） | Speech：`_config`→`config` + 装 `.file_recognize()`/`.stream_recognize()` | 迁 B 套完整实现到 A 套位置 |
| Translation | TranslationV1：`.text()` 已有 | Text：`_config`→`config` + 装 `.translate()`/`.detect()` | 已存在 |

## Risks / Trade-offs

- **[BREAKING 公开 API]** 移除第一代 + chain + AiService + 字段改名 → v0.18 窗口允许；零外部引用故实际影响面为零；CHANGELOG 写迁移表
- **[工作量集中]** DocumentAi 18 doc_type service → 实现已存在，装壳是机械批量工作
- **[Speech B→A 迁移]** 需保持 Builder/类型化 Response/测试完整 → 沿用 B 套测试覆盖，迁移后跑全量测试
- **[URL 错误修复]** 删除 B 套错误 URL 是行为变更（虽无人用）→ csv 对照确认正确 URL 留存
- **[openlark-client 注册]** AiClient 类型签名与 `new(Config)` 不变 → 仅内部导航变化，client.rs:105 编译即验证

## Testing Strategy

- **沿用**：第二代现有测试（DocumentAi 18 个生产级测试、Speech 迁移 B 套的 9 个测试/文件）
- **新增**：访问器链可达性测试——每个能力从 `AiClient` 经 accessor 链到达叶子 RequestBuilder，断言返回类型正确
- **死链验证**：`cargo clippy -W dead_code` 于 openlark-ai，确认无死链残留、无 `_config` 字段
- **URL 正确性**：保留 `api_list_export.csv` 对照（validator 工具），防止错误 URL 复活
- **集成**：`cargo build -p openlark-client` 确认 AiClient 注册无碍

## Migration Plan

v0.18 breaking 项与用户迁移路径：
- 移除 `openlark_ai::DocumentAiClient`（chain 版）：用户改用 `AiClient::new(cfg).document_ai()...`
- 移除 `ai::AiService` / `ai::v1::V1`：零引用，无迁移
- 4 入口字段 `_config`→`config`：内部字段，不影响外部（构造仍经 `new`）
- Speech 错误 URL 修正：原 B 套 `/v1/file/recognize` 从未正确工作，迁移到 `/v1/speech/file_recognize`
- CHANGELOG 按 v0.17→v0.18 迁移表风格补充 ai 导航收敛条目

## Spec Patch（回写 v1-sub-api-accessors delta）

回写 `specs/v1-sub-api-accessors/spec.md`，ADDED Requirements：

1. **导航链层级对齐 URL**：openlark-ai 的 v1 入口 accessor 链 SHALL 对齐飞书 API URL 路径层级（`/v1/{resource}/{action}` → `.v1().{resource}().{action}()`），中间层 service 对应 URL 路径段。
   - Scenario: DocumentAi 三层 `.v1().id_card().recognize()` 对齐 `/document_ai/v1/id_card/recognize`
   - Scenario: OCR/Translation 两层 `.v1().image().basic_recognize()` 对齐 `/v1/image/basic_recognize`

2. **endpoint URL 正确性**：openlark-ai 的 endpoint 常量 SHALL 与 `api_list_export.csv` 一致；废弃错误 URL（`/v1/file/recognize` 等）。
   - Scenario: Speech endpoint 为 `/v1/speech/file_recognize`（非 `/v1/file/recognize`）
   - Scenario: 4 能力 endpoint 与 csv 官方 URL 全量对照通过
