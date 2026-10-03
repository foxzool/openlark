//! 查询会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询会议室配置请求

#[derive(Debug, Clone)]
pub struct QueryRoomConfigRequest {
    /// 配置信息
    config: Config,
}

/// 查询会议室配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRoomConfigResponse {
    /// 会议室配置列表
    pub configs: Vec<RoomConfigItem>,
}

/// 会议室配置项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomConfigItem {
    /// 配置 ID
    pub config_id: String,
    /// 配置名称
    pub name: String,
}

impl ApiResponseTrait for QueryRoomConfigResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryRoomConfigRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_config/query>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<QueryRoomConfigResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<QueryRoomConfigResponse> {
        let api_request: ApiRequest<QueryRoomConfigResponse> =
            ApiRequest::get("/open-apis/vc/v1/room_configs/query").body(serde_json::to_vec(&body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "查询会议室配置").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../room_configs/query → QueryRoomConfigResponse 解析（data 信封）。
    #[tokio::test]
    async fn test_query_room_config_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/room_configs/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "configs": [
                        { "config_id": "rc_001", "name": "默认配置" }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = QueryRoomConfigRequest::new(config)
            .execute(json!({ "room_id": "room_001" }))
            .await
            .expect("查询会议室配置应成功");
        assert_eq!(resp.configs.len(), 1);
        assert_eq!(resp.configs[0].config_id, "rc_001");
        assert_eq!(resp.configs[0].name, "默认配置");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/room_configs/query"
        );
    }
}
