//! `encrypt_key` AES-CBC 解密与入站签名。
//!
//! 解密对齐官方文档 / Python `AESCipher`（PKCS7）。不采用官方 Go
//! `EventDecrypt` 的 `{`…`}` 截取（该写法对 JSON 事件碰巧可用，但无法通过
//! 官方 `hello world` 固定向量）。

use aes::Aes256;
use base64::Engine;
use cbc::cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
use openlark_core::error::CoreError;
use sha2::{Digest, Sha256};

use crate::error::validation_error;

/// 入站请求时间戳头（对齐 Go `EventRequestTimestamp`）。
pub const HEADER_REQUEST_TIMESTAMP: &str = "X-Lark-Request-Timestamp";
/// 入站请求 nonce 头（对齐 Go `EventRequestNonce`）。
pub const HEADER_REQUEST_NONCE: &str = "X-Lark-Request-Nonce";
/// 入站签名头（对齐 Go `EventSignature`）。
pub const HEADER_SIGNATURE: &str = "X-Lark-Signature";

type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// 解密事件密文字符串。
///
/// 1. Base64 解码
/// 2. 拒绝长度小于 32 或非 16 倍数（对齐 Python `AESCipher.decrypt`）
/// 3. `key = SHA256(encrypt_key)`
/// 4. 前 16 字节为 IV，其余为密文
/// 5. AES-256-CBC + PKCS7 去填充
pub fn decrypt_event(encrypt_b64: &str, encrypt_key: &str) -> Result<Vec<u8>, CoreError> {
    let buf = base64::engine::general_purpose::STANDARD
        .decode(encrypt_b64.trim())
        .map_err(|e| validation_error("encrypt", format!("base64 decode failed: {e}")))?;

    if buf.len() < 32 || !buf.len().is_multiple_of(16) {
        return Err(validation_error(
            "encrypt",
            "cipher length must be >= 32 and a multiple of 16",
        ));
    }

    let key = Sha256::digest(encrypt_key.as_bytes());
    let iv = &buf[..16];
    let mut cipher_body = buf[16..].to_vec();

    let plain = Aes256CbcDec::new_from_slices(key.as_slice(), iv)
        .map_err(|e| validation_error("encrypt", format!("AES cipher init failed: {e}")))?
        .decrypt_padded_mut::<Pkcs7>(&mut cipher_body)
        .map_err(|e| validation_error("encrypt", format!("AES decrypt/unpad failed: {e}")))?;

    Ok(plain.to_vec())
}

/// 计算入站签名（小写十六进制 SHA256）。
pub fn inbound_signature(timestamp: &str, nonce: &str, encrypt_key: &str, body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(timestamp.as_bytes());
    hasher.update(nonce.as_bytes());
    hasher.update(encrypt_key.as_bytes());
    hasher.update(body.as_bytes());
    hex_encode(&hasher.finalize())
}

/// 校验入站签名（常量时间比较）。
pub fn verify_inbound_signature(
    timestamp: &str,
    nonce: &str,
    encrypt_key: &str,
    body: &str,
    signature: &str,
) -> bool {
    let expected = inbound_signature(timestamp, nonce, encrypt_key, body);
    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
pub(crate) mod test_support {
    use aes::Aes256;
    use base64::Engine;
    use cbc::cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
    use sha2::{Digest, Sha256};

    type Aes256CbcEnc = cbc::Encryptor<Aes256>;

    /// 测试用加密（PKCS7），供 decrypt 单测 round-trip。
    pub fn encrypt_event_for_test(plaintext: &[u8], encrypt_key: &str, iv: &[u8; 16]) -> String {
        let key = Sha256::digest(encrypt_key.as_bytes());
        let mut buf = vec![0u8; plaintext.len() + 16];
        buf[..plaintext.len()].copy_from_slice(plaintext);
        let encrypted = Aes256CbcEnc::new_from_slices(key.as_slice(), iv)
            .expect("key/iv length")
            .encrypt_padded_mut::<Pkcs7>(&mut buf, plaintext.len())
            .expect("encrypt");
        let mut out = Vec::with_capacity(16 + encrypted.len());
        out.extend_from_slice(iv);
        out.extend_from_slice(encrypted);
        base64::engine::general_purpose::STANDARD.encode(out)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::encrypt_event_for_test;
    use super::*;

    #[test]
    fn decrypt_round_trip_json() {
        let key = "test_encrypt_key";
        let plain =
            br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1"},"event":{}}"#;
        let iv = [7u8; 16];
        let cipher = encrypt_event_for_test(plain, key, &iv);
        let out = decrypt_event(&cipher, key).expect("decrypt");
        assert_eq!(out, plain);
    }

    #[test]
    fn decrypt_official_hello_world_fixture() {
        // 飞书开放平台 encrypt_key 文档固定向量（Python AESCipher / openssl 可复现）
        let out = decrypt_event("P37w+VZImNgPEO1RBhJ6RtKl7n6zymIbEG1pReEzghk=", "test key")
            .expect("official fixture");
        assert_eq!(out, b"hello world");
    }

    #[test]
    fn decrypt_rejects_short_cipher() {
        let err = decrypt_event("AAAA", "k").expect_err("short");
        let msg = err.to_string();
        assert!(
            msg.contains("length") || msg.contains("base64") || msg.contains("multiple"),
            "unexpected err: {msg}"
        );
    }

    #[test]
    fn signature_matches_known_shape() {
        let sig = inbound_signature("ts", "nonce", "key", r#"{"a":1}"#);
        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(verify_inbound_signature(
            "ts",
            "nonce",
            "key",
            r#"{"a":1}"#,
            &sig
        ));
        assert!(!verify_inbound_signature(
            "ts",
            "nonce",
            "key",
            r#"{"a":1}"#,
            "deadbeef"
        ));
    }
}
