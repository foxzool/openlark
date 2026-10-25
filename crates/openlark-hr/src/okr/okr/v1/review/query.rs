//! 查询复盘信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/review/query>

use crate::okr::okr::v1::okr::batch_get::OkrUser;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 查询复盘信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 目标用户 ID 列表（必填，最多 5 个）
    user_ids: Vec<String>,
    /// 周期 ID 列表（必填，最多 5 个）
    period_ids: Vec<String>,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    ///
    /// - `user_ids`: 目标用户 ID 列表（最多 5 个）
    /// - `period_ids`: 周期 ID 列表（最多 5 个）
    pub fn new(config: Config, user_ids: Vec<String>, period_ids: Vec<String>) -> Self {
        Self {
            user_ids,
            period_ids,
            user_id_type: None,
            config,
        }
    }

    /// 设置用户 ID 类型（可选，open_id/union_id/user_id/people_admin_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 官方约束：user_ids/period_ids 均最多 5 个
        validate_required_list!(self.user_ids, 5, "user_ids 不能为空且不能超过 5 个");
        validate_required_list!(self.period_ids, 5, "period_ids 不能为空且不能超过 5 个");

        // 1. 构建端点
        let api_endpoint = OkrApiV1::ReviewQuery;
        // 官方类型为 string[]，与 okr/batch_get 的 okr_ids 相同以逗号拼接传递
        let mut request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url())
            .query("user_ids", self.user_ids.join(","))
            .query("period_ids", self.period_ids.join(","));

        // 2. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询复盘信息响应数据为空",
        )
        .await
    }
}

/// 查询复盘信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// OKR 复盘列表（okr_review[]）
    #[serde(default)]
    pub review_list: Vec<OkrReview>,
}

/// OKR 复盘（okr_review）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrReview {
    /// 复盘的用户
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<OkrUser>,
    /// 用户对应的 OKR 复盘列表
    #[serde(default)]
    pub review_period_list: Vec<OkrReviewPeriod>,
}

/// OKR 复盘周期（okr_review_period）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrReviewPeriod {
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// 复盘文档
    #[serde(default)]
    pub cycle_review_list: Vec<OkrReviewPeriodUrl>,
    /// 进展报告
    #[serde(default)]
    pub progress_report_list: Vec<OkrReviewPeriodUrl>,
}

/// 复盘文档/进展报告链接（okr_review_period_url）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrReviewPeriodUrl {
    /// 文档链接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间（毫秒时间戳字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_review_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.review_list[]（user_id + review_period_list 嵌套树）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"review_list": [{"review_period_list": [{"cycle_review_list": [{"create_time": "1646902033525", "url": "https://okr-boe1.feishu-boe.cn/docs/docbcZ4PTuEDd2MBJ9k2W4rOeY1"}], "period_id": "7067724095781142548", "progress_report_list": [{"create_time": "1646904637251", "url": "https://okr-boe1.feishu-boe.cn/docs/docbcthFL8qo5ENgYW5k3iTLZWf"}]}], "user_id": {"open_id": "ou_e6139117c300506837def50545420c6a", "user_id": "6969855501744834092"}}]}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/reviews/query"))
            .and(query_param("user_ids", "ou_1"))
            .and(query_param("period_ids", "period_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(
            config,
            vec!["ou_1".to_string()],
            vec!["period_1".to_string()],
        )
        .execute()
        .await
        .expect("okr_v1_review_query 应成功");

        assert_eq!(data.review_list.len(), 1);
        let review = &data.review_list[0];
        assert_eq!(
            review.user_id.as_ref().and_then(|u| u.open_id.as_deref()),
            Some("ou_e6139117c300506837def50545420c6a")
        );
        assert_eq!(review.review_period_list.len(), 1);
        let period = &review.review_period_list[0];
        assert_eq!(period.period_id.as_deref(), Some("7067724095781142548"));
        assert_eq!(period.cycle_review_list.len(), 1);
        assert_eq!(
            period.cycle_review_list[0].create_time.as_deref(),
            Some("1646902033525")
        );
        assert_eq!(period.progress_report_list.len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/reviews/query");
        // 必填查询参数 user_ids/period_ids 须出现在 query string 中
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_ids=ou_1"), "缺 user_ids: {query}");
        assert!(
            query.contains("period_ids=period_1"),
            "缺 period_ids: {query}"
        );
    }
}
