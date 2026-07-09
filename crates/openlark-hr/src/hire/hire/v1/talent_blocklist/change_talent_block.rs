//! 加入/移除屏蔽名单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent_blocklist/change_talent_block>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::GenericOperationResult;

/// 加入/移除屏蔽名单请求
#[derive(Debug, Clone)]
pub struct ChangeTalentBlockRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ChangeTalentBlockRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ChangeTalentBlockResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ChangeTalentBlockResponse> {
        let mut request = ApiRequest::<ChangeTalentBlockResponse>::post(
            "/open-apis/hire/v1/talent_blocklist/change_talent_block",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "加入/移除屏蔽名单响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 加入/移除屏蔽名单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChangeTalentBlockResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: GenericOperationResult,
}

impl ApiResponseTrait for ChangeTalentBlockResponse {
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

    /// 端到端：POST /open-apis/hire/v1/talent_blocklist/change_talent_block
    #[tokio::test]
    async fn test_change_talent_block_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/talent_blocklist/change_talent_block",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "operation": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ChangeTalentBlockRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
