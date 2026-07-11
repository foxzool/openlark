//! 完整会话本地 adapter 测试（#426 / #427）。
//!
//! 测试 seam：
//! - 公开入口：[`LarkWsClient::open`]
//! - 本地 HTTP endpoint（`/callback/ws/endpoint`）+ 本地 WebSocket peer
//! - 可观察结果：EventHandler 调用、peer 收到的响应帧、`open` 返回的关闭原因
//!
//! 不直接调用 FrameHandler / 状态机。
//! #427：分包组装 → 派发 → 同一会话写回；单包/多包只派发一次。

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

use super::{
    ClientConfig, EventDispatcherHandler, EventHandler, LarkWsClient, WsClientError, WsCloseReason,
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
    /// 持有 mock server 生命周期，保证 endpoint 在整个会话期间可用。
    mock_server: MockServer,
    listener: Option<TcpListener>,
}

impl LocalSessionHarness {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind local websocket listener");
        let ws_addr = listener
            .local_addr()
            .expect("local websocket address");

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
                        // 大间隔避免测试期间额外 ping 干扰；首 tick 仍会立刻发一次
                        "PingInterval": 3600
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

    /// 接受一次 WebSocket 连接并返回 peer stream。
    async fn accept_peer(
        &mut self,
    ) -> WebSocketStream<tokio::net::TcpStream> {
        let listener = self
            .listener
            .take()
            .expect("listener already consumed");
        let (stream, _) = timeout(SESSION_TIMEOUT, listener.accept())
            .await
            .expect("accept timed out")
            .expect("accept connection");
        accept_async(stream)
            .await
            .expect("websocket handshake")
    }
}

fn event_data_frame(payload: &[u8]) -> Frame {
    multipart_event_frame("full-session-msg-1", None, None, payload)
}

/// 构造数据事件帧；`sum`/`seq` 为 `Some` 时表示分包。
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
        method: 1, // data
        headers,
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.to_vec()),
        log_id_new: None,
    }
}

/// 从 peer 侧读取下一帧 protobuf Frame（跳过 WebSocket 层 ping/pong）。
async fn recv_next_frame(
    peer: &mut WebSocketStream<tokio::net::TcpStream>,
) -> Frame {
    loop {
        let msg = timeout(SESSION_TIMEOUT, peer.next())
            .await
            .expect("recv timed out")
            .expect("stream ended")
            .expect("websocket message");
        match msg {
            Message::Binary(data) => {
                return Frame::decode(&*data).expect("decode frame");
            }
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(_) => panic!("unexpected close while waiting for frame"),
            other => panic!("unexpected websocket message: {other:?}"),
        }
    }
}

/// 等待 method=1 的数据响应帧；忽略客户端发出的控制/ping 帧。
async fn recv_data_response_frame(
    peer: &mut WebSocketStream<tokio::net::TcpStream>,
) -> Frame {
    loop {
        let frame = recv_next_frame(peer).await;
        if frame.method == 1 {
            return frame;
        }
        // method == 0：客户端心跳 ping 等控制帧，忽略
    }
}

