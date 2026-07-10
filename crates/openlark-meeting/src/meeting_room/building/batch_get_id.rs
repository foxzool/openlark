//! 查询建筑物ID
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-building-id>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::meeting_room::responses::BatchGetBuildingIdResponse;
use crate::{common::api_utils::extract_response_data, endpoints::MEETING_ROOM};

/// 查询建筑物ID请求
pub struct BatchGetBuildingIdRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetBuildingIdRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-building-id>
    pub async fn execute(self) -> SDKResult<BatchGetBuildingIdResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchGetBuildingIdResponse> {
        // url: GET:/open-apis/meeting_room/building/batch_get_id
        let mut req: ApiRequest<BatchGetBuildingIdResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/building/batch_get_id"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询建筑物ID")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_room/building/batch_get_id → BatchGetBuildingIdResponse。
    #[tokio::test]
    async fn test_batch_get_id_building_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/building/batch_get_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "buildings": [
                        {
                            "building_id": "bldg_001",
                            "custom_bulding_id": "test01"
                        },
                        {
                            "building_id": "bldg_002",
                            "custom_bulding_id": "test02"
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

        let resp = BatchGetBuildingIdRequest::new(config)
            .query_param("page_size", "20")
            .execute()
            .await
            .expect("查询建筑物ID应成功");
        assert_eq!(resp.buildings[0].building_id, "bldg_001");
        assert_eq!(
            resp.buildings[0].custom_bulding_id.as_deref(),
            Some("test01")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/building/batch_get_id"
        );
        assert_eq!(received[0].url.query(), Some("page_size=20"));
    }
}
