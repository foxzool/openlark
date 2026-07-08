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
use crate::common::api_utils::missing_response_data_error;

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

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetAppResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error("获取多维表格", response.raw_response.request_id.clone())
        })
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../apps/{app_token} → GetAppResponse。
    #[tokio::test]
    async fn test_get_app_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/bitable/v1/apps/app001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "app": { "app_token": "app001", "name": "测试" } }
            })))
            .mount(&server).await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        GetAppRequest::new(config)
            .app_token("app001")
            .execute()
            .await
            .expect("获取多维表格应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/bitable/v1/apps/app001");
    }
}
