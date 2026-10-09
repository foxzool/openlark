use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Webhook HMAC-SHA256 签名计算器。
type HmacSha256 = Hmac<Sha256>;

/// 为飞书 webhook 生成签名。
///
/// 算法为 `base64(HMAC-SHA256(key="{timestamp}\n{secret}", message=""))`。
pub fn sign(timestamp: i64, secret: &str) -> String {
    use base64::engine::Engine;
    let key = format!("{timestamp}\n{secret}");
    let mac = HmacSha256::new_from_slice(key.as_bytes())
        .expect("HMAC-SHA256 accepts arbitrary-length keys");
    base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}

/// 获取当前 Unix 时间戳，单位为秒。
pub fn current_timestamp() -> i64 {
    // SAFETY: SystemTime::now() 返回的时间总是晚于或等于 UNIX_EPOCH (1970-01-01)，
    // 除非系统时钟被恶意修改为回退到 1970 年之前，这在正常运行的系统中不会发生。
    // duration_since 只有在系统时间早于 UNIX_EPOCH 时才会失败。
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time should never be before 1970-01-01 unless the system clock is maliciously modified")
        .as_secs() as i64
}

/// Constant-time byte comparison to prevent timing side-channel attacks
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

/// 校验飞书 webhook 签名是否匹配。
///
/// Uses constant-time comparison to prevent timing side-channel attacks.
pub fn verify_signature(timestamp: i64, secret: &str, signature: &str) -> bool {
    let computed = sign(timestamp, secret);
    constant_time_eq(computed.as_bytes(), signature.as_bytes())
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_feishu_vectors() {
        assert_eq!(
            sign(1234567890, "test-secret"),
            "qCaOcLimil1ehZl6GzN2CUL6wgdt4onZPxvw8V+3TzA="
        );
        assert_eq!(sign(0, "a"), "Tp8MNfpagxPJcttlh+SrLen5zPoTRDXb0y/HepnDrPA=");
    }

    #[test]
    fn test_sign_different_secrets() {
        let timestamp = 1599360473i64;
        let sig1 = sign(timestamp, "secret1");
        let sig2 = sign(timestamp, "secret2");

        // Different secrets should produce different signatures
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_sign_different_timestamps() {
        let secret = "test-secret";
        let sig1 = sign(1599360473, secret);
        let sig2 = sign(1599360474, secret);

        // Different timestamps should produce different signatures
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_verify_signature_valid() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";
        let signature = sign(timestamp, secret);

        // Verification should pass with correct signature
        assert!(verify_signature(timestamp, secret, &signature));
    }

    #[test]
    fn test_verify_signature_invalid() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";

        // Verification should fail with wrong signature
        assert!(!verify_signature(timestamp, secret, "invalid-signature"));
    }

    #[test]
    fn test_verify_signature_wrong_secret() {
        let timestamp = 1599360473i64;
        let secret = "test-secret";
        let signature = sign(timestamp, secret);

        // Verification should fail with wrong secret
        assert!(!verify_signature(timestamp, "wrong-secret", &signature));
    }

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();

        // Timestamp should be positive and reasonable (after 2020)
        assert!(ts > 1577836800); // 2020-01-01
        assert!(ts < 2000000000); // Before year 2033
    }

    #[test]
    fn test_constant_time_eq_equal() {
        assert!(constant_time_eq(b"abc", b"abc"));
    }

    #[test]
    fn test_constant_time_eq_not_equal() {
        assert!(!constant_time_eq(b"abc", b"abd"));
    }

    #[test]
    fn test_constant_time_eq_different_lengths() {
        assert!(!constant_time_eq(b"abc", b"abcd"));
    }
}
