//! 获取 Top 用户列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取 Top 用户列表请求
pub struct GetTopUserReportRequest {
    config: Config,
    /// 查询参数
    query: GetTopUserReportQuery,
}

/// 获取 Top 用户列表查询参数
///
/// start_time/end_time/limit/order_by 必填；unit/user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTopUserReportQuery {
    /// 开始时间（unix 时间，单位 sec）
    pub start_time: String,
    /// 结束时间（unix 时间，单位 sec）
    pub end_time: String,
    /// 取前多少位（默认 10，最多 100）
    pub limit: i32,
    /// 排序依据（降序）：1 会议数量；2 会议时长
    pub order_by: i32,
    /// 数据驻留地：0 中国大陆；1 美国；2 新加坡；3 日本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetTopUserReportRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetTopUserReportQuery {
                start_time: String::new(),
                end_time: String::new(),
                limit: 0,
                order_by: 0,
                unit: None,
                user_id_type: None,
            },
        }
    }

    /// 设置开始时间（unix 时间，单位 sec）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.query.start_time = start_time.into();
        self
    }

    /// 设置结束时间（unix 时间，单位 sec）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.query.end_time = end_time.into();
        self
    }

    /// 设置取前多少位（默认 10，最多 100）
    pub fn limit(mut self, limit: i32) -> Self {
        self.query.limit = limit;
        self
    }

    /// 设置排序依据（降序）：1 会议数量；2 会议时长
    pub fn order_by(mut self, order_by: i32) -> Self {
        self.query.order_by = order_by;
        self
    }

    /// 设置数据驻留地：0 中国大陆；1 美国；2 新加坡；3 日本
    pub fn unit(mut self, unit: i32) -> Self {
        self.query.unit = Some(unit);
        self
    }

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");

        // url: GET:/open-apis/vc/v1/reports/get_top_user
        let q = self.query;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/reports/get_top_user")
                .query("start_time", q.start_time)
                .query("end_time", q.end_time)
                .query("limit", q.limit.to_string())
                .query("order_by", q.order_by.to_string())
                .query_opt("unit", q.unit.map(|unit| unit.to_string()))
                .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "获取 Top 用户列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../reports/get_top_user → 裸 Value 解析（单层 data 信封）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_get_top_user_report_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reports/get_top_user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "top_user_report": [
                        {
                            "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                            "name": "name",
                            "user_type": 1,
                            "meeting_count": "100",
                            "meeting_duration": "3000"
                        }
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

        let resp = GetTopUserReportRequest::new(config)
            .start_time("1608888867")
            .end_time("1608889966")
            .limit(10)
            .order_by(1)
            .execute()
            .await
            .expect("获取 Top 用户列表应成功");
        assert_eq!(
            resp["top_user_report"][0]["id"],
            "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
        );
        assert_eq!(resp["top_user_report"][0]["meeting_count"], "100");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reports/get_top_user"
        );
        // 官方必填查询参数：start_time/end_time/limit/order_by
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1608888867"));
        assert!(query.contains("end_time=1608889966"));
        assert!(query.contains("limit=10"));
        assert!(query.contains("order_by=1"));
    }
}
