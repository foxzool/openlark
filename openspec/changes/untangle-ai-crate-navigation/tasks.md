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
