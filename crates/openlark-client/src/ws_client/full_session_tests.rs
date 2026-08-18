//! 完整会话本地 adapter 测试：墙钟时序 + 端点发现（#641）。
//!
//! 状态机行为已下沉至 [`super::session_behavior_tests`]（ADR-0006 / #640）。
//! 本文件只保留：
//! - B 组墙钟测试（与真实定时器的集成语义，端到端层最诚实）
//! - C 组端点发现测试（无 WS）
//!
//! 测试 seam：[`LarkWsClient::open`] / `open_with` + 本地 endpoint + WS peer。

#![cfg(feature = "websocket")]

use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use openlark_core::config::Config;
use prost::Message as ProstMessage;
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message, frame::coding::CloseCode};
use tokio_tungstenite::{WebSocketStream, accept_async};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::client::ClientConfig;
use super::frame_handler::{FRAME_METHOD_CONTROL, FRAME_METHOD_DATA};
use super::session::SessionOptions;
use super::{
    EventDispatcherHandler, EventHandler, InvalidStateKind, LarkWsClient, WsClientError,
    WsClientResult, WsCloseReason,
};

const SERVICE_ID: i32 = 42;
const SESSION_TIMEOUT: Duration = Duration::from_secs(10);

/// 本地完整会话 harness：wiremock endpoint + 本机 WebSocket peer。
struct LocalSessionHarness {
    mock_server: MockServer,
    listener: Option<TcpListener>,
}

impl LocalSessionHarness {
    async fn start_with_ping_interval(ping_interval_secs: i32) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind local websocket listener");
        let ws_addr = listener.local_addr().expect("local websocket address");

        let mock_server = MockServer::start().await;
        let ws_url = format!("ws://{ws_addr}/?service_id={SERVICE_ID}&device_id=test-device");

        Mock::given(method("POST"))
            .and(path("/callback/ws/endpoint"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "URL": ws_url,
                    "ClientConfig": {
                        "ReconnectCount": 1,
                        "ReconnectInterval": 1,
                        "ReconnectNonce": 0,
                        "PingInterval": ping_interval_secs
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        Self {
            mock_server,
            listener: Some(listener),
        }
    }

    fn config(&self) -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url(self.mock_server.uri())
            .allow_custom_base_url(true)
            .req_timeout(Duration::from_secs(5))
            .build()
    }

    async fn accept_peer(&mut self) -> WebSocketStream<tokio::net::TcpStream> {
        let listener = self.listener.take().expect("listener already consumed");
        let (stream, _) = timeout(SESSION_TIMEOUT, listener.accept())
            .await
            .expect("accept timed out")
            .expect("accept connection");
        accept_async(stream).await.expect("websocket handshake")
    }
}

/// 运行一次完整会话：peer 脚本与 `open_with` 并发，返回 open 结果与 peer 产出。
async fn run_session<F, Fut, T>(
    mut harness: LocalSessionHarness,
    event_handler: EventDispatcherHandler,
    options: SessionOptions,
    peer_script: F,
) -> (WsClientResult<()>, T)
where
    F: FnOnce(WebSocketStream<tokio::net::TcpStream>) -> Fut + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let config = Arc::new(harness.config());
    let (peer_done_tx, peer_done_rx) = oneshot::channel::<T>();

    let peer_task = tokio::spawn(async move {
        let peer = harness.accept_peer().await;
        let outcome = peer_script(peer).await;
        let _ = peer_done_tx.send(outcome);
    });

    tokio::task::yield_now().await;

    // 默认选项走公开 `open` seam（#426）；仅非默认 SessionOptions 用 open_with
    let open_result = timeout(SESSION_TIMEOUT, async move {
        if options == SessionOptions::default() {
            LarkWsClient::open(config, event_handler).await
        } else {
            LarkWsClient::open_with(config, event_handler, options).await
        }
    })
    .await
    .expect("open timed out");

    let peer_outcome = timeout(SESSION_TIMEOUT, peer_done_rx)
        .await
        .expect("peer done timed out")
        .expect("peer oneshot");
    peer_task.await.expect("peer task");

    (open_result, peer_outcome)
}

fn event_data_frame(payload: &[u8]) -> Frame {
    Frame {
        seq_id: 0,
        log_id: 100,
        service: SERVICE_ID,
        method: FRAME_METHOD_DATA,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "full-session-msg-1".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "trace-full-session-msg-1".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.to_vec()),
        log_id_new: None,
    }
}

