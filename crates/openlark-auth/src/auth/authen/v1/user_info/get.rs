//! 获取用户信息 API
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/get>
use crate::models::authen::UserInfoResponse;
///
/// API文档: <https://open.feishu.cn/document/server-docs/user-authentication/access-token/user_info>
///
/// 通过 `user_access_token` 获取登录用户的信息。
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

/// 获取用户信息请求
pub struct UserInfoRequestBuilder {
    user_access_token: String,
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// 获取用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfoResponseData {
    /// 用户信息响应
    pub data: UserInfoResponse,
}

impl ApiResponseTrait for UserInfoResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

impl UserInfoRequestBuilder {
    /// 创建 user_info 请求
    pub fn new(config: Config) -> Self {
        Self {
            user_access_token: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.user_access_token = user_access_token.into();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserInfoResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserInfoResponseData> {
        // 验证必填字段
        validate_required!(self.user_access_token, "用户访问令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::UserInfo;

        let mut api_request: ApiRequest<UserInfoResponseData> =
            ApiRequest::get(api_endpoint.path())
                .header(
                    "Authorization",
                    format!("Bearer {}", self.user_access_token),
                )
                .with_supported_access_token_types(vec![AccessTokenType::None]);

        // 添加查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request
                .query_mut()
                .insert("user_id_type".to_string(), user_id_type.clone());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("获取用户信息", "响应数据为空"))
    }
}

/// 用户信息API服务
#[derive(Debug)]
pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    /// 创建用户信息服务实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoRequestBuilder {
        UserInfoRequestBuilder::new(self.config.clone())
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UserInfoRequestBuilder, will be removed in v1.0 (#271)")]
pub type UserInfoBuilder = UserInfoRequestBuilder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path, query_param},
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_user_info_builder_new() {
        let config = create_test_config();
        let builder = UserInfoRequestBuilder::new(config);
        assert!(builder.user_access_token.is_empty());
        assert!(builder.user_id_type.is_none());
    }

    #[test]
    fn test_user_info_builder_chain() {
        let config = create_test_config();
        let builder = UserInfoRequestBuilder::new(config)
            .user_access_token("my_token")
            .user_id_type("open_id");
        assert_eq!(builder.user_access_token, "my_token");
        assert_eq!(builder.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_user_info_builder_user_access_token_chained() {
        let config = create_test_config();
        let builder = UserInfoRequestBuilder::new(config).user_access_token("chained_token");
        assert_eq!(builder.user_access_token, "chained_token");
    }

    #[test]
    fn test_user_info_builder_user_id_type_chained() {
        let config = create_test_config();
        let builder = UserInfoRequestBuilder::new(config).user_id_type("union_id");
        assert_eq!(builder.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_user_info_response_data_deserialization() {
        let json = r#"{"code":0,"msg":"success","data":{"open_id":"ou_def456","union_id":"on_abc123","name":"张三","en_name":"John Zhang"}}"#;
        let response: UserInfoResponseData = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.data.open_id, "ou_def456");
        assert_eq!(response.data.data.open_id, "ou_def456");
        assert_eq!(response.data.union_id, Some("on_abc123".to_string()));
        assert_eq!(response.data.name, Some("张三".to_string()));
        assert_eq!(response.data.en_name, Some("John Zhang".to_string()));
    }

    #[test]
    fn test_user_info_response_preserves_wrapper_construction() {
        let response = UserInfoResponseData {
            data: UserInfoResponse {
                data: crate::models::authen::UserInfo {
                    open_id: "ou_compat".to_string(),
                    union_id: None,
                    user_id: None,
                    name: None,
                    en_name: None,
                    email: None,
                    enterprise_email: None,
                    mobile: None,
                    avatar_url: None,
                    avatar: None,
                    status: None,
                    department_ids: None,
                    group_ids: None,
                    positions: None,
                    employee_no: None,
                    dingtalk_user_id: None,
                    enterprise_extension: None,
                    custom_attrs: None,
                    tenant_key: None,
                },
            },
        };

        assert_eq!(response.data.open_id, "ou_compat");
        assert_eq!(response.data.data.open_id, "ou_compat");
    }

    #[test]
    fn test_user_info_response_data_format() {
        assert_eq!(UserInfoResponseData::data_format(), ResponseFormat::Flatten);
    }

    #[test]
    fn test_user_info_service_new() {
        let config = create_test_config();
        let service = UserInfoService::new(config);
        assert!(service.config.app_id() == "test_app");
    }

    #[test]
    fn test_user_info_service_get() {
        let config = create_test_config();
        let service = UserInfoService::new(config);
        let builder = service.get();
        assert!(builder.user_access_token.is_empty());
    }

    #[tokio::test]
    async fn test_execute_sends_user_token_query_and_parses_top_level_user_info() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .and(header("authorization", "Bearer user_token"))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "open_id": "ou_def456",
                    "union_id": "on_abc123",
                    "name": "张三",
                    "en_name": "John Zhang"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let response = UserInfoRequestBuilder::new(config)
            .user_access_token("user_token")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("user_info 请求应成功");

        assert_eq!(response.data.open_id, "ou_def456");
        assert_eq!(response.data.data.open_id, "ou_def456");
        assert_eq!(response.data.union_id, Some("on_abc123".to_string()));
        assert_eq!(response.data.name, Some("张三".to_string()));
        assert_eq!(response.data.en_name, Some("John Zhang".to_string()));
    }

    #[tokio::test]
    async fn test_execute_missing_user_access_token_fails_before_network() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/authen/v1/user_info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "open_id": "ou_unexpected"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = UserInfoRequestBuilder::new(config).execute().await;

        assert!(result.is_err());
        let received_requests = server.received_requests().await.unwrap_or_default();
        assert!(received_requests.is_empty());
    }
}
