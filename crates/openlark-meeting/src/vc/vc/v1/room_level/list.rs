//! 查询会议室层级列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 查询会议室层级列表请求
pub struct ListRoomLevelRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListRoomLevelRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/room_levels
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(VC_V1_ROOM_LEVELS);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室层级列表")
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
        let request = ListRoomLevelRequest::new(config.clone())
            .query_param("key1".to_string(), "value1".to_string());
        let _ = request;
    }
}