async fn recv_next_frame(peer: &mut WebSocketStream<tokio::net::TcpStream>) -> Frame {
    loop {
        let msg = timeout(SESSION_TIMEOUT, peer.next())
            .await
            .expect("recv timed out")
            .expect("stream ended")
            .expect("websocket message");
        match msg {
            Message::Binary(data) => return Frame::decode(&*data).expect("decode frame"),
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(_) => panic!("unexpected close while waiting for frame"),
            other => panic!("unexpected websocket message: {other:?}"),
        }
    }
}

async fn recv_app_ping_frame(peer: &mut WebSocketStream<tokio::net::TcpStream>) -> Frame {
    loop {
        let frame = recv_next_frame(peer).await;
        if frame.method == FRAME_METHOD_CONTROL {
            let ty = frame
                .headers
                .iter()
                .find(|h| h.key == "type")
                .map(|h| h.value.as_str())
                .unwrap_or("");
            if ty == "ping" {
                return frame;
            }
        }
    }
}

fn pong_control_frame(ping_interval: i32) -> Frame {
    let payload = serde_json::to_vec(&json!({
        "ReconnectCount": 1,
        "ReconnectInterval": 1,
        "ReconnectNonce": 0,
        "PingInterval": ping_interval
    }))
    .expect("serialize ClientConfig");
    Frame {
        seq_id: 0,
        log_id: 0,
        service: SERVICE_ID,
        method: FRAME_METHOD_CONTROL,
        headers: vec![Header {
            key: "type".to_string(),
            value: "pong".to_string(),
        }],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload),
        log_id_new: None,
    }
}

fn assert_normal_close(result: WsClientResult<()>) {
    match result {
        Err(WsClientError::ConnectionClosed {
            reason:
                Some(WsCloseReason {
                    code: CloseCode::Normal,
                    ..
                }),
        }) => {}
        other => panic!("expected Normal ConnectionClosed, got: {other:?}"),
    }
}

// === B 组：墙钟时序（与真实定时器的集成语义） ===

#[tokio::test]
async fn full_session_pong_updates_ping_interval() {
    let (open_result, gap) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            let first = recv_app_ping_frame(&mut peer).await;
            assert_eq!(first.service, SERVICE_ID);

            // 应用 PingInterval=1 后，reset_after(1s) 应使下一 tick 约 1s 后触发；
            // 若 reset 失效，新 interval 会立即 tick，gap 会远小于 1s。
            peer.send(Message::Binary(
                pong_control_frame(1).encode_to_vec().into(),
            ))
            .await
            .expect("send pong");

            let t0 = tokio::time::Instant::now();
            timeout(Duration::from_secs(3), recv_app_ping_frame(&mut peer))
                .await
                .expect("second ping timed out — pong did not update interval?");
            let gap = t0.elapsed();

            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "pong interval test".into(),
            }))
            .await
            .ok();
            gap
        },
    )
    .await;

    assert!(
        gap >= Duration::from_millis(700),
        "second ping arrived too soon ({gap:?}); expected ~1s after reset_after(1s)"
    );
    assert!(
        gap <= Duration::from_millis(2500),
        "second ping too late ({gap:?}); expected ~1s interval"
    );
    assert_normal_close(open_result);
}

