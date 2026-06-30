---
change: untangle-ai-crate-navigation
design-doc: docs/superpowers/specs/2026-06-30-untangle-ai-crate-navigation-design.md
base-ref: ad4489b94752be032c897c26b8e206244b90d388
---

# untangle-ai-crate-navigation 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 按任务逐个实施。步骤用 checkbox（`- [ ]`）语法跟踪。

**Goal:** 把 openlark-ai 的 3 套并行导航范式收敛为单一 canonical（AiClient + 第二代 ai/ 下实现），4 能力（DocumentAi 18 + OCR 1 + Speech 2 + Translation 2）经 AiClient 链式可达，对齐飞书 URL 层级。

**Architecture:** 切换 `*Client.v1()` 到第二代 `ai/{capability}/v1/` 真实入口；为 4 能力装 accessor 把导航链接到已存在的叶子 RequestBuilder；迁移 Speech B 套完整实现到 A 套位置（保留 A 套正确 URL）；删除第一代顶层实现 + chain.rs + 范式 B 死聚合 + 错误 URL 常量。

**Tech Stack:** Rust 1.88+，openlark-core（Config/RequestBuilder/Transport），serde， tokio。crate：`openlark-ai`（主战场）、`openlark-client`（注册校验）。

## Global Constraints

- **导航层级对齐 URL**：DocumentAi 三层 `.v1().{doc_type}().{action}()`，OCR/Speech/Translation 两层 `.v1().{resource}().{action}()`。中间层 service 对应 URL 路径段，不拍平（D1）。
- **config 流转对齐 SparkV1（D3）**：各级 service（入口、中间层）持 `Arc<Config>`，叶子 accessor 解引用 clone 为 owned `Config` 喂 `XxxRequestBuilder::new(Config)`。
- **叶子 accessor 返回 `RequestBuilder`**（不是 `Request`）：与 chain.rs 旧实现和 Design Doc 导航链图一致，保持迁移前后公开 API 形态。
- **feature 门控保持现状（D6）**：`*Client.v1()` 已 `#[cfg(feature = "v1")]`，新增 accessor 沿用，不改 Cargo.toml features。
- **breaking 窗口**：v0.18 允许移除公开 API（第一代 + chain + AiService），零外部引用故影响面为零。
- **中文优先**：文档注释、commit message、doc 注释用中文；struct/方法名用英文 snake/PascalCase。
- **库代码禁用 `unwrap()`/`expect()`**。
- **每个 task 验收后**：tasks.md 对应项打勾 → `git commit`（不积攒）。
- **base-ref**：`ad4489b94752be032c897c26b8e206244b90d388`（main HEAD）。

## 关键事实校正（核查代码后，与 brainstorm-summary 的差异）

实施者必读，避免按过时描述操作：

1. **两份 `DocumentAiV1`**：
   - `ai::v1::DocumentAiV1`（**范式 B 死聚合**，`ai/v1/mod.rs`，字段 `config`，被 `service.rs` 的 `DocumentAiClient.v1()` 当前引用）
   - `ai::document_ai::v1::DocumentAiV1`（**第二代真实入口**，`ai/document_ai/v1/mod.rs`，字段 `_config`，孤儿待装 accessor）
   - 任务核心：把 `*Client.v1()` 从死聚合版重定向到第二代真实入口。
2. **字段 `_config` 仅存在于 4 处**（不是 brainstorm 说的"4 个 v1 入口"）：第二代 `ai::document_ai::v1::DocumentAiV1` + 3 个中间层 `Image`/`Text`/`Speech`。死聚合的 4 个 v1 入口 struct 字段已是 `config`。
3. **endpoints 两套 URL 常量并存**：正确（A 套/对齐 URL，如 `SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE`）+ 错误（B 套/拍平，如 `SPEECH_TO_TEXT_V1_FILE_RECOGNIZE=/v1/file/recognize`）。A 套叶子已用正确常量。
4. **A 套 Speech 实现简陋**（无 Builder、Response 用 `serde_json::Value`），**B 套完整**（Builder + 类型化 Result）。迁移 = B 套内容覆盖 A 套文件 + 把 B 套的 URL 常量引用改回 A 套正确的。
5. **SpeechToTextV1/TranslationV1/OcrV1 的中间层 accessor（`.speech()`/`.text()`/`.image()`）已存在**，无需补装；只需让中间层 service 装 action accessor。

## 文件结构（创建/修改/删除总览）

**修改：**
- `crates/openlark-ai/src/service.rs` — `*Client.v1()` 重定向到第二代入口（DocumentAiV1 重定向 + 其他 3 个确认）
- `crates/openlark-ai/src/ai/v1/mod.rs` — 删除死聚合 4 入口 struct + `V1` 聚合（保留 `DocumentAiV1` 等的同名重导出？否，直接删，service.rs 不再引用）
- `crates/openlark-ai/src/ai/mod.rs` — 删除 `AiService` + `pub mod v1`
- `crates/openlark-ai/src/ai/document_ai/v1/mod.rs` — `DocumentAiV1._config`→`config` + 装 18 doc_type accessor
- `crates/openlark-ai/src/ai/optical_char_recognition/v1/image/mod.rs` — `Image._config`→`config` + 装 `.basic_recognize()`
- `crates/openlark-ai/src/ai/translation/v1/text/mod.rs` — `Text._config`→`config` + 装 `.translate()`/`.detect()`
- `crates/openlark-ai/src/ai/speech_to_text/v1/speech/mod.rs` — `Speech._config`→`config` + 装 `.file_recognize()`/`.stream_recognize()`
- `crates/openlark-ai/src/ai/speech_to_text/v1/speech/file_recognize.rs` — 用 B 套完整实现覆盖 + 保留 A 套正确 URL 常量
- `crates/openlark-ai/src/ai/speech_to_text/v1/speech/stream_recognize.rs` — 同上
- `crates/openlark-ai/src/common/mod.rs` — 删 `pub mod chain` + chain re-export
- `crates/openlark-ai/src/lib.rs` — 删 line 72 `pub use common::chain::DocumentAiClient` + line 59 `pub mod speech_to_text`（第一代）+ line 55 `pub mod document_ai`（第一代）+ 更新模块文档示例
- `crates/openlark-ai/src/endpoints/mod.rs` — 删错误 URL 常量 + 别名 + 相关测试

