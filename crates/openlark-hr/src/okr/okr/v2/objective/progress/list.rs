//! 获取目标下的进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.progress/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_endpoints::OkrApiV2;
use crate::okr::okr::v2::common::models::ContentBlock;

/// 获取目标下的进展记录请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    objective_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            objective_id: String::new(),
        }
    }

    /// 设置路径参数 `objective_id`。
    pub fn objective_id(mut self, val: impl Into<String>) -> Self {
        self.objective_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListObjectiveProgressResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListObjectiveProgressResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = OkrApiV2::ObjectiveProgressList(self.objective_id).to_url();
        let req: ApiRequest<ListObjectiveProgressResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标下的进展记录", "响应数据为空")
        })
    }
}

/// 获取目标下的进展记录响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectiveProgressResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 进展列表。
    #[serde(default)]
    pub items: Option<Vec<Progress>>,
}

impl ApiResponseTrait for ListObjectiveProgressResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 进展记录。
#[derive(Debug, Clone, Deserialize)]
pub struct Progress {
    /// 进展的 ID。
    pub id: String,
    /// 进展的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 进展的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: ProgressOwner,
    /// 进展所属的实体类型。
    pub entity_type: i32,
    /// 进展所属的实体 ID。
    pub entity_id: String,
    /// 进展的内容（文档 block 结构，见 [`ContentBlock`]）。
    #[serde(default)]
    pub content: Option<ContentBlock>,
    /// 进展的进度。
    #[serde(default)]
    pub progress_rate: Option<ProgressRate>,
}

/// 进展所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct ProgressOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 进展进度。
#[derive(Debug, Clone, Deserialize)]
pub struct ProgressRate {
    /// 进展百分比，保留两位小数。
    #[serde(default)]
    pub progress_percent: Option<f64>,
    /// 进展状态。
    #[serde(default)]
    pub progress_status: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_list_objective_progress_response_deserialize() {
        let json = serde_json::json!({
            "has_more": false,
            "page_token": "token-1",
            "items": [
                {
                    "id": "P-1",
                    "create_time": "1700000000000",
                    "update_time": "1700000000000",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "entity_type": 2,
                    "entity_id": "O-123",
                    "progress_rate": {"progress_percent": 0.5, "progress_status": 1}
                }
            ]
        });
        let resp: ListObjectiveProgressResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.has_more, Some(false));
        let items = resp.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "P-1");
        assert_eq!(items[0].entity_type, 2);
        let rate = items[0].progress_rate.as_ref().unwrap();
        assert_eq!(rate.progress_percent, Some(0.5));
        assert_eq!(rate.progress_status, Some(1));
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_objective_progress_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/okr/v2/objectives/objective_001/progresses",
            ))
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

        let data = Request::new(std::sync::Arc::new(config))
            .objective_id("objective_001")
            .execute()
            .await
            .expect("okr_v2_objective_progress_list 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/objectives/objective_001/progresses"
        );
    }
}
