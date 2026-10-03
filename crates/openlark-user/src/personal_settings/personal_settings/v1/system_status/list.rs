//! 获取系统状态列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::models::SystemStatus;

/// 获取系统状态列表的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusListRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

/// 获取系统状态列表响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusListResponse {
    /// 系统状态列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SystemStatus>>,
    /// 是否还有更多项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SystemStatusListResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SystemStatusListRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小（1～50，默认 50）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行获取系统状态列表请求。
    pub async fn execute(self) -> SDKResult<SystemStatusListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusListResponse> {
        let mut req: ApiRequest<SystemStatusListResponse> =
            ApiRequest::get("/open-apis/personal_settings/v1/system_statuses")
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        if let Some(size) = self.page_size {
            req = req.query("page_size", size.to_string());
        }
        req = req.query_opt("page_token", self.page_token.as_ref());

        Transport::request_typed(req, &self.config, Some(option), "获取系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../system_statuses?page_size= → items/has_more。
    #[tokio::test]
    async fn test_list_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/personal_settings/v1/system_statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "system_status_id": "ss_001",
                        "title": "出差",
                        "icon_key": "GeneralBusinessTrip"
                    }],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = SystemStatusListRequest::new(config)
            .page_size(50)
            .execute()
            .await
            .expect("获取系统状态列表应成功");
        assert_eq!(resp.items.as_ref().unwrap().len(), 1);
        assert_eq!(resp.has_more, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=50"));
    }
}
