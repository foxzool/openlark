## 1. 死代码清理（范式 B 聚合体）

- [x] 1.1 删除 `ai::AiService` + `ai::v1::V1` 死聚合（含死聚合版 4 入口 struct `ai::v1::{DocumentAiV1,OcrV1,SpeechToTextV1,TranslationV1}`，与第二代同名不同物）；`service.rs` 四处 `*Client.v1()` 重定向到第二代 `ai::{capability}::v1::*` 真实入口（保留第二代 4 入口）。验证：`cargo test -p openlark-ai --lib` → 207 passed。

## 2. 4 能力 accessor 接线（核心）

- [x] 2.1 `DocumentAiV1`：装 18 doc_type accessor（三层 `.v1().{doc_type}().{action}()` 对齐 URL）+ 18 `{DocType}Service` + action accessor（17 recognize + resume.parse + contract.field_extraction）。验证：cargo test document_ai → 107 passed（含可达性测试）。
- [x] 2.2 `OcrV1.image()`（中间层已存在，保留对齐 URL `/v1/image/`）；`Image._config`→`config` + 装 `.basic_recognize()` → `BasicRecognizeRequestBuilder`。验证：`ai_client.ocr().v1().image().basic_recognize()` 可达（test_ocr_accessor_chain ok）。
- [x] 2.3 `SpeechToTextV1.speech()`（中间层已存在）；B 套完整实现迁移到 A 套位置 `ai/speech_to_text/v1/speech/` + A 套正确 URL（`/v1/speech/{file,stream}_recognize`）；`Speech._config`→`config` + 装 `.file_recognize()`/`.stream_recognize()`。验证：cargo test speech → 64 passed（含可达性测试）。注：B 套顶层目录 T10 删除；csv 无 speech_recognize。
- [x] 2.4 `TranslationV1.text()`（中间层已存在，保留对齐 URL `/v1/text/`）；`Text._config`→`config` + 装 `.translate()`/`.detect()` → `TextTranslate/TextDetectRequestBuilder`。验证：`ai_client.translation().v1().text().{translate,detect}()` 可达（test_translation_accessor_chain ok）。
- [x] 2.5 4 个 `_config` 字段（第二代 `DocumentAiV1` + `Image`/`Text`/`Speech` 中间层）全部恢复为 `config` 并被 accessor 消费（清除 #274 reserved 注释）。验证：T13 终检 `grep -rn "_config" ai/` 无残留。

## 3. 范式 A（chain.rs）退役

- [x] 3.1 删除 `src/common/chain.rs` + `common/mod.rs` chain re-export + `lib.rs` 顶层 `DocumentAiClient` re-export + 第一代 `src/document_ai/`（chain.rs 引用源）。验证：cargo test --lib → 198 passed；同名 `DocumentAiClient` 仅剩 `service::`（T13 grep 终检）。
- [x] 3.2 删除第一代顶层 `speech_to_text`（B 套源，已迁 A 套）+ 删除错误 URL 常量（OCR `BASIC_RECOGNIZE` / Speech `FILE`·`STREAM`·`SPEECH_RECOGNIZE` + 别名，非 csv 官方）+ `tests/document_ai_test.rs` 改用第二代 Builder（bank_card 用 `file`/`file_name`）。验证：cargo test 全量（lib+integration+doctest）全过。

## 4. lib.rs 与 openlark-client 收尾

- [x] 4.1 `lib.rs` 清理：chain 版 `DocumentAiClient` 已移除（T9）；`AiClient`/endpoints 导出完整；文档示例改为 `.document_ai().v1().id_card().recognize()`。验证：cargo doc 无 intra-doc 断链（bare_urls 为既有 docPath）。
- [x] 4.2 `openlark-client` client.rs AiClient 注册 + prelude 导出编译通过。验证：`cargo build -p openlark-client` → Finished。

## 5. 质量验证

- [x] 5.1 `cargo fmt --check`（全仓）通过。
- [x] 5.2 `cargo clippy --all-features --workspace` 通过，openlark-ai 无 dead_code 告警。
- [x] 5.3 `cargo test --all-features -p openlark-ai` 通过（含访问器链可达性测试）。
- [x] 5.4 终检：同名 `DocumentAiClient` 消除、Speech 实现单套、范式 A/B 死链清零、4 入口字段均为 `config`。

## Review Gate（review_mode=standard）

代码审查通过（reviewer subagent，base-ref ad4489b..HEAD，Ready to merge: Yes）：
- Critical：无；Important：无
- 3 个 Minor 均为预先存在的 codegen 模板问题（placeholder 测试 / `use serde_json` 冗余 / doctest 注释），非本 PR 引入，接受不修（surgical 原则，避免超范围）
- 审查重点全过：导航链对齐 URL、endpoint 与 `api_list_export.csv` 全量一致、Speech 迁移完整 URL 改对、breaking 删除零外部引用、config 流转一致
