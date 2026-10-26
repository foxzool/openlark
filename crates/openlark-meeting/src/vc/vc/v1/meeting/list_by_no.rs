//! 获取与会议号关联的会议列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取与会议号关联的会议列表请求
pub struct ListByNoMeetingRequest {
    config: Config,
    /// 查询参数
    query: ListByNoMeetingQuery,
}

/// 获取与会议号关联的会议列表查询参数
///
/// meeting_no/start_time/end_time 必填；page_token/page_size 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListByNoMeetingQuery {
    /// 9 位会议号（会议链接最后 9 位数）
    pub meeting_no: String,
    /// 查询开始时间（unix 时间，单位 sec），需小于 end_time
    pub start_time: String,
    /// 查询结束时间（unix 时间，单位 sec）
    pub end_time: String,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小（默认 20，最大 50）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListByNoMeetingRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: ListByNoMeetingQuery {
                meeting_no: String::new(),
                start_time: String::new(),
                end_time: String::new(),
                page_token: None,
                page_size: None,
            },
        }
    }

    /// 设置 9 位会议号（会议链接最后 9 位数）
    pub fn meeting_no(mut self, meeting_no: impl Into<String>) -> Self {
        self.query.meeting_no = meeting_no.into();
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

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（默认 20，最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.meeting_no, "meeting_no 不能为空");
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");

        // url: GET:/open-apis/vc/v1/meetings/list_by_no
        let q = self.query;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/meetings/list_by_no")
                .query("meeting_no", q.meeting_no)
                .query("start_time", q.start_time)
                .query("end_time", q.end_time)
                .query_opt("page_token", q.page_token)
                .query_opt("page_size", q.page_size.map(|size| size.to_string()));

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "获取与会议号关联的会议列表",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meetings/list_by_no → 裸 Value 解析（单层 data 信封）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_list_by_no_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/meetings/list_by_no"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": true,
                    "page_token": "2",
                    "meeting_briefs": [
                        { "id": "7011030664708603907", "meeting_no": "146453285", "topic": "测试标题" }
                    ]
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

        let resp = ListByNoMeetingRequest::new(config)
            .meeting_no("146453285")
            .start_time("1608888867")
            .end_time("1608889966")
            .page_size(10)
            .execute()
            .await
            .expect("获取与会议号关联的会议列表应成功");
        assert_eq!(resp["meeting_briefs"][0]["id"], "7011030664708603907");
        assert_eq!(resp["has_more"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/list_by_no"
        );
        // 官方必填查询参数：meeting_no/start_time/end_time
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("meeting_no=146453285"));
        assert!(query.contains("start_time=1608888867"));
        assert!(query.contains("end_time=1608889966"));
        assert!(query.contains("page_size=10"));
    }
}