#[tokio::test]
async fn full_session_heartbeat_timeout_is_observable() {
    // 仅 WS Ping 刷新存活；peer 不发 Ping → 超时。会话级注入超时。
    let options = SessionOptions {
        heartbeat_timeout: Duration::from_millis(250),
    };

    let (open_result, ()) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        EventDispatcherHandler::builder().build(),
        options,
        |mut peer| async move {
            // 读走客户端 app ping（Binary），不发 WebSocket Ping。
            let _ = timeout(Duration::from_secs(2), peer.next()).await;
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    assert!(
        matches!(
            open_result,
            Err(WsClientError::ConnectionClosed { reason: None })
        ),
        "expected heartbeat ConnectionClosed, got: {open_result:?}"
    );
}

/// 慢 EventHandler 不应阻塞 app-level ping 发出（串行 worker + spawn_blocking）。
#[tokio::test]
async fn full_session_slow_handler_does_not_block_app_ping() {
    use std::thread;
    use std::time::Instant;

    struct SlowHandler;
    impl EventHandler for SlowHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            thread::sleep(Duration::from_millis(800));
            Ok(())
        }
    }

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(EventDispatcherHandler::RAW_EVENT_KEY, SlowHandler)
        .expect("register")
        .build();

    let (open_result, ping_during_handler) = run_session(
        LocalSessionHarness::start_with_ping_interval(1).await,
        event_handler,
        SessionOptions::default(),
        |mut peer| async move {
            // 排空首 tick ping
            let _ = timeout(SESSION_TIMEOUT, recv_app_ping_frame(&mut peer)).await;

            let payload =
                br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"slow":true}}"#;
            peer.send(Message::Binary(
                event_data_frame(payload).encode_to_vec().into(),
            ))
            .await
            .expect("send event");

            // handler 阻塞 800ms 期间，主循环仍应能发 app ping（interval=1s 且首 tick 后约 1s）
            let t0 = Instant::now();
            let got_ping = timeout(Duration::from_millis(1500), recv_app_ping_frame(&mut peer))
                .await
                .is_ok();
            let elapsed = t0.elapsed();

            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "slow handler test".into(),
            }))
            .await
            .ok();
            // 若 handler 阻塞主循环，响应写回与后续 ping 会一起卡 ≥800ms 且更易超时
            (got_ping, elapsed)
        },
    )
    .await;

    let (got_ping, _elapsed) = ping_during_handler;
    assert!(
        got_ping,
        "expected app-level ping while slow handler runs (select must not block on handler)"
    );
    assert_normal_close(open_result);
}

/// 队列积压（多帧 + 慢 handler）时 app ping 仍应发出（try_send/outbox/reserve 不阻塞 select）。
#[tokio::test]
async fn full_session_backlog_does_not_block_app_ping() {
    use std::thread;

    struct SlowHandler;
    impl EventHandler for SlowHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            thread::sleep(Duration::from_millis(50));
            Ok(())
        }
    }

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(EventDispatcherHandler::RAW_EVENT_KEY, SlowHandler)
        .expect("register")
        .build();

    let (open_result, got_ping) = run_session(
        LocalSessionHarness::start_with_ping_interval(1).await,
        event_handler,
        SessionOptions::default(),
        |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, recv_app_ping_frame(&mut peer)).await;

            // 突发 80 帧（> HANDLER_QUEUE_CAP 64），触发 outbox 路径
            for i in 0..80u8 {
                let mut payload =
                    br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"n":0}}"#
                        .to_vec();
                if let Some(last) = payload.last_mut() {
                    *last = b'0' + (i % 10);
                }
                peer.send(Message::Binary(
                    event_data_frame(&payload).encode_to_vec().into(),
                ))
                .await
                .expect("send burst event");
            }

            let got_ping = timeout(Duration::from_millis(2000), recv_app_ping_frame(&mut peer))
                .await
                .is_ok();

            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "backlog ping test".into(),
            }))
            .await
            .ok();
            // 排空可能的 ACK，避免 peer 挂起
            while let Some(Ok(_)) = timeout(Duration::from_millis(100), peer.next())
                .await
                .ok()
                .flatten()
            {}
            got_ping
        },
    )
    .await;

    assert!(
        got_ping,
        "expected app-level ping while handler queue/outbox is backlogged"
    );
    assert_normal_close(open_result);
}

