# ADR: ws Session 读写 seam（泛型字节流 + 双 adapter）

- **状态**: Accepted（随 #640 落地）
- **日期**: 2026-08-17
- **决策者**: 架构评审 + 用户 grilling 共识（候选 B）
- **相关 issue**: #640（seam + 测试下沉）、#641（CloseIntent 收敛 + harness 瘦身）
- **来源**: `/improve-codebase-architecture` 候选 B（Session 状态机只有全链路一档测试 seam）

## 背景

`Session`（`session.rs`，478 行）持有 `WebSocketStream<MaybeTlsStream<TcpStream>>` 的
split 半部——字节流具体类型硬编码进字段。全部状态机行为（close reason 保留、
outbox/reserve、心跳、DataWhileClosing、串行派发、BacklogFull）只能经
`LocalSessionHarness`（wiremock HTTP + 真实 TCP listener + `accept_async`）测试，
且多个用例依赖真实时间（`thread::sleep(400-800ms)`、200-2500ms 墙钟窗口断言），
慢且对调度抖动脆弱。测试金字塔缺中间档：package/frame_handler/dispatcher 有纯单元
seam，session 一档都没有。

### 已核实约束（grilling 事实轮）

- Session 对 I/O 的使用面仅 4 处：`stream.next()`（select 分支）与 3× `sink.send(...)`
  （`SinkExt::send` 内含 flush；无显式 flush/close/unsplit）。
- worker spawn 只 clone `event_handler`，不持有字节流——泛型参数不进入 spawn 边界。
- `Session`/`SessionOptions`/`open_with` 均 `pub(crate)`；`LarkWsClient::open` 保持非泛型。
- workspace tokio 无 `test-util` feature（无 paused clock 先例）。

## 决策

`Session` 泛型于底层字节流：`Session<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>`，
字段为 `WebSocketStream<S>` 的 split 半部（split 仍在 `new` 内）。生产实例化
`S = MaybeTlsStream<TcpStream>`（`connect_async`，client.rs 唯一实例化点）；测试
adapter 用 `tokio::io::duplex` + 两侧 `WebSocketStream::from_raw_socket(Role::Client/Server)`
——tungstenite 给内存流真实 WS 成帧，Ping/Pong、`max_message_size` 守卫全部原样生效。

两个 adapter 坐实真 seam：tungstenite（生产）/ 内存双工（测试），非单实现假设。

### 明确不选的方案

| 方案 | 不选理由 |
|------|----------|
| 自定义 `WsIo` trait（recv/send） | 重新发明 `Stream`/`Sink` 已给的能力，多一层要维护的 interface |
| `Box<dyn Stream + Sink>` | 动态派发 + Pin 体操，为测试 seam 付不必要的运行时代价 |
| `test-util` paused clock | workspace 无先例；与 handler 内 blocking 池 sleep 混用交互微妙 |
| pub testing facade | 消费者只有本 crate 测试；core `TestServer` 的 cfg-test-死代码 + 文档漂移是反面教材 |

### adapter 位置

内存 adapter 为 `#[cfg(test)]` 模块内联（`session_behavior_tests.rs`），不 pub。
时序类测试（墙钟间隔断言、心跳超时）留在 harness 端到端层——时序是「与真实定时器
的集成语义」，端到端层测最诚实；新层测试零真实时间依赖（同步用 channel/barrier
原语替代 sleep）。

## 理由

1. **deletion test 通过**：删除 seam 泛型化会把全部 session 行为测试重新锁回
   全链路 harness——复杂度在 seam 处集中（4 个 I/O 调用点 + 1 个泛型参数），
   换来 478 行状态机的独立可测性。
2. **零新 abstraction**：没有新 trait、没有 Box——`WebSocketStream<S>` 本就泛型，
   只是把 `TcpStream` 的硬编码放开；生产路径代码不变（类型推断吃掉泛型）。
3. **tungstenite 自己就是 adapter 库**：`from_raw_socket` 让内存流获得与生产
   完全一致的成帧/协议语义，测试 adapter 不需要重新实现任何 WS 行为。

## 后果

### 正面

- Session 状态机测试脱离真实 TCP/wiremock/真实时间（close reason 保留、
  outbox、背压、串行、DataWhileClosing 均可在内存 seam 上确定性测试）。
- callback ACK 的 session 级 round-trip（base64 data 经 sink→对端）可测（#634 缺口）。
- harness 保留的时序/端点测试语义不变，作为 parity 与集成层。

### 负面 / 残差

- `Session<S>` 的泛型参数使 session.rs 内部签名略重（对外零暴露）。
- B组墙钟测试（6 个）仍留 harness，测试总时长不因本 ADR 缩短。

## 非目标

- 不动 ADR-0003 的 WS upgrade 独立传输归属（本 seam 只在 session 内部）。
- 不实现重连（#421 决策维持）。
- 不为 examples/下游提供 pub testing facade（无消费者）。
