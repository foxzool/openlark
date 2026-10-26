//! 查询会议室预定数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-4>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::endpoints::VC_V1_RESOURCE_RESERVATION_LIST;

/// 查询会议室预定数据请求
pub struct GetResourceReservationListRequest {
    config: Config,
    /// 查询参数
    query: GetResourceReservationListQuery,
}

/// 查询会议室预定数据查询参数
///
/// room_level_id/start_time/end_time/room_ids 必填；
/// need_topic/is_exclude/page_size/page_token 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResourceReservationListQuery {
    /// 层级 ID（传非 omb 前缀的异常 ID 时默认使用租户层级兜底）
    pub room_level_id: String,
    /// 查询开始时间（unix 时间，单位 sec）
    pub start_time: String,
    /// 查询结束时间（unix 时间，单位 sec）
    pub end_time: String,
    /// 待筛选的会议室 ID 列表
    pub room_ids: Vec<String>,
    /// 是否展示会议主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_topic: Option<bool>,
    /// false：获取 room_ids 传入会议室的预定数据；true：获取层级下过滤掉 room_ids 后剩余会议室的预定数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exclude: Option<bool>,
    /// 分页尺寸大小（默认 20，取值范围 20～100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetResourceReservationListRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetResourceReservationListQuery {
                room_level_id: String::new(),
                start_time: String::new(),
                end_time: String::new(),
                room_ids: Vec::new(),
                need_topic: None,
                is_exclude: None,
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置层级 ID
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.query.room_level_id = room_level_id.into();
        self
    }

    /// 设置查询开始时间（unix 时间，单位 sec）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.query.start_time = start_time.into();
        self
    }

    /// 设置查询结束时间（unix 时间，单位 sec）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.query.end_time = end_time.into();
        self
    }

    /// 设置待筛选的会议室 ID 列表（官方以 room_ids=a&room_ids=b 重复参数传递，此处以逗号拼接）
    pub fn room_ids(mut self, room_ids: Vec<String>) -> Self {
        self.query.room_ids = room_ids;
        self
    }

    /// 设置是否展示会议主题
    pub fn need_topic(mut self, need_topic: bool) -> Self {
        self.query.need_topic = Some(need_topic);
        self
    }

    /// 设置是否排除 room_ids 范围（默认 false）
    pub fn is_exclude(mut self, is_exclude: bool) -> Self {
        self.query.is_exclude = Some(is_exclude);
        self
    }

    /// 设置分页尺寸大小（默认 20，取值范围 20～100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-4>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.room_level_id, "room_level_id 不能为空");
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");
        validate_required!(self.query.room_ids, "room_ids 不能为空");

        // url: GET:/open-apis/vc/v1/resource_reservation_list
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(VC_V1_RESOURCE_RESERVATION_LIST)
            .query("room_level_id", q.room_level_id)
            .query("start_time", q.start_time)
            .query("end_time", q.end_time)
            .query("room_ids", q.room_ids.join(","))
            .query_opt("need_topic", q.need_topic.map(|v| v.to_string()))
            .query_opt("is_exclude", q.is_exclude.map(|v| v.to_string()))
            .query_opt("page_size", q.page_size.map(|v| v.to_string()))
            .query_opt("page_token", q.page_token);

        Transport::request_typed(req, &self.config, Some(option), "查询会议室预定数据").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../resource_reservation_list → data 信封解析 + 官方必填查询参数。
    #[tokio::test]
    async fn test_get_resource_reservation_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/resource_reservation_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "room_reservation_list": [
                        {
                            "room_id": "omm_4de32cf10a4358788ff4e09e37ebbf9b",
                            "room_name": "VIP Meeting Room",
                            "event_title": "飞书邀请的日程",
                            "reservation_status": "预定成功"
                        }
                    ],
                    "page_token": "20",
                    "has_more": true
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

        let resp = GetResourceReservationListRequest::new(config)
            .room_level_id("omb_57c9cc7d9a81e27e54c8fabfd02759e7")
            .start_time("1655276858")
            .end_time("1655277458")
            .room_ids(vec!["omm_eada1d61a550955240c28757e7dec3af".to_string()])
            .need_topic(true)
            .execute()
            .await
            .expect("查询会议室预定数据应成功");
        assert_eq!(
            resp["room_reservation_list"][0]["room_id"],
            "omm_4de32cf10a4358788ff4e09e37ebbf9b"
        );
        assert_eq!(resp["has_more"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/resource_reservation_list"
        );
        // 官方必填查询参数：room_level_id/start_time/end_time/room_ids
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("room_level_id=omb_57c9cc7d9a81e27e54c8fabfd02759e7"));
        assert!(query.contains("start_time=1655276858"));
        assert!(query.contains("end_time=1655277458"));
        assert!(query.contains("room_ids=omm_eada1d61a550955240c28757e7dec3af"));
        assert!(query.contains("need_topic=true"));
    }
}
