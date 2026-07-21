//! Bitable 获取多维表格详情API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::AppService;
use super::models::App;

/// 获取多维表格请求。
#[derive(Debug, Clone)]
pub struct GetAppRequest {
    /// 应用token
    app_token: String,
    /// 配置信息
    config: Config,
}

/// 获取多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppResponse {
    /// 应用信息
    pub app: App,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetAppRequest {
    /// 创建新的多维表格查询请求。
    /// 创建获取多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAppResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::AppGet(self.app_token.clone());

        // #439: method 来自 catalog
        let api_request: ApiRequest<GetAppResponse> = api_endpoint.to_request::<GetAppResponse>();

        // 发送请求
        Transport::request_typed(api_request, &self.config, Some(option), "获取多维表格").await
    }
}

impl AppService {
    /// 创建获取多维表格请求 builder。
    pub fn get_builder(&self, app_token: impl Into<String>) -> GetAppRequest {
        GetAppRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建获取多维表格请求
    pub fn get_app(&self, app_token: impl Into<String>) -> GetAppRequest {
        self.get_builder(app_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/bitable/v1/apps/{app_token} → GetAppResponse。
    /// 完整断言 method、path、auth（来自 catalog #439）和响应。
    #[tokio::test]
    async fn test_get_app_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/bitable/v1/apps/app%20token"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(header("X-Test-Option", "preserved"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "app": { "app_token": "app token", "name": "测试" } }
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
            .add_header("X-Test-Option", "preserved")
            .build();
        let resp = GetAppRequest::new(config)
            .app_token("app token")
            .execute_with_options(option)
            .await
            .expect("获取多维表格应成功");
        assert_eq!(resp.app.app_token, "app token");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "GET");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app%20token"
        );
        assert_eq!(
            received[0]
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok()),
            Some("Bearer test-tenant-token")
        );
    }
}
