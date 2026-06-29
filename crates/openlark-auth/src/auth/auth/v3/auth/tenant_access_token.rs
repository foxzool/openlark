//! 商店应用获取 tenant_access_token API
//! docPath: https://open.feishu.cn/document/server-docs/authentication-management/access-token/tenant_access_token
use crate::models::auth::TenantAccessTokenResponse;
///
/// API文档: https://open.feishu.cn/document/server-docs/authentication-management/access-token/tenant_access_token
///
/// 应用商店应用通过此接口获取 tenant_access_token，调用接口获取企业资源时，
/// 需要使用 tenant_access_token 作为授权凭证。
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct TenantAccessTokenBody {
    app_access_token: String,
    tenant_key: String,
}

/// 商店应用获取 tenant_access_token 请求
pub struct TenantAccessTokenRequestBuilder {
    app_access_token: String,
    tenant_key: String,
    /// 配置信息
    config: Config,
}

/// 商店应用获取 tenant_access_token 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct TenantAccessTokenResponseData {
    /// 租户访问令牌响应
    pub data: TenantAccessTokenResponse,
}

impl ApiResponseTrait for TenantAccessTokenResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

impl TenantAccessTokenRequestBuilder {
    /// 创建 tenant_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            app_access_token: String::new(),
            tenant_key: String::new(),
            config,
        }
    }

    /// 设置应用访问凭证
    pub fn app_access_token(mut self, app_access_token: impl Into<String>) -> Self {
        self.app_access_token = app_access_token.into();
        self
    }

    /// 设置租户标识
    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.tenant_key = tenant_key.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TenantAccessTokenResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TenantAccessTokenResponseData> {
        validate_required!(self.app_access_token, "应用访问凭证不能为空");
        validate_required!(self.tenant_key, "租户标识不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthApiV3;
        let api_endpoint = AuthApiV3::TenantAccessToken;

        // 构建请求体
        let request_body = TenantAccessTokenBody {
            app_access_token: self.app_access_token,
            tenant_key: self.tenant_key,
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<TenantAccessTokenResponseData> =
            ApiRequest::post(api_endpoint.path())
                .body(serde_json::to_value(&request_body)?)
                .with_supported_access_token_types(vec![AccessTokenType::None]);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取商店应用 tenant_access_token",
                "响应数据为空",
            )
        })
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TenantAccessTokenRequestBuilder, will be removed in v1.0 (#271)")]
pub type TenantAccessTokenBuilder = TenantAccessTokenRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, method, path},
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_tenant_access_token_builder_new() {
        let config = create_test_config();
        let builder = TenantAccessTokenRequestBuilder::new(config);
        assert!(builder.app_access_token.is_empty());
        assert!(builder.tenant_key.is_empty());
    }

    #[test]
    fn test_tenant_access_token_builder_chain() {
        let config = create_test_config();
        let builder = TenantAccessTokenRequestBuilder::new(config)
            .app_access_token("my_app_access_token")
            .tenant_key("my_tenant_key");
        assert_eq!(builder.app_access_token, "my_app_access_token");
        assert_eq!(builder.tenant_key, "my_tenant_key");
    }

    #[test]
    fn test_tenant_access_token_builder_app_access_token_chained() {
        let config = create_test_config();
        let builder = TenantAccessTokenRequestBuilder::new(config)
            .app_access_token("chained_app_access_token");
        assert_eq!(builder.app_access_token, "chained_app_access_token");
    }

    #[test]
    fn test_tenant_access_token_builder_tenant_key_chained() {
        let config = create_test_config();
        let builder = TenantAccessTokenRequestBuilder::new(config).tenant_key("chained_tenant_key");
        assert_eq!(builder.tenant_key, "chained_tenant_key");
    }

    #[test]
    fn test_tenant_access_token_response_data_deserialization() {
        let json = r#"{"code":0,"msg":"success","tenant_access_token":"token123","expire":7200}"#;
        let response: TenantAccessTokenResponseData =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.data.tenant_access_token, "token123");
        assert_eq!(response.data.expires_in, 7200);
    }

    #[test]
    fn test_tenant_access_token_response_data_format() {
        assert_eq!(
            TenantAccessTokenResponseData::data_format(),
            ResponseFormat::Flatten
        );
    }

    #[tokio::test]
    async fn test_execute_sends_app_token_tenant_key_and_no_authorization() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token"))
            .and(body_json(json!({
                "app_access_token": "app-token",
                "tenant_key": "tenant-001"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "tenant_access_token": "tenant-token",
                "expire": 7200
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .build();

        let response = TenantAccessTokenRequestBuilder::new(config)
            .app_access_token("app-token")
            .tenant_key("tenant-001")
            .execute()
            .await
            .expect("tenant_access_token 请求应成功");

        assert_eq!(response.data.tenant_access_token, "tenant-token");

        let received_requests = server.received_requests().await.unwrap_or_default();
        assert_eq!(received_requests.len(), 1);
        assert!(!received_requests[0].headers.contains_key("authorization"));
    }
}
