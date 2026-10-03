//! Session 行为测试：内存双工 seam（ADR-0006，#640）。
//!
//! 与 [`super::full_session_tests`]（wiremock + 真实 TCP + 真实时间的端到端层）互补，
//! 本层直接构造 `Session<DuplexStream>`：`tokio::io::duplex` + 两侧
//! `from_raw_socket` 让 tungstenite 在内存流上执行真实 WS 成帧（Ping/Pong、
//! `max_message_size` 守卫全部生效）。同步以 channel/gate 原语替代 `thread::sleep`
//! 排序，负向断言以「后到事件的 ACK」作正向同步点；唯一例外是串行派发用例——
//! 「证明串行」这一消极性质与零等待不可兼得，用有界反证窗口观察（见该用例注释）。
//!
//! 时序语义（墙钟间隔、心跳超时）留在 full_session_tests 端到端层。

#![cfg(feature = "websocket")]

use std::sync::mpsc as sync_mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use base64::Engine as _;
use futures_util::{SinkExt, StreamExt};
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use prost::Message as ProstMessage;
use tokio::io::{AsyncWriteExt, DuplexStream, duplex};
use tokio::sync::oneshot;
use tokio::time::timeout;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::protocol::Role;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};

use super::client::ClientConfig;
use super::frame_handler::{FRAME_METHOD_CONTROL, FRAME_METHOD_DATA};
use super::session::{Session, SessionOptions};
use super::{
    CallbackEventHandler, EventDispatcherHandler, EventHandler, WsClientError, WsClientResult,
    WsCloseReason,
};

const SERVICE_ID: i32 = 42;
const SESSION_TIMEOUT: Duration = Duration::from_secs(10);

type PeerWs = WebSocketStream<DuplexStream>;

/// 内存双工连接：client 侧交给 Session，server 侧作为对端。可选 client 侧
/// WS 配置（如 `max_message_size` 守卫，等价生产 `max_response_size` 路径）。
async fn in_memory_conn(
    client_ws_config: Option<WebSocketConfig>,
) -> (WebSocketStream<DuplexStream>, PeerWs) {
    // 容量须容纳 BacklogFull 用例的整批突发帧（~140 帧 × <1KB），避免对端 send 阻塞
    let (client_io, peer_io) = duplex(256 * 1024);
    let client = WebSocketStream::from_raw_socket(client_io, Role::Client, client_ws_config).await;
    let peer = WebSocketStream::from_raw_socket(peer_io, Role::Server, None).await;
    (client, peer)
}

/// 运行一次内存会话：peer 脚本与 `Session::run` 并发，返回 run 结果与 peer 产出。
async fn run_session<F, Fut, T>(
    ping_interval: i32,
    event_handler: EventDispatcherHandler,
    options: SessionOptions,
    peer_script: F,
) -> (WsClientResult<()>, T)
where
    F: FnOnce(PeerWs) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    run_session_with_client_config(ping_interval, None, event_handler, options, peer_script).await
}

async fn run_session_with_client_config<F, Fut, T>(
    ping_interval: i32,
    client_ws_config: Option<WebSocketConfig>,
    event_handler: EventDispatcherHandler,
    options: SessionOptions,
    peer_script: F,
) -> (WsClientResult<()>, T)
where
    F: FnOnce(PeerWs) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (conn, peer_ws) = in_memory_conn(client_ws_config).await;
    let session = Session::new(
        SERVICE_ID,
        ClientConfig {
            ping_interval: ping_interval.max(1),
        },
        conn,
        event_handler,
        options,
    );

    let (peer_done_tx, peer_done_rx) = oneshot::channel::<T>();
    let peer_task = tokio::spawn(async move {
        let _ = peer_done_tx.send(peer_script(peer_ws).await);
    });

    tokio::task::yield_now().await;

    let run_result = timeout(SESSION_TIMEOUT, session.run())
        .await
        .expect("session timed out");
    let peer_outcome = timeout(SESSION_TIMEOUT, peer_done_rx)
        .await
        .expect("peer done timed out")
        .expect("peer oneshot");
    peer_task.await.expect("peer task");

    (run_result, peer_outcome)
}

// === 慢 handler 的确定性替代：started 信号 + gate 放行（无 sleep） ===