**创建（18 个 doc_type service struct，机械批量）：**
- `crates/openlark-ai/src/ai/document_ai/v1/{doc_type}/mod.rs` 各补一个 `{DocType}Service` struct + action accessor（18 个文件修改，非新建）

**删除：**
- `crates/openlark-ai/src/common/chain.rs`
- `crates/openlark-ai/src/document_ai/`（第一代顶层，整个目录）
- `crates/openlark-ai/src/speech_to_text/`（第一代/B 套顶层，整个目录）

**新增测试：**
- `crates/openlark-ai/src/service.rs` 的 `#[cfg(test)] mod tests` — 4 能力访问器链可达性测试

---

## Task 1: 删除范式 B 死聚合（AiService + V1）

**Files:**
- Modify: `crates/openlark-ai/src/ai/mod.rs`
- Modify: `crates/openlark-ai/src/ai/v1/mod.rs`
- Test: `cargo build -p openlark-ai`（编译验证）

**Interfaces:**
- Consumes: `service.rs` 当前 `*Client.v1()` 引用 `super::ai::v1::{DocumentAiV1,OcrV1,SpeechToTextV1,TranslationV1}`（死聚合版）——本 task 先**不**改 service.rs，会临时编译失败；本 task 与 Task 2/4（重定向）在同一次提交内完成才能编译通过。
- Produces: `ai::v1` 模块不再有死聚合 `V1` 和 4 个入口 struct。

**重要**：本 task 删除 `ai/v1/mod.rs` 里的死聚合 `V1` + `DocumentAiV1`/`OcrV1`/`SpeechToTextV1`/`TranslationV1`（这 4 个是死聚合版，与第二代 `ai::document_ai::v1::DocumentAiV1` 同名但不同物）。删除后 `service.rs` 的 `*Client.v1()` 会断引用——**Task 2 紧接着重定向**，两个 task 在一次提交内完成。

- [ ] **Step 1: 重写 `ai/v1/mod.rs`，删除死聚合**

把 `crates/openlark-ai/src/ai/v1/mod.rs` 整个文件替换为空模块占位（即将整个删除）。实际上最干净的做法是**删除 `ai/v1/` 目录**并在 `ai/mod.rs` 移除 `pub mod v1`。先在 `ai/mod.rs` 删除 `pub mod v1`：

修改 `crates/openlark-ai/src/ai/mod.rs`，删除以下行：
```rust
/// v1 模块。
pub mod v1;
```
并删除整个 `AiService` struct + `impl AiService` + `impl Service for AiService` + `use crate::prelude::Config;` + `use openlark_core::trait_system::Service;` + `#[cfg(test)] mod tests`（AiService 相关）。

保留 `ai/mod.rs` 的：`#![allow(clippy::module_inception)]` + 模块文档 + 4 个 `pub mod {document_ai,optical_char_recognition,speech_to_text,translation};`。

- [ ] **Step 2: 删除 `ai/v1/` 目录**

```bash
rm -rf crates/openlark-ai/src/ai/v1
```

- [ ] **Step 3: 暂不编译（已知 service.rs 断引用，Task 2 修复）**

预期：`cargo build -p openlark-ai` 此时**会失败**（`service.rs` 引用 `super::ai::v1::DocumentAiV1` 等）。这是预期的，Task 2 重定向后即恢复。**不要单独提交本 task**——与 Task 2 一起提交。

**Commit（与 Task 2 合并提交）：** 见 Task 2 末尾。

---

## Task 2: 重定向 `*Client.v1()` 到第二代入口（编译恢复）

**Files:**
- Modify: `crates/openlark-ai/src/service.rs:DocumentAiClient.v1()` 等四处
- Test: `cargo build -p openlark-ai`

**Interfaces:**
- Consumes: Task 1 删除了死聚合；第二代入口已存在（`ai::document_ai::v1::DocumentAiV1` 等，字段 `_config` 待 Task 5 改名，但 `new(Arc<Config>)` 签名已对）。
- Produces: `*Client.v1()` 返回第二代入口类型；编译恢复。

- [ ] **Step 1: 修改 `service.rs` 四处 `v1()` 返回类型**

把 `crates/openlark-ai/src/service.rs` 中四处 `*Client.v1()` 的返回类型从死聚合路径改为第二代路径：

```rust
// DocumentAiClient::v1
#[cfg(feature = "v1")]
pub fn v1(&self) -> super::ai::document_ai::v1::DocumentAiV1 {
    super::ai::document_ai::v1::DocumentAiV1::new(self.config.clone())
}

// OcrClient::v1
#[cfg(feature = "v1")]
pub fn v1(&self) -> super::ai::optical_char_recognition::v1::OcrV1 {
    super::ai::optical_char_recognition::v1::OcrV1::new(self.config.clone())
}

// SpeechToTextClient::v1
#[cfg(feature = "v1")]
pub fn v1(&self) -> super::ai::speech_to_text::v1::SpeechToTextV1 {
    super::ai::speech_to_text::v1::SpeechToTextV1::new(self.config.clone())
}

// TranslationClient::v1
#[cfg(feature = "v1")]
pub fn v1(&self) -> super::ai::translation::v1::TranslationV1 {
    super::ai::translation::v1::TranslationV1::new(self.config.clone())
}
```

注意：第二代入口的 `new` 签名是 `pub fn new(config: Arc<Config>) -> Self`（与死聚合一致），`self.config.clone()` 产出 `Arc<Config>`，签名匹配。

- [ ] **Step 2: 编译验证**

Run: `cargo build -p openlark-ai`
Expected: PASS（第二代入口 `new` 签名与死聚合相同，重定向后即恢复编译）。如失败检查 `OcrV1`/`TranslationV1`/`SpeechToTextV1` 的 `new` 签名是否确为 `Arc<Config>`（已核查：是）。

- [ ] **Step 3: 提交（合并 Task 1 + Task 2）**

```bash
git add crates/openlark-ai/src/ai/mod.rs crates/openlark-ai/src/service.rs
# ai/v1 目录已删除，git add -A 捕获删除
git add -A crates/openlark-ai/src/ai/v1
git commit -m "refactor(ai): 删除范式 B 死聚合，*Client.v1() 重定向到第二代入口

- 删除 ai::AiService + ai::v1::V1 死聚合（范式 B，零外部引用）
- 删除 ai/v1/mod.rs 中 4 个死聚合入口 struct（与第二代同名不同物）
- service.rs 四个 *Client.v1() 改指向 ai::{capability}::v1::* 第二代真实入口
- 编译恢复（第二代 new(Arc<Config>) 签名与死聚合一致）

Refs: #275"
```

