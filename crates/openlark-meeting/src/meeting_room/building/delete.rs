//! 删除建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::meeting_room::responses::DeleteBuildingResponse;

/// 删除建筑物请求
pub struct DeleteBuildingRequest {
    config: Config,
    building_id: String,
}

impl DeleteBuildingRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building>
    pub async fn execute(self) -> SDKResult<DeleteBuildingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteBuildingResponse> {
        validate_required!(self.building_id, "building_id 不能为空");

        // url: DELETE:/open-apis/meeting_room/buildings/:building_id
        let api_endpoint = MeetingRoomApi::BuildingDelete(self.building_id.clone());
        let req: ApiRequest<DeleteBuildingResponse> = ApiRequest::delete(api_endpoint.to_url())
            .body(serde_json::json!({
                "building_id": self.building_id
            }));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        // 官方示例无 data 字段；成功且缺省时返回空响应。
        if !resp.is_success() {
            return Err(openlark_core::error::api_error(
                resp.code() as u16,
                "删除建筑物",
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

    /// 端到端：DELETE .../meeting_room/buildings/{building_id} → DeleteBuildingResponse。
    #[tokio::test]
    async fn test_delete_building_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
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

        let resp = DeleteBuildingRequest::new(config)
            .building_id("bldg_001")
            .execute()
            .await
            .expect("删除建筑物应成功");
        assert_eq!(resp, DeleteBuildingResponse {});

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/buildings/bldg_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
