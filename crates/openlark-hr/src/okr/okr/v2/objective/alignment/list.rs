//! 获取目标的对齐信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.alignment/get>

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
use crate::okr::okr::v2::common::models::Alignment;

/// 获取目标的对齐信息请求。
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
    pub async fn execute(self) -> SDKResult<ListObjectiveAlignmentResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListObjectiveAlignmentResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = OkrApiV2::ObjectiveAlignmentList(self.objective_id).to_url();
        let req: ApiRequest<ListObjectiveAlignmentResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标的对齐信息", "响应数据为空")
        })
    }
}

/// 获取目标的对齐信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectiveAlignmentResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 对齐列表。
    #[serde(default)]
    pub items: Option<Vec<Alignment>>,
}

impl ApiResponseTrait for ListObjectiveAlignmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
    fn test_list_objective_alignment_response_deserialize() {
        let json = serde_json::json!({
            "has_more": false,
            "page_token": "token-1",
            "items": [
                {
                    "id": "AL-1",
                    "create_time": "1700000000000",
                    "update_time": "1700000000000",
                    "from_owner": {"owner_type": "user", "user_id": "ou_from"},
                    "to_owner": {"owner_type": "user", "user_id": "ou_to"},
                    "from_entity_type": 2,
                    "from_entity_id": "O-from",
                    "to_entity_type": 2,
                    "to_entity_id": "O-to"
                }
            ]
        });
        let resp: ListObjectiveAlignmentResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.has_more, Some(false));
        assert_eq!(resp.page_token, Some("token-1".to_string()));
        let items = resp.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "AL-1");
        assert_eq!(items[0].from_entity_type, 2);
        assert_eq!(items[0].from_owner.user_id, Some("ou_from".to_string()));
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_objective_alignment_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/okr/v2/objectives/objective_001/alignments",
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
            .expect("okr_v2_objective_alignment_list 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/objectives/objective_001/alignments"
        );
    }
}