/// Closing 后再收到 Binary → 必须得到 InvalidStateTransition（证明可达状态错误路径）。
///
/// 用短心跳 + 慢 handler 进入 Closing（仍有 inflight），再由 peer 发送 late Binary。
/// 不用 peer.close()（会触发 SendAfterClosing，无法再送 Binary）。
#[tokio::test]
async fn full_session_data_after_close_is_invalid_state() {
    use std::thread;

    struct SlowHandler;
    impl EventHandler for SlowHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            thread::sleep(Duration::from_millis(800));
            Ok(())
        }
    }

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(EventDispatcherHandler::RAW_EVENT_KEY, SlowHandler)
        .expect("register")
        .build();

    let options = SessionOptions {
        heartbeat_timeout: Duration::from_millis(200),
    };

    let (open_result, ()) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        event_handler,
        options,
        |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.send(Message::Binary(
                event_data_frame(br#"{"header":{"event_type":"slow"},"event":{}}"#)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send slow event");
            // 等会话因无 WS Ping 进入 Closing（heartbeat 200ms，checkout ≤200ms），
            // 且 handler 仍 inflight（800ms）
            tokio::time::sleep(Duration::from_millis(400)).await;
            peer.send(Message::Binary(
                event_data_frame(br#"{"header":{"event_type":"late"}}"#)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send late binary while session Closing");
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    match open_result {
        Err(WsClientError::InvalidStateTransition {
            kind: InvalidStateKind::DataWhileClosing,
        }) => {}
        other => panic!("expected InvalidStateTransition(DataWhileClosing), got: {other:?}"),
    }
}

/// 入站 WebSocket Ping 会刷新存活计时，避免心跳超时（#421 US 8 正向路径）。
#[tokio::test]
async fn full_session_ws_ping_refreshes_heartbeat() {
    let options = SessionOptions {
        heartbeat_timeout: Duration::from_millis(400),
    };

    let (open_result, ()) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        EventDispatcherHandler::builder().build(),
        options,
        |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            // 每 150ms 发一次 WS Ping，覆盖 400ms 超时窗口
            for _ in 0..4 {
                peer.send(Message::Ping(vec![b'h', b'b'].into()))
                    .await
                    .expect("send ws ping");
                tokio::time::sleep(Duration::from_millis(150)).await;
            }
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "ping refresh test".into(),
            }))
            .await
            .ok();
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    // 若 Ping 未刷新存活，会先以 ConnectionClosed { reason: None } 超时
    assert_normal_close(open_result);
}

// === C 组：端点发现（无 WS） ===

#[test]
fn local_endpoint_client_config_shape_matches_production() {
    // 生产 JSON 可含 Reconnect*；仅 PingInterval 被反序列化消费
    let raw =
        br#"{"ReconnectCount":1,"ReconnectInterval":1,"ReconnectNonce":0,"PingInterval":3600}"#;
    let cfg: ClientConfig = serde_json::from_slice(raw).expect("ClientConfig shape");
    assert_eq!(cfg.ping_interval, 3600);
}

/// 端点发现错误用 config：复用 LocalSessionHarness 的构建规则，但不绑 WS listener。
fn endpoint_only_config(mock_server: &MockServer) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(mock_server.uri())
        .allow_custom_base_url(true)
        .req_timeout(Duration::from_secs(5))
        .build()
}

/// code!=0 的飞书业务错误经 Transport 解码为 `CoreError::Api`，`?` 落入
/// `WsClientError::RequestError`；从 `X-Tt-Logid` 头提取的 request_id 必须保住。
#[tokio::test]
async fn open_endpoint_business_error_wraps_core_error_with_request_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/callback/ws/endpoint"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("X-Tt-Logid", "rid-524")
                .set_body_json(json!({
                    "code": 1000040343,
                    "msg": "no available endpoint",
                    "data": null,
                })),
        )
        .mount(&mock_server)
        .await;

    let config = Arc::new(endpoint_only_config(&mock_server));
    let result = LarkWsClient::open(config, EventDispatcherHandler::builder().build()).await;

    match result {
        Err(WsClientError::RequestError(core)) => {
            assert_eq!(
                core.ctx().request_id(),
                Some("rid-524"),
                "端点业务错误必须保住从 X-Tt-Logid 提取的 request_id"
            );
            assert!(
                matches!(core, openlark_core::error::CoreError::Api(_)),
                "code!=0 应解码为 CoreError::Api，got: {core:?}"
            );
        }
        other => panic!("expected RequestError(CoreError::Api), got: {other:?}"),
    }
}

/// code:0 但 data 缺 `URL` 字段：`EndPointResponse.url = None` → `open_with` 报 `UnexpectedResponse`。
#[tokio::test]
async fn open_endpoint_success_without_url_is_unexpected_response() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/callback/ws/endpoint"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": { "ClientConfig": { "PingInterval": 3600 } },
        })))
        .mount(&mock_server)
        .await;

    let config = Arc::new(endpoint_only_config(&mock_server));
    let result = LarkWsClient::open(config, EventDispatcherHandler::builder().build()).await;

    match result {
        Err(WsClientError::UnexpectedResponse) => {}
        other => panic!("expected UnexpectedResponse for missing URL, got: {other:?}"),
    }
}
