//! 创建会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat,
    req_option::RequestOption},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建会议室层级请求
#[derive(Debug, Clone)]
pub struct CreateRoomLevelRequest {
    config: Config,
}

/// 创建会议室层级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomLevelResponse {
    /// 会议室层级 ID
    pub room_level_id: String,
}

impl ApiResponseTrait for CreateRoomLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoomLevelRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateRoomLevelResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateRoomLevelResponse> {
        let api_endpoint = VcApiV1::RoomLevelCreate;
        let req: ApiRequest<CreateRoomLevelResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "创建会议室层级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建会议室层级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder().app_id("test_app").app_secret("test_secret").build();
        let request = CreateRoomLevelRequest::new(config.clone());
        let _ = request;
    }
}