/// handler 进入时发 started 信号，然后阻塞等待 gate 放行——「慢 handler」的
/// 确定性形态：测试在确切知道 handler 已在 worker 中运行后再推进下一事件。
struct GatedHandler {
    started: tokio::sync::mpsc::Sender<()>,
    // std Receiver 是 !Sync；包 Mutex 以满足 EventHandler 的 Send + Sync
    gate: Mutex<sync_mpsc::Receiver<()>>,
}

impl EventHandler for GatedHandler {
    fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.started
            .blocking_send(())
            .expect("started channel alive");
        self.gate
            .lock()
            .expect("gate mutex")
            .recv()
            .expect("gate released before channel closed");
        Ok(())
    }
}

fn gated_handler() -> (
    EventDispatcherHandler,
    tokio::sync::mpsc::Receiver<()>,
    sync_mpsc::Sender<()>,
) {
    let (started_tx, started_rx) = tokio::sync::mpsc::channel::<()>(1);
    let (gate_tx, gate_rx) = sync_mpsc::channel::<()>();
    let handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            GatedHandler {
                started: started_tx,
                gate: Mutex::new(gate_rx),
            },
        )
        .expect("register gated handler")
        .build();
    (handler, started_rx, gate_tx)
}

/// 记录调用次数与最后 payload 的原始事件处理器。
struct CountingHandler {
    calls: Arc<std::sync::atomic::AtomicUsize>,
    last_payload: Arc<Mutex<Vec<u8>>>,
}

impl EventHandler for CountingHandler {
    fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::sync::atomic::Ordering;
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self.last_payload.lock().expect("payload mutex") = payload.to_vec();
        Ok(())
    }
}

fn counting_handler(
    calls: &Arc<std::sync::atomic::AtomicUsize>,
    last_payload: &Arc<Mutex<Vec<u8>>>,
) -> EventDispatcherHandler {
    EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            CountingHandler {
                calls: Arc::clone(calls),
                last_payload: Arc::clone(last_payload),
            },
        )
        .expect("register counting handler")
        .build()
}

// === 帧构造 / 接收 helpers ===

fn event_data_frame(payload: &[u8]) -> Frame {
    multipart_event_frame("session-msg-1", None, None, payload)
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
        method: FRAME_METHOD_DATA,
        headers,
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.to_vec()),
        log_id_new: None,
    }
}

fn malformed_pong_frame() -> Frame {
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
        payload: Some(b"{ not-json".to_vec()),
        log_id_new: None,
    }
}

fn invalid_method_frame() -> Frame {
    Frame {
        seq_id: 0,
        log_id: 0,
        service: SERVICE_ID,
        method: 99,
        headers: vec![],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"x".to_vec()),
        log_id_new: None,
    }
}

/// server→client 的裸 WebSocket Binary 帧（无 mask）。用于 peer 端 tungstenite
/// 已 CLOSING、无法再 `send` 时绕过其状态机违约发送数据帧。
fn raw_binary_ws_frame(payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + payload.len());
    frame.push(0x82); // FIN + opcode 2 (binary)；server→client 不 mask
    if payload.len() <= 125 {
        frame.push(payload.len() as u8);
    } else {
        frame.push(126);
        frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    }
    frame.extend_from_slice(payload);
    frame
}

async fn recv_next_frame(peer: &mut PeerWs) -> Frame {
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

async fn recv_data_response_frame(peer: &mut PeerWs) -> Frame {
    loop {
        let frame = recv_next_frame(peer).await;
        if frame.method == FRAME_METHOD_DATA {
            return frame;
        }
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

// === 用例 ===

/// 事件派发 + ACK 帧写回（method/code/biz_rt），经真实 sink→对端路径。
#[tokio::test]
async fn session_dispatches_handler_and_emits_response_frame() {
    use std::sync::atomic::Ordering;

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_payload = Arc::new(Mutex::new(Vec::new()));
    let event_payload =
        br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"text":"hi"}}"#;

    let (open_result, response_frame) = run_session(
        3600,
        counting_handler(&calls, &last_payload),
        SessionOptions::default(),
        move |mut peer| async move {
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
    assert_eq!(response_frame.method, FRAME_METHOD_DATA);
    let body = String::from_utf8(response_frame.payload.expect("payload")).expect("utf8");
    assert!(body.contains("\"code\":200"), "got: {body}");
    assert!(response_frame.headers.iter().any(|h| h.key == "biz_rt"));
    assert_normal_close(open_result);
}

/// 对端 Close(Away) 的 code/message 经会话结果可观察。
#[tokio::test]
async fn session_remote_close_reason_is_observable() {
    let (run_result, ()) = run_session(
        3600,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
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

    match run_result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason { code, message }),
        }) => {
            assert_eq!(code, CloseCode::Away);
            assert_eq!(message, "server restarting");
        }
        other => panic!("expected remote close reason, got: {other:?}"),
    }
}

