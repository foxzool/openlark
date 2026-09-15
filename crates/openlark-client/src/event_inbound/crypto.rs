//! `encrypt_key` AES-CBC 解密与入站签名（对齐官方 Go `larkevent`）。

use aes::Aes256;
use base64::Engine;
use cbc::cipher::{BlockDecryptMut, KeyIvInit, block_padding::NoPadding};
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
/// 步骤（与 Go `EventDecrypt` 一致）
/// 1. Base64 解码
/// 2. `key = SHA256(encrypt_key)`
/// 3. 前 16 字节为 IV，其余为密文
/// 4. AES-256-CBC 解密后截取首个 `{` 到末个 `}`
pub fn decrypt_event(encrypt_b64: &str, encrypt_key: &str) -> Result<Vec<u8>, CoreError> {
    let buf = base64::engine::general_purpose::STANDARD
        .decode(encrypt_b64.trim())
        .map_err(|e| validation_error("encrypt", format!("base64 decode failed: {e}")))?;

    if buf.len() < 16 {
        return Err(validation_error("encrypt", "cipher too short"));
    }

    let key = Sha256::digest(encrypt_key.as_bytes());
    let iv = &buf[..16];
    let mut cipher_body = buf[16..].to_vec();

    if cipher_body.is_empty() || !cipher_body.len().is_multiple_of(16) {
        return Err(validation_error(
            "encrypt",
            "ciphertext is not a multiple of the block size",
        ));
    }

    Aes256CbcDec::new_from_slices(key.as_slice(), iv)
        .map_err(|e| validation_error("encrypt", format!("AES cipher init failed: {e}")))?
        .decrypt_padded_mut::<NoPadding>(&mut cipher_body)
        .map_err(|e| validation_error("encrypt", format!("AES decrypt failed: {e}")))?;

    let text = String::from_utf8_lossy(&cipher_body);
    let start = text.find('{').unwrap_or(0);
    let end = text.rfind('}').unwrap_or(cipher_body.len().saturating_sub(1));
    if end < start {
        return Err(validation_error("encrypt", "decrypted payload has no JSON object"));
    }
    Ok(cipher_body[start..=end].to_vec())
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
    use super::*;
    use super::test_support::encrypt_event_for_test;

    #[test]
    fn decrypt_round_trip_json() {
        let key = "test_encrypt_key";
        let plain = br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1"},"event":{}}"#;
        let iv = [7u8; 16];
        let cipher = encrypt_event_for_test(plain, key, &iv);
        let out = decrypt_event(&cipher, key).expect("decrypt");
        assert_eq!(out, plain);
    }

    #[test]
    fn decrypt_rejects_short_cipher() {
        let err = decrypt_event("AAAA", "k").expect_err("short");
        assert!(err.to_string().contains("short") || err.to_string().contains("base64"));
    }

    #[test]
    fn signature_matches_known_shape() {
        let sig = inbound_signature("ts", "nonce", "key", r#"{"a":1}"#);
        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(verify_inbound_signature("ts", "nonce", "key", r#"{"a":1}"#, &sig));
        assert!(!verify_inbound_signature(
            "ts",
            "nonce",
            "key",
            r#"{"a":1}"#,
            "deadbeef"
        ));
    }
}
