//! 创建建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建建筑物请求
pub struct CreateBuildingRequest {
    config: Config,
}

impl CreateBuildingRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/buildings
        let api_endpoint = MeetingRoomApi::BuildingCreate;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建建筑物")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建建筑物")
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
}
