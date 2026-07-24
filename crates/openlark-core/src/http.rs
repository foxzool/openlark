use std::marker::PhantomData;

use reqwest::RequestBuilder;
use tracing::debug;
use tracing::{Instrument, info_span};

use crate::{
    SDKResult,
    api::ApiResponseTrait,
    api::{ApiRequest, Response},
    config::Config,
    constants::*,
    error::CoreError,
    req_option::RequestOption,
    request_execution::{ResponseDecoder, UnifiedRequestBuilder},
};

/// HTTP 传输层
///
/// 负责处理 API 请求的发送和响应接收，支持多种认证令牌类型
pub struct Transport<T> {
    phantom_data: PhantomData<T>,
}

impl<T> Default for Transport<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Transport<T> {
    /// 创建新的 Transport 实例
    pub fn new() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

impl<T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>> Transport<T> {
    /// 发送 API 请求
    ///
    /// # 参数
    /// - `req`: API 请求对象
    /// - `config`: 客户端配置
    /// - `option`: 请求选项（可选）
    ///
    /// # 返回
    /// API 响应结果
    pub async fn request<R: Send>(
        req: ApiRequest<R>,
        config: &Config,
        option: Option<RequestOption>,
    ) -> Result<Response<T>, CoreError> {
        // Create span for HTTP request tracing
        let span = info_span!(
            "http_request",
            method = %req.method(),
            path = %req.api_path(),
            app_id = %config.app_id,
            duration_ms = tracing::field::Empty,
            status = tracing::field::Empty,
        );

        async move {
            let start_time = std::time::Instant::now();
            let option = option.unwrap_or_default();

            let mut token_types = req.supported_access_token_types();
            if token_types.is_empty() {
                token_types = vec![AccessTokenType::None];
            }

            let result: Result<_, _> = async {
                crate::auth::policy::validate_token_type(&token_types, &option)?;
                let access_token_type = crate::auth::policy::determine_token_type(
                    &token_types,
                    &option,
                    config.enable_token_cache,
                );
                validate(config, &option, access_token_type)?;

                Self::do_request(req, access_token_type, config, option).await
            }
            .await;

            // Record metrics in current span
            let current_span = tracing::Span::current();
            let duration_ms = start_time.elapsed().as_millis() as u64;
            current_span.record("duration_ms", duration_ms);

            match &result {
                Ok(response) => {
                    current_span.record(
                        "status",
                        if response.is_success() {
                            "success"
                        } else {
                            "api_error"
                        },
                    );
                }
                Err(_) => {
                    current_span.record("status", "error");
                }
            }

            result
        }
        .instrument(span)
        .await
    }

    /// Canonical typed-request 入口（#479）：焊合 [`Transport::request`] + 抽取 typed `T`。
    ///
    /// 成功时直接返回 typed `T`；抽取失败时错误经 [`Response::decode`] 的 `map_context`
    /// 机制携带 `operation`（= `extract_response_data`）+ `resource`（= `context`）
    /// + 响应携带的 `request_id`，便于排障与飞书服务端对账。
    ///
    /// 全部 leaf 已迁入本入口，含删除/空成功类 API（由响应类型的 [`ApiResponseTrait`] 声明
    /// 解码策略）。兄弟入口 [`Transport::request`] 仅留给需要原始 `Response<T>` 的下载 /
    /// 自定义抽取路径（按 Content-Disposition 取文件名、size 上限校验、对外返回
    /// `Response<Vec<u8>>`），不再经 `ensure_success`（#470 后零消费者，随 #506 删除）。
    pub async fn request_typed<R: Send>(
        req: ApiRequest<R>,
        config: &Config,
        option: Option<RequestOption>,
        context: &str,
    ) -> Result<T, CoreError> {
        let response = Self::request(req, config, option).await?;
        response.decode(context)
    }

    async fn do_request<R: Send>(
        mut http_req: ApiRequest<R>,
        access_token_type: AccessTokenType,
        config: &Config,
        option: RequestOption,
    ) -> SDKResult<Response<T>> {
        // 纯委托 ReqTranslator 已吸收：直接走统一请求构建（#422 / #432）
        let req =
            UnifiedRequestBuilder::build(&mut http_req, access_token_type, config, &option).await?;
        debug!(
            method = %http_req.method(),
            path = %http_req.api_path(),
            "Sending request"
        );
        let resp = Self::do_send(
            req,
            http_req.to_bytes(),
            !http_req.file().is_empty(),
            config.max_response_size(),
        )
        .await?;
        debug!(
            success = resp.is_success(),
            code = resp.raw_response.code,
            msg = %resp.raw_response.msg,
            "Received response"
        );

        crate::auth::app_ticket::recover_app_ticket_if_needed(
            resp.is_success(),
            resp.raw_response.code,
            config,
        )
        .await?;

        Ok(resp)
    }

