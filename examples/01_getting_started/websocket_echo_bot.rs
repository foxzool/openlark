//! WebSocket Echo Bot 示例。
//!
//! 展示如何使用 openlark 根 crate 建立飞书长连接、接收消息事件，并用服务端 API 回显文本消息。
//!
//! 运行前需要在飞书开放平台启用机器人、订阅 `im.message.receive_v1` 事件，
//! 并开通发送消息相关权限。
//!
//! ```bash
//! export OPENLARK_APP_ID="cli_xxx"
//! export OPENLARK_APP_SECRET="xxx"
//! cargo run --example websocket_echo_bot --no-default-features --features "communication,websocket"
//! ```

use std::sync::Arc;
use std::time::Duration;

use open_lark::auth::AuthService;
use open_lark::communication::im::v1::message::{
    create::{CreateMessageBody, CreateMessageRequest},
    models::ReceiveIdType,
};
use open_lark::ws_client::{EventDispatcherHandler, EventHandler, LarkWsClient};
use open_lark::{Config, CoreConfig, RequestOption};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let runtime_config = RuntimeConfig::from_env()?;
    println!("🚀 启动 WebSocket Echo Bot");
    println!("🌐 API Base URL: {}", runtime_config.base_url);
    println!(
        "📦 WebSocket 最大帧大小: {} bytes",
        runtime_config.max_response_size
    );
    println!(
        "🔁 回显发送: {}",
        if runtime_config.echo_enabled {
            "enabled"
        } else {
            "disabled"
        }
    );

    let ws_config = Config::builder()
        .app_id(runtime_config.app_id.clone())
        .app_secret(runtime_config.app_secret.clone())
        .base_url(runtime_config.base_url.clone())
        .timeout(Duration::from_secs(runtime_config.timeout_secs))
        .max_response_size(runtime_config.max_response_size)
        .build()
        .map_err(|e| format!("构建 WebSocket 配置失败: {e}"))?;

    let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    tokio::spawn(process_payload_loop(payload_rx, runtime_config.clone()));

    let event_handler = EventDispatcherHandler::builder()
        .payload_sender(payload_tx)
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            LoggingRawEventHandler,
        )
        .map_err(|e| format!("注册 WebSocket 原始事件处理器失败: {e}"))?
        .build();

    println!("🔌 正在建立飞书长连接...");
    LarkWsClient::open(Arc::new(ws_config), event_handler).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct RuntimeConfig {
    app_id: String,
    app_secret: String,
    base_url: String,
    timeout_secs: u64,
    max_response_size: u64,
    echo_enabled: bool,
    echo_prefix: String,
}

impl RuntimeConfig {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id =
            std::env::var("OPENLARK_APP_ID").map_err(|_| "未找到环境变量 OPENLARK_APP_ID")?;
        let app_secret = std::env::var("OPENLARK_APP_SECRET")
            .map_err(|_| "未找到环境变量 OPENLARK_APP_SECRET")?;
        let base_url = std::env::var("OPENLARK_BASE_URL")
            .unwrap_or_else(|_| "https://open.feishu.cn".to_string());
        let timeout_secs = parse_env_u64("OPENLARK_TIMEOUT", 30)?;
        if timeout_secs == 0 {
            return Err("OPENLARK_TIMEOUT 必须大于 0".into());
        }
        let max_response_size = parse_env_u64("OPENLARK_MAX_RESPONSE_SIZE", 100 * 1024 * 1024)?;
        if max_response_size == 0 {
            return Err("OPENLARK_MAX_RESPONSE_SIZE 必须大于 0".into());
        }
        let echo_enabled = parse_env_bool("OPENLARK_WS_ECHO_ENABLED", true)?;
        let echo_prefix =
            std::env::var("OPENLARK_WS_ECHO_PREFIX").unwrap_or_else(|_| "Echo: ".to_string());

        Ok(Self {
            app_id,
            app_secret,
            base_url,
            timeout_secs,
            max_response_size,
            echo_enabled,
            echo_prefix,
        })
    }

    fn core_config(&self) -> CoreConfig {
        CoreConfig::builder()
            .app_id(self.app_id.clone())
            .app_secret(self.app_secret.clone())
            .base_url(self.base_url.clone())
            .enable_token_cache(false)
            .req_timeout(Duration::from_secs(self.timeout_secs))
            .max_response_size(self.max_response_size)
            .build()
    }
}

