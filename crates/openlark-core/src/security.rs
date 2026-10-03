//! Security utilities for handling sensitive data

/// Mask sensitive strings for safe logging (shows first 4 and last 4 chars)
///
/// # Examples
///
/// ```
/// use openlark_core::security::mask_sensitive;
///
/// assert_eq!(mask_sensitive("my_secret_token_12345"), "my_s***2345");
/// assert_eq!(mask_sensitive("short"), "***");
/// ```
pub fn mask_sensitive(s: &str) -> String {
    if s.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}***{}", &s[..4], &s[s.len() - 4..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_long_string() {
        assert_eq!(mask_sensitive("my_secret_token_12345"), "my_s***2345");
        assert_eq!(mask_sensitive("abcdefghijk"), "abcd***hijk");
    }

    #[test]
    fn test_mask_sensitive_short_string() {
        assert_eq!(mask_sensitive("short"), "***");
        assert_eq!(mask_sensitive("12345678"), "***");
        assert_eq!(mask_sensitive(""), "***");
    }

    #[test]
    fn test_mask_sensitive_exactly_9_chars() {
        assert_eq!(mask_sensitive("123456789"), "1234***6789");
    }
}
