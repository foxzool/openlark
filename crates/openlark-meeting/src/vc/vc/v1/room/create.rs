//! 创建会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 创建会议室请求
#[derive(Debug, Clone)]
pub struct CreateRoomRequest {
    /// 配置信息
    config: Config,
}

/// 创建会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomResponse {
    /// 会议室 ID
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateRoomResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateRoomResponse> {
        let api_endpoint = VcApiV1::RoomCreate;
        let api_request: ApiRequest<CreateRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建会议室")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateRoomRequest::new(config.clone());
        let _ = request;
    }
}
