//! Open-Lark AI Module
//!
//! 飞书AI智能服务模块，提供完整的AI和智能助手功能。

#![allow(clippy::module_inception)] // 允许模块与父模块同名，这是 bizTag 组织模式
//!
//! ## 主要功能
//!
//! - **文档AI识别**: 简历、身份证、驾驶证、银行卡、营业执照等证件识别
//! - **OCR识别**: 光学字符识别，将图片中的文字转换为可编辑文本
//! - **语音转文字**: 将语音转换为文本
//! - **翻译服务**: 文本翻译和语言检测
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_ai::{AiClient, endpoints::*, prelude::Config};
//!
//! // 使用客户端链式调用
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! let client = AiClient::new(config);
//! // client.document_ai().v1()...
//!
//! // 使用端点常量
//! let resume_endpoint = DOCUMENT_AI_RESUME_PARSE;
//! let ocr_endpoint = OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE;
//! let translate_endpoint = TRANSLATION_V1_TEXT_TRANSLATE;
//! ```
//!
//! ## 端点组织
//!
//! - `document_ai`: 文档AI识别API端点
//! - `ocr`: 光学字符识别API端点
//! - `speech_to_text`: 语音转文字API端点
//! - `translation`: 翻译服务API端点

// 导入通用工具模块
/// common 模块。
pub mod common;

// 导入服务端点模块
/// endpoints 模块。
pub mod endpoints;

// AI service modules
/// ai 模块。
pub mod ai;

// 服务入口
/// service 模块。
pub mod service;

// 重新导出服务客户端
pub use service::AiClient;

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{SDKResult, config::Config};
}