---

## Task 3: DocumentAi — 18 doc_type service struct + DocumentAiV1 accessor（核心，机械批量）

**Files:**
- Modify: `crates/openlark-ai/src/ai/document_ai/v1/mod.rs`（DocumentAiV1 `_config`→`config` + 装 18 accessor）
- Modify: `crates/openlark-ai/src/ai/document_ai/v1/{doc_type}/mod.rs` × 18（各建 service struct + action accessor）
- Test: `cargo test -p openlark-ai document_ai`（已有 18 个叶子测试）

**Interfaces:**
- Consumes: 18 个叶子 `XxxRequestBuilder::new(Config)`（已存在，owned Config）。
- Produces: `DocumentAiV1::{doc_type}() -> {DocType}Service`，`{DocType}Service::{action}() -> XxxRequestBuilder`。

**doc_type → service 名 → action 文件 → Builder 类型 映射表（18 个）：**

| doc_type | Service 名 | action 文件 | action 方法 | Builder 类型 |
|----------|-----------|------------|------------|-------------|
| resume | ResumeService | parse.rs | parse() | ResumeParseRequestBuilder |
| id_card | IdCardService | recognize.rs | recognize() | IdCardRecognizeRequestBuilder |
| bank_card | BankCardService | recognize.rs | recognize() | BankCardRecognizeRequestBuilder |
| business_card | BusinessCardService | recognize.rs | recognize() | BusinessCardRecognizeRequestBuilder |
| business_license | BusinessLicenseService | recognize.rs | recognize() | BusinessLicenseRecognizeRequestBuilder |
| chinese_passport | ChinesePassportService | recognize.rs | recognize() | ChinesePassportRecognizeRequestBuilder |
| contract | ContractService | field_extraction.rs | field_extraction() | ContractFieldExtractionRequestBuilder |
| driving_license | DrivingLicenseService | recognize.rs | recognize() | DrivingLicenseRecognizeRequestBuilder |
| food_manage_license | FoodManageLicenseService | recognize.rs | recognize() | FoodManageLicenseRecognizeRequestBuilder |
| food_produce_license | FoodProduceLicenseService | recognize.rs | recognize() | FoodProduceLicenseRecognizeRequestBuilder |
| health_certificate | HealthCertificateService | recognize.rs | recognize() | HealthCertificateRecognizeRequestBuilder |
| hkm_mainland_travel_permit | HkmMainlandTravelPermitService | recognize.rs | recognize() | HkmMainlandTravelPermitRecognizeRequestBuilder |
| tw_mainland_travel_permit | TwMainlandTravelPermitService | recognize.rs | recognize() | TwMainlandTravelPermitRecognizeRequestBuilder |
| taxi_invoice | TaxiInvoiceService | recognize.rs | recognize() | TaxiInvoiceRecognizeRequestBuilder |
| train_invoice | TrainInvoiceService | recognize.rs | recognize() | TrainInvoiceRecognizeRequestBuilder |
| vat_invoice | VatInvoiceService | recognize.rs | recognize() | VatInvoiceRecognizeRequestBuilder |
| vehicle_invoice | VehicleInvoiceService | recognize.rs | recognize() | VehicleInvoiceRecognizeRequestBuilder |
| vehicle_license | VehicleLicenseService | recognize.rs | recognize() | VehicleLicenseRecognizeRequestBuilder |

**批量模式说明**：17 个 `recognize.rs` 的 service struct 模板完全相同（只改 doc_type 名 + Builder 类型名）；`resume/parse.rs` 和 `contract/field_extraction.rs` 仅 action 方法名不同。可按下方模板批量生成。

- [ ] **Step 1: 写一个 doc_type service struct（以 id_card 为模板）**

修改 `crates/openlark-ai/src/ai/document_ai/v1/id_card/mod.rs`，在 `pub mod recognize;` 之后追加：

```rust
use openlark_core::config::Config;
use std::sync::Arc;

/// id_card 资源服务（对齐 URL `/document_ai/v1/id_card`）。
#[derive(Debug, Clone)]
pub struct IdCardService {
    config: Arc<Config>,
}

impl IdCardService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 身份证识别（对齐 URL `/document_ai/v1/id_card/recognize`）。
    pub fn recognize(&self) -> recognize::IdCardRecognizeRequestBuilder {
        recognize::IdCardRecognizeRequestBuilder::new((*self.config).clone())
    }
}
```

- [ ] **Step 2: 跑 id_card 现有测试确认未破坏叶子**

Run: `cargo test -p openlark-ai --lib document_ai::v1::id_card`
Expected: PASS（叶子文件未改，只是补了上层 service）。

- [ ] **Step 3: 批量套用模板到其余 16 个 recognize doc_type**

对其余 16 个 `recognize.rs` 类 doc_type（bank_card / business_card / business_license / chinese_passport / driving_license / food_manage_license / food_produce_license / health_certificate / hkm_mainland_travel_permit / tw_mainland_travel_permit / taxi_invoice / train_invoice / vat_invoice / vehicle_invoice / vehicle_license），在各 `mod.rs` 的 `pub mod recognize;` 后追加同模板，替换 3 处：struct/impl 名（`{DocType}Service`）、doc 注释 doc_type 名、Builder 类型名（`recognize::{DocType}RecognizeRequestBuilder`）。

- [ ] **Step 4: resume doc_type（action=parse）**

修改 `crates/openlark-ai/src/ai/document_ai/v1/resume/mod.rs`，在 `pub mod parse;` 后追加：

```rust
use openlark_core::config::Config;
use std::sync::Arc;

/// resume 资源服务（对齐 URL `/document_ai/v1/resume`）。
#[derive(Debug, Clone)]
pub struct ResumeService {
    config: Arc<Config>,
}

impl ResumeService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 简历解析（对齐 URL `/document_ai/v1/resume/parse`）。
    pub fn parse(&self) -> parse::ResumeParseRequestBuilder {
        parse::ResumeParseRequestBuilder::new((*self.config).clone())
    }
}
```

- [ ] **Step 5: contract doc_type（action=field_extraction）**

修改 `crates/openlark-ai/src/ai/document_ai/v1/contract/mod.rs`，在 `pub mod field_extraction;` 后追加同模板，struct 名 `ContractService`，方法：

