//! 查询席位活跃详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/seat_activity/list>
//! docPath: <https://open.feishu.cn/document/apaas-v1/seat_activity/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询席位活跃详情 Builder
#[derive(Debug, Clone)]
pub struct SeatActivityListRequestBuilder {
    config: Config,
    /// 查询参数
    query: SeatActivityListQuery,
}

impl SeatActivityListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: SeatActivityListQuery {
                seat_type: String::new(),
                page_size: 0,
                page_token: None,
            },
        }
    }

    /// 设置席位类型：`per_user`（平台席位）/ `per_user_per_app`（应用访问席位）
    pub fn seat_type(mut self, seat_type: impl Into<String>) -> Self {
        self.query.seat_type = seat_type.into();
        self
    }

    /// 设置分页大小，范围 [0, 500]
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = page_size;
        self
    }

    /// 设置分页标记（第一次请求不填；使用返回的新 page_token 继续遍历）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SeatActivityListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SeatActivityListResponse> {
        validate_required!(self.query.seat_type, "席位类型不能为空");

        let url = "/open-apis/apaas/v1/seat_activities";

        let mut req: ApiRequest<SeatActivityListResponse> = ApiRequest::get(url);
        req = req.query("seat_type", self.query.seat_type.clone());
        req = req.query("page_size", self.query.page_size.to_string());
        if let Some(page_token) = self.query.page_token {
            req = req.query("page_token", page_token);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询席位活跃详情").await
    }
}

/// 查询席位活跃详情查询参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatActivityListQuery {
    /// 席位类型，枚举值：per_user（平台席位）、per_user_per_app（应用访问席位）
    #[serde(rename = "seat_type")]
    pub seat_type: String,
    /// 分页大小，范围 [0, 500]
    #[serde(rename = "page_size")]
    pub page_size: i32,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    #[serde(rename = "page_token", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 席位活跃信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatActivity {
    /// 席位 ID
    #[serde(rename = "seat_id")]
    seat_id: String,
    /// 用户 ID
    #[serde(rename = "user_id")]
    user_id: String,
    /// 活跃时间
    #[serde(rename = "active_time")]
    active_time: i64,
    /// 活跃类型
    #[serde(rename = "activity_type")]
    activity_type: String,
}

/// 查询席位活跃详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeatActivityListResponse {
    /// 席位活跃列表
    #[serde(rename = "items")]
    pub items: Vec<SeatActivity>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for SeatActivityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to SeatActivityListRequestBuilder, will be removed in v1.0 (#271)")]
pub type SeatActivityListBuilder = SeatActivityListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../seat_activities?seat_type=...&page_size=... → 强类型 SeatActivityListResponse。
    #[tokio::test]
    async fn test_list_seat_activities_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/seat_activities"))
            .and(query_param("seat_type", "per_user"))
            .and(query_param("page_size", "10"))
            .and(query_param("page_token", "token_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "seat_id": "seat_001",
                            "user_id": "1001",
                            "active_time": 1727199298656i64,
                            "activity_type": "app_visit"
                        }
                    ],
                    "has_more": false,
                    "page": 1,
                    "page_size": 10
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

        let resp = SeatActivityListRequestBuilder::new(config)
            .seat_type("per_user")
            .page_size(10)
            .page_token("token_001")
            .execute()
            .await
            .expect("查询席位活跃详情应成功");
        assert_eq!(resp.items.len(), 1);
        assert!(!resp.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 查询参数须与官方文档一致：seat_type/page_size 必填 + page_token 可选
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("seat_type=per_user"));
        assert!(query.contains("page_size=10"));
        assert!(query.contains("page_token=token_001"));
    }
}
