//! 获取用户列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取用户列表请求
///
/// 支持分页及部门过滤。
#[derive(Debug)]
pub struct ListUsersRequest {
    /// 配置信息。
    config: Config,
    /// 页面大小（可选，默认 20，最大 100）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
    /// 部门 ID 过滤（可选）。
    department_id: Option<String>,
}

/// 获取用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    /// 是否有更多数据。
    pub has_more: bool,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 用户列表（飞书文档未给出明确 schema，暂用 `serde_json::Value` 透传）。
    pub items: Option<Vec<serde_json::Value>>,
}

impl openlark_core::api::ApiResponseTrait for ListUsersResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

impl ListUsersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            department_id: None,
        }
    }

    /// 设置页面大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门 ID 过滤。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListUsersResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListUsersResponse> {
        let req: ApiRequest<ListUsersResponse> = ApiRequest::get("/open-apis/acs/v1/users")
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token.as_ref())
            .query_opt("department_id", self.department_id.as_ref())
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        if !resp.is_success() {
            return resp.into_result();
        }
        Ok(resp.data.unwrap_or(ListUsersResponse {
            has_more: false,
            page_token: None,
            items: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../users + query 拼装 → 强类型 ListUsersResponse 反序列化。
    #[tokio::test]
    async fn test_list_users_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/acs/v1/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": true,
                    "page_token": "next_page",
                    "items": [{ "user_id": "u_001" }, { "user_id": "u_002" }]
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

        let resp = ListUsersRequest::new(config)
            .page_size(20)
            .department_id("dept_001")
            .execute()
            .await
            .expect("获取用户列表应成功");

        assert!(resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("next_page"));
        assert_eq!(resp.items.as_ref().unwrap().len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=20"));
        assert!(query.contains("department_id=dept_001"));
    }
}
