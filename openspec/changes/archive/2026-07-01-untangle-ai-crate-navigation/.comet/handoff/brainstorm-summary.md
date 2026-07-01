# Brainstorm Summary

- Change: untangle-ai-crate-navigation
- Date: 2026-06-30

## 已确认事实（csv 官方基准 api_list_export.csv 核对）

### URL 真伪对照表（设计根基）

| 能力 | 官方 URL（csv） | csv 数量 | 代码现状 | 导航链层级（已定） |
|------|----------------|---------|---------|-------------------|
| DocumentAi | `/document_ai/v1/{doc_type}/{action}` | 18 | 第二代 `ai/document_ai/v1/{doc_type}/` 18 个 ✅ 对齐；第一代顶层 `document_ai/` 5 个 ❌ 拍平不全 | `.v1().{doc_type}().{action}()` 三层 |
| OCR | `/optical_char_recognition/v1/image/basic_recognize` | 1 | `ai/.../image/basic_recognize` ✅；多余常量 `/v1/basic_recognize` ❌ | `.v1().image().basic_recognize()` 两层 |
| Speech | `/speech_to_text/v1/speech/{file,stream}_recognize` | 2 | A 套 `ai/.../speech/` URL ✅ 对但实现简陋；B 套顶层 URL ❌ 错但实现完整 | `.v1().speech().{file,stream}_recognize()` 两层 |
| Translation | `/translation/v1/text/{detect,translate}` | 2 | `ai/.../text/` ✅ 对齐 | `.v1().text().{detect,translate}()` 两层 |

### 核心发现：两代实现并存（重新定义 change 性质）

ai crate 的混乱本质是**第一代 vs 第二代实现并存**：
- **第一代**（顶层 `document_ai/`、`speech_to_text/`）：早期实现，层级拍平、URL 部分错误（Speech B 套 `/v1/file/recognize` 不在 csv）、覆盖不全（DocumentAi 5/18）
- **第二代**（`ai/` 下）：对齐 URL 层级、覆盖完整（DocumentAi 18/18）、目录结构精确映射 URL，但入口孤儿未接进 AiClient

**chain.rs 的 RecognizeResource 活链引用的是第一代 DocumentAi 实现**（5 个）——虽活但接的是旧/不全的实现。

### 正确 untangle 方向（已确认）
全面采用第二代（`ai/` 下）实现 → 废弃第一代（顶层）→ 把 AiClient 链接到第二代。**不是迁移 chain.rs 旧实现**（这修正了 open 阶段 proposal 的措辞）。

## 5 个 Open Questions 的答案（URL 已定）

1. **导航层级**：保留 URL 对齐的中间层，不拍平。DocumentAi 三层、OCR/Speech/Translation 两层。
2. **OCR Image / Translation Text 中间层**：保留（URL 官方就是 `/image/`、`/text/`）。
3. **Speech 合并**：用 A 套 URL（csv 官方），实现需补全（A 套简陋）；B 套 URL 错删之，csv 无 speech_recognize 故 B 套第 3 个也删。**补全方式待确认**。
4. **feature 门控**：保持现状（`*Client.v1()` 已 `#[cfg(feature="v1")]`），不改。
5. **实现归位**：统一到 `ai/{capability}/v1/{resource}/{action}.rs`，废弃顶层第一代目录。

## 已确认决策（用户 brainstorming 确认）

- **DocumentAi 层级**：三层对齐 URL（`.v1().{doc_type}().{action}()`），为 18 个 doc_type 各建 service struct + action accessor，DocumentAiV1 装 18 个 doc_type accessor。对齐 SparkV1 深嵌套范式。
- **Speech 合并**：B 套完整实现（Builder + 类型化 Result + free function + 测试）迁到 A 套位置 `ai/speech_to_text/v1/speech/` + A 套正确 URL（`/v1/speech/{file,stream}_recognize`），删 B 套顶层 + 错误 URL 常量 + csv 无的 speech_recognize。
- **第一代删除**：顶层 `document_ai/`（5 个）+ 顶层 `speech_to_text/`（B 套）+ chain.rs + AiService 死聚合，全部删除（v0.18 breaking，零外部引用）。
- **导航层级**：保留 URL 对齐的中间层，不拍平（DocumentAi 三层、OCR/Speech/Translation 两层）。
- **feature 门控**：保持现状 `#[cfg(feature="v1")]`，不改。

