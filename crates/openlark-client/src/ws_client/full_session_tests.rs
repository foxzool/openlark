//! 完整会话本地 adapter 测试（#426–#429 + 单 session 重构）。
//!
//! 测试 seam：[`LarkWsClient::open`] / `open_with` + 本地 endpoint + WS peer。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
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
use super::session::SessionOptions;
use super::{
    EventDispatcherHandler, EventHandler, LarkWsClient, WsClientError, WsClientResult,
    WsCloseReason,
};

const SERVICE_ID: i32 = 42;
const SESSION_TIMEOUT: Duration = Duration::from_secs(10);

/// 记录原始事件处理器调用次数。
struct CountingHandler {
    calls: Arc<AtomicUsize>,
    last_payload: Arc<std::sync::Mutex<Vec<u8>>>,
}

impl EventHandler for CountingHandler {
    fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self
            .last_payload
            .lock()
            .expect("payload mutex should not be poisoned") = payload.to_vec();
        Ok(())
    }
}

/// 本地完整会话 harness：wiremock endpoint + 本机 WebSocket peer。
struct LocalSessionHarness {
    mock_server: MockServer,
    listener: Option<TcpListener>,
}

impl LocalSessionHarness {
    async fn start() -> Self {
        Self::start_with_ping_interval(3600).await
    }

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
        accept_async(stream)
            .await
            .expect("websocket handshake")
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
    Fut: std::future::Future<Output = T> + Send + 'static,
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

    let open_result = timeout(
        SESSION_TIMEOUT,
        LarkWsClient::open_with(config, event_handler, options),
    )
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
    multipart_event_frame("full-session-msg-1", None, None, payload)
}

fn multipart_event_frame(
    message_id: &str,
    sum: Option<usize>,
    seq: Option<usize>,
    payload: &[u8],
) -> Frame {
    let mut headers = vec![
        Header {
            key: "type".to_string(),
            value: "event".to_string(),
        },
        Header {
            key: "message_id".to_string(),
            value: message_id.to_string(),
        },
        Header {
            key: "trace_id".to_string(),
            value: format!("trace-{message_id}"),
        },
    ];
    if let Some(sum) = sum {
        headers.push(Header {
            key: "sum".to_string(),
            value: sum.to_string(),
        });
    }
    if let Some(seq) = seq {
        headers.push(Header {
            key: "seq".to_string(),
            value: seq.to_string(),
        });
    }

    Frame {
        seq_id: seq.unwrap_or(0) as u64,
        log_id: 100,
        service: SERVICE_ID,
        method: 1,
        headers,
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

async fn recv_data_response_frame(peer: &mut WebSocketStream<tokio::net::TcpStream>) -> Frame {
    loop {
        let frame = recv_next_frame(peer).await;
        if frame.method == 1 {
            return frame;
        }
    }
}

async fn recv_app_ping_frame(peer: &mut WebSocketStream<tokio::net::TcpStream>) -> Frame {
    loop {
        let frame = recv_next_frame(peer).await;
        if frame.method == 0 {
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
        method: 0,
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

fn malformed_pong_frame() -> Frame {
    Frame {
        seq_id: 0,
        log_id: 0,
        service: SERVICE_ID,
        method: 0,
        headers: vec![Header {
            key: "type".to_string(),
            value: "pong".to_string(),
        }],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"{ not-json".to_vec()),
        log_id_new: None,
    }
}

fn assert_normal_close(result: WsClientResult<()>) {
    match result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason {
                code: CloseCode::Normal,
                ..
            }),
        }) => {}
        other => panic!("expected Normal ConnectionClosed, got: {other:?}"),
    }
}

#[tokio::test]
async fn full_session_dispatches_handler_and_emits_response_frame() {
    let calls = Arc::new(AtomicUsize::new(0));
    let last_payload = Arc::new(std::sync::Mutex::new(Vec::new()));
    let event_payload =
        br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"text":"hi"}}"#;

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            CountingHandler {
                calls: Arc::clone(&calls),
                last_payload: Arc::clone(&last_payload),
            },
        )
        .expect("register raw handler")
        .build();

    let (open_result, response_frame) = run_session(
        LocalSessionHarness::start().await,
        event_handler,
        SessionOptions::default(),
        move |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.send(Message::Binary(
                event_data_frame(event_payload).encode_to_vec().into(),
            ))
            .await
            .expect("send event");
            let response = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "session complete".into(),
            }))
            .await
            .ok();
            response
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        last_payload.lock().expect("mutex").as_slice(),
        event_payload
    );
    assert_eq!(response_frame.method, 1);
    let body = String::from_utf8(response_frame.payload.expect("payload")).expect("utf8");
    assert!(body.contains("\"code\":200"), "got: {body}");
    assert!(response_frame.headers.iter().any(|h| h.key == "biz_rt"));
    assert_normal_close(open_result);
}

#[tokio::test]
async fn full_session_remote_close_reason_is_observable() {
    let (open_result, ()) = run_session(
        LocalSessionHarness::start().await,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Away,
                reason: "server restarting".into(),
            }))
            .await
            .ok();
            while let Some(Ok(msg)) = peer.next().await {
                if matches!(msg, Message::Close(_)) {
                    break;
                }
            }
        },
    )
    .await;

    match open_result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason { code, message }),
        }) => {
            assert_eq!(code, CloseCode::Away);
            assert_eq!(message, "server restarting");
        }
        other => panic!("expected remote close reason, got: {other:?}"),
    }
}

