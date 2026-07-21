//! 查询建筑物详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/query-building-details>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::endpoints::MEETING_ROOM;
use crate::meeting_room::responses::BatchGetBuildingResponse;

/// 查询建筑物详情请求
pub struct BatchGetBuildingRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetBuildingRequest {
    /// 创建新的请求构建器。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/query-building-details>
    pub async fn execute(self) -> SDKResult<BatchGetBuildingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchGetBuildingResponse> {
        // url: GET:/open-apis/meeting_room/building/batch_get
        let mut req: ApiRequest<BatchGetBuildingResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/building/batch_get"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "查询建筑物详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_room/building/batch_get → BatchGetBuildingResponse。
    #[tokio::test]
    async fn test_batch_get_building_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/building/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "buildings": [
                        {
                            "building_id": "bldg_001",
                            "name": "1号楼",
                            "floors": ["F1"],
                            "country_id": "1814991",
                            "district_id": "2034437"
                        }
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

        let resp = BatchGetBuildingRequest::new(config)
            .query_param("page_size", "10")
            .execute()
            .await
            .expect("查询建筑物详情应成功");
        assert_eq!(resp.buildings[0].building_id, "bldg_001");
        assert_eq!(resp.buildings[0].name.as_deref(), Some("1号楼"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/building/batch_get"
        );
        assert_eq!(received[0].url.query(), Some("page_size=10"));
    }
}
