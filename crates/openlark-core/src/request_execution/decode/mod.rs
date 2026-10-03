//! 类型化响应解码（`ResponseDecoder`）。
//!
//! Data 走单一 Value 路径；Binary/Text/Custom 共用 raw-body 管线。
//! 缺 `data` 时只认 [`ApiResponseTrait::empty_success`]，不做 `{}` 探测。
//! raw 路径先识别 HTTP / 业务错误 envelope，再调用 `from_*`（Codex #451 review）。

use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use tracing::debug;
use tracing::{Instrument, info_span};

use crate::{
    SDKResult,
    api::{ApiResponseTrait, BaseResponse, ErrorInfo, RawResponse, Response, ResponseFormat},
    constants::{CUSTOM_REQUEST_ID, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID},
    content_disposition,
    error::{network_error, validation_error},
    observability::ResponseTracker,
};

/// 读取响应体，带大小限制保护。
async fn read_body_with_limit(
    response: reqwest::Response,
    max_size: u64,
) -> Result<Vec<u8>, crate::error::CoreError> {
    if let Some(content_length) = response.content_length()
        && content_length > max_size
    {
        return Err(crate::error::CoreError::response_too_large(
            max_size,
            content_length,
        ));
    }

    let mut total_size: u64 = 0;
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| network_error(e.to_string()))?;
        total_size += chunk.len() as u64;
        if total_size > max_size {
            return Err(crate::error::CoreError::response_too_large(
                max_size, total_size,
            ));
        }
        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

