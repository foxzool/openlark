//! Event (事件系统) 端点定义
//!
//! 事件系统 - 事件订阅、处理、分发机制。

/// 事件订阅列表接口。
pub const EVENT_V1_SUBSCRIPTIONS: &str = "/open-apis/event/v1/subscriptions";
/// 创建事件订阅接口。
pub const EVENT_V1_SUBSCRIPTION_CREATE: &str = "/open-apis/event/v1/subscriptions/create";
/// 获取单个事件订阅接口。
pub const EVENT_V1_SUBSCRIPTION_GET: &str = "/open-apis/event/v1/subscriptions/{subscription_id}";
/// 删除单个事件订阅接口。
pub const EVENT_V1_SUBSCRIPTION_DELETE: &str =
    "/open-apis/event/v1/subscriptions/{subscription_id}";
/// 更新单个事件订阅接口。
pub const EVENT_V1_SUBSCRIPTION_UPDATE: &str =
    "/open-apis/event/v1/subscriptions/{subscription_id}";

/// 事件历史列表接口。
pub const EVENT_V1_HISTORY: &str = "/open-apis/event/v1/history";
/// 事件历史重放接口。
pub const EVENT_V1_HISTORY_REPLAY: &str = "/open-apis/event/v1/history/replay";

/// 事件分发器接口。
pub const EVENT_V1_DISPATCHER: &str = "/open-apis/event/v1/dispatcher";
/// 事件分发器状态接口。
pub const EVENT_V1_DISPATCHER_STATUS: &str = "/open-apis/event/v1/dispatcher/status";

/// 事件推送出口 IP 查询接口。
pub const EVENT_V1_OUTBOUND_IP: &str = "/open-apis/event/v1/outbound_ip";

/// 获取长连接在线数量接口。
pub const EVENT_V1_CONNECTION: &str = "/open-apis/event/v1/connection";

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_event_endpoints() {
        assert!(EVENT_V1_SUBSCRIPTIONS.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_SUBSCRIPTIONS.contains("subscriptions"));
        assert!(EVENT_V1_HISTORY.contains("history"));
        assert!(EVENT_V1_DISPATCHER.contains("dispatcher"));
        assert!(EVENT_V1_SUBSCRIPTION_CREATE.contains("create"));
        assert!(EVENT_V1_SUBSCRIPTION_DELETE.contains("{subscription_id}"));
        assert!(EVENT_V1_CONNECTION.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_CONNECTION.ends_with("/connection"));
    }
}
