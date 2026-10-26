//! 查询参会人明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 查询参会人明细请求
#[derive(Debug, Clone)]
pub struct GetParticipantListRequest {
    config: Config,
    /// 查询参数
    query: GetParticipantListQuery,
}

/// 查询参会人明细查询参数
///
/// meeting_start_time/meeting_end_time/meeting_no 必填；
/// meeting_status/user_id/room_id/page_size/page_token/webinar_user_role/user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParticipantListQuery {
    /// 会议开始时间（unix 时间，单位 sec）
    pub meeting_start_time: String,
    /// 会议结束时间（unix 时间，单位 sec；进行中会议可填当前时间）
    pub meeting_end_time: String,
    /// 9 位会议号
    pub meeting_no: String,
    /// 会议状态：1 进行中；2 已结束；3 待召开（不传默认为已结束会议）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_status: Option<i32>,
    /// 按参会飞书用户筛选（最多一个筛选条件）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 按参会 Rooms 筛选（最多一个筛选条件）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// 分页尺寸大小（默认 20，取值范围 20～100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 查询网络研讨会时的观众类型："0" 嘉宾，"3" 观众
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webinar_user_role: Option<String>,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 查询参会人明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetParticipantListResponse {
    /// 参会人列表数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetParticipantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetParticipantListRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetParticipantListQuery {
                meeting_start_time: String::new(),
                meeting_end_time: String::new(),
                meeting_no: String::new(),
                meeting_status: None,
                user_id: None,
                room_id: None,
                page_size: None,
                page_token: None,
                webinar_user_role: None,
                user_id_type: None,
            },
        }
    }

    /// 设置会议开始时间（unix 时间，单位 sec）
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

    /// 设置会议状态：1 进行中；2 已结束；3 待召开
    pub fn meeting_status(mut self, meeting_status: i32) -> Self {
        self.query.meeting_status = Some(meeting_status);
        self
    }

    /// 设置按参会飞书用户筛选（最多一个筛选条件）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.query.user_id = Some(user_id.into());
        self
    }

    /// 设置按参会 Rooms 筛选（最多一个筛选条件）
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

    /// 设置查询网络研讨会时的观众类型："0" 嘉宾，"3" 观众
    pub fn webinar_user_role(mut self, webinar_user_role: impl Into<String>) -> Self {
        self.query.webinar_user_role = Some(webinar_user_role.into());
        self
    }

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>
    pub async fn execute(self) -> SDKResult<GetParticipantListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetParticipantListResponse> {
        validate_required!(self.query.meeting_start_time, "meeting_start_time 不能为空");
        validate_required!(self.query.meeting_end_time, "meeting_end_time 不能为空");
        validate_required!(self.query.meeting_no, "meeting_no 不能为空");

        let api_endpoint = VcApiV1::ParticipantListList;
        let q = self.query;
        let req: ApiRequest<GetParticipantListResponse> = ApiRequest::get(api_endpoint.to_url())
            .query("meeting_start_time", q.meeting_start_time)
            .query("meeting_end_time", q.meeting_end_time)
            .query("meeting_no", q.meeting_no)
            .query_opt("meeting_status", q.meeting_status.map(|v| v.to_string()))
            .query_opt("user_id", q.user_id)
            .query_opt("room_id", q.room_id)
            .query_opt("page_size", q.page_size.map(|v| v.to_string()))
            .query_opt("page_token", q.page_token)
            .query_opt("webinar_user_role", q.webinar_user_role)
            .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询参会人明细").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../participant_list + 官方必填查询参数 + 强类型 GetParticipantListResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_participant_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/participant_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "participants": [
                            { "participant_name": "kehan", "user_id": "8efq90", "join_time": "2022.12.23 11:16:59 (GMT+08:00)" }
                        ],
                        "page_token": "20",
                        "has_more": true
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

        let resp = GetParticipantListRequest::new(config)
            .meeting_start_time("1655276858")
            .meeting_end_time("1655277458")
            .meeting_no("123456789")
            .meeting_status(2)
            .page_size(20)
            .execute()
            .await
            .expect("查询参会人明细应成功");
        assert_eq!(resp.data["participants"][0]["participant_name"], "kehan");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/participant_list");
        // 官方必填查询参数：meeting_start_time/meeting_end_time/meeting_no
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("meeting_start_time=1655276858"));
        assert!(query.contains("meeting_end_time=1655277458"));
        assert!(query.contains("meeting_no=123456789"));
        assert!(query.contains("meeting_status=2"));
        assert!(query.contains("page_size=20"));
    }
}
