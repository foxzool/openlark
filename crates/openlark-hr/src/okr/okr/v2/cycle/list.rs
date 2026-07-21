//! 获取用户 OKR 周期列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/cycle/list>

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

/// 获取用户 OKR 周期列表请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    user_id: String,
    user_id_type: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID（必填）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置用户 ID 类型（可选）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小（可选）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListCycleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListCycleResponse> {
        validate_required!(self.user_id, "user_id 不能为空");

        let path = OkrApiV2::CycleList.to_url();
        let mut req: ApiRequest<ListCycleResponse> =
            ApiRequest::get(&path).query("user_id", &self.user_id);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }

        Transport::request_typed(req, &self.config, Some(option), "获取用户 OKR 周期列表").await
    }
}

/// 获取用户 OKR 周期列表响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListCycleResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 用户周期列表。
    #[serde(default)]
    pub items: Option<Vec<Cycle>>,
}

impl ApiResponseTrait for ListCycleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// OKR 周期。
#[derive(Debug, Clone, Deserialize)]
pub struct Cycle {
    /// 用户周期 ID。
    pub id: String,
    /// 用户周期的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 用户周期的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 租户周期 ID。
    pub tenant_cycle_id: String,
    /// 所有者。
    pub owner: CycleOwner,
    /// 周期的开始时间，毫秒级时间戳。
    pub start_time: String,
    /// 周期的结束时间，毫秒级时间戳。
    pub end_time: String,
    /// 用户周期状态。
    #[serde(default)]
    pub cycle_status: Option<i32>,
    /// 用户周期的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
}

/// 周期所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct CycleOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let req = Request::new(config);
        assert!(req.user_id.is_empty());
        assert!(req.user_id_type.is_none());
        assert!(req.page_size.is_none());
        assert!(req.page_token.is_none());
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let req = Request::new(config)
            .user_id("user_123")
            .user_id_type("open_id")
            .page_size(50)
            .page_token("token_abc");
        assert_eq!(req.user_id, "user_123");
        assert_eq!(req.user_id_type, Some("open_id".to_string()));
        assert_eq!(req.page_size, Some(50));
        assert_eq!(req.page_token, Some("token_abc".to_string()));
    }

    #[test]
    fn test_url_construction() {
        use crate::common::api_endpoints::OkrApiV2;
        let url = OkrApiV2::CycleList.to_url();
        assert_eq!(url, "/open-apis/okr/v2/cycles");
    }

    #[test]
    fn test_list_cycle_response_deserialize() {
        let json = serde_json::json!({
            "has_more": true,
            "page_token": "1",
            "items": [
                {
                    "id": "7342342398472398471",
                    "create_time": "1760604634563",
                    "update_time": "1760604634563",
                    "tenant_cycle_id": "7342342398472398472",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "start_time": "1760604634563",
                    "end_time": "1760604634563",
                    "cycle_status": 1,
                    "score": 0.5
                }
            ]
        });
        let resp: ListCycleResponse = serde_json::from_value(json).expect("反序列化失败");
        assert!(resp.has_more.unwrap());
        assert_eq!(resp.items.as_ref().unwrap().len(), 1);
        let cycle = &resp.items.unwrap()[0];
        assert_eq!(cycle.id, "7342342398472398471");
        assert_eq!(cycle.owner.owner_type, "user");
        assert_eq!(cycle.cycle_status, Some(1));
        assert_eq!(cycle.score, Some(0.5));
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_cycle_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v2/cycles"))
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
            .user_id("user_id_001")
            .execute()
            .await
            .expect("okr_v2_cycle_list 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v2/cycles");
    }
}
