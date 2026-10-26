//! 搜索会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/search>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 搜索会议室层级请求
pub struct SearchRoomLevelRequest {
    config: Config,
    /// 查询参数
    query: SearchRoomLevelQuery,
}

/// 搜索会议室层级查询参数
///
/// custom_level_ids 必填（多个自定义层级 ID 以英文逗号分隔）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRoomLevelQuery {
    /// 用于查询指定会议室层级的自定义会议室层级 ID（多个以英文逗号分隔，如 "1000,1001"）
    pub custom_level_ids: String,
}

impl SearchRoomLevelRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: SearchRoomLevelQuery {
                custom_level_ids: String::new(),
            },
        }
    }

    /// 设置自定义会议室层级 ID（多个以英文逗号分隔，如 "1000,1001"）
    pub fn custom_level_ids(mut self, custom_level_ids: impl Into<String>) -> Self {
        self.query.custom_level_ids = custom_level_ids.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/search>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.custom_level_ids, "custom_level_ids 不能为空");

        // url: GET:/open-apis/vc/v1/room_levels/search
        let q = self.query;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/search", VC_V1_ROOM_LEVELS))
                .query("custom_level_ids", q.custom_level_ids);

        Transport::request_typed(req, &self.config, Some(option), "搜索会议室层级").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/room_levels/search → 裸 Value 解析（单层 resp["field"]）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_search_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/room_levels/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "level_ids": [
                        "omb_4ad1a2c7a2fbc5fc9570f38456931293"
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

        let resp = SearchRoomLevelRequest::new(config)
            .custom_level_ids("1000,1001")
            .execute()
            .await
            .expect("搜索会议室层级应成功");
        assert_eq!(
            resp["level_ids"][0],
            json!("omb_4ad1a2c7a2fbc5fc9570f38456931293")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/room_levels/search"
        );
        // 官方必填查询参数：custom_level_ids
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("custom_level_ids=1000%2C1001"));
    }
}
