//! 获取被评估人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/reviewee/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取被评估人信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query: RevieweeQueryParams,
    /// 请求体
    body: QueryRequestBody,
}

/// 获取被评估人信息查询参数
///
/// user_id_type/page_token/page_size 均可选（user_id_type 默认 open_id，page_size 默认 20，取值 1～50）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevieweeQueryParams {
    /// 用户 ID 类型（open_id/union_id/user_id/people_admin_id，默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小（默认 20，取值范围 1～50）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 获取被评估人信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 周期 ID（必填），可通过【获取周期列表】接口获取
    pub semester_id: String,
    /// 用户 ID 列表（可选，最多 50 个），与入参 user_id_type 类型一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 项目 ID 列表（可选），可通过【获取项目列表】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_ids: Option<Vec<String>>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: RevieweeQueryParams {
                user_id_type: None,
                page_token: None,
                page_size: None,
            },
            body: QueryRequestBody {
                semester_id: String::new(),
                user_ids: None,
                activity_ids: None,
            },
        }
    }

    /// 设置周期 ID（必填）
    pub fn semester_id(mut self, semester_id: impl Into<String>) -> Self {
        self.body.semester_id = semester_id.into();
        self
    }

    /// 设置用户 ID 列表（最多 50 个）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = Some(user_ids);
        self
    }

    /// 设置项目 ID 列表
    pub fn activity_ids(mut self, activity_ids: Vec<String>) -> Self {
        self.body.activity_ids = Some(activity_ids);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（取值范围 1～50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = Some(page_size);
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
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.body.semester_id, "semester_id 不能为空");

        let api_endpoint = PerformanceApiV1::RevieweeQuery;
        let RevieweeQueryParams {
            user_id_type,
            page_token,
            page_size,
        } = self.query;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", user_id_type)
            .query_opt("page_token", page_token)
            .query_opt("page_size", page_size.map(|v| v.to_string()))
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取被评估人信息响应数据为空",
        )
        .await
    }
}

/// 获取被评估人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 被评估人列表
    pub items: Vec<Reviewee>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 被评估人信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reviewee {
    /// 用户 ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 部门 ID
    pub department_id: Option<String>,
    /// 绩效状态
    pub status: i32,
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
    /// （官方请求体必填 semester_id；user_id_type/page_size 为可选查询参数）
    #[tokio::test]
    async fn test_performance_v2_reviewee_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"items": [], "has_more": false}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/reviewees/query"))
            .and(query_param("user_id_type", "open_id"))
            .and(query_param("page_size", "30"))
            .and(body_json(json!({
                "semester_id": "6992035450862224940",
                "user_ids": ["ou_3245842393d09e9428ad4655da6e30b3"]
            })))
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

        let data = QueryRequest::new(config)
            .semester_id("6992035450862224940")
            .user_ids(vec!["ou_3245842393d09e9428ad4655da6e30b3".to_string()])
            .user_id_type("open_id")
            .page_size(30)
            .execute()
            .await
            .expect("performance_v2_reviewee_query 应成功");

        assert!(data.items.is_empty());
        assert!(!data.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/reviewees/query"
        );
    }
}