    /// 执行 HTTP 请求
    pub(crate) async fn do_send(
        raw_request: RequestBuilder,
        body: Vec<u8>,
        multi_part: bool,
        max_response_size: u64,
    ) -> SDKResult<Response<T>> {
        // Create span for network request tracing
        let span = info_span!(
            "http_send",
            multi_part = multi_part,
            body_size = body.len(),
            response_code = tracing::field::Empty,
            response_size = tracing::field::Empty,
        );

        async move {
            let future = if multi_part {
                raw_request.send()
            } else {
                raw_request.body(body).send()
            };

            match future.await {
                Ok(response) => {
                    let status_code = response.status();
                    tracing::Span::current().record("response_code", status_code.as_u16());

                    // 使用改进的响应处理器，单次解析而非双重解析
                    ResponseDecoder::handle_response(response, max_response_size).await
                }
                Err(err) => {
                    debug!("Request error: {err:?}");
                    tracing::Span::current().record("response_code", 0_u16); // Indicate network error
                    Err(err.into())
                }
            }
        }
        .instrument(span)
        .await
    }
}

fn validate(
    config: &Config,
    option: &RequestOption,
    access_token_type: AccessTokenType,
) -> Result<(), CoreError> {
    if config.app_id.is_empty() {
        return Err(crate::error::validation_error("app_id", "AppId is empty"));
    }

    if config.app_secret.is_empty() {
        return Err(crate::error::validation_error(
            "app_secret",
            "AppSecret is empty",
        ));
    }

    crate::auth::policy::validate_authorization(config, option, access_token_type)?;

    if option.header.contains_key(HTTP_HEADER_KEY_REQUEST_ID) {
        return Err(crate::error::validation_error(
            "header",
            format!("use {HTTP_HEADER_KEY_REQUEST_ID} as header key is not allowed"),
        ));
    }
    if option.header.contains_key(HTTP_HEADER_REQUEST_ID) {
        return Err(crate::error::validation_error(
            "header",
            format!("use {HTTP_HEADER_REQUEST_ID} as header key is not allowed"),
        ));
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod test {
    use std::collections::HashMap;

    use crate::{
        config::Config,
        constants::{AccessTokenType, AppType, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID},
        http::validate,
        req_option::RequestOption,
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_config_marketplace() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::Marketplace)
            .build()
    }

    #[test]
    fn test_validate_empty_app_id() {
        let config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_empty_app_secret() {
        let config = Config::builder().app_id("test_id").app_secret("").build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_no_cache_missing_access_tokens() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_no_cache_with_tokens() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = Some("token".to_string());

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_tenant_no_key() {
        let config = create_test_config_marketplace();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_marketplace_tenant_with_key() {
        let config = create_test_config_marketplace();
        let mut option = RequestOption::default();
        option.tenant_key = Some("tenant_key".to_string());

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_token_empty() {
        let config = create_test_config();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_user_token_present() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_forbidden_header_key_request_id() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert(HTTP_HEADER_KEY_REQUEST_ID.to_string(), "test".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_forbidden_header_request_id() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert(HTTP_HEADER_REQUEST_ID.to_string(), "test".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_valid_config() {
        let config = create_test_config();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_cache_none_token_type() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_file_name_whitespace_handling() {
        let raw = " attachment ; filename=\"test.txt\" ; filename*=UTF-8''spaced%20file.txt ";
        let file_name = crate::content_disposition::extract_filename(raw).unwrap();
        assert_eq!(file_name, "spaced%20file.txt");
    }

    #[test]
    fn test_decode_file_name_no_equals() {
        let raw = "attachment; filename*UTF-8''invalid.txt";
        let file_name = crate::content_disposition::extract_filename(raw);
        assert!(file_name.is_none());
    }

    #[test]
    fn test_validate_config_with_all_required_fields() {
        let config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("valid_app_secret")
            .enable_token_cache(true)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_app_with_valid_tenant_key() {
        let config = Config::builder()
            .app_id("marketplace_app")
            .app_secret("marketplace_secret")
            .app_type(AppType::Marketplace)
            .build();
        let mut option = RequestOption::default();
        option.tenant_key = Some("valid_tenant_key".to_string());

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_app_type_with_non_tenant_token() {
        let config = Config::builder()
            .app_id("marketplace_app")
            .app_secret("marketplace_secret")
            .app_type(AppType::Marketplace)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        // User token type needs user_access_token to be present
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());

        // App token type should pass without additional requirements for marketplace
        let result = validate(&config, &RequestOption::default(), AccessTokenType::App);
        assert!(result.is_ok());

        // None token type should also pass
        let result = validate(&config, &RequestOption::default(), AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_cache_with_multiple_token_types() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .enable_token_cache(false)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());
        option.tenant_access_token = Some("tenant_token".to_string());
        option.app_access_token = Some("app_token".to_string());

        // Should validate successfully with any token present
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_token_type_with_empty_user_token() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        option.tenant_access_token = Some("tenant_token".to_string()); // Other tokens present

        // Should fail when User token type but user_access_token is empty
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
        if let Err(crate::error::CoreError::Validation { message, .. }) = result {
            assert!(message.contains("user access token is empty"));
        }
    }

    #[test]
    fn test_validate_forbidden_headers_custom_values() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("X-Request-Id".to_string(), "custom_id".to_string());
        header.insert("Custom-Header".to_string(), "value".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_forbidden_headers_request_id_variation() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("Request-Id".to_string(), "another_id".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(
            result,
            Err(crate::error::CoreError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_allowed_custom_headers() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("Authorization".to_string(), "Bearer token".to_string());
        header.insert("Content-Type".to_string(), "application/json".to_string());
        header.insert("Custom-App-Header".to_string(), "value".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_file_name_missing_utf8_prefix() {
        let raw = "attachment; filename*=''missing_utf8.txt";
        let file_name = crate::content_disposition::extract_filename(raw);
        assert_eq!(file_name, Some("missing_utf8.txt".to_string()));
    }

    #[test]
    fn test_decode_file_name_malformed_filename_star() {
        let raw = "attachment; filename*=UTF-8";
        let file_name = crate::content_disposition::extract_filename(raw);
        assert_eq!(file_name, None);
    }

    #[test]
    fn test_decode_file_name_multiple_filename_star_entries() {
        let raw = "attachment; filename*=UTF-8''first.txt; filename*=UTF-8''second.txt";
        let file_name = crate::content_disposition::extract_filename(raw).unwrap();
        // Should return the first match
        assert_eq!(file_name, "first.txt");
    }

    #[test]
    fn test_decode_file_name_special_characters() {
        let raw = "attachment; filename*=UTF-8''special%20%21%40%23.txt";
        let file_name = crate::content_disposition::extract_filename(raw).unwrap();
        assert_eq!(file_name, "special%20%21%40%23.txt");
    }

    #[test]
    fn test_decode_file_name_empty_filename() {
        let raw = "attachment; filename*=UTF-8''";
        let file_name = crate::content_disposition::extract_filename(raw).unwrap();
        assert_eq!(file_name, "");
    }

    #[test]
    fn test_validate_with_cache_enabled_various_token_types() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .enable_token_cache(true)
            .build();
        let option = RequestOption::default();

        // With cache enabled, should validate OK for all token types
        assert!(validate(&config, &option, AccessTokenType::None).is_ok());
        assert!(validate(&config, &option, AccessTokenType::App).is_ok());
        assert!(validate(&config, &option, AccessTokenType::Tenant).is_ok());
    }

    #[test]
    fn test_validate_self_build_app_type_with_tenant_token() {
        let config = Config::builder()
            .app_id("self_build_app")
            .app_secret("self_build_secret")
            .app_type(AppType::SelfBuild)
            .build();
        let option = RequestOption::default();

        // Self-build apps should validate OK without tenant_key
        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_comprehensive_error_messages() {
        let config_empty_id = Config::builder().app_id("").app_secret("secret").build();

        let config_empty_secret = Config::builder().app_id("app_id").app_secret("").build();

        let option = RequestOption::default();

        // Test specific error messages
        if let Err(crate::error::CoreError::Validation { message, .. }) =
            validate(&config_empty_id, &option, AccessTokenType::None)
        {
            assert_eq!(message, "AppId is empty");
        } else {
            panic!("Expected IllegalParamError for empty app_id");
        }

        if let Err(crate::error::CoreError::Validation { message, .. }) =
            validate(&config_empty_secret, &option, AccessTokenType::None)
        {
            assert_eq!(message, "AppSecret is empty");
        } else {
            panic!("Expected IllegalParamError for empty app_secret");
        }
    }
}