```rust
    /// 合同字段提取（对齐 URL `/document_ai/v1/contract/field_extraction`）。
    pub fn field_extraction(&self) -> field_extraction::ContractFieldExtractionRequestBuilder {
        field_extraction::ContractFieldExtractionRequestBuilder::new((*self.config).clone())
    }
```

- [ ] **Step 6: DocumentAiV1 装 18 doc_type accessor + `_config`→`config`**

修改 `crates/openlark-ai/src/ai/document_ai/v1/mod.rs`：

1. 把 4 个 doc_type service struct 通过 `use` 引入（或直接用路径 `id_card::IdCardService`）。推荐在 accessor 内直接用路径，无需 use。
2. 把 `DocumentAiV1` 的字段 `_config` 改为 `config`，删除 `// reserved：待装访问器/execute（见 #274，不完整脚手架）` 注释。
3. 在 `impl DocumentAiV1` 内补 18 个 accessor（每个返回对应 `{DocType}Service`），并在 `new` 后保留。

`impl DocumentAiV1` 最终形态（节选，18 个 accessor 全列出）：

```rust
impl DocumentAiV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// resume 资源（对齐 URL `/document_ai/v1/resume`）。
    pub fn resume(&self) -> resume::ResumeService {
        resume::ResumeService::new(self.config.clone())
    }

    /// id_card 资源（对齐 URL `/document_ai/v1/id_card`）。
    pub fn id_card(&self) -> id_card::IdCardService {
        id_card::IdCardService::new(self.config.clone())
    }

    /// bank_card 资源。
    pub fn bank_card(&self) -> bank_card::BankCardService {
        bank_card::BankCardService::new(self.config.clone())
    }

    // ...（business_card / business_license / chinese_passport / contract /
    //      driving_license / food_manage_license / food_produce_license /
    //      health_certificate / hkm_mainland_travel_permit /
    //      tw_mainland_travel_permit / taxi_invoice / train_invoice /
    //      vat_invoice / vehicle_invoice / vehicle_license 共 18 个，
    //      每个返回 {doc_type}::{DocType}Service，传入 self.config.clone()）
}
```

`DocumentAiV1` struct 改为：

```rust
#[derive(Clone)]
pub struct DocumentAiV1 {
    config: Arc<Config>,
}
```

（删除 `_config` 前下划线和 reserved 注释。`#[derive(Debug, Clone)]` 中 `Debug` 可选，原文件无 Debug，保持 `#[derive(Clone)]` 即可。）

- [ ] **Step 7: 编译验证**

Run: `cargo build -p openlark-ai`
Expected: PASS。

- [ ] **Step 8: 跑 DocumentAi 全量叶子测试**

Run: `cargo test -p openlark-ai --lib document_ai`
Expected: PASS（18 个 doc_type 现有测试不破坏）。

- [ ] **Step 9: 提交**

```bash
git add crates/openlark-ai/src/ai/document_ai/
git commit -m "feat(ai/document_ai): 装 18 doc_type service + DocumentAiV1 accessor

- DocumentAiV1._config → config（清除 #274 reserved）
- DocumentAiV1 装 18 个 doc_type accessor（.resume()/.id_card()/...）
- 18 doc_type 各建 {DocType}Service + action accessor
  （17 recognize + resume.parse + contract.field_extraction）
- 导航链三层对齐 URL：.v1().{doc_type}().{action}()
- 叶子实现已存在（18 个生产级 RequestBuilder），仅装导航壳

Refs: #275"
```

---

## Task 4: OCR — Image 中间层装 `.basic_recognize()`

**Files:**
- Modify: `crates/openlark-ai/src/ai/optical_char_recognition/v1/image/mod.rs`
- Test: `cargo test -p openlark-ai optical_char_recognition`

**Interfaces:**
- Consumes: `BasicRecognizeRequestBuilder::new(Config)`（已存在于 `basic_recognize.rs:141`）。
- Produces: `Image::basic_recognize() -> BasicRecognizeRequestBuilder`。

- [ ] **Step 1: 修改 `image/mod.rs`**

把 `crates/openlark-ai/src/ai/optical_char_recognition/v1/image/mod.rs` 的 `Image` struct + impl 替换为：

```rust
/// Image OCR API
#[derive(Clone)]
pub struct Image {
    config: Arc<Config>,
}

impl Image {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// OCR 基础识别（对齐 URL `/optical_char_recognition/v1/image/basic_recognize`）。
    pub fn basic_recognize(&self) -> basic_recognize::BasicRecognizeRequestBuilder {
        basic_recognize::BasicRecognizeRequestBuilder::new((*self.config).clone())
    }
}
```

（`_config`→`config`，删 reserved 注释，加 accessor。保留 `pub mod basic_recognize;` 和测试模块。）

- [ ] **Step 2: 编译 + 测试**

Run: `cargo build -p openlark-ai && cargo test -p openlark-ai --lib optical_char_recognition`
Expected: PASS。

- [ ] **Step 3: 提交**

```bash
git add crates/openlark-ai/src/ai/optical_char_recognition/v1/image/mod.rs
git commit -m "feat(ai/ocr): Image 装基础识别访问器

- Image._config → config（清除 #274 reserved）
- Image::basic_recognize() → BasicRecognizeRequestBuilder
- 导航链两层对齐 URL：.v1().image().basic_recognize()

Refs: #275"
```

---

## Task 5: Translation — Text 中间层装 `.translate()` / `.detect()`

**Files:**
- Modify: `crates/openlark-ai/src/ai/translation/v1/text/mod.rs`
- Test: `cargo test -p openlark-ai translation`

**Interfaces:**
- Consumes: `TextTranslateRequestBuilder::new(Config)`（translate.rs:115）、`TextDetectRequestBuilder::new(Config)`（detect.rs:95）。
- Produces: `Text::translate() -> TextTranslateRequestBuilder`、`Text::detect() -> TextDetectRequestBuilder`。

- [ ] **Step 1: 修改 `text/mod.rs`**

把 `Text` struct + impl 替换为：

```rust
/// Text translation API
#[derive(Clone)]
pub struct Text {
    config: Arc<Config>,
}

impl Text {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 文本翻译（对齐 URL `/translation/v1/text/translate`）。
    pub fn translate(&self) -> translate::TextTranslateRequestBuilder {
        translate::TextTranslateRequestBuilder::new((*self.config).clone())
    }

    /// 语种检测（对齐 URL `/translation/v1/text/detect`）。
    pub fn detect(&self) -> detect::TextDetectRequestBuilder {
        detect::TextDetectRequestBuilder::new((*self.config).clone())
    }
}
```

