//! 飞书 protobuf Frame 头部 key 常量与查找（package / frame_handler 共用）。

use lark_websocket_protobuf::pbbp2::Header;

/// 帧类型：`event` / `ping` / `pong` / `card` 等。
pub(crate) const HDR_TYPE: &str = "type";
/// 业务 message_id（分包聚合键）。
pub(crate) const HDR_MESSAGE_ID: &str = "message_id";
/// 链路 trace_id。
pub(crate) const HDR_TRACE_ID: &str = "trace_id";
/// 分包总数。
pub(crate) const HDR_SUM: &str = "sum";
/// 分包序号（0-based）。
pub(crate) const HDR_SEQ: &str = "seq";

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
