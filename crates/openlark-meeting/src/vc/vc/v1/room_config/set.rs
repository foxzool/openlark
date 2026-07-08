//! 设置会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 设置会议室配置请求
pub struct SetRoomConfigRequest {
    config: Config,
}

impl SetRoomConfigRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default(), body)
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
        body: serde_json::Value,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_configs/set
        let url = "/open-apis/vc/v1/room_configs/set";
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(url).body(serialize_params(&body, "设置会议室配置")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "设置会议室配置")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../room_configs/set → data 信封解析。
    #[tokio::test]
    async fn test_set_room_config_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/room_configs/set"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "updated": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = SetRoomConfigRequest::new(config)
            .execute(json!({ "room_id": "room_001", "config": {} }))
            .await
            .expect("设置会议室配置应成功");
        assert_eq!(resp["updated"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/room_configs/set");
    }
}