/// 对端直接 drop（无 Close 帧）：双工流 EOF，会话以无 reason 关闭可观察。
#[tokio::test]
async fn session_abrupt_peer_drop_is_observable_as_session_error() {
    let (run_result, ()) = run_session(
        3600,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |peer| async move {
            drop(peer);
        },
    )
    .await;

    match run_result {
        Err(WsClientError::WsError(_)) | Err(WsClientError::ConnectionClosed { reason: None }) => {}
        other => panic!("expected session transport/close error, got: {other:?}"),
    }
}

/// 乱序多包（seq1 先到）只派发一次，payload 按序拼接。
#[tokio::test]
async fn session_multipart_out_of_order_dispatches_once() {
    use std::sync::atomic::Ordering;

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_payload = Arc::new(Mutex::new(Vec::new()));
    let part0 = b"Hello ";
    let part1 = b"World!";

    let (run_result, response_frame) = run_session(
        3600,
        counting_handler(&calls, &last_payload),
        SessionOptions::default(),
        move |mut peer| async move {
            peer.send(Message::Binary(
                multipart_event_frame("multipart-ood-1", Some(2), Some(1), part1)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send part1");
            peer.send(Message::Binary(
                multipart_event_frame("multipart-ood-1", Some(2), Some(0), part0)
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
        b"Hello World!"
    );
    let body = String::from_utf8(response_frame.payload.expect("payload")).expect("utf8");
    assert!(body.contains("\"code\":200"), "got: {body}");
    assert_normal_close(run_result);
}

/// 不完整多包（只到 seq0/sum2）扣留不派发。负向断言的确定性同步：紧随其后
/// 发一个完整事件并等它的 ACK——串行 worker + 到达序保证此时早到的扣留帧
/// 若会派发必然已派发（calls 会是 2 而非 1）。
#[tokio::test]
async fn session_multipart_incomplete_does_not_dispatch() {
    use std::sync::atomic::Ordering;

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_payload = Arc::new(Mutex::new(Vec::new()));
    let valid_payload = br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"ok":1}}"#;

    let (run_result, ()) = run_session(
        3600,
        counting_handler(&calls, &last_payload),
        SessionOptions::default(),
        move |mut peer| async move {
            peer.send(Message::Binary(
                multipart_event_frame("multipart-incomplete-1", Some(2), Some(0), b"only-part-0")
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send incomplete");
            // 确定性同步点：后到完整事件的 ACK
            peer.send(Message::Binary(
                event_data_frame(valid_payload).encode_to_vec().into(),
            ))
            .await
            .expect("send valid follow-up");
            let _ = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "incomplete package test".into(),
            }))
            .await
            .ok();
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1, "扣留帧不得派发");
    assert_eq!(
        last_payload.lock().expect("mutex").as_slice(),
        valid_payload
    );
    assert_normal_close(run_result);
}

/// 无效 frame method（99）经会话 Result 可观察。
#[tokio::test]
async fn session_invalid_frame_method_is_session_error() {
    let (run_result, ()) = run_session(
        3600,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            peer.send(Message::Binary(
                invalid_method_frame().encode_to_vec().into(),
            ))
            .await
            .expect("send invalid method frame");
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    match run_result {
        Err(WsClientError::InvalidFrameMethod { method }) => {
            assert_eq!(method, 99);
        }
        other => panic!("expected InvalidFrameMethod, got: {other:?}"),
    }
}

/// 超大帧受 client 侧 `max_message_size` 守卫拒绝（等价生产 max_response_size 路径；
/// from_raw_socket 携带同一 WebSocketConfig 语义）。
#[tokio::test]
async fn session_oversized_frame_is_rejected() {
    const TINY_MAX: usize = 512;
    // 与生产 client.rs 的 max_response_size 路径对齐：双守卫同时设置
    let ws_config = WebSocketConfig::default()
        .max_message_size(Some(TINY_MAX))
        .max_frame_size(Some(TINY_MAX));

    let (run_result, ()) = run_session_with_client_config(
        3600,
        Some(ws_config),
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            let huge = vec![0u8; 4096];
            let _ = peer.send(Message::Binary(huge.into())).await;
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    // 精确断言守卫拒绝路径：宽三选一（WsError/ConnectionClosed/ProstError）在守卫
    // 失效时仍会被 4096 零字节帧的 prost 解码错误兜住（对拍审查实证），抓不住
    // 配置链路退化，故锚定 MessageTooLong 本体。
    match run_result {
        Err(WsClientError::WsError(err)) => match *err {
            tungstenite::Error::Capacity(tungstenite::error::CapacityError::MessageTooLong {
                size,
                max_size,
            }) => {
                assert_eq!(size, 4096);
                assert_eq!(max_size, TINY_MAX);
            }
            other => panic!("expected Capacity(MessageTooLong), got: {other:?}"),
        },
        other => panic!("expected WsError(Capacity(MessageTooLong)), got: {other:?}"),
    }
}

/// 非 JSON 的 pong 控制帧 → MalformedControlFrame。
#[tokio::test]
async fn session_malformed_pong_is_session_error() {
    let (run_result, ()) = run_session(
        3600,
        EventDispatcherHandler::builder().build(),
        SessionOptions::default(),
        |mut peer| async move {
            peer.send(Message::Binary(
                malformed_pong_frame().encode_to_vec().into(),
            ))
            .await
            .expect("send malformed pong");
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    match run_result {
        Err(WsClientError::MalformedControlFrame { message }) => {
            assert!(
                message.contains("invalid ClientConfig") || message.contains("malformed"),
                "unexpected message: {message}"
            );
        }
        other => panic!("expected MalformedControlFrame, got: {other:?}"),
    }
}

/// Close + inflight handler：排空后远端关闭原因保留（不被后续路径覆盖）。
/// 确定性形态：handler gated（确切 inflight）时对端发 Close(Away)，随后放行。
#[tokio::test]
async fn session_close_reason_preserved_with_inflight_handler() {
    let (handler, mut started_rx, gate_tx) = gated_handler();

    let (run_result, ()) =
        run_session(3600, handler, SessionOptions::default(), move |mut peer| {
            async move {
                peer.send(Message::Binary(
                    event_data_frame(br#"{"header":{"event_type":"slow"},"event":{}}"#)
                        .encode_to_vec()
                        .into(),
                ))
                .await
                .expect("send event");
                // 等到 handler 确切进入 worker（inflight）
                started_rx.recv().await.expect("handler started");
                peer.close(Some(CloseFrame {
                    code: CloseCode::Away,
                    reason: "server restarting".into(),
                }))
                .await
                .ok();
                // 放行 handler，让会话排空并 idle 终止
                gate_tx.send(()).expect("release gate");
                while let Some(Ok(_)) = peer.next().await {}
            }
        })
        .await;

    match run_result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason { code, message }),
        }) => {
            assert_eq!(code, CloseCode::Away);
            assert_eq!(message, "server restarting");
        }
        other => panic!("expected ConnectionClosed with Away reason, got: {other:?}"),
    }
}

/// 远端 Close(Away) 后违约再发数据帧：该传输错误不得覆盖已记录的关闭原因
/// （#421 US9）。确定性形态：handler 保持 gated（inflight）期间发 Close 与
/// 违约 Binary，随后放行让 worker 回收不超时。
#[tokio::test]
async fn session_data_after_remote_close_preserves_close_reason() {
    let (handler, mut started_rx, gate_tx) = gated_handler();

    let (run_result, ()) =
        run_session(3600, handler, SessionOptions::default(), move |mut peer| {
            async move {
                peer.send(Message::Binary(
                    event_data_frame(br#"{"header":{"event_type":"slow"},"event":{}}"#)
                        .encode_to_vec()
                        .into(),
                ))
                .await
                .expect("send event");
                started_rx.recv().await.expect("handler started");
                peer.send(Message::Close(Some(CloseFrame {
                    code: CloseCode::Away,
                    reason: "server restarting".into(),
                })))
                .await
                .expect("send close");
                // peer 端 tungstenite 已 CLOSING，无法再 send Binary；raw write 违约发送
                let late = event_data_frame(br#"{"header":{"event_type":"late"}}"#).encode_to_vec();
                peer.get_mut()
                    .write_all(&raw_binary_ws_frame(&late))
                    .await
                    .expect("raw write late binary");
                // 放行 worker，避免 run 收尾等 WORKER_SHUTDOWN_TIMEOUT
                gate_tx.send(()).expect("release gate");
                while let Some(Ok(_)) = peer.next().await {}
            }
        })
        .await;

    match run_result {
        Err(WsClientError::ConnectionClosed {
            reason: Some(WsCloseReason { code, message }),
        }) => {
            assert_eq!(code, CloseCode::Away);
            assert_eq!(message, "server restarting");
        }
        other => panic!("expected ConnectionClosed with Away reason, got: {other:?}"),
    }
}

/// 非法多包（空 message_id）：扣留、不派发（确定性同步同 incomplete 用例）。
#[tokio::test]
async fn session_multipart_empty_message_id_does_not_dispatch() {
    use std::sync::atomic::Ordering;

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_payload = Arc::new(Mutex::new(Vec::new()));
    let valid_payload = br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"ok":2}}"#;

    let (run_result, ()) = run_session(
        3600,
        counting_handler(&calls, &last_payload),
        SessionOptions::default(),
        move |mut peer| async move {
            let mut frame = multipart_event_frame("", Some(2), Some(0), b"withheld-empty-mid");
            frame.headers.retain(|h| h.key != "message_id");
            frame.headers.push(Header {
                key: "message_id".to_string(),
                value: String::new(),
            });
            peer.send(Message::Binary(frame.encode_to_vec().into()))
                .await
                .expect("send invalid multipart");
            peer.send(Message::Binary(
                event_data_frame(valid_payload).encode_to_vec().into(),
            ))
            .await
            .expect("send valid follow-up");
            let _ = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "empty message_id withhold".into(),
            }))
            .await
            .ok();
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1, "空 message_id 帧不得派发");
    assert_eq!(
        last_payload.lock().expect("mutex").as_slice(),
        valid_payload
    );
    assert_normal_close(run_result);
}

/// 非法多包（seq >= sum）：扣留、不派发（确定性同步同上）。
#[tokio::test]
async fn session_multipart_seq_out_of_range_does_not_dispatch() {
    use std::sync::atomic::Ordering;

    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_payload = Arc::new(Mutex::new(Vec::new()));
    let valid_payload = br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"ok":3}}"#;

    let (run_result, ()) = run_session(
        3600,
        counting_handler(&calls, &last_payload),
        SessionOptions::default(),
        move |mut peer| async move {
            peer.send(Message::Binary(
                multipart_event_frame("oob-msg", Some(2), Some(5), b"withheld-oob-seq")
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send oob multipart");
            peer.send(Message::Binary(
                event_data_frame(valid_payload).encode_to_vec().into(),
            ))
            .await
            .expect("send valid follow-up");
            let _ = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "seq oob withhold".into(),
            }))
            .await
            .ok();
        },
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1, "seq 越界帧不得派发");
    assert_eq!(
        last_payload.lock().expect("mutex").as_slice(),
        valid_payload
    );
    assert_normal_close(run_result);
}

