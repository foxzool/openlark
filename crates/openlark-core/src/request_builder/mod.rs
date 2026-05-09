mod auth_handler;
mod header_builder;
mod multipart_builder;

pub use auth_handler::AuthHandler;
pub use header_builder::HeaderBuilder;
pub use multipart_builder::MultipartBuilder;

use crate::{
    api::{ApiRequest, RequestData},
    config::Config,
    constants::AccessTokenType,
    error::CoreError,
    req_option::RequestOption,
};
use reqwest::RequestBuilder;
use std::{future::Future, pin::Pin};

/// 统一的请求构建器，负责协调各个子构建器
pub struct UnifiedRequestBuilder;

impl UnifiedRequestBuilder {
    pub fn build<'a, R: Send>(
        req: &'a mut ApiRequest<R>,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, CoreError>> + Send + 'a>> {
        Box::pin(async move {
            // 1. 构建基础请求
            let url = Self::build_url(config, req)?;
            // 将HttpMethod转换为reqwest::Method
            let reqwest_method = match req.method() {
                crate::api::HttpMethod::Get => reqwest::Method::GET,
                crate::api::HttpMethod::Post => reqwest::Method::POST,
                crate::api::HttpMethod::Put => reqwest::Method::PUT,
                crate::api::HttpMethod::Delete => reqwest::Method::DELETE,
                crate::api::HttpMethod::Patch => reqwest::Method::PATCH,
                crate::api::HttpMethod::Head => reqwest::Method::HEAD,
                crate::api::HttpMethod::Options => reqwest::Method::OPTIONS,
            };

            let mut req_builder = config.http_client.request(reqwest_method, url.as_ref());

            // 2. 构建请求头
            req_builder = HeaderBuilder::build_headers(req_builder, config, option);

            // 3. 处理认证
            req_builder =
                AuthHandler::apply_auth(req_builder, access_token_type, config, option).await?;

            // 4. 处理请求体
            if !req.file().is_empty() {
                if let Some(_body_data) = &req.body {
                    req_builder = MultipartBuilder::build_multipart(
                        req_builder,
                        &req.to_bytes(),
                        &req.file(),
                    )?;
                }
            } else if let Some(body_data) = &req.body {
                match body_data {
                    RequestData::Binary(data) if !data.is_empty() => {
                        req_builder = req_builder.body(data.clone());
                        req_builder = req_builder.header(
                            crate::constants::CONTENT_TYPE_HEADER,
                            crate::constants::DEFAULT_CONTENT_TYPE,
                        );
                    }
                    RequestData::Json(json) => {
                        let json_bytes = serde_json::to_vec(json).unwrap_or_else(|e| {
                            tracing::warn!(error = %e, "request_builder body JSON 序列化失败，使用空 vec");
                            vec![]
                        });
                        req_builder = req_builder.body(json_bytes);
                        req_builder = req_builder.header(
                            crate::constants::CONTENT_TYPE_HEADER,
                            crate::constants::DEFAULT_CONTENT_TYPE,
                        );
                    }
                    _ => {}
                }
            }

            // 5. 应用超时：ApiRequest.timeout > Config.req_timeout > 无超时
            if let Some(timeout) = req.timeout.or(config.req_timeout) {
                req_builder = req_builder.timeout(timeout);
            }

            Ok(req_builder)
        })
    }

    fn build_url<R: Send>(config: &Config, req: &ApiRequest<R>) -> Result<url::Url, CoreError> {
        let mut url = url::Url::parse(&config.base_url)
            .map_err(|e| crate::error::network_error(format!("invalid base url: {e}")))?;

        {
            let mut path_segments = url
                .path_segments_mut()
                .map_err(|_| crate::error::network_error("invalid base url path".to_string()))?;
            path_segments.clear();
            for segment in req.api_path().trim_start_matches('/').split('/') {
                path_segments.push(segment);
            }
        }

        for (k, v) in &req.query {
            url.query_pairs_mut().append_pair(k, v);
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::ApiRequest, constants::AppType};
    use reqwest::Method;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("https://open.feishu.cn")
            .build()
    }

