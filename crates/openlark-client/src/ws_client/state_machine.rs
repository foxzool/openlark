use tokio_tungstenite::tungstenite::protocol::CloseFrame;

/// WebSocket 连接状态机
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// 初始状态
    Initial,
    /// 正在连接
    Connecting,
    /// 已连接
    Connected,
    /// 正在断开连接
    Disconnecting,
    /// 已断开连接
    Disconnected {
        /// 断开连接原因。
        reason: Option<CloseReason>,
    },
    /// 连接错误
    Error {
        /// 错误描述。
        message: String,
    },
}

/// 连接关闭原因
#[derive(Debug, Clone, PartialEq)]
pub struct CloseReason {
    /// 关闭状态码。
    pub code: u16,
    /// 关闭原因描述。
    pub reason: String,
}

impl From<CloseFrame> for CloseReason {
    fn from(frame: CloseFrame) -> Self {
        Self {
            code: frame.code.into(),
            reason: frame.reason.to_string(),
        }
    }
}

/// 状态机事件
#[derive(Debug, Clone)]
pub enum StateMachineEvent {
    /// 开始连接
    StartConnection,
    /// 连接成功
    ConnectionEstablished,
    /// 收到 Ping
    PingReceived,
    /// 收到 Pong
    PongReceived,
    /// 收到数据
    DataReceived,
    /// 请求断开连接
    RequestDisconnect,
    /// 连接关闭
    ConnectionClosed(Option<CloseReason>),
    /// 发生错误
    ErrorOccurred(String),
}

/// WebSocket 状态机
pub struct WebSocketStateMachine {
    state: ConnectionState,
}

impl WebSocketStateMachine {
    /// 创建新的 WebSocket 状态机。
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Initial,
        }
    }

    /// 获取当前状态
    pub fn current_state(&self) -> &ConnectionState {
        &self.state
    }

    /// 处理事件并转换状态
    pub fn handle_event(&mut self, event: StateMachineEvent) -> Result<(), String> {
        use ConnectionState::*;
        use StateMachineEvent::*;

        let new_state = match (&self.state, event.clone()) {
            // 从初始状态开始连接
            (Initial, StartConnection) => Connecting,

            // 连接成功
            (Connecting, ConnectionEstablished) => Connected,

            // 在已连接状态下的各种事件
            (Connected, PingReceived) => Connected,
            (Connected, PongReceived) => Connected,
            (Connected, DataReceived) => Connected,
            (Connected, RequestDisconnect) => Disconnecting,

            // 断开连接
            (Disconnecting, ConnectionClosed(reason)) => Disconnected { reason },
            (Connected, ConnectionClosed(reason)) => Disconnected { reason },

            // 错误处理
            (_, ErrorOccurred(msg)) => Error { message: msg },

            // 非法状态转换
            _ => {
                return Err(format!(
                    "Invalid state transition from {:?} with event {:?}",
                    self.state, event
                ));
            }
        };

        self.state = new_state;
        Ok(())
    }

    /// 检查是否可以发送数据
    pub fn can_send_data(&self) -> bool {
        matches!(self.state, ConnectionState::Connected)
    }

    /// 检查是否正在连接
    pub fn is_connecting(&self) -> bool {
        matches!(self.state, ConnectionState::Connecting)
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        matches!(self.state, ConnectionState::Connected)
    }

    /// 检查是否已断开
    pub fn is_disconnected(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::Disconnected { .. } | ConnectionState::Error { .. }
        )
    }
}

impl Default for WebSocketStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let mut sm = WebSocketStateMachine::new();

        // 初始状态
        assert_eq!(sm.current_state(), &ConnectionState::Initial);

        // 开始连接
        assert!(sm.handle_event(StateMachineEvent::StartConnection).is_ok());
        assert_eq!(sm.current_state(), &ConnectionState::Connecting);

        // 连接成功
        assert!(
            sm.handle_event(StateMachineEvent::ConnectionEstablished)
                .is_ok()
        );
        assert_eq!(sm.current_state(), &ConnectionState::Connected);
        assert!(sm.can_send_data());

        // 收到数据
        assert!(sm.handle_event(StateMachineEvent::DataReceived).is_ok());
        assert_eq!(sm.current_state(), &ConnectionState::Connected);

        // 断开连接
        assert!(
            sm.handle_event(StateMachineEvent::RequestDisconnect)
                .is_ok()
        );
        assert_eq!(sm.current_state(), &ConnectionState::Disconnecting);

        // 连接关闭
        let close_reason = Some(CloseReason {
            code: 1000,
            reason: "Normal closure".to_string(),
        });
        assert!(
            sm.handle_event(StateMachineEvent::ConnectionClosed(close_reason.clone()))
                .is_ok()
        );
        assert_eq!(
            sm.current_state(),
            &ConnectionState::Disconnected {
                reason: close_reason
            }
        );
    }

    #[test]
    fn test_invalid_transitions() {
        let mut sm = WebSocketStateMachine::new();

        // 不能从初始状态直接收到数据
        assert!(sm.handle_event(StateMachineEvent::DataReceived).is_err());
    }
}