/// EventHandler panic → 会话以 HandlerPanicked 结束。
#[tokio::test]
async fn session_handler_panic_is_session_error() {
    struct PanicHandler;
    impl EventHandler for PanicHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            panic!("intentional handler panic for session_behavior test");
        }
    }

    let event_handler = EventDispatcherHandler::builder()
        .register_raw(EventDispatcherHandler::RAW_EVENT_KEY, PanicHandler)
        .expect("register")
        .build();

    let (run_result, ()) = run_session(
        3600,
        event_handler,
        SessionOptions::default(),
        |mut peer| async move {
            let payload =
                br#"{"header":{"event_type":"im.message.receive_v1"},"event":{"panic":true}}"#;
            peer.send(Message::Binary(
                event_data_frame(payload).encode_to_vec().into(),
            ))
            .await
            .expect("send event");
            while let Some(Ok(_)) = peer.next().await {}
        },
    )
    .await;

    match run_result {
        Err(WsClientError::HandlerPanicked) => {}
        other => panic!("expected HandlerPanicked, got: {other:?}"),
    }
}

/// 事件 handler 串行：慢(1) 先到且确切运行中，快(2) 后到入队——完成顺序必须
/// 仍为 [1, 2]（串行 worker 不重排）。
///
/// 「证明串行」是消极性质，与零等待不可兼得（串行实现下 handler(2) 的 started
/// 信号永不到来，纯 channel 原语没有观察窗口）。本用例用**有界反证窗口**：
/// 2 入队后等 handler(2) 的 started 信号至多 500ms——并发派发回归会在此窗口内
/// 现形（started2 到达即失败）；串行实现下窗口超时后才放行 gate1 继续正向断言。
/// 该 500ms 是观察窗口而非排序依赖（对拍审查实证：无此窗口时注入并发 worker
/// 8/8 假阴性）。
#[tokio::test]
async fn session_handlers_run_serially_in_arrival_order() {
    struct OrderedHandler {
        log: Arc<Mutex<Vec<u8>>>,
        started1: tokio::sync::mpsc::Sender<()>,
        started2: tokio::sync::mpsc::Sender<()>,
        gate1: Mutex<sync_mpsc::Receiver<()>>,
    }
    impl EventHandler for OrderedHandler {
        fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let id = payload.last().copied().unwrap_or(0);
            match id {
                1 => {
                    self.started1
                        .blocking_send(())
                        .expect("started1 channel alive");
                    self.gate1
                        .lock()
                        .expect("gate1 mutex")
                        .recv()
                        .expect("gate1 released");
                }
                2 => {
                    // 若 worker 并发派发，此信号会在反证窗口内到达
                    let _ = self.started2.blocking_send(());
                }
                _ => {}
            }
            self.log.lock().expect("mutex").push(id);
            Ok(())
        }
    }

    let log = Arc::new(Mutex::new(Vec::new()));
    let (started1_tx, mut started1_rx) = tokio::sync::mpsc::channel::<()>(1);
    let (started2_tx, mut started2_rx) = tokio::sync::mpsc::channel::<()>(1);
    let (gate1_tx, gate1_rx) = sync_mpsc::channel::<()>();
    let event_handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            OrderedHandler {
                log: Arc::clone(&log),
                started1: started1_tx,
                started2: started2_tx,
                gate1: Mutex::new(gate1_rx),
            },
        )
        .expect("register")
        .build();

    let (run_result, ()) = run_session(
        3600,
        event_handler,
        SessionOptions::default(),
        move |mut peer| {
            async move {
                let mut p1 = br#"{"header":{"event_type":"t"},"event":{"n":1}}"#.to_vec();
                p1.push(1);
                let mut p2 = br#"{"header":{"event_type":"t"},"event":{"n":2}}"#.to_vec();
                p2.push(2);
                peer.send(Message::Binary(
                    event_data_frame(&p1).encode_to_vec().into(),
                ))
                .await
                .expect("send 1");
                started1_rx.recv().await.expect("handler 1 started");
                // 2 在 1 仍运行期间入队
                peer.send(Message::Binary(
                    event_data_frame(&p2).encode_to_vec().into(),
                ))
                .await
                .expect("send 2");
                // 有界反证窗口：并发派发会让 started2 在此窗口内到达
                if timeout(Duration::from_millis(500), started2_rx.recv())
                    .await
                    .is_ok()
                {
                    panic!("handler(2) 在 handler(1) 未完成时开始——worker 不是串行派发");
                }
                // 放行 1；两个 ACK 按完成序到达
                gate1_tx.send(()).expect("release gate1");
                let _ = recv_data_response_frame(&mut peer).await;
                let _ = recv_data_response_frame(&mut peer).await;
                peer.close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: "serial order test".into(),
                }))
                .await
                .ok();
            }
        },
    )
    .await;

    assert_eq!(
        log.lock().expect("mutex").as_slice(),
        &[1, 2],
        "handlers must run in arrival order (serial worker)"
    );
    assert_normal_close(run_result);
}

