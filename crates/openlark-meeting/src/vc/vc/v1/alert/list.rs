//! 获取告警记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/alert/list>

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

/// 获取告警记录请求

#[derive(Debug, Clone)]
pub struct ListAlertRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query: ListAlertQuery,
}

/// 获取告警记录查询参数
///
/// start_time/end_time 必填；query_type/query_value/page_size/page_token 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAlertQuery {
    /// 开始时间（unix 时间，单位秒）
    pub start_time: String,
    /// 结束时间（unix 时间，单位秒）
    pub end_time: String,
    /// 查询对象类型：1 会议室；2 企业会议室连接器；3 SIP 会议室系统（不填返回所有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<i32>,
    /// 查询对象 ID（会议室 ID 或企业会议室连接器 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_value: Option<String>,
    /// 请求期望返回的告警记录数量（默认 100，最大 200）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取告警记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAlertResponse {
    /// 告警列表
    pub alerts: Vec<AlertItem>,
    /// 是否有下一页
    pub has_more: Option<bool>,
}

/// 告警项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertItem {
    /// 告警 ID
    pub alert_id: String,
    /// 告警级别
    pub level: String,
    /// 告警内容
    pub content: String,
}

impl ApiResponseTrait for ListAlertResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListAlertRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: ListAlertQuery {
                start_time: String::new(),
                end_time: String::new(),
                query_type: None,
                query_value: None,
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置开始时间（unix 时间，单位秒）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.query.start_time = start_time.into();
        self
    }

    /// 设置结束时间（unix 时间，单位秒）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.query.end_time = end_time.into();
        self
    }

    /// 设置查询对象类型：1 会议室；2 企业会议室连接器；3 SIP 会议室系统
    pub fn query_type(mut self, query_type: i32) -> Self {
        self.query.query_type = Some(query_type);
        self
    }

    /// 设置查询对象 ID（会议室 ID 或企业会议室连接器 ID）
    pub fn query_value(mut self, query_value: impl Into<String>) -> Self {
        self.query.query_value = Some(query_value.into());
        self
    }

    /// 设置请求期望返回的告警记录数量（默认 100，最大 200）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/alert/list>
    pub async fn execute(self) -> SDKResult<ListAlertResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListAlertResponse> {
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");

        let api_endpoint = VcApiV1::AlertList;
        let q = self.query;
        let api_request: ApiRequest<ListAlertResponse> = ApiRequest::get(api_endpoint.to_url())
            .query("start_time", q.start_time)
            .query("end_time", q.end_time)
            .query_opt("query_type", q.query_type.map(|v| v.to_string()))
            .query_opt("query_value", q.query_value)
            .query_opt("page_size", q.page_size.map(|v| v.to_string()))
            .query_opt("page_token", q.page_token);

        Transport::request_typed(api_request, &self.config, Some(option), "获取告警记录").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../alerts → 强类型 ListAlertResponse 解析 + 官方必填查询参数。
    #[tokio::test]
    async fn test_list_alert_sends_required_query_params() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/alerts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "alerts": [
                        { "alert_id": "7115030004018184212", "level": "2", "content": "battery < 50%" }
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

        let resp = ListAlertRequest::new(config)
            .start_time("1608888867")
            .end_time("1608889966")
            .query_type(1)
            .page_size(100)
            .execute()
            .await
            .expect("获取告警记录应成功");
        assert_eq!(resp.alerts.len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/alerts");
        // 官方必填查询参数：start_time/end_time
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1608888867"));
        assert!(query.contains("end_time=1608889966"));
        assert!(query.contains("query_type=1"));
        assert!(query.contains("page_size=100"));
    }
}