（保留 `pub mod detect; pub mod translate;` 和测试模块。）

- [ ] **Step 2: 编译 + 测试**

Run: `cargo build -p openlark-ai && cargo test -p openlark-ai --lib translation`
Expected: PASS。

- [ ] **Step 3: 提交**

```bash
git add crates/openlark-ai/src/ai/translation/v1/text/mod.rs
git commit -m "feat(ai/translation): Text 装翻译/检测访问器

- Text._config → config（清除 #274 reserved）
- Text::translate() → TextTranslateRequestBuilder
- Text::detect() → TextDetectRequestBuilder
- 导航链两层对齐 URL：.v1().text().{translate,detect}()

Refs: #275"
```

---

## Task 6: Speech — 迁移 B 套完整实现到 A 套位置（保留 A 套正确 URL）

**Files:**
- Modify: `crates/openlark-ai/src/ai/speech_to_text/v1/speech/file_recognize.rs`（A 套，用 B 套内容覆盖）
- Modify: `crates/openlark-ai/src/ai/speech_to_text/v1/speech/stream_recognize.rs`（A 套，用 B 套内容覆盖）
- Modify: `crates/openlark-ai/src/ai/speech_to_text/v1/speech/mod.rs`（Speech `_config`→`config` + 装 accessor）
- Test: `cargo test -p openlark-ai speech_to_text`

**Interfaces:**
- Consumes: B 套 `speech_to_text/speech_to_text/v1/recognize/{file,stream}_recognize.rs`（完整实现：Builder + 类型化 Result + 测试）。
- Produces: A 套 `file_recognize::FileRecognizeRequestBuilder`、`stream_recognize::StreamRecognizeRequestBuilder`（带完整 Builder）。

**核心操作**：B 套文件内容更完整（324/386 行 vs A 套 112/120 行），但有 2 处必须改：
1. URL 常量：B 套用 `SPEECH_TO_TEXT_V1_FILE_RECOGNIZE`（错误 `/v1/file/recognize`）→ 改为 A 套的 `SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE`（正确 `/v1/speech/file_recognize`）。stream 同理：`SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE` → `SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE`。
2. 模块文档 docPath：B 套已是 `speech_to_text-v1/file_recognize`（正确），保留。

- [ ] **Step 1: 用 B 套内容覆盖 A 套 file_recognize.rs，再改 URL 常量**

```bash
cp crates/openlark-ai/src/speech_to_text/speech_to_text/v1/recognize/file_recognize.rs \
   crates/openlark-ai/src/ai/speech_to_text/v1/speech/file_recognize.rs
```

然后修改新文件 `crates/openlark-ai/src/ai/speech_to_text/v1/speech/file_recognize.rs`：

把：
```rust
use crate::endpoints::SPEECH_TO_TEXT_V1_FILE_RECOGNIZE;
```
改为：
```rust
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE;
```

把 `ApiRequest::post(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE)` 改为 `ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE)`（共 1 处，在 `execute_with_options`）。

- [ ] **Step 2: 用 B 套内容覆盖 A 套 stream_recognize.rs，再改 URL 常量**

```bash
cp crates/openlark-ai/src/speech_to_text/speech_to_text/v1/recognize/stream_recognize.rs \
   crates/openlark-ai/src/ai/speech_to_text/v1/speech/stream_recognize.rs
```

修改新文件，把 `use crate::endpoints::SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE;` 改为 `use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE;`，`ApiRequest::post(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE)` 改为 `ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE)`。

- [ ] **Step 3: 修改 `speech/mod.rs`，Speech 装 accessor + `_config`→`config`**

把 `crates/openlark-ai/src/ai/speech_to_text/v1/speech/mod.rs` 的 `Speech` struct + impl 替换为：

```rust
/// Speech recognition API
#[derive(Clone)]
pub struct Speech {
    config: Arc<Config>,
}

impl Speech {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 语音文件识别（对齐 URL `/speech_to_text/v1/speech/file_recognize`）。
    pub fn file_recognize(&self) -> file_recognize::FileRecognizeRequestBuilder {
        file_recognize::FileRecognizeRequestBuilder::new((*self.config).clone())
    }

    /// 流式语音识别（对齐 URL `/speech_to_text/v1/speech/stream_recognize`）。
    pub fn stream_recognize(&self) -> stream_recognize::StreamRecognizeRequestBuilder {
        stream_recognize::StreamRecognizeRequestBuilder::new((*self.config).clone())
    }
}
```

（保留 `pub mod file_recognize; pub mod stream_recognize;` 和测试模块。）

- [ ] **Step 4: 编译**

Run: `cargo build -p openlark-ai`
Expected: PASS。如失败检查 B 套是否引用了 A 套路径上不存在的符号（B 套 free function 应已自包含）。

- [ ] **Step 5: 跑 Speech 测试（B 套带测试，迁移后应全过）**

Run: `cargo test -p openlark-ai --lib speech_to_text`
Expected: PASS。B 套测试已随文件迁入 A 套位置。

- [ ] **Step 6: 提交**

```bash
git add crates/openlark-ai/src/ai/speech_to_text/
git commit -m "feat(ai/speech): 迁移 B 套完整实现到 A 套位置 + 保留 A 套正确 URL

- file_recognize/stream_recognize：用 B 套完整实现覆盖 A 套
  （B 套带 Builder + 类型化 Result + 测试，A 套原简陋用 serde_json::Value）
- URL 常量改回 A 套正确的：
  SPEECH_TO_TEXT_V1_FILE_RECOGNIZE → SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE
  SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE → SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE
- Speech._config → config + 装 .file_recognize()/.stream_recognize()
- 导航链两层对齐 URL：.v1().speech().{file,stream}_recognize()

Refs: #275"
```

---

## Task 7: 新增访问器链可达性测试（4 能力）

**Files:**
- Modify: `crates/openlark-ai/src/service.rs`（`#[cfg(test)] mod tests` 追加）

**Interfaces:**
- Consumes: Task 3-6 装好的 accessor 链。
- Produces: 4 个测试断言 `AiClient → 叶子 RequestBuilder` 返回类型正确。

- [ ] **Step 1: 在 `service.rs` 测试模块追加 4 个可达性测试**