async fn process_payload_loop(
    mut payload_rx: mpsc::UnboundedReceiver<Vec<u8>>,
    runtime_config: RuntimeConfig,
) {
    while let Some(payload) = payload_rx.recv().await {
        if let Err(err) = handle_payload(&runtime_config, &payload).await {
            eprintln!("❌ 处理事件失败: {err}");
        }
    }
}

async fn handle_payload(
    runtime_config: &RuntimeConfig,
    payload: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let envelope: EventEnvelope = match serde_json::from_slice(payload) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("⚠️ 忽略无法解析的事件载荷: {err}");
            return Ok(());
        }
    };

    if envelope.header.event_type != "im.message.receive_v1" {
        return Ok(());
    }

    if envelope.event.message.message_type != "text" {
        println!("ℹ️ 跳过非文本消息: {}", envelope.event.message.message_type);
        return Ok(());
    }

    let text = extract_text(&envelope.event.message.content)?;
    if text.trim().is_empty() {
        println!("ℹ️ 跳过空文本消息");
        return Ok(());
    }

    if should_skip_echo(&text, &runtime_config.echo_prefix) {
        println!("ℹ️ 跳过疑似 Echo Bot 自己发出的消息");
        return Ok(());
    }

    let (receive_id, receive_id_type) = resolve_receive_target(&envelope.event)?;
    let echo_text = format!("{}{}", runtime_config.echo_prefix, text);
    if runtime_config.echo_enabled {
        send_echo_message(runtime_config, &receive_id, receive_id_type, &echo_text).await?;
    } else {
        println!(
            "🧪 dry-run: receive_id_type={}, receive_id={}, text={}",
            receive_id_type.as_str(),
            receive_id,
            echo_text
        );
        return Ok(());
    }

    println!(
        "✅ Echo 成功: receive_id_type={}, receive_id={receive_id}",
        receive_id_type.as_str()
    );
    Ok(())
}

fn extract_text(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content_json: TextContent =
        serde_json::from_str(content).map_err(|e| format!("解析文本消息 content 失败: {e}"))?;

    Ok(content_json.text)
}

fn resolve_receive_target(
    event: &EventBody,
) -> Result<(String, ReceiveIdType), Box<dyn std::error::Error>> {
    if event.message.chat_type == "p2p" {
        let open_id = event.sender.sender_id.open_id.clone();
        if open_id.is_empty() {
            return Err("p2p 消息缺少 sender.open_id".into());
        }
        return Ok((open_id, ReceiveIdType::OpenId));
    }

    if let Some(chat) = &event.chat
        && !chat.chat_id.is_empty()
    {
        return Ok((chat.chat_id.clone(), ReceiveIdType::ChatId));
    }

    if let Some(chat_id) = &event.message.chat_id
        && !chat_id.is_empty()
    {
        return Ok((chat_id.clone(), ReceiveIdType::ChatId));
    }

    Err("群聊消息缺少 chat_id".into())
}

fn should_skip_echo(text: &str, echo_prefix: &str) -> bool {
    !echo_prefix.is_empty() && text.starts_with(echo_prefix)
}

async fn send_echo_message(
    runtime_config: &RuntimeConfig,
    receive_id: &str,
    receive_id_type: ReceiveIdType,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let tenant_access_token = fetch_tenant_access_token(runtime_config).await?;
    let body = CreateMessageBody {
        receive_id: receive_id.to_string(),
        msg_type: "text".to_string(),
        content: json!({ "text": text }).to_string(),
        uuid: None,
    };
    let option = RequestOption::builder()
        .tenant_access_token(tenant_access_token)
        .build();

    CreateMessageRequest::new(runtime_config.core_config())
        .receive_id_type(receive_id_type)
        .execute_with_options(body, option)
        .await
        .map_err(|e| format!("发送 Echo 失败: {e}"))?;

    Ok(())
}

async fn fetch_tenant_access_token(
    runtime_config: &RuntimeConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let auth_service = AuthService::new(runtime_config.core_config());
    let token_response = auth_service
        .v3()
        .tenant_access_token_internal()
        .app_id(runtime_config.app_id.clone())
        .app_secret(runtime_config.app_secret.clone())
        .execute()
        .await
        .map_err(|e| format!("获取 tenant_access_token 失败: {e}"))?;

    Ok(token_response.data.tenant_access_token)
}

fn parse_env_u64(name: &str, default: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let Ok(value) = std::env::var(name) else {
        return Ok(default);
    };
    let value = value.trim();
    if value.is_empty() {
        return Ok(default);
    }
    value
        .parse::<u64>()
        .map_err(|e| format!("{name} 必须是无符号整数: {e}").into())
}

