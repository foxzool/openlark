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