/// 从响应头提取请求标识（多键兼容）。
fn request_id_from_headers(headers: &reqwest::header::HeaderMap) -> Option<String> {
    const CANDIDATES: &[&str] = &[
        HTTP_HEADER_KEY_REQUEST_ID,
        HTTP_HEADER_REQUEST_ID,
        CUSTOM_REQUEST_ID,
        "x-tt-logid",
        "X-Tt-Logid",
    ];
    for key in CANDIDATES {
        if let Some(v) = headers.get(*key).and_then(|h| h.to_str().ok()) {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn request_id_from_json(value: &Value) -> Option<String> {
    value
        .get("request_id")
        .or_else(|| value.get("log_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn error_info_from_json(value: &Value) -> Option<ErrorInfo> {
    value
        .get("error")
        .cloned()
        .and_then(|e| serde_json::from_value(e).ok())
}

fn raw_from_envelope(
    value: &Value,
    header_request_id: Option<String>,
    fallback_code: i32,
    fallback_msg: impl Into<String>,
) -> RawResponse {
    let code = value
        .get("code")
        .and_then(|c| c.as_i64())
        .map(|c| c as i32)
        .unwrap_or(fallback_code);
    let msg = value
        .get("msg")
        .and_then(|m| m.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| fallback_msg.into());
    let request_id = request_id_from_json(value).or(header_request_id);
    let error = error_info_from_json(value).or_else(|| {
        if code != 0 {
            Some(ErrorInfo {
                code,
                message: msg.clone(),
                details: None,
            })
        } else {
            None
        }
    });
    RawResponse {
        code,
        msg,
        request_id,
        data: value.get("data").cloned().filter(|d| !d.is_null()),
        error,
    }
}

fn success_raw_response(request_id: Option<String>) -> RawResponse {
    RawResponse {
        code: 0,
        msg: "success".to_string(),
        request_id,
        data: None,
        error: None,
    }
}

/// 成功响应缺 `data` 字段：仅用类型声明的 [`ApiResponseTrait::empty_success`]。
fn resolve_missing_data_field<T: ApiResponseTrait>() -> Result<Option<T>, String> {
    if let Some(empty) = T::empty_success() {
        return Ok(Some(empty));
    }
    if T::requires_payload() {
        Err("成功响应缺少必需的 data 字段".to_string())
    } else {
        Ok(None)
    }
}

fn fail_payload(tracker: ResponseTracker, error_msg: String) -> crate::error::CoreError {
    tracker.error(&error_msg);
    validation_error("api_response_data", error_msg)
}

fn require_decoded_payload<T: ApiResponseTrait>(
    decoded: Option<T>,
    missing_msg: impl Into<String>,
) -> Result<Option<T>, String> {
    match decoded {
        Some(parsed) => Ok(Some(parsed)),
        None if T::requires_payload() => Err(missing_msg.into()),
        None => Ok(None),
    }
}

/// 若 body 为带 `code` 的 JSON 业务 envelope 且 `code != 0`，返回解析后的 RawResponse。
fn try_api_error_envelope(body: &[u8], header_request_id: Option<String>) -> Option<RawResponse> {
    let value: Value = serde_json::from_slice(body).ok()?;
    let code = value.get("code")?.as_i64()? as i32;
    if code == 0 {
        return None;
    }
    Some(raw_from_envelope(
        &value,
        header_request_id,
        code,
        "api error",
    ))
}

/// 按 `ResponseFormat` 做类型化响应解码。
pub struct ResponseDecoder;

impl ResponseDecoder {
    /// 处理响应：按 `T::data_format()` 分派。
    pub async fn handle_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let format = T::data_format();
        let span = info_span!(
            "response_handling",
            format = format.as_label(),
            status_code = response.status().as_u16(),
            content_length = tracing::field::Empty,
            processing_duration_ms = tracing::field::Empty,
        );

        async move {
            let start_time = std::time::Instant::now();
            if let Some(length) = response.content_length() {
                tracing::Span::current().record("content_length", length);
            }

            let result = match format {
                ResponseFormat::Data => Self::handle_data_response(response, max_size).await,
                ResponseFormat::Flatten => Self::handle_flatten_response(response, max_size).await,
                ResponseFormat::Binary | ResponseFormat::Text | ResponseFormat::Custom => {
                    Self::handle_raw_format(response, max_size, format).await
                }
            };

            let duration_ms = start_time.elapsed().as_millis() as u64;
            tracing::Span::current().record("processing_duration_ms", duration_ms);
            result
        }
        .instrument(span)
        .await
    }

    /// Data：单一 Value 路径。
    async fn handle_data_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let tracker = ResponseTracker::start("json_data", response.content_length());
        let header_request_id = request_id_from_headers(response.headers());
        let body_bytes = read_body_with_limit(response, max_size).await?;
        tracker.parsing_complete();

        let raw_value: Value = match serde_json::from_slice(&body_bytes) {
            Ok(v) => v,
            Err(e) => {
                let error_msg = format!("Failed to parse response JSON: {e}");
                tracker.error(&error_msg);
                return Err(validation_error("api_response", error_msg));
            }
        };

        let raw_response = raw_from_envelope(&raw_value, header_request_id, -1, "Unknown error");
        let code = raw_response.code;

        let data = if code == 0 {
            match raw_value.get("data") {
                Some(data_value) if !data_value.is_null() => {
                    match serde_json::from_value::<T>(data_value.clone()) {
                        Ok(parsed) => Some(parsed),
                        Err(e) if T::requires_payload() => {
                            return Err(fail_payload(
                                tracker,
                                format!("成功响应 data 字段无法解析为期望类型: {e}"),
                            ));
                        }
                        Err(e) => {
                            debug!("optional data parse failed: {e}");
                            None
                        }
                    }
                }
                _ => match resolve_missing_data_field::<T>() {
                    Ok(data) => data,
                    Err(msg) => return Err(fail_payload(tracker, msg)),
                },
            }
        } else {
            None
        };

        tracker.validation_complete();
        tracker.success();
        Ok(BaseResponse { raw_response, data })
    }

    /// Flatten：整包 JSON 同时作为 raw 与业务类型。
    async fn handle_flatten_response<T: ApiResponseTrait + for<'de> Deserialize<'de>>(
        response: reqwest::Response,
        max_size: u64,
    ) -> SDKResult<Response<T>> {
        let tracker = ResponseTracker::start("json_flatten", response.content_length());
        let header_request_id = request_id_from_headers(response.headers());
        let body_bytes = read_body_with_limit(response, max_size).await?;

        let raw_value: Value = match serde_json::from_slice(&body_bytes) {
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

        let mut raw_response: RawResponse = match serde_json::from_value(raw_value.clone()) {
            Ok(response) => response,
            Err(e) => {
                let error_msg = format!("Failed to parse raw response: {e}");
                tracker.error(&error_msg);
                return Err(validation_error("response", error_msg));
            }
        };
        if raw_response.request_id.is_none() {
            raw_response.request_id =
                header_request_id.or_else(|| request_id_from_json(&raw_value));
        }
        if raw_response.error.is_none() {
            raw_response.error = error_info_from_json(&raw_value);
        }

        let data = if raw_response.code == 0 {
            match serde_json::from_value::<T>(raw_value) {
                Ok(parsed_data) => {
                    tracker.validation_complete();
                    Some(parsed_data)
                }
                Err(e) if T::requires_payload() => {
                    let error_msg = format!("成功 Flatten 响应无法解析为期望类型: {e}");
                    tracker.error(&error_msg);
                    return Err(validation_error("flatten_response", error_msg));
                }
                Err(e) => {
                    debug!("Failed to parse optional flatten response: {e}");
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

    /// Binary / Text / Custom：先识别 HTTP / 业务错误，再 `from_*`。
    async fn handle_raw_format<T: ApiResponseTrait>(
        response: reqwest::Response,
        max_size: u64,
        format: ResponseFormat,
    ) -> SDKResult<Response<T>> {
        let label = format.as_label();
        let tracker = ResponseTracker::start(label, response.content_length());
        let status = response.status();
        let header_request_id = request_id_from_headers(response.headers());

        let file_name = response
            .headers()
            .get("Content-Disposition")
            .and_then(|header| header.to_str().ok())
            .and_then(content_disposition::extract_filename)
            .unwrap_or_default();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);

        let body_bytes = match read_body_with_limit(response, max_size).await {
            Ok(data) => data,
            Err(e) => {
                tracker.error(&format!("Failed to read {label} response: {e}"));
                return Err(e);
            }
        };
        tracker.parsing_complete();

        // HTTP 非成功：不得伪装为 code=0 成功 payload
        if !status.is_success() {
            if let Some(raw) = try_api_error_envelope(&body_bytes, header_request_id.clone()) {
                tracker.success();
                return Ok(BaseResponse {
                    raw_response: raw,
                    data: None,
                });
            }
            let status_code = status.as_u16() as i32;
            let msg = status
                .canonical_reason()
                .unwrap_or("http error")
                .to_string();
            tracker.success();
            return Ok(BaseResponse {
                raw_response: RawResponse {
                    code: status_code,
                    msg: msg.clone(),
                    request_id: header_request_id,
                    data: None,
                    error: Some(ErrorInfo {
                        code: status_code,
                        message: msg,
                        details: None,
                    }),
                },
                data: None,
            });
        }

        // HTTP 成功但 body 为飞书业务错误 envelope（常见于下载失败 JSON）
        if let Some(raw) = try_api_error_envelope(&body_bytes, header_request_id.clone()) {
            tracker.success();
            return Ok(BaseResponse {
                raw_response: raw,
                data: None,
            });
        }

        let decoded = match format {
            ResponseFormat::Binary => T::from_binary(file_name, body_bytes),
            ResponseFormat::Text => {
                let text = match String::from_utf8(body_bytes) {
                    Ok(t) => t,
                    Err(e) => {
                        let error_msg = format!("Text 响应不是合法 UTF-8: {e}");
                        tracker.error(&error_msg);
                        return Err(validation_error("text_response", error_msg));
                    }
                };
                T::from_text(text)
            }
            ResponseFormat::Custom => T::from_custom(body_bytes, content_type.as_deref()),
            ResponseFormat::Data | ResponseFormat::Flatten => unreachable!("raw pipeline only"),
        };

        let data = match require_decoded_payload(
            decoded,
            format!("{label} 响应解码失败：类型未实现对应 from_* 或返回 None"),
        ) {
            Ok(data) => data,
            Err(error_msg) => {
                tracker.error(&error_msg);
                return Err(validation_error(format!("{label}_response"), error_msg));
            }
        };

        tracker.success();
        Ok(BaseResponse {
            raw_response: success_raw_response(header_request_id),
            data,
        })
    }
}

// 文件名须与 mod 名一致，供 tools/check_mod_reachability 识别 cfg(test) 挂载。
#[cfg(test)]
mod decode_tests;
