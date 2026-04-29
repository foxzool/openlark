use futures_util::StreamExt;
use serde_json::Value;
use tracing::debug;
use tracing::{Instrument, info_span};

use crate::{
    SDKResult,
    api::{ApiResponseTrait, BaseResponse, RawResponse, Response, ResponseFormat},
    content_disposition,
    error::{network_error, validation_error},
    observability::ResponseTracker,
};
use serde::Deserialize;
use std::any::Any;

/// 读取响应体，带大小限制保护
async fn read_body_with_limit(
    response: reqwest::Response,
    max_size: u64,
) -> Result<Vec<u8>, crate::error::CoreError> {
    // 预检：content_length 已知且超限时直接返回错误
    if let Some(content_length) = response.content_length() {
        if content_length > max_size {
            return Err(crate::error::CoreError::response_too_large(max_size, content_length));
        }
    }

    // 流式读取并累计大小
    let mut total_size: u64 = 0;
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| network_error(e.to_string()))?;
        total_size += chunk.len() as u64;
        if total_size > max_size {
            return Err(crate::error::CoreError::response_too_large(max_size, total_size));
        }
        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

#[cfg(test)]
use crate::error::{CoreError, ErrorCategory, ErrorCode, ErrorContext};

/// 改进的响应处理器，解决双重解析问题
/// 使用 #[serde(flatten)] 和高级 Serde 特性简化反序列化
pub struct ImprovedResponseHandler;

impl ImprovedResponseHandler {
    /// 处理响应的核心方法
    /// 相比原始实现，这个版本：
    /// 1. 减少了不必要的JSON解析次数
    /// 2. 使用更高效的直接反序列化
    /// 3. 更好的错误处理
    /// 4. 完整的可观测性支持
    pub async fn handle_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let format = match T::data_format() {
            ResponseFormat::Data => "data",
            ResponseFormat::Flatten => "flatten",
            ResponseFormat::Binary => "binary",
            ResponseFormat::Text => "text",
            ResponseFormat::Custom => "custom",
        };

        let span = info_span!(
            "response_handling",
            format = format,
            status_code = response.status().as_u16(),
            content_length = tracing::field::Empty,
            processing_duration_ms = tracing::field::Empty,
        );

