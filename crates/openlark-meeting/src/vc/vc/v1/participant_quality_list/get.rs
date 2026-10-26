//! 查询参会人会议质量数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-3>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 查询参会人会议质量数据请求
pub struct GetParticipantQualityListRequest {
    config: Config,
    /// 查询参数
    query: GetParticipantQualityListQuery,
}

/// 查询参会人会议质量数据查询参数
///
/// meeting_start_time/meeting_end_time/meeting_no/join_time 必填；
/// user_id/room_id 二选一；page_size/page_token/user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParticipantQualityListQuery {
    /// 会议开始时间（需要精确到一分钟，unix 时间，单位 sec）
    pub meeting_start_time: String,
    /// 会议结束时间（unix 时间，单位 sec）
    pub meeting_end_time: String,
    /// 9 位会议号
    pub meeting_no: String,
    /// 参会人入会时间（unix 时间，单位 sec），可从「查询参会人明细」返回结果获取
    pub join_time: String,
    /// 参会人为 Lark 用户时填入，room_id 和 user_id 必须只填一个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 参会人为 Rooms 时填入，room_id 和 user_id 必须只填一个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// 分页尺寸大小（默认 20，取值范围 20～100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetParticipantQualityListRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetParticipantQualityListQuery {
                meeting_start_time: String::new(),
                meeting_end_time: String::new(),
                meeting_no: String::new(),
                join_time: String::new(),
                user_id: None,
                room_id: None,
                page_size: None,
                page_token: None,
                user_id_type: None,
            },
        }
    }

    /// 设置会议开始时间（需要精确到一分钟，unix 时间，单位 sec）
    pub fn meeting_start_time(mut self, meeting_start_time: impl Into<String>) -> Self {
        self.query.meeting_start_time = meeting_start_time.into();
        self
    }

    /// 设置会议结束时间（unix 时间，单位 sec）
    pub fn meeting_end_time(mut self, meeting_end_time: impl Into<String>) -> Self {
        self.query.meeting_end_time = meeting_end_time.into();
        self
    }

    /// 设置 9 位会议号
    pub fn meeting_no(mut self, meeting_no: impl Into<String>) -> Self {
        self.query.meeting_no = meeting_no.into();
        self
    }

    /// 设置参会人入会时间（unix 时间，单位 sec）
    pub fn join_time(mut self, join_time: impl Into<String>) -> Self {
        self.query.join_time = join_time.into();
        self
    }

    /// 设置参会飞书用户 ID（room_id 和 user_id 必须只填一个）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.query.user_id = Some(user_id.into());
        self
    }

    /// 设置参会 Rooms ID（room_id 和 user_id 必须只填一个）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.query.room_id = Some(room_id.into());
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

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-3>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.meeting_start_time, "meeting_start_time 不能为空");
        validate_required!(self.query.meeting_end_time, "meeting_end_time 不能为空");
        validate_required!(self.query.meeting_no, "meeting_no 不能为空");
        validate_required!(self.query.join_time, "join_time 不能为空");

        // url: GET:/open-apis/vc/v1/participant_quality_list
        let api_endpoint = VcApiV1::ParticipantQualityListList;
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url())
            .query("meeting_start_time", q.meeting_start_time)
            .query("meeting_end_time", q.meeting_end_time)
            .query("meeting_no", q.meeting_no)
            .query("join_time", q.join_time)
            .query_opt("user_id", q.user_id)
            .query_opt("room_id", q.room_id)
            .query_opt("page_size", q.page_size.map(|v| v.to_string()))
            .query_opt("page_token", q.page_token)
            .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询参会人会议质量数据").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../participant_quality_list + 官方必填查询参数 + 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_participant_quality_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/participant_quality_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "participant_quality_list": [
                        { "network": { "time": "2022.12.23 11:16:00 (GMT+08:00)", "network_delay": "100ms" } }
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

        let resp = GetParticipantQualityListRequest::new(config)
            .meeting_start_time("1655276858")
            .meeting_end_time("1655277458")
            .meeting_no("123456789")
            .join_time("1655276858")
            .room_id("omm_eada1d61a550955240c28757e7dec3af")
            .page_size(20)
            .execute()
            .await
            .expect("查询参会人会议质量数据应成功");
        assert_eq!(
            resp["participant_quality_list"][0]["network"]["network_delay"],
            "100ms"
        );
        assert_eq!(resp["has_more"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/participant_quality_list"
        );
        // 官方必填查询参数：meeting_start_time/meeting_end_time/meeting_no/join_time
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("meeting_start_time=1655276858"));
        assert!(query.contains("meeting_end_time=1655277458"));
        assert!(query.contains("meeting_no=123456789"));
        assert!(query.contains("join_time=1655276858"));
        assert!(query.contains("room_id=omm_eada1d61a550955240c28757e7dec3af"));
    }
}
