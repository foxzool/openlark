//! 向管理员申请授权
//!
//! 官方文档无请求体（cURL 示例无 `-d`，Go SDK 亦无参调用），授权范围由
//! 服务端按应用未授权权限自行推导，本接口为无 body 的 POST。
//!
//! docPath: <https://open.feishu.cn/document/application-v6/scope/apply>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 向管理员申请授权的请求。
#[derive(Debug, Clone)]
pub struct ApplyScopeRequest {
    config: Arc<Config>,
}

/// 向管理员申请授权的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyScopeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ApplyScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 成功但无 `data` 字段时的空成功（apply 类 API，与旧 `unwrap_or` 默认值一致）。
    fn empty_success() -> Option<Self> {
        Some(Self { data: None })
    }
}

impl ApplyScopeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行向管理员申请授权请求。
    pub async fn execute(self) -> SDKResult<ApplyScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ApplyScopeResponse> {
        let path = "/open-apis/application/v6/scopes/apply";
        let req: ApiRequest<ApplyScopeResponse> = ApiRequest::post(path);
        Transport::request_typed(req, &self.config, Some(option), "向管理员申请授权").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../scopes/apply（无请求体）→ 响应解析（官方 data 无结构）。
    #[tokio::test]
    async fn test_apply_scope_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/application/v6/scopes/apply"))
            .and(wiremock::matchers::body_bytes(b"".to_vec()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success"
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

        let resp = ApplyScopeRequest::new(config)
            .execute()
            .await
            .expect("向管理员申请授权应成功");
        // 官方 data 无结构，成功即可（无 data 字段走 empty_success）
        assert!(resp.data.is_none());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/scopes/apply"
        );
        // 官方无请求体：不得发送任何 body
        assert!(received[0].body.is_empty());
    }
}
