//! 更新招聘官网推广渠道
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.channel/update>

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

/// 更新招聘官网推广渠道请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    website_id: String,
    channel_id: String,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            website_id: String::new(),
            channel_id: String::new(),
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: String) -> Self {
        self.website_id = website_id;
        self
    }

    /// 设置 `channel_id`。
    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = channel_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        validate_required!(self.website_id.trim(), "官网 ID 不能为空");
        validate_required!(self.channel_id.trim(), "渠道 ID 不能为空");

        let request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/websites/{}/channels/{}",
            self.website_id, self.channel_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新招聘官网推广渠道响应数据为空",
        )
        .await
    }
}

/// 更新招聘官网推广渠道响应
///
/// 官方响应 `data` 即 `website_channel_info` 本体（扁平 `{id, name, link, code}`，无嵌套容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道链接。
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道推广码。
    pub code: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/hire/v1/websites/test001/channels/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/hire/v1/websites/test001/channels/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "id": "7085989097067563300",
                    "name": "微信推广渠道",
                    "link": "http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5",
                    "code": "A1KM6A5"
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

        let resp = UpdateRequest::new(config)
            .website_id("test001".to_string())
            .channel_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.id.as_deref(), Some("7085989097067563300"));
        assert_eq!(resp.name.as_deref(), Some("微信推广渠道"));
        assert_eq!(
            resp.link.as_deref(),
            Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5")
        );
        assert_eq!(resp.code.as_deref(), Some("A1KM6A5"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
