//! 获取招聘官网推广渠道列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.channel/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::WebsiteChannelInfo;

/// 获取招聘官网推广渠道列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: None,
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");

        let request = ApiRequest::<ListResponse>::get(format!(
            "/open-apis/hire/v1/websites/{website_id}/channels"
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取招聘官网推广渠道列表响应数据为空",
        )
        .await
    }
}

/// 获取招聘官网推广渠道列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    /// 官网推广渠道列表（官方键为 `website_channel_list`）。
    #[serde(default, alias = "website_channel_list", alias = "channels")]
    pub items: Vec<WebsiteChannelInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/websites/test001/channels
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/websites/test001/channels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEwLCJ0aW1NTUyMjM2NzIsImlkIjpudWxsfQ",
                    "website_channel_list": [
                        {
                            "id": "7085989097067563300",
                            "name": "微信推广渠道",
                            "link": "http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5",
                            "code": "A1KM6A5"
                        }
                    ]
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

        let resp = ListRequest::new(config)
            .website_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id.as_deref(), Some("7085989097067563300"));
        assert_eq!(resp.items[0].name.as_deref(), Some("微信推广渠道"));
        assert_eq!(
            resp.items[0].link.as_deref(),
            Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5")
        );
        assert_eq!(resp.items[0].code.as_deref(), Some("A1KM6A5"));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