/// 队列(64) + outbox(64) 双满 → BacklogFull。确定性形态：首个 handler gated
/// 占住 worker（确切运行中），随后突发帧灌满两级缓冲；第 129 帧之后必然触发。
///
/// 本用例耗时下限为 `WORKER_SHUTDOWN_TIMEOUT`（5s）：BacklogFull 后 worker 手握
/// 65 个待发 outcome（hold 1 + channel 64）而 outcome 通道容量 64、receiver 活到
/// `run()` 返回——第 65 次 send 阻塞直到超时 abort。与旧 full_session 同款用例
/// 行为一致（parity）；断言本身不依赖时间。
#[tokio::test]
async fn session_backlog_full_is_session_error() {
    // channel 64 + outbox 64 + 1 在飞 ≈ 需 ≥129 帧；多送抗边界
    const BURST: usize = 140;

    let (handler, mut started_rx, gate_tx) = gated_handler();

    let (run_result, ()) = run_session(3600, handler, SessionOptions::default(), move |mut peer| {
        async move {
            peer.send(Message::Binary(
                event_data_frame(br#"{"header":{"event_type":"hold"},"event":{}}"#)
                    .encode_to_vec()
                    .into(),
            ))
            .await
            .expect("send hold event");
            started_rx.recv().await.expect("handler started");

            for i in 0..BURST {
                let payload = format!(
                    r#"{{"header":{{"event_type":"im.message.receive_v1"}},"event":{{"n":{i}}}}}"#
                );
                peer.send(Message::Binary(
                    multipart_event_frame(&format!("backlog-{i}"), None, None, payload.as_bytes())
                        .encode_to_vec()
                        .into(),
                ))
                .await
                .expect("send burst frame");
            }

            // 放行 worker（gate 不影响断言：BacklogFull 在突发期间已确定触发）
            gate_tx.send(()).expect("release gate");
            while let Some(Ok(_)) = peer.next().await {}
        }
    })
    .await;

    match run_result {
        Err(WsClientError::BacklogFull { message }) => {
            assert!(
                message.contains("64"),
                "BacklogFull message should mention capacity, got: {message}"
            );
        }
        other => panic!("expected BacklogFull, got: {other:?}"),
    }
}

/// callback ACK 的 session 级 round-trip（#634 缺口）：callback handler 返回的
/// 业务 JSON 经 ACK 帧 `data` 字段以 base64 写回，对端逐字节解码验证——
/// 此前该路径只有 frame 级单测，未经真实 session→sink 线。
#[tokio::test]
async fn session_callback_ack_round_trips_base64_data() {
    struct ToastCallbackHandler;
    impl CallbackEventHandler for ToastCallbackHandler {
        fn handle(
            &self,
            _payload: &[u8],
        ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Some(serde_json::json!({
                "toast": {"type": "info", "title": "hi"}
            })))
        }
    }

    let event_handler = EventDispatcherHandler::builder()
        .register_callback("card.action.trigger", ToastCallbackHandler)
        .expect("register callback")
        .build();

    let payload = br#"{"header":{"event_type":"card.action.trigger"},"event":{"action":"btn"}}"#;

    let (run_result, ack_frame) = run_session(
        3600,
        event_handler,
        SessionOptions::default(),
        move |mut peer| async move {
            peer.send(Message::Binary(
                event_data_frame(payload).encode_to_vec().into(),
            ))
            .await
            .expect("send callback event");
            let ack = recv_data_response_frame(&mut peer).await;
            peer.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "callback ack test".into(),
            }))
            .await
            .ok();
            ack
        },
    )
    .await;

    assert_eq!(ack_frame.method, FRAME_METHOD_DATA);
    let ack_json: serde_json::Value =
        serde_json::from_slice(ack_frame.payload.expect("ack payload").as_slice())
            .expect("ack payload is json");
    assert_eq!(
        ack_json.get("code").and_then(|v| v.as_i64()),
        Some(200),
        "callback path ACK code 应为 200，got: {ack_json}"
    );
    let data_b64 = ack_json
        .get("data")
        .and_then(|v| v.as_str())
        .expect("ACK data 字段应为 base64 字符串");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .expect("base64 decode");
    // 逐字节断言：dispatcher 内 serde_json::to_vec 的输出与同值 json! 的 to_vec
    // 在 BTreeMap 键序下逐字节一致（serde_json 无 preserve_order feature）
    let expected = serde_json::to_vec(&serde_json::json!({
        "toast": {"type": "info", "title": "hi"}
    }))
    .expect("serialize expected");
    assert_eq!(
        decoded, expected,
        "base64(data) 解码后应逐字节等于 callback 返回的业务 JSON 序列化"
    );
    assert_normal_close(run_result);
}
