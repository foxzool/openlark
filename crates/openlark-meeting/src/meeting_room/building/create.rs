//! 创建建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::serialize_params;
use crate::meeting_room::responses::CreateBuildingResponse;

/// 创建建筑物请求
pub struct CreateBuildingRequest {
    config: Config,
}

impl CreateBuildingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateBuildingResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateBuildingResponse> {
        // url: POST:/open-apis/meeting_room/buildings
        let api_endpoint = MeetingRoomApi::BuildingCreate;
        let req: ApiRequest<CreateBuildingResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建建筑物")?);

        Transport::request_typed(req, &self.config, Some(option), "创建建筑物").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_building_request_builder() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBuildingRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
    }

    #[test]
    fn test_create_building_request_new() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBuildingRequest::new(config);

        // 验证配置正确
        assert_eq!(request.config.app_id(), "test_app");
        assert_eq!(request.config.app_secret(), "test_secret");
    }

    /// 端到端：POST .../meeting_room/buildings → CreateBuildingResponse。
    #[tokio::test]
    async fn test_create_building_returns_typed_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/buildings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "building_id": "omb_8ec170b937536a5d87c23b418b83f9bb" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateBuildingRequest::new(config)
            .execute(json!({ "name": "测试建筑" }))
            .await
            .expect("创建建筑物应成功");
        assert_eq!(resp.building_id, "omb_8ec170b937536a5d87c23b418b83f9bb");
    }
}