#[tokio::test]
async fn full_session_dispatches_handler_and_emits_response_frame() {
    let mut harness = LocalSessionHarness::start().await;

    let calls = Arc::new(AtomicUsize::new(0));
    let last_payload = Arc::new(std::sync::Mutex::new(Vec::new()));
    let event_payload = br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"text":"hi"}}"#;

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

    let config = Arc::new(harness.config());

    let (peer_done_tx, peer_done_rx) = oneshot::channel::<Frame>();

    // peer_task 先进入 accept；open 的 connect 会 park 直到 accept 完成
    let peer_task = tokio::spawn(async move {
        let mut peer = harness.accept_peer().await;

        // 客户端 interval 首 tick 会立刻发 ping；先排空一条入站消息再下发事件，顺序更确定
        let _ = timeout(SESSION_TIMEOUT, peer.next()).await;

        let outbound = event_data_frame(event_payload);
        peer.send(Message::Binary(outbound.encode_to_vec().into()))
            .await
            .expect("send event frame");

        let response = recv_data_response_frame(&mut peer).await;

        // 正常关闭（带 reason）
        peer.close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "session complete".into(),
        }))
        .await
        .ok();

        let _ = peer_done_tx.send(response);
    });

    tokio::task::yield_now().await;

    let open_task = tokio::spawn(async move { LarkWsClient::open(config, event_handler).await });

    let open_result = timeout(SESSION_TIMEOUT, open_task)
        .await
        .expect("open task timed out")
        .expect("open task join");

    let response_frame = timeout(SESSION_TIMEOUT, peer_done_rx)
        .await
        .expect("peer done timed out")
        .expect("peer response");

    peer_task.await.expect("peer task");

    // handler 从 LarkWsClient 会话路径被调用
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        last_payload
            .lock()
            .expect("payload mutex")
            .as_slice(),
        event_payload
    );

    // 响应帧写回同一会话（peer 可观察）
    assert_eq!(response_frame.method, 1);
    let response_body =
        String::from_utf8(response_frame.payload.expect("response payload")).expect("utf8");
    assert!(
        response_body.contains("\"code\":200"),
        "expected success ack, got: {response_body}"
    );
    assert!(
        response_frame.headers.iter().any(|h| h.key == "biz_rt"),
        "response should include biz_rt header"
    );

    // 远端正常关闭原因经 open Result 可观察
    match open_result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason { code, message }),
        }) => {
            assert_eq!(code, CloseCode::Normal);
            assert_eq!(message, "session complete");
        }
        other => panic!("expected ConnectionClosed with Normal reason, got: {other:?}"),
    }
}

#[tokio::test]
async fn full_session_remote_close_reason_is_observable() {
    let mut harness = LocalSessionHarness::start().await;
    let config = Arc::new(harness.config());

    let event_handler = EventDispatcherHandler::builder().build();

    let peer_task = tokio::spawn(async move {
        let mut peer = harness.accept_peer().await;
        // 等客户端首条消息（通常是 interval 首 tick 的 ping），再发送 Close，
        // 避免与出站 ping 写竞态导致 WsError 盖住 ConnectionClosed 原因。
        let _ = timeout(SESSION_TIMEOUT, peer.next()).await;
        peer.close(Some(CloseFrame {
            code: CloseCode::Away,
            reason: "server restarting".into(),
        }))
        .await
        .ok();
        // 读到 close 完成
        while let Some(Ok(msg)) = peer.next().await {
            if matches!(msg, Message::Close(_)) {
                break;
            }
        }
    });

    tokio::task::yield_now().await;

    let open_result = timeout(
        SESSION_TIMEOUT,
        LarkWsClient::open(config, event_handler),
    )
    .await
    .expect("open timed out");

    peer_task.await.expect("peer task");

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
    let mut harness = LocalSessionHarness::start().await;
    let config = Arc::new(harness.config());
    let event_handler = EventDispatcherHandler::builder().build();

    let peer_task = tokio::spawn(async move {
        let peer = harness.accept_peer().await;
        // 直接 drop peer：无 WebSocket Close 握手，客户端应通过 open Result 看到会话错误
        drop(peer);
    });

    tokio::task::yield_now().await;

    let open_result = timeout(
        SESSION_TIMEOUT,
        LarkWsClient::open(config, event_handler),
    )
    .await
    .expect("open timed out");

    peer_task.await.expect("peer task");

    // 未完成 Close 握手时，tungstenite 通常报 Protocol(ResetWithoutClosingHandshake)；
    // 也可能落到 ConnectionClosed { reason: None }。两者都说明会话错误可观察。
    match open_result {
        Err(WsClientError::WsError(_))
        | Err(WsClientError::ConnectionClosed { reason: None }) => {}
        other => panic!("expected session transport/close error, got: {other:?}"),
    }
}

