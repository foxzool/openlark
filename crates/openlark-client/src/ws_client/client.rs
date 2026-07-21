//! WebSocket 公开入口与 endpoint 发现。
//!
//! 会话协议实现见 [`super::session::Session`]。

use std::collections::HashMap;
use std::sync::Arc;

use log::{debug, info};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::connect_async_with_config;
use url::Url;

use super::dispatcher::EventDispatcherHandler;
use super::session::{Session, SessionOptions, WsClientError, WsClientResult};

/// WebSocket endpoint API 专用响应结构（顶层 code/msg/data）
#[derive(Debug, Deserialize)]
struct WsEndpointApiResponse<T> {
    #[serde(default)]
    code: i32,
    #[serde(default)]
    msg: String,
    data: Option<T>,
}

fn map_ws_api_error(code: i32, message: String) -> WsClientError {
    match code {
        1 | 1000040343 => WsClientError::ServerError { code, message },
        _ => WsClientError::ClientError { code, message },
    }
}

fn extract_endpoint_response(
    resp: WsEndpointApiResponse<EndPointResponse>,
) -> WsClientResult<EndPointResponse> {
    if resp.code != 0 {
        return Err(map_ws_api_error(resp.code, resp.msg));
    }

    let end_point = resp.data.ok_or(WsClientError::UnexpectedResponse)?;
    if end_point.url.as_ref().is_none_or(|url| url.is_empty()) {
        return Err(WsClientError::ServerError {
            code: 500,
            message: "No available endpoint".to_string(),
        });
    }

    Ok(end_point)
}

const END_POINT_URL: &str = "/callback/ws/endpoint";

/// 飞书 WebSocket 客户端入口。
///
/// 连接建立后由内部单一 session loop 拥有：I/O、心跳、控制帧、分包、事件调度与写回。
pub struct LarkWsClient;

impl LarkWsClient {
    /// 建立 WebSocket 长连接并运行完整会话，直到关闭或错误。
    ///
    /// # 返回
    ///
    /// 生产路径在会话终止时几乎总是 `Err`：
    /// - `Err(WsClientError::ConnectionClosed { reason })`：对端 Close（含正常关闭
    ///   code）或入站空闲超时；**正常断开也是 `Err`，调用方请匹配此变体**
    /// - 其它 `Err`：端点查询、传输、malformed 控制帧、未知 frame method、
    ///   非法会话状态等
    ///
    /// 入站空闲超时**仅**在收到 WebSocket 层 `Ping` 时刷新（与历史行为一致）。
    pub async fn open(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        Self::open_with(config, event_handler, SessionOptions::default()).await
    }

    /// 与 [`open`](Self::open) 相同，可注入会话选项（测试用心跳超时等）。
    pub(crate) async fn open_with(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
        options: SessionOptions,
    ) -> WsClientResult<()> {
        let end_point = Self::get_conn_url(&config).await?;
        let conn_url = end_point.url.ok_or(WsClientError::UnexpectedResponse)?;
        let client_config = end_point
            .client_config
            .ok_or(WsClientError::UnexpectedResponse)?;
        let url = Url::parse(&conn_url)?;
        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        let service_id = query_pairs
            .get("service_id")
            .ok_or(WsClientError::UnexpectedResponse)?
            .parse()
            .map_err(|_| WsClientError::UnexpectedResponse)?;

        let ws_config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default()
            .max_message_size(Some(config.max_response_size() as usize))
            .max_frame_size(Some(config.max_response_size() as usize));

        let (conn, _response) = connect_async_with_config(conn_url, Some(ws_config), false).await?;
        info!("connected to {url}");

        Session::new(service_id, client_config, conn, event_handler, options)
            .run()
            .await
    }

    /// 获取连接配置
    async fn get_conn_url(
        config: &Arc<openlark_core::config::Config>,
    ) -> WsClientResult<EndPointResponse> {
        let body = json!({
            "AppID": config.app_id(),
            "AppSecret": config.app_secret()
        });

        let mut http_builder = Client::builder();
        if let Some(timeout) = config.req_timeout() {
            http_builder = http_builder.timeout(timeout);
        }
        let http_client = http_builder.build()?;

        let base_url = config.base_url().trim_end_matches('/');
        let req = http_client
            .post(format!("{base_url}{END_POINT_URL}"))
            .header("locale", "zh")
            .json(&body)
            .send()
            .await?;

        let resp = req
            .json::<WsEndpointApiResponse<EndPointResponse>>()
            .await?;
        debug!("{:?}", resp.data);

        extract_endpoint_response(resp)
    }
}

/// WebSocket 端点查询响应（crate 内部）。
#[derive(Debug, Deserialize)]
pub(crate) struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<ClientConfig>,
}

/// 服务端下发的 WebSocket 客户端配置（crate 内部）。
///
/// 会话仅消费 `PingInterval`。endpoint/pong JSON 中可能还带 `Reconnect*` 字段，
/// serde 默认忽略未知键；本 crate 不实现重连策略（#421）。
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ClientConfig {
    #[serde(rename = "PingInterval")]
    pub(crate) ping_interval: i32,
}

#[cfg(test)]
mod tests {
    use super::{
        WsClientError, WsEndpointApiResponse, extract_endpoint_response, map_ws_api_error,
    };

    #[test]
    fn test_ws_endpoint_error_response_not_treated_as_success() {
        let payload = r#"{"code":400,"msg":"Bad Request"}"#;
        let parsed = serde_json::from_str::<WsEndpointApiResponse<serde_json::Value>>(payload)
            .expect("endpoint response should deserialize");

        assert_eq!(parsed.code, 400);
        assert_eq!(parsed.msg, "Bad Request");
        assert!(parsed.data.is_none());

        let mapped = map_ws_api_error(parsed.code, parsed.msg);
        assert!(matches!(
            mapped,
            WsClientError::ClientError { code: 400, .. }
        ));
    }

    #[test]
    fn test_ws_endpoint_success_without_data_returns_unexpected_response() {
        let resp = WsEndpointApiResponse::<super::EndPointResponse> {
            code: 0,
            msg: "success".to_string(),
            data: None,
        };

        let result = extract_endpoint_response(resp);
        assert!(matches!(result, Err(WsClientError::UnexpectedResponse)));
    }

    #[test]
    fn test_ws_endpoint_server_error_mapping_is_preserved() {
        let mapped = map_ws_api_error(1000040343, "No available endpoint".to_string());
        assert!(matches!(
            mapped,
            WsClientError::ServerError {
                code: 1000040343,
                ..
            }
        ));
    }
}