在 `crates/openlark-ai/src/service.rs` 的 `#[cfg(test)] mod tests` 内追加（保留现有 5 个 client 创建测试）：

```rust
    #[test]
    fn test_document_ai_accessor_chain() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let client = AiClient::new(config);
        // 三层链：.document_ai().v1().id_card().recognize()
        let builder = client
            .document_ai()
            .v1()
            .id_card()
            .recognize();
        // 编译期断言返回类型正确（赋值即验证类型）
        let _ = builder;
    }

    #[test]
    fn test_ocr_accessor_chain() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let client = AiClient::new(config);
        let _builder = client.ocr().v1().image().basic_recognize();
    }

    #[test]
    fn test_speech_accessor_chain() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let client = AiClient::new(config);
        let _file = client.speech_to_text().v1().speech().file_recognize();
        let _stream = client.speech_to_text().v1().speech().stream_recognize();
    }

    #[test]
    fn test_translation_accessor_chain() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let client = AiClient::new(config);
        let _translate = client.translation().v1().text().translate();
        let _detect = client.translation().v1().text().detect();
    }
```

这些测试主要靠**编译期类型检查**验证链可达（返回类型不对则编译失败），运行期只验证不 panic。

- [ ] **Step 2: 跑测试**

Run: `cargo test -p openlark-ai --lib service::tests`
Expected: PASS（4 个新测试通过）。

- [ ] **Step 3: 提交**

```bash
git add crates/openlark-ai/src/service.rs
git commit -m "test(ai): 新增 4 能力访问器链可达性测试

每个能力从 AiClient 经 accessor 链到达叶子 RequestBuilder，
编译期断言返回类型正确，运行期验证不 panic。

Refs: #275"
```

---

## Task 8: 删除第一代顶层 document_ai/（chain.rs 引用源）

**Files:**
- Delete: `crates/openlark-ai/src/document_ai/`（整个目录，5 个叶子 + mod 链）
- Modify: `crates/openlark-ai/src/lib.rs:55`（删 `pub mod document_ai;`）

**Interfaces:**
- Consumes: Task 3 已让 DocumentAi 经第二代链可达；chain.rs（Task 9 删）当前引用第一代，本 task 与 Task 9 在一次提交内完成（否则 chain.rs 断引用编译失败）。
- Produces: 第一代 DocumentAi 实现清除。

**重要**：chain.rs 的 `RecognizeResource::{resume,id_card,...}_parse/recognize()` 引用 `crate::document_ai::document_ai::v1::recognize::*`。删除第一代会让 chain.rs 编译失败。**本 task 与 Task 9（删 chain.rs）必须同提交**。

- [ ] **Step 1: 删除第一代 document_ai 目录**

```bash
rm -rf crates/openlark-ai/src/document_ai
```

- [ ] **Step 2: lib.rs 删除 `pub mod document_ai;`**

修改 `crates/openlark-ai/src/lib.rs`，删除第 55 行：
```rust
pub mod document_ai;
```

- [ ] **Step 3: 暂不编译（chain.rs 断引用，Task 9 修复）**

预期：`cargo build -p openlark-ai` 失败（chain.rs 引用已删的第一代）。与 Task 9 一起提交。

---

## Task 9: 删除 chain.rs（范式 A）+ 顶层 DocumentAiClient re-export

**Files:**
- Delete: `crates/openlark-ai/src/common/chain.rs`
- Modify: `crates/openlark-ai/src/common/mod.rs`（删 `pub mod chain` + chain re-export）
- Modify: `crates/openlark-ai/src/lib.rs:72`（删 `pub use common::chain::DocumentAiClient;`）

**Interfaces:**
- Consumes: Task 2 已让 `service::DocumentAiClient`（第二代）成为唯一 canonical；同名 `chain::DocumentAiClient` 删除后无冲突。
- Produces: 范式 A 死链清除；同名 `DocumentAiClient` 仅剩 `service::DocumentAiClient`。

- [ ] **Step 1: 删除 chain.rs**

```bash
rm crates/openlark-ai/src/common/chain.rs
```

- [ ] **Step 2: 修改 `common/mod.rs`**

把 `crates/openlark-ai/src/common/mod.rs` 中：
```rust
// 链式调用入口
pub mod chain;
```
删除，并删除：
```rust
// 重导出链式调用入口
pub use chain::{DocumentAiClient, DocumentAiV1Client, RecognizeResource};
```

保留 `pub mod api_utils;` 和 `pub use api_utils::{ensure_success, extract_response_data, serialize_params};`。

- [ ] **Step 3: lib.rs 删除 chain 版 DocumentAiClient re-export**

修改 `crates/openlark-ai/src/lib.rs`，删除第 72 行：
```rust
pub use common::chain::DocumentAiClient;
```

（`pub use service::AiClient;` 保留不动。）

- [ ] **Step 4: 编译验证（合并 Task 8 + 9）**

Run: `cargo build -p openlark-ai`
Expected: PASS（第一代 + chain 都删，第二代链已完整）。

- [ ] **Step 5: 跑全量 ai 测试**

Run: `cargo test -p openlark-ai --lib`
Expected: PASS。

- [ ] **Step 6: 提交（合并 Task 8 + 9）**

```bash
git add -A crates/openlark-ai/src/document_ai crates/openlark-ai/src/common crates/openlark-ai/src/lib.rs
git commit -m "refactor(ai): 删除第一代顶层 document_ai + chain.rs（范式 A）

- 删除 src/document_ai/（第一代，5 个叶子，被 chain.rs 引用）
- 删除 common/chain.rs（范式 A，RecognizeResource 活链引用第一代）
- 删除 common/mod.rs 的 chain re-export
- 删除 lib.rs 顶层 DocumentAiClient re-export（chain 版）
- 同名 DocumentAiClient 冲突消除，仅剩 service::DocumentAiClient（canonical）
- DocumentAi 能力已由 Task 3 第二代链承接

Refs: #275"
```

---

## Task 10: 删除第一代顶层 speech_to_text/（B 套源，已迁移）

**Files:**
- Delete: `crates/openlark-ai/src/speech_to_text/`（整个目录）
- Modify: `crates/openlark-ai/src/lib.rs:59`（删 `pub mod speech_to_text;`）

**Interfaces:**
- Consumes: Task 6 已把 B 套完整实现迁到 A 套位置；顶层 B 套目录零外部引用（第一代）。
- Produces: 第一代 Speech B 套清除；Speech 实现单套（A 套位置）。

