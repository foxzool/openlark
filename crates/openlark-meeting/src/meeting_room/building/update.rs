//! 更新建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-building>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::meeting_room::responses::UpdateBuildingResponse;
use crate::{common::api_endpoints::MeetingRoomApi, common::api_utils::serialize_params};

/// 更新建筑物请求
pub struct UpdateBuildingRequest {
    config: Config,
    building_id: String,
}

impl UpdateBuildingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            building_id: String::new(),
        }
    }

    /// 建筑物 ID
    pub fn building_id(mut self, building_id: impl Into<String>) -> Self {
        self.building_id = building_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-building>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<UpdateBuildingResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<UpdateBuildingResponse> {
        validate_required!(self.building_id, "building_id 不能为空");

        let api_endpoint = MeetingRoomApi::BuildingPatch(self.building_id.clone());
        let req: ApiRequest<UpdateBuildingResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "更新建筑物")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        // 官方示例无 data 字段；成功且缺省时返回空响应。
        if !resp.is_success() {
            return Err(openlark_core::error::api_error(
                resp.code() as u16,
                "更新建筑物",
                resp.message().to_string(),
                resp.raw().request_id.clone(),
            ));
        }
        Ok(resp.data.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../meeting_room/buildings/{building_id} → UpdateBuildingResponse。
    #[tokio::test]
    async fn test_update_building_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/buildings/bldg_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success"
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UpdateBuildingRequest::new(config)
            .building_id("bldg_001")
            .execute(json!({ "name": "更新1号楼" }))
            .await
            .expect("更新建筑物应成功");
        assert_eq!(resp, UpdateBuildingResponse {});

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/buildings/bldg_001"
        );
        assert_eq!(received[0].method, "POST");
    }
}
