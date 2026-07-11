//! 飞书 protobuf Frame 头部查找（package / frame_handler 共用）。

use lark_websocket_protobuf::pbbp2::Header;

/// 取第一个匹配 key 的 header value（借用）。
pub(crate) fn header_value<'a>(headers: &'a [Header], key: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|h| h.key == key)
        .map(|h| h.value.as_str())
}

/// 取第一个匹配 key 的 header，并解析为 `usize`。
pub(crate) fn header_usize(headers: &[Header], key: &str) -> Option<usize> {
    header_value(headers, key).and_then(|v| v.parse().ok())
}
