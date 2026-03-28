//! 获取会议室列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;

/// 获取会议室列表请求
pub struct ListRoomRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListRoomRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/room/list
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/room/list", MEETING_ROOM));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会议室列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_room_request_builder() {
        let config = Config::default();
        let request = ListRoomRequest::new(config)
            .query_param("building_id", "bld_123")
            .query_param("floor_name", "3F");

        assert_eq!(request.query_params.len(), 2);
        assert_eq!(
            request.query_params[0],
            ("building_id".to_string(), "bld_123".to_string())
        );
        assert_eq!(
            request.query_params[1],
            ("floor_name".to_string(), "3F".to_string())
        );
    }

    #[test]
    fn test_list_room_request_minimal() {
        let config = Config::default();
        let request = ListRoomRequest::new(config);

        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_list_room_request_single_param() {
        let config = Config::default();
        let request = ListRoomRequest::new(config).query_param("page_size", "50");

        assert_eq!(request.query_params.len(), 1);
        assert_eq!(
            request.query_params[0],
            ("page_size".to_string(), "50".to_string())
        );
    }
}