#[tokio::test]
async fn full_session_abrupt_peer_drop_is_observable_as_session_error() {
    let (open_result, ()) = run_session(
        LocalSessionHarness::start().await,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |peer| async move {
            drop(peer);
        },
    )
    .await;

    match open_result {
        Err(WsClientError::WsError(_))
        | Err(WsClientError::ConnectionClosed { reason: None }) => {}
        other => panic!("expected session transport/close error, got: {other:?}"),
    }
}

#[tokio::test]
async fn full_session_multipart_out_of_order_dispatches_once() {
    let calls = Arc::new(AtomicUsize::new(0));
    let last_payload = Arc::new(std::sync::Mutex::new(Vec::new()));
    let part0 = b"Hello ";
    let part1 = b"World!";
    let combined = b"Hello World!";
    let message_id = "multipart-ood-1";

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            CountingHandler {
                calls: Arc::clone(&calls),
                last_payload: Arc::clone(&last_payload),
            },
        )
        .expect("register")
        .build();

    let (open_result, response_frame) = run_session(
        LocalSessionHarness::start().await,
        event_handler,
        SessionOptions::default(),
        move |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.send(Message::Binary(
                multipart_event_frame(message_id, Some(2), Some(1), part1)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send part1");
            peer.send(Message::Binary(
                multipart_event_frame(message_id, Some(2), Some(0), part0)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send part0");
            let response = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "multipart complete".into(),
            }))
            .await
            .ok();
            response
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        last_payload.lock().expect("mutex").as_slice(),
        combined.as_slice()
    );
    let body = String::from_utf8(response_frame.payload.expect("payload")).expect("utf8");
    assert!(body.contains("\"code\":200"), "got: {body}");
    assert_normal_close(open_result);
}

#[tokio::test]
async fn full_session_multipart_incomplete_does_not_dispatch() {
    let calls = Arc::new(AtomicUsize::new(0));
    let last_payload = Arc::new(std::sync::Mutex::new(Vec::new()));
    let message_id = "multipart-incomplete-1";

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            CountingHandler {
                calls: Arc::clone(&calls),
                last_payload: Arc::clone(&last_payload),
            },
        )
        .expect("register")
        .build();

    let (open_result, data_responses) = run_session(
        LocalSessionHarness::start().await,
        event_handler,
        SessionOptions::default(),
        move |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.send(Message::Binary(
                multipart_event_frame(message_id, Some(2), Some(0), b"only-part-0")
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send incomplete");

            let mut data_responses = 0usize;
            let deadline = tokio::time::Instant::now() + Duration::from_millis(200);
            loop {
                let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
                if remaining.is_zero() {
                    break;
                }
                match timeout(remaining, peer.next()).await {
                    Ok(Some(Ok(Message::Binary(data)))) => {
                        let frame = Frame::decode(&*data).expect("decode");
                        if frame.method == 1 {
                            data_responses += 1;
                        }
                    }
                    Ok(Some(Ok(Message::Ping(_) | Message::Pong(_)))) => continue,
                    Ok(Some(Ok(_))) | Ok(Some(Err(_))) | Ok(None) | Err(_) => break,
                }
            }

            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "incomplete package test".into(),
            }))
            .await
            .ok();
            data_responses
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(data_responses, 0);
    assert_normal_close(open_result);
}

#[tokio::test]
async fn full_session_pong_updates_ping_interval() {
    let (open_result, ()) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            let first = recv_app_ping_frame(&mut peer).await;
            assert_eq!(first.service, SERVICE_ID);

            peer.send(Message::Binary(
                pong_control_frame(1).encode_to_vec().into(),
            ))
            .await
            .expect("send pong");

            timeout(Duration::from_secs(3), recv_app_ping_frame(&mut peer))
                .await
                .expect("second ping timed out — pong did not update interval?");

            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "pong interval test".into(),
            }))
            .await
            .ok();
        },
    )
    .await;

    assert_normal_close(open_result);
}

#[tokio::test]
async fn full_session_malformed_pong_is_session_error() {
    let (open_result, ()) = run_session(
        LocalSessionHarness::start().await,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
            peer.send(Message::Binary(
                malformed_pong_frame().encode_to_vec().into(),
            ))
            .await
            .expect("send malformed pong");
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    match open_result {
        Err(WsClientError::MalformedControlFrame { message }) => {
            assert!(
                message.contains("invalid ClientConfig") || message.contains("malformed"),
                "unexpected message: {message}"
            );
        }
        other => panic!("expected MalformedControlFrame, got: {other:?}"),
    }
}

#[tokio::test]
async fn full_session_heartbeat_timeout_is_observable() {
    // 会话级注入超时，无需全局状态 / 串行锁
    let options = SessionOptions {
        heartbeat_timeout: Duration::from_millis(250),
    };

    let (open_result, ()) = run_session(
        LocalSessionHarness::start_with_ping_interval(3600).await,
        EventDispatcherHandler::builder().build(),
        options,
        |mut peer| async move {
            // 不发送任何帧；会话会因 inactivity 超时。
            // 仍读一下可能的首 ping，避免写缓冲阻塞。
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

#[test]
fn local_endpoint_client_config_shape_matches_production() {
    let raw =
        br#"{"ReconnectCount":1,"ReconnectInterval":1,"ReconnectNonce":0,"PingInterval":3600}"#;
    let cfg: ClientConfig = serde_json::from_slice(raw).expect("ClientConfig shape");
    assert_eq!(cfg.ping_interval, 3600);
    assert_eq!(cfg.reconnect_count, 1);
}