**注意**：`ai::speech_to_text`（第二代）与顶层 `speech_to_text`（第一代）同名模块。删的是**顶层** `src/speech_to_text/`，**不是** `src/ai/speech_to_text/`。

- [ ] **Step 1: 删除顶层 speech_to_text 目录**

```bash
rm -rf crates/openlark-ai/src/speech_to_text
```

- [ ] **Step 2: lib.rs 删除顶层 `pub mod speech_to_text;`**

修改 `crates/openlark-ai/src/lib.rs`，删除第 59 行：
```rust
pub mod speech_to_text;
```

（**不删** `ai` 模块下的 `pub mod speech_to_text`——那是 `ai/mod.rs` 内的，是第二代。）

- [ ] **Step 3: 编译 + 测试**

Run: `cargo build -p openlark-ai && cargo test -p openlark-ai --lib`
Expected: PASS。

- [ ] **Step 4: 提交**

```bash
git add -A crates/openlark-ai/src/speech_to_text crates/openlark-ai/src/lib.rs
git commit -m "refactor(ai): 删除第一代顶层 speech_to_text（B 套已迁移）

- 删除 src/speech_to_text/（第一代 B 套，实现已迁到 ai/speech_to_text/v1/speech/）
- 删除 csv 无的 speech_recognize（B 套第 3 个 API，飞书无此端点）
- lib.rs 删除顶层 pub mod speech_to_text
- Speech 实现收敛为单套（A 套位置 + A 套正确 URL）
- 注意：ai/speech_to_text（第二代）保留不动

Refs: #275"
```

---

## Task 11: 删除错误 URL 常量（endpoints/mod.rs）

**Files:**
- Modify: `crates/openlark-ai/src/endpoints/mod.rs`

**Interfaces:**
- Consumes: Task 6 已把 A 套 Speech 改用正确常量；错误常量（B 套/拍平）零引用。
- Produces: endpoints 仅保留对齐 csv 的正确 URL 常量。

**待删错误常量清单（核查 endpoints/mod.rs 确认行号后删）：**
- `OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE = "/open-apis/optical_char_recognition/v1/basic_recognize"`（错误，少了 `/image/`；正确是 `OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE`）
- `SPEECH_TO_TEXT_V1_FILE_RECOGNIZE = "/open-apis/speech_to_text/v1/file/recognize"`（错误，应是 `/v1/speech/file_recognize`）
- `SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE = "/open-apis/speech_to_text/v1/stream/recognize"`（错误）
- `SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE = "/open-apis/speech_to_text/v1/speech/recognize"`（csv 无）
- 别名 `OCR_BASIC_RECOGNIZE`（指向错误的 `OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE`）
- 别名 `SPEECH_RECOGNIZE`（指向 csv 无的 `SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE`）
- 模块文档示例中引用这些错误常量的行
- 测试模块中引用这些错误常量的断言

**保留的正确常量：**
- `OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE`
- `SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE`
- `SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE`
- 所有 `DOCUMENT_AI_*`（已对齐 csv）
- 所有 `TRANSLATION_V1_*`（已对齐 csv）

- [ ] **Step 1: 先 grep 确认错误常量零引用**

```bash
cd crates/openlark-ai && grep -rnE "OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE\b|SPEECH_TO_TEXT_V1_FILE_RECOGNIZE\b|SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE\b|SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE\b|OCR_BASIC_RECOGNIZE\b|SPEECH_RECOGNIZE\b" src --include="*.rs" | grep -v "endpoints/mod.rs"
```
Expected: 空（Task 6 已把 Speech 叶子改用正确常量；OCR 叶子已用 `OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE`）。如非空，先修复引用。

- [ ] **Step 2: 删除 endpoints/mod.rs 中的错误常量 + 别名 + 文档 + 测试**

逐个删除上述清单中的常量定义、别名、模块文档示例行（`//! let ocr_basic_endpoint = OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;` 等改为正确常量或删）、测试断言（`assert!(OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE...)` 等）。

**保留**正确常量及其测试。测试数组中删除错误常量项。

- [ ] **Step 3: 编译 + 测试**

Run: `cargo build -p openlark-ai && cargo test -p openlark-ai --lib endpoints`
Expected: PASS。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-ai/src/endpoints/mod.rs
git commit -m "refactor(ai/endpoints): 删除错误 URL 常量（B 套/拍平，非 csv 官方）

删除（与 api_list_export.csv 不符）：
- OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE（/v1/basic_recognize，少 /image/）
- SPEECH_TO_TEXT_V1_FILE_RECOGNIZE（/v1/file/recognize，错）
- SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE（/v1/stream/recognize，错）
- SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE（csv 无此端点）
- 别名 OCR_BASIC_RECOGNIZE / SPEECH_RECOGNIZE

保留正确（对齐 csv）：
- OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE
- SPEECH_TO_TEXT_V1_SPEECH_{FILE,STREAM}_RECOGNIZE

Refs: #275"
```

---

## Task 12: lib.rs 模块文档更新 + openlark-client 注册校验

**Files:**
- Modify: `crates/openlark-ai/src/lib.rs`（模块文档示例）
- Test: `cargo build -p openlark-client`、`cargo doc -p openlark-ai --all-features`

**Interfaces:**
- Consumes: Task 2-11 完成的导航链。
- Produces: 文档示例与新导航链一致；openlark-client AiClient 注册编译通过。

- [ ] **Step 1: 更新 lib.rs 模块文档示例**

修改 `crates/openlark-ai/src/lib.rs` 顶部模块文档，把示例行：
```rust
//! // client.document_ai().v1()...
```
更新为反映新三层链：
```rust
//! // client.document_ai().v1().id_card().recognize()...
```

把文档示例中引用已删常量的行（如 `let ocr_endpoint = OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;`）改为正确常量 `OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE`。

- [ ] **Step 2: 编译 openlark-client（AiClient 注册校验）**

Run: `cargo build -p openlark-client`
Expected: PASS。`client.rs:105` 的 `ty: openlark_ai::AiClient` 类型签名不变，内部导航变化不影响注册。

- [ ] **Step 3: 跑 cargo doc 验证无文档断链**

Run: `cargo doc -p openlark-ai --all-features`
Expected: PASS（无 warning 关于断链）。如出现 intra-doc link 警告，修复。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-ai/src/lib.rs
git commit -m "docs(ai): 更新模块文档示例为新导航链

- 示例改为 .document_ai().v1().id_card().recognize()...
- 引用常量改用正确的 IMAGE_BASIC_RECOGNIZE

Refs: #275"
```

