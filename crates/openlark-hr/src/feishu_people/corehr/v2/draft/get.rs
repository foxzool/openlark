//! 根据组织架构调整 ID 查询发起的流程信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/draft/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据组织架构调整 ID 查询发起的流程信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    draft_id: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            draft_id: None,
        }
    }

    /// 设置 `draft_id`。
    pub fn draft_id(mut self, draft_id: String) -> Self {
        self.draft_id = Some(draft_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let draft_id = self.draft_id.unwrap_or_default();
        validate_required!(draft_id.trim(), "draft_id 不能为空");

        let request =
            ApiRequest::<GetResponse>::get(format!("/open-apis/corehr/v2/drafts/{draft_id}"));

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 根据组织架构调整 ID 查询发起的流程信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 响应数据
    /// draft信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<DraftInfo>,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftInfo {
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

impl ApiResponseTrait for GetResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/drafts/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/drafts/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetRequest::new(config)
            .draft_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
