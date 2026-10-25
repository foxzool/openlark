//! 获取用户自定义常用的应用
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v5/application/favourite>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 收藏应用的请求。
#[derive(Debug, Clone)]
pub struct GetFavouriteAppsRequest {
    config: Arc<Config>,
    /// 查询参数 `language`（应用信息的语言版本，可选 zh_cn/en_us/ja_jp）。
    language: Option<String>,
    /// 查询参数 `page_token`。
    page_token: Option<String>,
    /// 查询参数 `page_size`（默认 10）。
    page_size: Option<i32>,
}

/// 收藏应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFavouriteAppsResponse {
    /// 分页标记，当 has_more 为 true 时返回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总的数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 是否还有更多项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 应用数据列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<Vec<FavouriteApp>>,
}

/// 收藏应用信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavouriteApp {
    /// 应用 id。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 应用类型：app-普通应用 / shortcut-快捷入口。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// 应用图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 可以打开应用的方式列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_methods: Option<Vec<FavouriteAppOpenMethod>>,
}

/// 打开应用的方式。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavouriteAppOpenMethod {
    /// applink 类型：mobile_gadget-移动端小程序 / mobile_web-移动端 Web / pc_gadget-PC 端小程序 / pc_web-PC 端 Web / bot-机器人。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// applink 链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applink: Option<String>,
}

impl ApiResponseTrait for GetFavouriteAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetFavouriteAppsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            language: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置应用信息的语言版本（可选 zh_cn/en_us/ja_jp）。
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（默认 10）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行收藏应用请求。
    pub async fn execute(self) -> SDKResult<GetFavouriteAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFavouriteAppsResponse> {
        // 官方文档 Authorization 要求 user_access_token。
        let req: ApiRequest<GetFavouriteAppsResponse> =
            ApiRequest::get("/open-apis/application/v5/applications/favourite")
                .query_opt("language", self.language)
                .query_opt("page_token", self.page_token)
                .query_opt("page_size", self.page_size.map(|v| v.to_string()))
                .with_supported_access_token_types(vec![AccessTokenType::User]);

        Transport::request_typed(req, &self.config, Some(option), "获取收藏应用").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/favourite → 类型化 GetFavouriteAppsResponse 解析。
    #[tokio::test]
    async fn test_get_favourite_apps_returns_typed_apps_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v5/applications/favourite"))
            .and(query_param("language", "zh_cn"))
            .and(query_param("page_size", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "page_token": "new-e11ee058b4a8ed2881da11ac7e37c4fc",
                    "total_count": 7,
                    "has_more": false,
                    "app_list": [
                        {
                            "app_id": "cli_9f9f8825d53xxxx",
                            "app_name": "效率工具",
                            "description": "提升办公效率的一款应用",
                            "app_type": "app",
                            "avatar_url": "https://s1-xxxxxx/static-resource/v1/ca597e6e-28c1-4178-961a-1xxxxx",
                            "open_methods": [
                                {
                                    "type": "mobile_gadget",
                                    "applink": "https://applink.feishu.cn/client/web_app/open?appId=cli_9f9f8825d53xxxx"
                                }
                            ]
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetFavouriteAppsRequest::new(config)
            .language("zh_cn")
            .page_size(10)
            .execute()
            .await
            .expect("获取收藏应用应成功");

        assert_eq!(
            resp.page_token.as_deref(),
            Some("new-e11ee058b4a8ed2881da11ac7e37c4fc")
        );
        assert_eq!(resp.total_count, Some(7));
        assert_eq!(resp.has_more, Some(false));
        let app_list = resp.app_list.expect("app_list 应存在");
        assert_eq!(app_list.len(), 1);
        assert_eq!(app_list[0].app_id.as_deref(), Some("cli_9f9f8825d53xxxx"));
        assert_eq!(app_list[0].app_name.as_deref(), Some("效率工具"));
        assert_eq!(app_list[0].app_type.as_deref(), Some("app"));
        let open_methods = app_list[0]
            .open_methods
            .as_ref()
            .expect("open_methods 应存在");
        assert_eq!(open_methods[0].r#type.as_deref(), Some("mobile_gadget"));
        assert_eq!(
            open_methods[0].applink.as_deref(),
            Some("https://applink.feishu.cn/client/web_app/open?appId=cli_9f9f8825d53xxxx")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v5/applications/favourite"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("language=zh_cn"));
        assert!(query.contains("page_size=10"));
    }
}
