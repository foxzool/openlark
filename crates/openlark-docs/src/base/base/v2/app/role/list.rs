//! 列出自定义角色
//!
//! docPath: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2>

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::{BaseApiV2, CatalogEndpoint};

/// 列出自定义角色
#[derive(Debug)]
pub struct List {
    config: Config,
    app_token: String,
    req: ListReq,
}

#[derive(Debug, Serialize, Deserialize)]
/// 列出自定义角色请求参数。
pub struct ListReq {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// 列出自定义角色响应。
pub struct ListResp {
    /// 角色列表
    #[serde(default)]
    pub items: Vec<AppRole>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多
    #[serde(default)]
    pub has_more: bool,
    /// 总数
    #[serde(default)]
    pub total: i32,
}

impl List {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: ListReq {
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.req.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.req.page_token = Some(page_token.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        if let Some(page_size) = self.req.page_size
            && page_size <= 0
        {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须为正整数",
            ));
        }

        let api_endpoint = BaseApiV2::RoleList(self.app_token);

        // #438: method 来自 catalog
        let mut api_request: ApiRequest<ListResp> = api_endpoint.to_request();
        api_request = api_request.query_opt("page_size", self.req.page_size.map(|v| v.to_string()));
        api_request = api_request.query_opt("page_token", self.req.page_token);

        Transport::request_typed(api_request, &self.config, Some(option), "列出自定义角色").await
    }
}

impl ApiResponseTrait for ListResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../roles → ListResp。
    /// 同时断言 method、path、auth（catalog 提供）和响应（#438）。
    #[tokio::test]
    async fn test_list_roles_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/base/v2/apps/app001/roles"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "items": [], "has_more": false }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let option = openlark_core::req_option::RequestOption::builder()
            .tenant_access_token("test-tenant-token")
            .build();
        let resp = List::new(config)
            .app_token("app001")
            .execute_with_options(option)
            .await
            .expect("列出角色应成功");
        assert!(resp.items.is_empty());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "GET");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/base/v2/apps/app001/roles"
        );
        assert_eq!(
            received[0]
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok()),
            Some("Bearer test-tenant-token")
        );
    }

    #[test]
    fn test_list_roles_uses_get_from_catalog_438() {
        let ep = BaseApiV2::RoleList("app".into());
        let req: openlark_core::api::ApiRequest<ListResp> = ep.to_request();
        assert_eq!(req.method(), &openlark_core::api::HttpMethod::Get);
        assert_eq!(
            req.supported_access_token_types(),
            vec![
                openlark_core::constants::AccessTokenType::User,
                openlark_core::constants::AccessTokenType::Tenant
            ]
        );
    }
}