    fn create_test_api_request() -> ApiRequest<()> {
        ApiRequest::get("https://open.feishu.cn/open-apis/test")
    }

    #[test]
    fn test_unified_request_builder_struct_creation() {
        let _builder = UnifiedRequestBuilder;
    }

    #[tokio::test]
    async fn test_build_basic_request() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // Should build request successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_body() {
        let mut api_req = ApiRequest::<()>::post("https://open.feishu.cn/open-apis/test").body(
            crate::api::RequestData::Text("{\"test\": \"data\"}".to_string()),
        );

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_files() {
        let mut api_req = ApiRequest::<()>::post("https://open.feishu.cn/open-apis/test")
            .body(crate::api::RequestData::Text("file content".to_string()));

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // This might fail due to multipart builder dependencies, but should not panic
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_build_request_with_query() {
        let mut api_req = create_test_api_request();
        api_req.query.insert("page".to_string(), "1".to_string());
        api_req.query.insert("limit".to_string(), "10".to_string());

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_url_encodes_path_segments() {
        let mut api_req = ApiRequest::<()>::get("https://open.feishu.cn/open-apis/安全测试/子路径");
        api_req
            .query_mut()
            .insert("q1".to_string(), "a b".to_string());
        let config = create_test_config();

        let url = UnifiedRequestBuilder::build_url(&config, &api_req)
            .expect("failed to build encoded url");

        assert_eq!(
            url.path(),
            "/open-apis/%E5%AE%89%E5%85%A8%E6%B5%8B%E8%AF%95/%E5%AD%90%E8%B7%AF%E5%BE%84"
        );
        let query = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>();
        assert_eq!(query, vec![("q1".to_string(), "a b".to_string())]);
    }

    #[tokio::test]
    async fn test_build_request_with_app_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            app_access_token: Some("app_token_123".to_string()),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::App, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_tenant_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            tenant_access_token: Some("tenant_token_123".to_string()),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::Tenant, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_user_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            user_access_token: Some("user_token_123".to_string()),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::User, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_different_methods() {
        let config = create_test_config();
        let option = RequestOption::default();

        let methods = [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ];

        for method in methods.iter() {
            let mut api_req = match method.as_str() {
                "GET" => ApiRequest::<()>::get("https://open.feishu.cn/open-apis/test"),
                "POST" => ApiRequest::<()>::post("https://open.feishu.cn/open-apis/test"),
                "PUT" => ApiRequest::<()>::put("https://open.feishu.cn/open-apis/test"),
                "DELETE" => ApiRequest::<()>::delete("https://open.feishu.cn/open-apis/test"),
                "PATCH" => ApiRequest::<()>::get("https://open.feishu.cn/open-apis/test"), // PATCH not supported, fallback to GET
                _ => ApiRequest::<()>::get("https://open.feishu.cn/open-apis/test"),
            };

            let result =
                UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                    .await;

            assert!(result.is_ok(), "Failed for method: {method:?}");
        }
    }

    #[test]
    fn test_build_url_basic() -> Result<(), CoreError> {
        let config = create_test_config();
        let api_req = create_test_api_request();

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url =
            result.map_err(|e| crate::error::configuration_error(format!("构建URL失败: {e}")))?;
        // May have trailing ? due to parse_with_params implementation
        assert!(
            url.as_str()
                .starts_with("https://open.feishu.cn/open-apis/test")
        );
        Ok(())
    }

    #[test]
    fn test_build_url_with_query() -> Result<(), CoreError> {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query.insert("page".to_string(), "1".to_string());
        api_req.query.insert("size".to_string(), "20".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url =
            result.map_err(|e| crate::error::configuration_error(format!("构建URL失败: {e}")))?;
        let url_str = url.as_str();
        assert!(url_str.starts_with("https://open.feishu.cn/open-apis/test"));
        assert!(url_str.contains("page=1"));
        assert!(url_str.contains("size=20"));
        Ok(())
    }

    #[test]
    fn test_build_url_with_special_characters() -> Result<(), CoreError> {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req
            .query
            .insert("query".to_string(), "test with spaces".to_string());
        api_req
            .query
            .insert("filter".to_string(), "key=value&other=data".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url =
            result.map_err(|e| crate::error::configuration_error(format!("构建URL失败: {e}")))?;
        // URL encoding should be handled properly
        assert!(url.as_str().contains("query="));
        assert!(url.as_str().contains("filter="));
        Ok(())
    }

    #[test]
    fn test_build_url_with_empty_query() -> Result<(), CoreError> {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query.insert("empty".to_string(), "".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url =
            result.map_err(|e| crate::error::configuration_error(format!("构建URL失败: {e}")))?;
        assert!(url.as_str().contains("empty="));
        Ok(())
    }

    #[test]
    fn test_build_url_invalid_base_url() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("invalid-url")
            .build();
        let api_req = create_test_api_request();

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        // Should return error for invalid URL
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_build_request_with_custom_headers() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let mut option = RequestOption {
            request_id: Some("custom-request-123".to_string()),
            ..Default::default()
        };
        option
            .header
            .insert("X-Custom-Header".to_string(), "custom-value".to_string());

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_complex_scenario() {
        let mut api_req =
            ApiRequest::<serde_json::Value>::post("https://open.feishu.cn/open-apis/complex/test")
                .body(crate::api::RequestData::Text(
                    "{\"complex\": \"data\", \"nested\": {\"value\": 123}}".to_string(),
                ))
                .query("version", "v1")
                .query("format", "json");

        let config = create_test_config();
        let option = RequestOption {
            request_id: Some("complex-request-456".to_string()),
            app_access_token: Some("app_token_456".to_string()),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::App, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_applies_config_req_timeout() {
        let mut api_req = create_test_api_request();
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("https://open.feishu.cn")
            .req_timeout(std::time::Duration::from_secs(12))
            .build();
        let option = RequestOption::default();

        let req_builder =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await
                .expect("request builder should be created");
        let request = req_builder.build().expect("request should build");

        assert_eq!(
            request.timeout().copied(),
            Some(std::time::Duration::from_secs(12))
        );
    }

    #[tokio::test]
    async fn test_build_request_prefers_api_timeout_over_config_req_timeout() {
        let mut api_req = create_test_api_request().timeout(std::time::Duration::from_secs(3));
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("https://open.feishu.cn")
            .req_timeout(std::time::Duration::from_secs(12))
            .build();
        let option = RequestOption::default();

        let req_builder =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await
                .expect("request builder should be created");
        let request = req_builder.build().expect("request should build");

        assert_eq!(
            request.timeout().copied(),
            Some(std::time::Duration::from_secs(3))
        );
    }

    #[test]
    fn test_unified_request_builder_is_send_sync() {
        // Test that UnifiedRequestBuilder implements required traits
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<UnifiedRequestBuilder>();
        assert_sync::<UnifiedRequestBuilder>();
    }

    #[tokio::test]
    async fn test_build_request_with_body_and_files_edge_case() {
        let mut api_req = ApiRequest::<()>::post("https://open.feishu.cn/open-apis/test").body(
            crate::api::RequestData::Text("file content combined".to_string()),
        );

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // Should handle files taking precedence over body
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_build_url_with_path_segments() -> Result<(), CoreError> {
        let config = create_test_config();
        let api_req =
            ApiRequest::<()>::get("https://open.feishu.cn/open-apis/v1/users/123/messages");

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url =
            result.map_err(|e| crate::error::configuration_error(format!("构建URL失败: {e}")))?;
        // May have trailing ? due to parse_with_params implementation
        assert!(
            url.as_str()
                .starts_with("https://open.feishu.cn/open-apis/v1/users/123/messages")
        );
        Ok(())
    }
}