/// #427：多包乱序到达时只组装一次、只派发一次，响应经同一会话写回。
#[tokio::test]
async fn full_session_multipart_out_of_order_dispatches_once() {
    let mut harness = LocalSessionHarness::start().await;

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
        .expect("register raw handler")
        .build();

    let config = Arc::new(harness.config());
    let (peer_done_tx, peer_done_rx) = oneshot::channel::<Frame>();

    let peer_task = tokio::spawn(async move {
        let mut peer = harness.accept_peer().await;
        let _ = timeout(SESSION_TIMEOUT, peer.next()).await;

        // 乱序：先 seq=1，再 seq=0
        let frame_seq1 = multipart_event_frame(message_id, Some(2), Some(1), part1);
        let frame_seq0 = multipart_event_frame(message_id, Some(2), Some(0), part0);
        peer.send(Message::Binary(frame_seq1.encode_to_vec().into()))
            .await
            .expect("send part1");
        peer.send(Message::Binary(frame_seq0.encode_to_vec().into()))
            .await
            .expect("send part0");

        let response = recv_data_response_frame(&mut peer).await;

        peer.close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "multipart complete".into(),
        }))
        .await
        .ok();

        let _ = peer_done_tx.send(response);
    });

    tokio::task::yield_now().await;
    let open_task = tokio::spawn(async move { LarkWsClient::open(config, event_handler).await });

    let open_result = timeout(SESSION_TIMEOUT, open_task)
        .await
        .expect("open task timed out")
        .expect("open task join");
    let response_frame = timeout(SESSION_TIMEOUT, peer_done_rx)
        .await
        .expect("peer done timed out")
        .expect("peer response");
    peer_task.await.expect("peer task");

    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "multipart event must be dispatched exactly once"
    );
    assert_eq!(
        last_payload.lock().expect("payload mutex").as_slice(),
        combined.as_slice()
    );

    assert_eq!(response_frame.method, 1);
    let body = String::from_utf8(response_frame.payload.expect("payload")).expect("utf8");
    assert!(body.contains("\"code\":200"), "got: {body}");

    assert!(matches!(
        open_result,
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason {
                code: CloseCode::Normal,
                ..
            })
        })
    ));
}

/// #427：缺包时不得派发 handler，也不得写回业务响应帧。
#[tokio::test]
async fn full_session_multipart_incomplete_does_not_dispatch() {
    let mut harness = LocalSessionHarness::start().await;

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
        .expect("register raw handler")
        .build();

    let config = Arc::new(harness.config());
    let (peer_done_tx, peer_done_rx) = oneshot::channel::<usize>();

    let peer_task = tokio::spawn(async move {
        let mut peer = harness.accept_peer().await;
        let _ = timeout(SESSION_TIMEOUT, peer.next()).await;

        // 只发 seq=0，缺 seq=1
        let partial = multipart_event_frame(message_id, Some(2), Some(0), b"only-part-0");
        peer.send(Message::Binary(partial.encode_to_vec().into()))
            .await
            .expect("send incomplete part");

        // 短暂等待：若错误地派发，应在此窗口内收到 method=1 响应
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

        let _ = peer_done_tx.send(data_responses);
    });

    tokio::task::yield_now().await;
    let open_result = timeout(
        SESSION_TIMEOUT,
        LarkWsClient::open(config, event_handler),
    )
    .await
    .expect("open timed out");

    let data_responses = timeout(SESSION_TIMEOUT, peer_done_rx)
        .await
        .expect("peer done timed out")
        .expect("peer result");
    peer_task.await.expect("peer task");

    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "incomplete multipart must not dispatch handler"
    );
    assert_eq!(
        data_responses, 0,
        "incomplete multipart must not emit data response frame"
    );
    assert!(matches!(
        open_result,
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason {
                code: CloseCode::Normal,
                ..
            })
        })
    ));
}

// 确保 ClientConfig 反序列化字段与 endpoint 脚本一致（编译期/本地 harness 契约）
#[test]
fn local_endpoint_client_config_shape_matches_production() {
    let raw = br#"{"ReconnectCount":1,"ReconnectInterval":1,"ReconnectNonce":0,"PingInterval":3600}"#;
    let cfg: ClientConfig = serde_json::from_slice(raw).expect("ClientConfig shape");
    assert_eq!(cfg.ping_interval, 3600);
    assert_eq!(cfg.reconnect_count, 1);
}