---

## Task 13: 质量验证（fmt + clippy + 全量测试 + 死链清零终检）

**Files:**
- 无（验证 task）

- [ ] **Step 1: cargo fmt --check**

Run: `cargo fmt --check`
Expected: PASS（无 diff 输出）。如失败，先 `cargo fmt` 再提交格式化。

**CI 提示**：CI lint job 第一步是 `cargo fmt --check`（见 [[run-cargo-fmt-check-before-push]]），clippy 通过≠fmt 通过，必须显式跑。

- [ ] **Step 2: cargo clippy（dead_code 验证死链清零）**

Run: `cargo clippy -p openlark-ai --all-features -- -W dead_code -D warnings`
Expected: PASS，**无 dead_code 告警**。重点确认：
- 无 `_config` 字段残留（4 处 _config 应全部改为 config 并被 accessor 消费）
- 无未使用 import（删第一代/chain 后的孤儿 use）

Run（全仓）: `cargo clippy --all-features --workspace -- -D warnings`
Expected: PASS。

- [ ] **Step 3: cargo test 全量**

Run: `cargo test --all-features -p openlark-ai`
Expected: PASS（含 Task 7 的 4 个可达性测试 + DocumentAi 18 叶子 + Speech 迁移测试 + endpoints 测试）。

- [ ] **Step 4: 终检 grep（死链清零确认）**

```bash
cd crates/openlark-ai/src && \
echo "=== AiService 残留 ===" && grep -rn "struct AiService\|ai::AiService" . || echo "无" && \
echo "=== chain 残留 ===" && grep -rn "common::chain\|chain::DocumentAiClient" . || echo "无" && \
echo "=== _config 残留 ===" && grep -rn "_config" ai/ || echo "无" && \
echo "=== 第一代 document_ai 顶层目录 ===" && ls document_ai 2>/dev/null || echo "已删" && \
echo "=== 第一代 speech_to_text 顶层目录 ===" && ls speech_to_text 2>/dev/null || echo "已删" && \
echo "=== 同名 DocumentAiClient 计数（应仅 service.rs 1 处定义）===" && grep -rn "pub struct DocumentAiClient" .
```
Expected：
- AiService：无
- chain：无
- `_config`：无（ai/ 下）
- 第一代 document_ai 目录：已删
- 第一代 speech_to_text 目录：已删
- `DocumentAiClient` 定义：仅 `service.rs` 1 处

- [ ] **Step 5: 全仓构建确认无回归**

Run: `cargo build --all-features --workspace`
Expected: PASS。

- [ ] **Step 6: 提交（如有 fmt 修复）**

如 Step 1 触发了格式化：
```bash
git add -A
git commit -m "style(ai): cargo fmt 格式化

Refs: #275"
```
如全部通过无修改，本 task 无提交。

---

## Self-Review 核查

**1. Spec coverage（Design Doc 6 项决策）：**
- D1 导航层级对齐 URL → Task 3（DocumentAi 三层）+ Task 4/5/6（OCR/Translation/Speech 两层）✓
- D2 DocumentAi 18 doc_type service → Task 3 ✓
- D3 config 流转对齐 SparkV1 → Task 3/4/5/6 各 service 持 `Arc<Config>`，叶子 `(*self.config).clone()` ✓
- D4 Speech = B 套实现 + A 套 URL → Task 6 ✓
- D5 死链与第一代删除 → Task 1/8/9/10/11 ✓
- D6 feature 门控保持现状 → Task 2 重定向沿用 `#[cfg(feature="v1")]` ✓
- Migration（CHANGELOG）→ **注意：本计划未含 CHANGELOG 条目**，因 tasks.md 未列。建议实施者在收尾时按 v0.17→v0.18 迁移表补 CHANGELOG，但非本计划 task（避免超范围）。

**2. 死链删除清单全覆盖（brainstorm-summary）：**
- 范式 B AiService + V1 → Task 1 ✓
- chain.rs + lib.rs:72 + common/mod.rs → Task 9 ✓
- 第一代 document_ai（5 个）→ Task 8 ✓
- 第一代 speech_to_text（B 套）→ Task 10 ✓
- 错误 URL 常量（4 个 + 别名）→ Task 11 ✓

**3. 待装 accessor 全覆盖（Design 工作清单表）：**
- DocumentAi：DocumentAiV1 `_config`→`config` + 18 accessor + 18 service → Task 3 ✓
- OCR：Image `_config`→`config` + `.basic_recognize()` → Task 4 ✓
- Speech：Speech `_config`→`config` + `.file_recognize()`/`.stream_recognize()` → Task 6 ✓
- Translation：Text `_config`→`config` + `.translate()`/`.detect()` → Task 5 ✓

**4. 类型一致性：** 所有叶子 accessor 返回 `XxxRequestBuilder`（与 chain.rs 旧实现和 Design 导航链图一致）；所有 service `new(Arc<Config>)`；所有叶子 Builder `new(Config)`（owned）。已核查 18 个 doc_type + OCR + Translation + Speech 叶子的 Builder 类型名与映射表一致。

**5. tasks.md 13 项映射：** 1.1→T1+T2 / 2.1→T3 / 2.2→T4 / 2.3→T6 / 2.4→T5 / 2.5→T3+T4+T5+T6（_config 改名分散在各能力 task）/ 3.1→T8+T9 / 4.1→T12 / 4.2→T12 / 5.1-5.4→T13。全覆盖。

**6. 依赖排序：** T1→T2（重定向恢复编译）→T3-T6（4 能力 accessor，可并行但建议顺序）→T7（测试）→T8+T9（删第一代+chain，同提交）→T10（删 B 套）→T11（删错误 URL）→T12（文档收尾）→T13（质量验证）。死链清理在前（T1）、错误 URL 删除在 accessor 接线后（T11 在 T6 后，确保 Speech 叶子已改用正确常量）。

---

## 执行交接

计划已保存至 `docs/superpowers/plans/2026-06-30-untangle-ai-crate-navigation.md`。两种执行方式：

**1. Subagent 驱动（推荐）** — 每个 task 派发独立 subagent，task 间审查，迭代快。
**2. 内联执行** — 在当前会话用 executing-plans 批量执行，带检查点审查。

选择哪种？