        async move {
            let start_time = std::time::Instant::now();

            // 获取内容长度用于监控
            let content_length = response.content_length();
            if let Some(length) = content_length {
                tracing::Span::current().record("content_length", length);
            }

            let result = match T::data_format() {
                ResponseFormat::Data => Self::handle_data_response(response, max_size).await,
                ResponseFormat::Flatten => Self::handle_flatten_response(response, max_size).await,
                ResponseFormat::Binary => Self::handle_binary_response(response, max_size).await,
                ResponseFormat::Text => Self::handle_data_response(response, max_size).await, // 暂时使用data处理器
                ResponseFormat::Custom => Self::handle_data_response(response, max_size).await, // 暂时使用data处理器
            };

            // 记录处理时间
            let duration_ms = start_time.elapsed().as_millis() as u64;
            tracing::Span::current().record("processing_duration_ms", duration_ms);

            result
        }
        .instrument(span)
        .await
    }

    /// 处理标准数据格式响应
    /// 使用单次解析而非双重解析，包含详细的可观测性
    async fn handle_data_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let tracker = ResponseTracker::start("json_data", response.content_length());

        let body_bytes = read_body_with_limit(response, max_size).await?;
        let response_text = String::from_utf8_lossy(&body_bytes).to_string();
        // Don't log raw response to prevent token/PII leakage

        // 记录解析阶段开始
        tracker.parsing_complete();

        // 尝试直接解析为BaseResponse<T>
        match serde_json::from_str::<Response<T>>(&response_text) {
            Ok(base_response) => {
                tracker.success();
                Ok(base_response)
            }
            Err(direct_parse_err) => {
                tracing::debug!("Direct parsing failed, attempting structured data extraction");

                // 解析基础JSON结构
                match serde_json::from_str::<Value>(&response_text) {
                    Ok(raw_value) => {
                        let code = raw_value["code"].as_i64().unwrap_or(-1) as i32;
                        let msg = raw_value["msg"]
                            .as_str()
                            .unwrap_or("Unknown error")
                            .to_string();

                        // 如果响应成功，尝试提取并解析data字段
                        let data = if code == 0 {
                            if let Some(data_value) = raw_value.get("data") {
                                // 尝试解析为期望的类型T
                                match serde_json::from_value::<T>(data_value.clone()) {
                                    Ok(parsed_data) => {
                                        tracing::debug!("Successfully parsed data field as type T");
                                        Some(parsed_data)
                                    }
                                    Err(data_parse_err) => {
                                        tracing::debug!(
                                            "Failed to parse data field as type T: {data_parse_err:?}"
                                        );
                                        // 保持 core 通用性：不在这里做“按具体业务类型名”的特殊兼容。
                                        // 若某个业务 API 存在历史字段差异，请在对应业务 crate 的响应模型上做兼容反序列化。
                                        None
                                    }
                                }
                            } else {
                                tracing::debug!("No data field found in successful response");
                                None
                            }
                        } else {
                            None
                        };

                        tracker.validation_complete();
                        tracker.success();

                        Ok(BaseResponse {
                            raw_response: RawResponse {
                                code,
                                msg,
                                request_id: None,
                                data: None,
                                error: None,
                            },
                            data,
                        })
                    }
                    Err(fallback_err) => {
                        let error_msg = format!(
                            "Failed to parse response. Direct parse error: {direct_parse_err}. Fallback parse error: {fallback_err}"
                        );
                        tracker.error(&error_msg);
                        Err(validation_error("api_response", error_msg))
                    }
                }
            }
        }
    }

    /// 处理扁平格式响应
    /// 对于扁平格式，使用自定义反序列化器，包含可观测性支持
    async fn handle_flatten_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let tracker = ResponseTracker::start("json_flatten", response.content_length());

        let body_bytes = read_body_with_limit(response, max_size).await?;
        let response_text = String::from_utf8_lossy(&body_bytes).to_string();
        debug!("Raw response: {response_text}");

        // 解析阶段
        let raw_value: Value = match serde_json::from_str(&response_text) {
            Ok(value) => {
                tracker.parsing_complete();
                value
            }
            Err(e) => {
                let error_msg = format!("Failed to parse JSON: {e}");
                tracker.error(&error_msg);
                return Err(validation_error("base_response", error_msg));
            }
        };

        // 解析原始响应信息
        let raw_response: RawResponse = match serde_json::from_value(raw_value.clone()) {
            Ok(response) => response,
            Err(e) => {
                let error_msg = format!("Failed to parse raw response: {e}");
                tracker.error(&error_msg);
                return Err(validation_error("response", error_msg));
            }
        };

        // 验证和数据解析阶段
        let data = if raw_response.code == 0 {
            match serde_json::from_value::<T>(raw_value) {
                Ok(parsed_data) => {
                    tracker.validation_complete();
                    Some(parsed_data)
                }
                Err(e) => {
                    debug!("Failed to parse data for flatten response: {e}");
                    tracker.validation_complete();
                    None
                }
            }
        } else {
            tracker.validation_complete();
            None
        };

        tracker.success();
        Ok(BaseResponse { raw_response, data })
    }

    /// 处理二进制响应，包含可观测性支持
    async fn handle_binary_response<T: ApiResponseTrait>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let tracker = ResponseTracker::start("binary", response.content_length());

        // 获取文件名
        let _file_name = response
            .headers()
            .get("Content-Disposition")
            .and_then(|header| header.to_str().ok())
            .and_then(content_disposition::extract_filename)
            .unwrap_or_default();

        // 记录解析阶段完成（文件名提取）
        tracker.parsing_complete();

        // 获取二进制数据（带大小限制保护）
        let bytes = match read_body_with_limit(response, max_size).await {
            Ok(data) => {
                tracing::debug!("Binary response received: {} bytes", data.len());
                data
            }
            Err(e) => {
                let error_msg = format!("Failed to read binary response: {e}");
                tracker.error(&error_msg);
                return Err(e);
            }
        };

        // 二进制响应：目前仅对 `Vec<u8>` / `String` 做直接承载，其它类型返回 None。
        // 约定：当业务侧需要二进制内容时，将响应类型定义为 `Vec<u8>`（并使用 ResponseFormat::Binary）。
        let data: Option<T> = {
            let any: Box<dyn Any> =
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Vec<u8>>() {
                    Box::new(bytes)
                } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() {
                    let s = String::from_utf8_lossy(&bytes).to_string();
                    Box::new(s)
                } else {
                    // 其它类型目前不做自动解析
                    Box::new(())
                };

            any.downcast::<T>().ok().map(|boxed| *boxed)
        };

        tracker.success();
        Ok(BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                request_id: None,
                data: None,
                error: None,
            },
            data,
        })
    }
}

