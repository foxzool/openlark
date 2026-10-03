//! Event (事件系统) 端点定义
//!
//! 仅保留有 Request/Response 实现的端点常量。
//! subscriptions/history/dispatcher 等无实现的占位常量已移除（见 issue #226）。

/// 事件推送出口 IP 查询接口。
///
/// 对应实现：`event::event::v1::outbound_ip::list`
pub const EVENT_V1_OUTBOUND_IP: &str = "/open-apis/event/v1/outbound_ip";

/// 获取长连接在线数量接口。
pub const EVENT_V1_CONNECTION: &str = "/open-apis/event/v1/connection";

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_event_endpoints() {
        assert!(EVENT_V1_OUTBOUND_IP.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_OUTBOUND_IP.ends_with("/outbound_ip"));
        assert!(EVENT_V1_CONNECTION.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_CONNECTION.ends_with("/connection"));
    }
}
