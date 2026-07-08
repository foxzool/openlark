//! 搜索会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/search>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 搜索会议室层级请求
pub struct SearchRoomLevelRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl SearchRoomLevelRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
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
        // url: GET:/open-apis/vc/v1/room_levels/search
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/search", VC_V1_ROOM_LEVELS));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索会议室层级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/room_levels/search → 裸 Value 解析（单层 resp["field"]）+ query 断言。
    #[tokio::test]
    async fn test_search_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/room_levels/search"))
            .and(query_param("page_size", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "room_levels": [
                        { "room_level_id": "lvl_001", "name": "一楼层级" }
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
            .query_param("page_size", "20")
            .execute()
            .await
            .expect("搜索会议室层级应成功");
        assert_eq!(resp["room_levels"][0]["room_level_id"], json!("lvl_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/room_levels/search"
        );
    }
}