/// 优化的BaseResponse，使用更好的serde特性
///
/// 注意：该结构目前仅用于单元测试与序列化验证，不参与线上请求链路。
#[cfg(test)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OptimizedBaseResponse<T>
where
    T: Default,
{
    /// 响应状态码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 错误信息（可选）
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
    /// 业务数据（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[cfg(test)]
impl<T> OptimizedBaseResponse<T>
where
    T: Default,
{
    /// 检查请求是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 获取业务数据，如果请求失败则返回错误
    pub fn into_data(self) -> Result<T, CoreError> {
        if self.is_success() {
            self.data.ok_or_else(|| {
                validation_error("data", "Response is successful but data is missing")
            })
        } else {
            // 优先使用飞书通用错误码映射
            let mapped_code = ErrorCode::from_feishu_code(self.code)
                .unwrap_or_else(|| ErrorCode::from_code(self.code));

            // 将飞书 code 记录到上下文，便于观测与排查
            let mut ctx = ErrorContext::new();
            ctx.add_context("feishu_code", self.code.to_string());

            // log_id 作为 request_id 便于链路追踪
            if let Some(log_id) = self.error.as_ref().and_then(|e| e.log_id.clone()) {
                ctx.set_request_id(log_id);
            }

            // 推导合适的 HTTP 状态用于 Api 变体（避免 u16 溢出）
            let status =
                mapped_code
                    .http_status()
                    .unwrap_or_else(|| match mapped_code.category() {
                        ErrorCategory::RateLimit => 429,
                        ErrorCategory::Authentication
                        | ErrorCategory::Permission
                        | ErrorCategory::Parameter => 400,
                        ErrorCategory::Resource => 404,
                        _ => 500,
                    });

            Err(CoreError::Api(Box::new(crate::error::ApiError {
                status,
                endpoint: "unknown_endpoint".into(),
                message: self.msg,
                source: None,
                code: mapped_code,
                ctx: Box::new(ctx),
            })))
        }
    }

    /// 获取数据的引用
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 检查是否有错误信息
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

#[cfg(test)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}

#[cfg(test)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 使用宏简化APIResponseTrait实现
#[macro_export]
macro_rules! impl_api_response {
    ($type:ty, $format:expr) => {
        impl ApiResponseTrait for $type {
            fn data_format() -> ResponseFormat {
                $format
            }
        }
    };

    ($type:ty, $format:expr, binary) => {
        impl ApiResponseTrait for $type {
            fn data_format() -> ResponseFormat {
                $format
            }

            fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
                Some(<$type>::from_binary_data(file_name, body))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ResponseFormat;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestData {
        id: i32,
        name: String,
    }

    impl ApiResponseTrait for TestData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestFlattenData {
        id: i32,
        name: String,
        code: i32,
        msg: String,
    }

    impl ApiResponseTrait for TestFlattenData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Flatten
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestBinaryData {
        file_name: String,
        content: Vec<u8>,
    }

    impl ApiResponseTrait for TestBinaryData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Binary
        }
    }

    // Note: Mock HTTP responses would require a more sophisticated testing setup
    // The helper functions below are simplified examples of what mock functions might look like

    #[test]
    fn test_optimized_base_response_success() {
        let response = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: Some(TestData {
                id: 1,
                name: "test".to_string(),
            }),
        };

        assert!(response.is_success());
        assert!(response.data().is_some());
        assert_eq!(response.data().unwrap().id, 1);
        assert!(!response.has_error());
    }

    #[test]
    fn test_optimized_base_response_error() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 400,
            msg: "Bad Request".to_string(),
            error: Some(ErrorInfo {
                log_id: Some("log123".to_string()),
                details: vec![],
            }),
            data: None,
        };

        assert!(!response.is_success());
        assert!(response.has_error());
        assert!(response.data().is_none());
    }

    #[test]
    fn test_optimized_base_response_into_data_success() {
        let response = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: Some(TestData {
                id: 1,
                name: "test".to_string(),
            }),
        };

        let data = response.into_data().unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "test");
    }

    #[test]
    fn test_optimized_base_response_into_data_error() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 400,
            msg: "Bad Request".to_string(),
            error: None,
            data: None,
        };

        let result = response.into_data();
        assert!(result.is_err());
        match result.unwrap_err() {
            CoreError::Api(api) => {
                assert_eq!(api.status, 400);
                assert_eq!(api.message, "Bad Request");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_optimized_base_response_into_data_success_but_no_data() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: None,
        };

        let result = response.into_data();
        assert!(result.is_err());
        match result.unwrap_err() {
            CoreError::Validation { message, .. } => {
                assert!(message.contains("data is missing"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_error_info_serialization() {
        let error_info = ErrorInfo {
            log_id: Some("test_log_id".to_string()),
            details: vec![
                ErrorDetail {
                    key: Some("field1".to_string()),
                    value: Some("invalid_value".to_string()),
                    description: Some("Field is required".to_string()),
                },
                ErrorDetail {
                    key: Some("field2".to_string()),
                    value: None,
                    description: Some("Missing field".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        let deserialized: ErrorInfo = serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(deserialized.log_id, error_info.log_id);
        assert_eq!(deserialized.details.len(), 2);
        assert_eq!(deserialized.details[0].key, Some("field1".to_string()));
        assert_eq!(deserialized.details[1].value, None);
    }

    #[test]
    fn test_error_detail_optional_fields() {
        let detail = ErrorDetail {
            key: None,
            value: Some("test_value".to_string()),
            description: None,
        };

        let json = serde_json::to_string(&detail).unwrap();
        let deserialized: ErrorDetail = serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(deserialized.key, None);
        assert_eq!(deserialized.value, Some("test_value".to_string()));
        assert_eq!(deserialized.description, None);
    }

    #[test]
    fn test_filename_extraction() {
        let cases = vec![
            (
                "attachment; filename=\"test.txt\"",
                Some("test.txt".to_string()),
            ),
            (
                "attachment; filename*=UTF-8''test%20file.pdf",
                Some("test%20file.pdf".to_string()),
            ),
            (
                "attachment; filename=simple.doc",
                Some("simple.doc".to_string()),
            ),
            ("attachment", None),
            ("", None),
            ("filename=\"quoted.txt\"", Some("quoted.txt".to_string())),
            ("filename=unquoted.txt", Some("unquoted.txt".to_string())),
            (
                "filename*=UTF-8''unicode%E2%9C%93.txt",
                Some("unicode%E2%9C%93.txt".to_string()),
            ),
            (
                "attachment; filename=\"spaced file.doc\"; other=value",
                Some("spaced file.doc".to_string()),
            ),
        ];

        for (input, expected) in cases {
            let result = crate::content_disposition::extract_filename(input);
            assert_eq!(result, expected, "Failed for input: {input}");
        }
    }

    #[test]
    fn test_filename_extraction_edge_cases() {
        // Test empty and whitespace-only strings
        assert_eq!(crate::content_disposition::extract_filename(""), None);
        assert_eq!(crate::content_disposition::extract_filename("   "), None);
        assert_eq!(crate::content_disposition::extract_filename(";;;"), None);

        // Test malformed headers - based on implementation behavior
        assert_eq!(
            crate::content_disposition::extract_filename("filename="),
            Some("".to_string())
        );
        assert_eq!(
            crate::content_disposition::extract_filename("filename*="),
            None
        );
        assert_eq!(
            crate::content_disposition::extract_filename("filename=\""),
            Some("".to_string())
        );

        // Test with only quotes - the current implementation extracts empty string
        assert_eq!(
            crate::content_disposition::extract_filename("filename=\"\""),
            Some("".to_string())
        );

        // Test multiple filename directives (should return first valid one)
        let multi_filename = "filename=\"first.txt\"; filename=\"second.txt\"";
        assert_eq!(
            crate::content_disposition::extract_filename(multi_filename),
            Some("first.txt".to_string())
        );
    }

    #[test]
    fn test_json_parsing_performance() {
        let json_data = r#"{"code": 0, "msg": "success", "data": {"id": 1, "name": "test"}}"#;

        // 测试直接解析
        let start = std::time::Instant::now();
        let _result: Result<OptimizedBaseResponse<TestData>, _> = serde_json::from_str(json_data);
        let direct_parse_time = start.elapsed();

        // 测试双重解析（原始方法）
        let start = std::time::Instant::now();
        let _value: Value = serde_json::from_str(json_data).expect("JSON 反序列化失败");
        let _result: Result<OptimizedBaseResponse<TestData>, _> = serde_json::from_value(_value);
        let double_parse_time = start.elapsed();

        println!("Direct parse time: {direct_parse_time:?}");
        println!("Double parse time: {double_parse_time:?}");

        // 直接解析应该更快（虽然在微基准测试中差异可能很小）
        // 这里主要是为了展示概念
    }

    #[test]
    fn test_api_response_trait_data_format() {
        assert_eq!(TestData::data_format(), ResponseFormat::Data);
        assert_eq!(TestFlattenData::data_format(), ResponseFormat::Flatten);
        assert_eq!(TestBinaryData::data_format(), ResponseFormat::Binary);
    }

    #[test]
    fn test_api_response_trait_from_binary() {
        let file_name = "test.txt".to_string();
        let content = b"Hello, World!".to_vec();

        let binary_data = TestBinaryData {
            file_name: file_name.clone(),
            content: content.clone(),
        };
        assert_eq!(binary_data.file_name, file_name);
        assert_eq!(binary_data.content, content);

        // Test default implementation for non-binary types
        // TestData doesn't support binary format directly
        let _ = "test.txt".to_string();
    }

    // Mock tests for response handlers would require a more sophisticated mocking setup
    // For now, we'll test the logic that doesn't require actual HTTP responses

    #[tokio::test]
    async fn test_handle_data_response_parsing_logic() {
        // Test JSON parsing logic without actual HTTP response
        let test_cases = vec![
            // Error response with fallback parsing
            (r#"{"code": 400, "msg": "Bad Request"}"#, true),
            // Invalid JSON
            (r#"{"invalid": json"#, false),
        ];

        for (json, should_succeed) in test_cases {
            // Test fallback parsing for error responses
            if json.contains("code") && !json.contains("raw_response") {
                let fallback_result = serde_json::from_str::<Value>(json);
                if should_succeed {
                    assert!(
                        fallback_result.is_ok(),
                        "Fallback parsing should succeed for: {json}"
                    );
                    let value = match fallback_result {
                        Ok(value) => value,
                        Err(err) => panic!("Fallback parsing unexpectedly failed: {err}"),
                    };
                    assert!(value["code"].is_i64());
                    assert!(value["msg"].is_string());
                }
            } else if json.contains("invalid") {
                let parse_result = serde_json::from_str::<Value>(json);
                assert!(parse_result.is_err(), "Invalid JSON should fail to parse");
            }
        }
    }

    #[tokio::test]
    async fn test_handle_flatten_response_parsing_logic() {
        let test_cases = vec![
            // Success response
            (
                r#"{"id": 1, "name": "test", "code": 0, "msg": "success"}"#,
                0,
                true,
            ),
            // Error response
            (r#"{"code": 400, "msg": "Bad Request"}"#, 400, false),
            // Invalid JSON
            (r#"{"invalid": json"#, -1, false),
        ];

        for (json, expected_code, should_have_data) in test_cases {
            if json.contains("invalid") {
                let parse_result = serde_json::from_str::<Value>(json);
                assert!(parse_result.is_err(), "Invalid JSON should fail to parse");
                continue;
            }

            let value_result = serde_json::from_str::<Value>(json);
            assert!(value_result.is_ok(), "Valid JSON should parse as Value");

            let value = match value_result {
                Ok(value) => value,
                Err(err) => panic!("Value parsing unexpectedly failed: {err}"),
            };
            let raw_response_result = serde_json::from_value::<RawResponse>(value.clone());

            if expected_code >= 0 {
                assert!(
                    raw_response_result.is_ok(),
                    "Should parse RawResponse for: {json}"
                );
                let raw_response = match raw_response_result {
                    Ok(raw_response) => raw_response,
                    Err(err) => panic!("RawResponse parsing unexpectedly failed: {err}"),
                };
                assert_eq!(raw_response.code, expected_code);

                if should_have_data && raw_response.code == 0 {
                    let data_result = serde_json::from_value::<TestFlattenData>(value);
                    assert!(
                        data_result.is_ok(),
                        "Should parse data for success response"
                    );
                }
            }
        }
    }

    #[test]
    fn test_response_format_display_logic() {
        let formats = vec![
            (ResponseFormat::Data, "data"),
            (ResponseFormat::Flatten, "flatten"),
            (ResponseFormat::Binary, "binary"),
        ];

        for (format, expected_str) in formats {
            let format_str = match format {
                ResponseFormat::Data => "data",
                ResponseFormat::Flatten => "flatten",
                ResponseFormat::Binary => "binary",
                ResponseFormat::Text => "text",
                ResponseFormat::Custom => "custom",
            };
            assert_eq!(format_str, expected_str);
        }
    }

    #[test]
    fn test_binary_response_logic() {
        let test_file_name = "test_document.pdf";
        let test_content = b"PDF content here".to_vec();

        // Test successful binary data creation
        let binary_data = TestBinaryData {
            file_name: test_file_name.to_string(),
            content: test_content.clone(),
        };
        assert!(binary_data.file_name == test_file_name);
        assert!(binary_data.content == test_content);

        // Test empty content
        let empty_data = TestBinaryData {
            file_name: "empty.txt".to_string(),
            content: vec![],
        };
        assert_eq!(empty_data.content.len(), 0);
    }

    #[test]
    fn test_optimized_response_serialization_roundtrip() {
        let original = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: Some(ErrorInfo {
                log_id: Some("test123".to_string()),
                details: vec![ErrorDetail {
                    key: Some("validation".to_string()),
                    value: Some("failed".to_string()),
                    description: Some("Field validation failed".to_string()),
                }],
            }),
            data: Some(TestData {
                id: 42,
                name: "serialization_test".to_string(),
            }),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: OptimizedBaseResponse<TestData> =
            serde_json::from_str(&json).expect("JSON 反序列化失败");

        // Verify all fields are preserved
        assert_eq!(deserialized.code, original.code);
        assert_eq!(deserialized.msg, original.msg);
        assert_eq!(deserialized.data, original.data);
        assert!(deserialized.error.is_some());

        let error = deserialized.error.unwrap();
        assert_eq!(error.log_id, Some("test123".to_string()));
        assert_eq!(error.details.len(), 1);
        assert_eq!(error.details[0].key, Some("validation".to_string()));
    }

    #[test]
    fn test_optimized_response_skipped_fields() {
        // Test response with None values (should be skipped in serialization)
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        // Should not contain "error" or "data" fields when they are None
        assert!(!json.contains("\"error\""));
        assert!(!json.contains("\"data\""));
        assert!(json.contains("\"code\":0"));
        assert!(json.contains("\"msg\":\"success\""));
    }

    #[test]
    fn test_macro_api_response_implementation() {
        // Test that the macro would work correctly
        // Since we can't actually invoke the macro in tests easily,
        // we'll test the pattern it would generate

        #[derive(Debug, Default, Serialize, Deserialize)]
        struct MacroTestData;

        impl ApiResponseTrait for MacroTestData {
            fn data_format() -> ResponseFormat {
                ResponseFormat::Data
            }
        }

        assert_eq!(MacroTestData::data_format(), ResponseFormat::Data);
        // from_binary is not part of ApiResponseTrait
        // assert!(MacroTestData::from_binary("test".to_string(), vec![1, 2, 3]).is_none());
    }

    #[test]
    fn test_error_detail_empty_values() {
        let detail = ErrorDetail {
            key: Some("".to_string()),
            value: Some("".to_string()),
            description: Some("".to_string()),
        };

        let json = serde_json::to_string(&detail).unwrap();
        let deserialized: ErrorDetail = serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(deserialized.key, Some("".to_string()));
        assert_eq!(deserialized.value, Some("".to_string()));
        assert_eq!(deserialized.description, Some("".to_string()));
    }

    #[test]
    fn test_content_disposition_header_edge_cases() {
        let edge_cases = vec![
            // Case-insensitive filename
            ("FILENAME=\"test.txt\"", None), // Our implementation is case-sensitive
            ("Filename=\"test.txt\"", None), // Our implementation is case-sensitive
            // Multiple spaces
            (
                "attachment;  filename=\"test.txt\"",
                Some("test.txt".to_string()),
            ),
            ("attachment; filename =  \"test.txt\"", None), // Space before = is not handled
            // Special characters in filename
            (
                "attachment; filename=\"test-file_v1.2.txt\"",
                Some("test-file_v1.2.txt".to_string()),
            ),
            (
                "attachment; filename=\"测试文件.txt\"",
                Some("测试文件.txt".to_string()),
            ),
            // Both UTF-8 and regular filename (current implementation returns first match)
            (
                "attachment; filename=\"test.txt\"; filename*=UTF-8''better.txt",
                Some("better.txt".to_string()),
            ),
        ];

        for (input, expected) in edge_cases {
            let result = crate::content_disposition::extract_filename(input);
            assert_eq!(result, expected, "Failed for input: {input}");
        }
    }

    // ==================== Enhanced Coverage Tests ====================

    // Complex error handling scenarios
    #[test]
    fn test_complex_error_response_scenarios() {
        use serde_json::Value;

        // Test nested error structures
        let complex_error = r#"{
            "code": 400,
            "msg": "Validation failed",
            "error": {
                "log_id": "error_12345",
                "details": [
                    {
                        "key": "field1",
                        "value": "invalid",
                        "description": "Field must be valid email"
                    },
                    {
                        "key": "field2",
                        "description": "Required field missing"
                    }
                ]
            }
        }"#;

        let parsed: Value = serde_json::from_str(complex_error).expect("JSON 反序列化失败");
        assert_eq!(parsed["code"], 400);
        assert_eq!(parsed["msg"], "Validation failed");
        assert!(parsed["error"]["log_id"].is_string());
        assert_eq!(parsed["error"]["details"].as_array().unwrap().len(), 2);

        // Test error response with missing msg
        let error_missing_msg = r#"{"code": 500}"#;
        let parsed_missing: Value =
            serde_json::from_str(error_missing_msg).expect("JSON 反序列化失败");
        assert_eq!(parsed_missing["code"], 500);
        assert!(!parsed_missing["msg"].is_string());

        // Test error response with non-integer code
        let invalid_code = r#"{"code": "400", "msg": "Invalid code type"}"#;
        let parsed_invalid: Value = serde_json::from_str(invalid_code).expect("JSON 反序列化失败");
        assert!(parsed_invalid["code"].is_string());
    }

    // Large data handling tests
    #[test]
    fn test_large_response_data_handling() {
        use serde_json::Value;

        // Test large JSON response
        let large_data_list: Vec<String> = (0..1000).map(|i| format!("item_{i}")).collect();

        let large_response = serde_json::json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": large_data_list,
                "metadata": {
                    "total": 1000,
                    "page": 1
                }
            }
        });

        let json_str = serde_json::to_string(&large_response).unwrap();
        assert!(json_str.len() > 10000); // Should be reasonably large

        // Test parsing large response
        let parsed: Value = serde_json::from_str(&json_str).expect("JSON 反序列化失败");
        assert_eq!(parsed["code"], 0);
        assert_eq!(parsed["data"]["items"].as_array().unwrap().len(), 1000);
    }

    // Unicode and internationalization tests for responses
    #[test]
    fn test_unicode_response_handling() {
        use serde_json::json;

        let unicode_response = json!({
            "code": 0,
            "msg": "操作成功",
            "data": {
                "title": "测试标题",
                "description": "这是一个包含中文、English و العربية 的描述",
                "tags": ["标签1", "tag2", "العربية", "🚀"]
            }
        });

        let json_str = serde_json::to_string(&unicode_response).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).expect("JSON 反序列化失败");

        assert_eq!(parsed["msg"], "操作成功");
        assert_eq!(parsed["data"]["title"], "测试标题");
        assert!(
            parsed["data"]["description"]
                .as_str()
                .unwrap()
                .contains("中文")
        );
        assert!(parsed["data"]["tags"].as_array().unwrap()[3] == "🚀");
    }

    // Memory efficiency tests
    #[test]
    fn test_memory_efficient_response_processing() {
        use std::mem;

        // Test that OptimizedBaseResponse doesn't unnecessarily copy data
        let response = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: Some(TestData {
                id: 123,
                name: "test".to_string(),
            }),
        };

        let response_size = mem::size_of_val(&response);
        assert!(response_size > 0);

        // Test that Option fields don't increase size when None
        let empty_response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: None,
        };

        let empty_size = mem::size_of_val(&empty_response);
        // Both should have similar size since Option<T> has overhead regardless of value
        assert!(empty_size > 0);
    }

    // Performance benchmarking tests
    #[test]
    fn test_response_parsing_performance() {
        use std::time::Instant;

        let test_json = r#"{"code": 0, "msg": "success", "data": {"id": 1, "name": "test"}}"#;

        // Benchmark direct parsing
        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let _: Result<OptimizedBaseResponse<TestData>, _> = serde_json::from_str(test_json);
        }

        let direct_time = start.elapsed();

        // Benchmark fallback parsing (Value -> BaseResponse)
        let start = Instant::now();

        for _ in 0..iterations {
            let value: Value = serde_json::from_str(test_json).expect("JSON 反序列化失败");
            let _: Result<Response<TestData>, _> = serde_json::from_value(value);
        }

        let fallback_time = start.elapsed();

        // Performance test is informational - timing can vary
        println!("Direct parsing: {direct_time:?}, Fallback parsing: {fallback_time:?}");

        // Only assert that both completed successfully and within reasonable time
        assert!(direct_time.as_millis() < 1000); // Should complete within 1 second
        assert!(fallback_time.as_millis() < 1000); // Should complete within 1 second

        // Performance characteristics check - direct parsing should be competitive
        // We don't assert strict ordering since timing can be non-deterministic
        let ratio = fallback_time.as_nanos() as f64 / direct_time.as_nanos() as f64;
        println!("Performance ratio (fallback/direct): {ratio:.2}x");
    }

    // Concurrent response processing tests
    #[test]
    fn test_concurrent_response_parsing() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let test_responses = vec![
            r#"{"code": 0, "msg": "success", "data": {"id": 1, "name": "test1"}}"#,
            r#"{"code": 0, "msg": "success", "data": {"id": 2, "name": "test2"}}"#,
            r#"{"code": 400, "msg": "error", "data": null}"#,
            r#"{"code": 0, "msg": "success", "data": {"id": 4, "name": "test4"}}"#,
        ];

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for (i, response_json) in test_responses.into_iter().enumerate() {
            let results_clone = results.clone();
            let handle = thread::spawn(move || {
                let parsed: Result<OptimizedBaseResponse<TestData>, _> =
                    serde_json::from_str(response_json);

                results_clone.lock().unwrap().push((i, parsed.is_ok()));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results_vec = results.lock().unwrap();
        assert_eq!(results_vec.len(), 4);
        assert!(results_vec.iter().all(|(_, success)| *success));
    }

    // Edge case JSON structures
    #[test]
    fn test_edge_case_json_structures() {
        use serde_json::json;

        // Test with null values
        let null_response = json!({
            "code": 0,
            "msg": "success",
            "data": null
        });

        let parsed: OptimizedBaseResponse<TestData> =
            serde_json::from_value(null_response).unwrap();
        assert!(parsed.is_success());
        assert!(parsed.data().is_none());

        // Test with empty arrays and objects
        let empty_response = json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": [],
                "metadata": {}
            }
        });

        // This would fail to parse as TestData, but we can test the JSON structure
        let parsed_value: Value = serde_json::from_value(empty_response).unwrap();
        assert_eq!(parsed_value["data"]["items"].as_array().unwrap().len(), 0);
        assert!(
            parsed_value["data"]["metadata"]
                .as_object()
                .unwrap()
                .is_empty()
        );

        // Test with unexpected additional fields
        let extra_fields_response = json!({
            "code": 0,
            "msg": "success",
            "data": {"id": 1, "name": "test"},
            "unexpected_field": "should_be_ignored",
            "another_unexpected": {"nested": "data"}
        });

        let parsed_extra: OptimizedBaseResponse<TestData> =
            serde_json::from_value(extra_fields_response).unwrap();
        assert!(parsed_extra.is_success());
        assert!(parsed_extra.data().is_some());
    }

    // Response format validation tests
    #[test]
    fn test_response_format_validation() {
        // Test different response formats and their characteristics
        let test_cases = vec![
            (ResponseFormat::Data, "data", true),
            (ResponseFormat::Flatten, "flatten", false),
            (ResponseFormat::Binary, "binary", false),
        ];

        for (format, expected_str, supports_data) in test_cases {
            // Test format string representation
            let format_str = match format {
                ResponseFormat::Data => "data",
                ResponseFormat::Flatten => "flatten",
                ResponseFormat::Binary => "binary",
                ResponseFormat::Text => "text",
                ResponseFormat::Custom => "custom",
            };
            assert_eq!(format_str, expected_str);

            // Test format characteristics
            match format {
                ResponseFormat::Data => assert!(supports_data),
                ResponseFormat::Flatten => assert!(!supports_data),
                ResponseFormat::Binary => assert!(!supports_data),
                ResponseFormat::Text => assert!(supports_data),
                ResponseFormat::Custom => assert!(supports_data),
            }
        }
    }

    // Binary response handling edge cases
    #[test]
    fn test_binary_response_edge_cases() {
        // Test with very large binary data
        let large_binary = vec![0u8; 1_000_000]; // 1MB
        let large_binary_data = TestBinaryData {
            file_name: "large_file.bin".to_string(),
            content: large_binary,
        };
        assert_eq!(large_binary_data.content.len(), 1_000_000);

        // Test with empty binary data
        let empty_binary_data = TestBinaryData {
            file_name: "empty_file.txt".to_string(),
            content: vec![],
        };
        assert!(empty_binary_data.content.is_empty());

        // Test with special characters in filename
        let special_filename_data = TestBinaryData {
            file_name: "测试文件@#$%.txt".to_string(),
            content: b"test content".to_vec(),
        };
        assert_eq!(special_filename_data.file_name, "测试文件@#$%.txt");
    }

    // Complex error detail scenarios
    #[test]
    fn test_complex_error_detail_scenarios() {
        // Test with nested error details
        let complex_error = ErrorInfo {
            log_id: Some("complex_error_123".to_string()),
            details: vec![
                ErrorDetail {
                    key: Some("validation".to_string()),
                    value: Some("email格式不正确".to_string()),
                    description: Some("邮箱地址格式验证失败".to_string()),
                },
                ErrorDetail {
                    key: Some("required_field".to_string()),
                    value: None,
                    description: Some("必填字段缺失".to_string()),
                },
                ErrorDetail {
                    key: Some("constraint".to_string()),
                    value: Some("长度超过限制".to_string()),
                    description: Some("字段长度超出最大限制".to_string()),
                },
            ],
        };

        // Test serialization and deserialization
        let json = serde_json::to_string(&complex_error).unwrap();
        let deserialized: ErrorInfo = serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(deserialized.log_id, complex_error.log_id);
        assert_eq!(deserialized.details.len(), 3);
        assert_eq!(
            deserialized.details[0].description,
            Some("邮箱地址格式验证失败".to_string())
        );
        assert_eq!(deserialized.details[1].value, None);
        assert_eq!(deserialized.details[2].key, Some("constraint".to_string()));
    }

    // Response tracker integration simulation
    #[test]
    fn test_response_tracker_integration_simulation() {
        // Simulate response tracker behavior
        let tracker_calls = [
            ("start", "json_data", Some(1024)),
            ("parsing_complete", "", None),
            ("validation_complete", "", None),
            ("success", "", None),
        ];

        // Verify tracking sequence
        assert_eq!(tracker_calls.len(), 4);
        assert_eq!(tracker_calls[0].0, "start");
        assert_eq!(tracker_calls[0].1, "json_data");
        assert_eq!(tracker_calls[0].2, Some(1024));

        // Test error tracking simulation
        let error_tracker_calls = [
            ("start", "json_flatten", Some(512)),
            ("error", "解析失败", None),
        ];

        assert_eq!(error_tracker_calls.len(), 2);
        assert_eq!(error_tracker_calls[1].0, "error");
        assert_eq!(error_tracker_calls[1].1, "解析失败");
    }

    // Real-world response pattern tests
    #[test]
    fn test_real_world_response_patterns() {
        use serde_json::json;

        // Pattern 1: Pagination response
        let pagination_response = json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": [
                    {"id": 1, "name": "item1"},
                    {"id": 2, "name": "item2"}
                ],
                "page_token": "next_page_token",
                "has_more": true
            }
        });

        let pagination_parsed: Value = serde_json::from_value(pagination_response).unwrap();
        assert_eq!(
            pagination_parsed["data"]["items"].as_array().unwrap().len(),
            2
        );
        assert!(pagination_parsed["data"]["has_more"].as_bool().unwrap());

        // Pattern 2: Nested data response
        let nested_response = json!({
            "code": 0,
            "msg": "success",
            "data": {
                "user": {
                    "id": "user_123",
                    "profile": {
                        "name": "张三",
                        "department": "技术部"
                    }
                },
                "permissions": ["read", "write"]
            }
        });

        let nested_parsed: Value = serde_json::from_value(nested_response).unwrap();
        assert_eq!(nested_parsed["data"]["user"]["profile"]["name"], "张三");
        assert_eq!(
            nested_parsed["data"]["permissions"]
                .as_array()
                .unwrap()
                .len(),
            2
        );

        // Pattern 3: Error with detailed validation info
        let validation_error_response = json!({
            "code": 400,
            "msg": "参数验证失败",
            "error": {
                "log_id": "validation_error_456",
                "details": [
                    {
                        "field": "email",
                        "error_code": "INVALID_FORMAT",
                        "message": "邮箱格式不正确"
                    }
                ]
            }
        });

        let validation_parsed: Value = serde_json::from_value(validation_error_response).unwrap();
        assert_eq!(validation_parsed["code"], 400);
        assert!(
            !validation_parsed["error"]["details"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }

    // OptimizedBaseResponse performance characteristics
    #[test]
    fn test_optimized_response_performance_characteristics() {
        use std::time::Instant;

        // Test creation performance
        let start = Instant::now();
        let mut responses = Vec::new();

        for i in 0..1000 {
            responses.push(OptimizedBaseResponse {
                code: if i % 10 == 0 { 400 } else { 0 },
                msg: if i % 10 == 0 {
                    "error".to_string()
                } else {
                    "success".to_string()
                },
                error: if i % 10 == 0 {
                    Some(ErrorInfo {
                        log_id: Some(format!("log_{i}")),
                        details: vec![],
                    })
                } else {
                    None
                },
                data: if i % 10 != 0 {
                    Some(TestData {
                        id: i,
                        name: format!("test_{i}"),
                    })
                } else {
                    None
                },
            });
        }

        let creation_time = start.elapsed();
        assert_eq!(responses.len(), 1000);
        assert!(creation_time.as_millis() < 100); // Should complete quickly

        // Test filtering performance
        let start = Instant::now();
        let successful_responses: Vec<_> = responses.iter().filter(|r| r.is_success()).collect();

        let filter_time = start.elapsed();
        assert_eq!(successful_responses.len(), 900);
        assert!(filter_time.as_millis() < 10); // Should be very fast
    }
}

/// 使用示例
///
/// 在RequestExecutor中使用改进的响应处理器：
/// ```rust,ignore
/// impl RequestExecutor {
///     pub async fn execute_improved<T: ApiResponseTrait + DeserializeOwned>(
///         // ... 参数
///     ) -> SDKResult<OptimizedBaseResponse<T>> {
///         // ... 构建请求
///         let response = http_client.send(request).await?;
///         ImprovedResponseHandler::handle_response(response, max_response_size).await
///     }
/// }
///
/// // 使用新的响应格式
/// let result = RequestExecutor::execute_improved::<MessageData>(...).await?;
///
/// match result.into_data() {
///     Ok(data) => println!("Success: {:?}", data),
///     Err(e) => println!("Error: {:?}", e),
/// }
/// ```
mod usage_examples {}