## 中间层 service 现状（决定 accessor 工作量）

第二代 `ai/` 下中间层状态不一致：

| 能力 | v1 入口 | 中间层 accessor | 中间层 service | 叶子实现 |
|------|---------|----------------|---------------|---------|
| DocumentAi | `DocumentAiV1`（孤儿 _config）| ❌ 无 | ❌ 18 doc_type 无 service struct（id_card/mod.rs 仅 `pub mod recognize`）| ✅ 18 个完整（resume/parse.rs 生产级）|
| OCR | `OcrV1` | ✅ 已装 `.image()` | `Image`（孤儿 _config）| ✅ basic_recognize 完整 |
| Speech | `SpeechToTextV1` | 待确认 `.speech()` | `Speech`（孤儿）| A 套简陋（无 Builder/类型化 Response）|
| Translation | `TranslationV1` | ✅ 已装 `.text()` | `Text`（孤儿 _config）| ✅ translate/detect |

DocumentAi 是工作量最大的部分：18 个 doc_type 需建 service struct + accessor + DocumentAiV1 装 18 个 doc_type accessor。

## 最终导航链设计（候选，待用户确认 DocumentAi 层级）

```
AiClient（canonical，openlark-client 注册）
├─ .document_ai() → DocumentAiClient → .v1() → DocumentAiV1
│   ├─ .resume() → ResumeService → .parse() → ResumeParseRequestBuilder
│   ├─ .id_card() → IdCardService → .recognize() → ...
│   └─ ...（18 doc_type 三层，对齐 URL /document_ai/v1/{doc_type}/{action}）
├─ .ocr() → OcrClient → .v1() → OcrV1 → .image() → Image → .basic_recognize() → ...
├─ .speech_to_text() → ... → SpeechToTextV1 → .speech() → Speech → .file_recognize()/.stream_recognize()
│                                          （B 套完整实现 + A 套正确 URL）
└─ .translation() → ... → TranslationV1 → .text() → Text → .translate()/.detect()
```

config 流转：各级 service 持 `Arc<Config>`，叶子 accessor 解引用 clone 为 owned `Config` 喂 `RequestBuilder::new(Config)`（对齐 SparkV1）。

## 死链/冗余删除清单

- 范式 B 死聚合：`ai::AiService` + `ai::v1::V1`
- 范式 A：`common/chain.rs` + `lib.rs:72` 顶层 DocumentAiClient re-export + `common/mod.rs` chain re-export
- 第一代 DocumentAi：顶层 `document_ai/document_ai/v1/recognize/`（5 个，被 chain.rs 引用）
- 第一代 Speech B 套：顶层 `speech_to_text/speech_to_text/`（实现迁到 A 套位置后删）
- 错误 endpoint 常量：`/v1/basic_recognize`、`/v1/file/recognize`、`/v1/stream/recognize`、`/v1/speech/recognize`

## 测试策略

- 沿用第二代现有测试（DocumentAi 18 个已有完整测试、Speech 迁移 B 套带测试）
- 新增访问器链可达性测试：每个能力 `AiClient → 叶子 RequestBuilder` 路径返回正确类型
- `cargo clippy -W dead_code` 验证死链清零、无 _config 残留
- URL 正确性保留 `api_list_export.csv` 对照（validator 工具），防错误 URL 复活
- openlark-client 注册的 AiClient 编译通过（client.rs:105）

## Spec Patch 候选

- v1-sub-api-accessors delta：补充「导航链层级 SHALL 对齐飞书 API URL 路径层级」的验收场景（当前 spec 只说"链式 accessor"，未明确层级对齐 URL）
- 补充 Speech/OCR 的 URL 正确性场景（防 B 套错误 URL 复活）
