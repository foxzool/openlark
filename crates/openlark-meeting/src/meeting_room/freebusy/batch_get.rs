//! 查询会议室忙闲
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/query-room-availability>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;
use crate::meeting_room::responses::BatchGetFreebusyResponse;

/// 查询会议室忙闲请求
pub struct BatchGetFreebusyRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetFreebusyRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/query-room-availability>
    pub async fn execute(self) -> SDKResult<BatchGetFreebusyResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchGetFreebusyResponse> {
        // url: GET:/open-apis/meeting_room/freebusy/batch_get
        let mut req: ApiRequest<BatchGetFreebusyResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/freebusy/batch_get"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室忙闲")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_room/freebusy/batch_get → BatchGetFreebusyResponse。
    #[tokio::test]
    async fn test_batch_get_freebusy_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/freebusy/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "time_max": "2019-09-04T09:45:00+08:00",
                    "time_min": "2019-09-04T08:45:00+08:00",
                    "free_busy": {
                        "room_001": [{
                            "start_time": "2019-09-04T09:00:00+08:00",
                            "end_time": "2019-09-04T09:30:00+08:00",
                            "uid": "bff6b51f-b7c1-40c6-b8ef-aef966c9ffc7",
                            "original_time": 0,
                            "organizer_info": {
                                "name": "张三",
                                "open_id": "ou_xxx"
                            }
                        }]
                    }
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

        let resp = BatchGetFreebusyRequest::new(config)
            .query_param("time_scope", "day")
            .execute()
            .await
            .expect("查询会议室忙闲应成功");
        let slots = resp.free_busy.get("room_001").expect("应有 room_001 忙闲");
        assert_eq!(
            slots[0].uid.as_deref(),
            Some("bff6b51f-b7c1-40c6-b8ef-aef966c9ffc7")
        );
        assert_eq!(
            slots[0].organizer_info.as_ref().unwrap().name.as_deref(),
            Some("张三")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/freebusy/batch_get"
        );
        assert_eq!(received[0].url.query(), Some("time_scope=day"));
    }
}
