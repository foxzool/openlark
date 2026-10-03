#![allow(clippy::module_inception)]
//! AI service module
//!
//! 提供 AI 服务模块，包括文档 AI、OCR、语音转文字和翻译服务。

pub mod document_ai;
/// optical_char_recognition 模块。
pub mod optical_char_recognition;
/// speech_to_text 模块。
pub mod speech_to_text;
/// translation 模块。
pub mod translation;