fn parse_env_bool(name: &str, default: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let Ok(value) = std::env::var(name) else {
        return Ok(default);
    };
    match value.trim().to_ascii_lowercase().as_str() {
        "" => Ok(default),
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(format!("{name} 必须是 true/false 或 1/0").into()),
    }
}

struct LoggingRawEventHandler;

impl EventHandler for LoggingRawEventHandler {
    fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event_type = serde_json::from_slice::<EventHeaderOnlyEnvelope>(payload)
            .ok()
            .map(|envelope| envelope.header.event_type)
            .filter(|event_type| !event_type.trim().is_empty())
            .unwrap_or_else(|| "unknown".to_string());
        println!(
            "📨 收到 WebSocket 事件: event_type={event_type}, payload_size={} bytes",
            payload.len()
        );
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct EventHeaderOnlyEnvelope {
    header: EventHeader,
}

#[derive(Debug, Deserialize)]
struct EventEnvelope {
    header: EventHeader,
    event: EventBody,
}

#[derive(Debug, Deserialize)]
struct EventHeader {
    event_type: String,
}

#[derive(Debug, Deserialize)]
struct EventBody {
    sender: Sender,
    message: Message,
    #[serde(default)]
    chat: Option<Chat>,
}

#[derive(Debug, Deserialize)]
struct Sender {
    sender_id: SenderId,
}

#[derive(Debug, Deserialize)]
struct SenderId {
    open_id: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    message_type: String,
    content: String,
    chat_type: String,
    #[serde(default)]
    chat_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Chat {
    chat_id: String,
}

#[derive(Debug, Deserialize)]
struct TextContent {
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_success() {
        let text = extract_text(r#"{"text":"hello"}"#).expect("should parse text content");
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_resolve_receive_target_for_p2p() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "p2p".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let (receive_id, receive_id_type) =
            resolve_receive_target(&event).expect("should resolve p2p receive target");
        assert_eq!(receive_id, "ou_test_user");
        assert_eq!(receive_id_type, ReceiveIdType::OpenId);
    }

    #[test]
    fn test_resolve_receive_target_for_group() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: None,
            },
            chat: Some(Chat {
                chat_id: "oc_group_001".to_string(),
            }),
        };

        let (receive_id, receive_id_type) =
            resolve_receive_target(&event).expect("should resolve group receive target");
        assert_eq!(receive_id, "oc_group_001");
        assert_eq!(receive_id_type, ReceiveIdType::ChatId);
    }

    #[test]
    fn test_extract_text_invalid_json() {
        let result = extract_text("not-json");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_receive_target_group_fallback_message_chat_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: Some("oc_group_from_message".to_string()),
            },
            chat: None,
        };

        let (receive_id, receive_id_type) =
            resolve_receive_target(&event).expect("should resolve from message.chat_id");
        assert_eq!(receive_id, "oc_group_from_message");
        assert_eq!(receive_id_type, ReceiveIdType::ChatId);
    }

    #[test]
    fn test_resolve_receive_target_p2p_missing_open_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: String::new(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "p2p".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let result = resolve_receive_target(&event);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_receive_target_group_missing_chat_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let result = resolve_receive_target(&event);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_payload_invalid_json_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
            max_response_size: 1024 * 1024,
            echo_enabled: true,
            echo_prefix: "Echo: ".to_string(),
        };

        let result = handle_payload(&runtime_config, b"invalid-json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_non_message_event_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
            max_response_size: 1024 * 1024,
            echo_enabled: true,
            echo_prefix: "Echo: ".to_string(),
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.chat.member.added_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "text",
                    "content": "{\"text\":\"hi\"}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_non_text_event_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
            max_response_size: 1024 * 1024,
            echo_enabled: true,
            echo_prefix: "Echo: ".to_string(),
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.message.receive_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "image",
                    "content": "{}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_empty_text_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
            max_response_size: 1024 * 1024,
            echo_enabled: true,
            echo_prefix: "Echo: ".to_string(),
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.message.receive_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "text",
                    "content": "{\"text\":\"   \"}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_skip_echo_with_prefix() {
        assert!(should_skip_echo("Echo: hello", "Echo: "));
        assert!(!should_skip_echo("hello", "Echo: "));
        assert!(!should_skip_echo("Echo: hello", ""));
    }
}
