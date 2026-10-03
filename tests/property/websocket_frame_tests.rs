//! WebSocket 相关属性测试（#429）。
//!
//! 帧处理器 / 状态机已收回为 implementation detail；完整会话与帧边界
//! 覆盖见 `openlark-client` 的 `ws_client::full_session_tests` 与
//! `frame_handler` / `state_machine` 模块内单元测试。
//!
//! 此处仅对**公开**事件分发 seam 做健壮性抽检。

#[cfg(test)]
mod event_dispatcher_properties {
    use open_lark::ws_client::{EventDispatcherHandler, EventHandler};
    use proptest::prelude::*;
    use std::error::Error;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingHandler {
        calls: Arc<AtomicUsize>,
    }

    impl EventHandler for CountingHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    proptest! {
        /// 任意可打印 payload 不应使公开 dispatcher panic
        #[test]
        fn dispatcher_never_panics_on_arbitrary_payload(
            payload in prop::collection::vec(any::<u8>(), 0..4096)
        ) {
            let handler = EventDispatcherHandler::builder().build();
            let _ = handler.do_without_validation(&payload);
        }
    }

    proptest! {
        /// raw catch-all 对任意 payload 至多调用一次
        #[test]
        fn raw_handler_called_at_most_once(
            payload in prop::collection::vec(any::<u8>(), 0..1024)
        ) {
            let calls = Arc::new(AtomicUsize::new(0));
            let handler = EventDispatcherHandler::builder()
                .register_raw(
                    EventDispatcherHandler::RAW_EVENT_KEY,
                    CountingHandler {
                        calls: Arc::clone(&calls),
                    },
                )
                .expect("register")
                .build();
            let _ = handler.do_without_validation(&payload);
            assert!(calls.load(Ordering::SeqCst) <= 1);
        }
    }
}
