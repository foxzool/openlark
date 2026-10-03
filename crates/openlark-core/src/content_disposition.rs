//! Content-Disposition 解析工具
//!
//! 目标：在 core 内集中处理响应头解析逻辑，避免各处重复实现（DRY）。

/// Sanitize a filename to prevent path traversal attacks.
///
/// Removes:
/// - Path separators (`/`, `\`)
/// - Null bytes
/// - `..` path traversal segments
/// - Leading dots (hidden files)
/// - Leading/trailing whitespace
///
/// Limits filename length to 255 bytes.
fn sanitize_filename(name: &str) -> String {
    let name = name.trim();

    // Remove null bytes
    let name: String = name.chars().filter(|c| *c != '\0').collect();

    // Take only the last component (after any path separators that might remain)
    let name = name.rsplit(['/', '\\']).next().unwrap_or(&name);

    // Remove leading dots to prevent hidden files / traversal
    let name = name.trim_start_matches('.');

    // Collapse multiple whitespace, trim
    let name: String = name.split_whitespace().collect::<Vec<_>>().join(" ");

    // Limit length to 255 bytes
    let mut result = String::new();
    let mut byte_count = 0;
    for ch in name.chars() {
        let char_bytes = ch.len_utf8();
        if byte_count + char_bytes > 255 {
            break;
        }
        result.push(ch);
        byte_count += char_bytes;
    }

    result
}

/// 从 `Content-Disposition` 头中提取文件名。
///
/// 支持：
/// - `filename*=UTF-8''xxx`（RFC 5987 / RFC 6266 形式：`charset'lang'value`）
/// - `filename="xxx"` / `filename=xxx`
pub(crate) fn extract_filename(content_disposition: &str) -> Option<String> {
    let mut fallback_filename: Option<String> = None;

    for part in content_disposition.split(';') {
        let part = part.trim();

        if let Some(rest) = part.strip_prefix("filename*=") {
            let rest = rest.trim_matches('"');
            // `charset'lang'value`，lang 允许为空：`UTF-8''file.txt`
            let mut it = rest.splitn(3, '\'');
            let _charset = it.next();
            let _lang = it.next();
            if let Some(value) = it.next() {
                return Some(sanitize_filename(value));
            }

            // 兼容：若格式不完整，则忽略
            continue;
        }

        if let Some(filename) = part.strip_prefix("filename=") {
            // `filename*` 存在时应优先使用它；这里先记录 fallback，继续扫描后续项。
            if fallback_filename.is_none() {
                fallback_filename = Some(sanitize_filename(filename.trim_matches('"')));
            }
        }
    }

    fallback_filename
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_filename_utf8_star() {
        let raw = "attachment; filename=\"upload_all.rs\"; filename*=UTF-8''upload_all.rs";
        assert_eq!(extract_filename(raw).as_deref(), Some("upload_all.rs"));
    }

    #[test]
    fn extract_filename_star_missing_charset() {
        let raw = "attachment; filename*=''missing_utf8.txt";
        assert_eq!(extract_filename(raw).as_deref(), Some("missing_utf8.txt"));
    }

    #[test]
    fn extract_filename_star_malformed() {
        let raw = "attachment; filename*=UTF-8";
        assert_eq!(extract_filename(raw).as_deref(), None);
    }

    #[test]
    fn extract_filename_quoted() {
        let raw = "attachment; filename=\"simple.txt\"";
        assert_eq!(extract_filename(raw).as_deref(), Some("simple.txt"));
    }

    #[test]
    fn extract_filename_unquoted() {
        let raw = "attachment; filename=simple.txt";
        assert_eq!(extract_filename(raw).as_deref(), Some("simple.txt"));
    }

    #[test]
    fn extract_filename_multiple_parts() {
        let raw = "attachment; charset=utf-8; filename*=UTF-8''complex%20name.txt; other=value";
        assert_eq!(extract_filename(raw).as_deref(), Some("complex%20name.txt"));
    }

    #[test]
    fn extract_filename_empty() {
        assert_eq!(extract_filename("").as_deref(), None);
    }

    // Security tests for path traversal prevention
    #[test]
    fn sanitize_removes_path_traversal() {
        let raw = r#"attachment; filename="../../../etc/passwd""#;
        let result = extract_filename(raw);
        assert_eq!(result.as_deref(), Some("passwd"));
    }

    #[test]
    fn sanitize_removes_path_separators() {
        let raw = r#"attachment; filename="foo/bar/baz.txt""#;
        let result = extract_filename(raw);
        assert_eq!(result.as_deref(), Some("baz.txt"));
    }

    #[test]
    fn sanitize_removes_backslash() {
        let raw = r#"attachment; filename="foo\bar\baz.txt""#;
        let result = extract_filename(raw);
        assert_eq!(result.as_deref(), Some("baz.txt"));
    }

    #[test]
    fn sanitize_removes_null_bytes() {
        let raw = "attachment; filename=\"foo\0bar.txt\"";
        let result = extract_filename(raw);
        assert_eq!(result.as_deref(), Some("foobar.txt"));
    }

    #[test]
    fn sanitize_limits_length() {
        let long_name = "a".repeat(300);
        let raw = format!("attachment; filename=\"{long_name}\"");
        let result = extract_filename(&raw);
        let name = result.unwrap();
        assert!(name.len() <= 255);
    }

    #[test]
    fn sanitize_removes_leading_dots() {
        let raw = r#"attachment; filename=".hidden""#;
        let result = extract_filename(raw);
        assert_eq!(result.as_deref(), Some("hidden"));
    }
}
