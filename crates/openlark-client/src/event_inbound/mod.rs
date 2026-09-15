//! HTTP 事件入站适配器。
//!
//! 对接飞书/Lark 控制台「将事件发送至开发者服务器」：`encrypt_key` AES-CBC 解密、
//! `url_verification` Challenge、以及 `X-Lark-Signature` **入站**验签。
//!
//! # 与 `openlark-webhook` 的区别
//!
//! | | 本模块 (`event_inbound`) | `openlark-webhook` |
//! |---|---|---|
//! | 方向 | 平台 → 开发者服务器（入站） | 开发者 → 自定义机器人 hook（出站） |
//! | 签名 | `SHA256(timestamp+nonce+encrypt_key+body)` 十六进制 | `HMAC-SHA256` base64（`timestamp\\nsecret`） |
//! | 加密 | `encrypt_key` AES-CBC | 无 |
//!
//! 算法对齐官方文档 / Python `AESCipher`（AES-256-CBC + PKCS7）与入站签名
//! `SHA256(timestamp+nonce+encrypt_key+body)`。不采用官方 Go `EventDecrypt` 的
//! `{`…`}` 截取。

mod crypto;
mod handler;

pub use crypto::{
    decrypt_event, inbound_signature, verify_inbound_signature, HEADER_REQUEST_NONCE,
    HEADER_REQUEST_TIMESTAMP, HEADER_SIGNATURE,
};
pub use handler::{
    HttpEventInbound, HttpEventInboundBuilder, HttpEventRequest, HttpEventResponse,
};
