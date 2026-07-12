//! API响应类型定义
//!
//! 独立的响应处理系统，替代api_resp模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 原始响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    /// 响应代码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 请求数据ID
    pub request_id: Option<String>,
    /// 额外数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<ErrorInfo>,
}

/// 错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误代码
    pub code: i32,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl Default for RawResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            request_id: None,
            data: None,
            error: None,
        }
    }
}

impl RawResponse {
    /// 创建成功响应
    pub fn success() -> Self {
        Self::default()
    }

    /// 创建带数据的成功响应
    pub fn success_with_data(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, msg: impl Into<String> + Clone) -> Self {
        let msg_str = msg.into();
        Self {
            code,
            msg: msg_str.clone(),
            error: Some(ErrorInfo {
                code,
                message: msg_str,
                details: None,
            }),
            ..Default::default()
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&ErrorInfo> {
        self.error.as_ref()
    }
}

/// 响应格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseFormat {
    /// 标准数据格式
    #[serde(rename = "data")]
    Data,
    /// 扁平格式
    #[serde(rename = "flatten")]
    Flatten,
    /// 二进制数据
    #[serde(rename = "binary")]
    Binary,
    /// 文本数据
    #[serde(rename = "text")]
    Text,
    /// 自定义格式
    #[serde(rename = "custom")]
    Custom,
}

/// API 响应特征：声明解码策略，由 Transport 请求执行层按策略解码。
///
/// - [`data_format`] 选择解码路径（不得静默降级到 Data）
/// - [`requires_payload`]：成功（业务 code=0 / Binary/Text 可读）时是否必须解出 `data`
/// - Binary / Text / Custom 通过类型方法参与解码，避免运行时 `TypeId` 猜测
pub trait ApiResponseTrait: Sized + Send + Sync + 'static {
    /// 获取响应数据格式
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 成功响应是否必须携带可解码 payload。
    /// `()` 等无体响应返回 `false`；默认 `true`。
    fn requires_payload() -> bool {
        true
    }

    /// Binary 解码：保留文件名 metadata，由类型自行映射。
    fn from_binary(_file_name: String, _body: Vec<u8>) -> Option<Self> {
        None
    }

    /// Text 解码：原始响应体按 UTF-8 文本处理。
    fn from_text(_text: String) -> Option<Self> {
        None
    }

    /// Custom 解码：原始字节 + Content-Type，未实现则解码失败。
    fn from_custom(_body: Vec<u8>, _content_type: Option<&str>) -> Option<Self> {
        None
    }
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 原始响应
    pub raw_response: RawResponse,
}

impl<T> Response<T> {
    /// 创建新响应
    pub fn new(data: Option<T>, raw_response: RawResponse) -> Self {
        Self { data, raw_response }
    }

    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            raw_response: RawResponse::success(),
        }
    }

    /// 创建空成功响应
    pub fn success_empty() -> Self {
        Self {
            data: None,
            raw_response: RawResponse::success(),
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, msg: impl Into<String> + Clone) -> Self {
        Self {
            data: None,
            raw_response: RawResponse::error(code, msg),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.raw_response.is_success()
    }

    /// 获取响应代码
    pub fn code(&self) -> i32 {
        self.raw_response.code
    }

    /// 获取响应消息
    pub fn message(&self) -> &str {
        &self.raw_response.msg
    }

    /// 获取响应消息（兼容方法）
    pub fn msg(&self) -> &str {
        &self.raw_response.msg
    }

    /// 获取数据
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 获取原始响应
    pub fn raw(&self) -> &RawResponse {
        &self.raw_response
    }

    /// 转换为结果类型
    pub fn into_result(self) -> Result<T, crate::error::CoreError> {
        let is_success = self.is_success();
        let code = self.raw_response.code;
        let request_id = self.raw_response.request_id.clone();

        if is_success {
            match self.data {
                Some(data) => Ok(data),
                None => Err(crate::error::api_error(
                    code as u16,
                    "response",
                    "响应数据为空",
                    request_id,
                )),
            }
        } else {
            Err(crate::error::api_error(
                code as u16,
                "response",
                self.raw_response.msg.clone(),
                request_id,
            ))
        }
    }
}

// 为常见类型实现 ApiResponseTrait
impl ApiResponseTrait for serde_json::Value {}
impl ApiResponseTrait for String {
    fn from_text(text: String) -> Option<Self> {
        Some(text)
    }
}
impl ApiResponseTrait for Vec<u8> {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(_file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(body)
    }
}
impl ApiResponseTrait for () {
    fn requires_payload() -> bool {
        false
    }
}

// 类型别名，用于向后兼容
/// 基础响应类型别名
pub type BaseResponse<T> = Response<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_response_default() {
        let response = RawResponse::default();
        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert!(response.request_id.is_none());
        assert!(response.data.is_none());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_raw_response_success() {
        let response = RawResponse::success();
        assert!(response.is_success());
    }

    #[test]
    fn test_raw_response_success_with_data() {
        let data = serde_json::json!({"key": "value"});
        let response = RawResponse::success_with_data(data.clone());
        assert!(response.is_success());
        assert_eq!(response.data, Some(data));
    }

    #[test]
    fn test_raw_response_error() {
        let response = RawResponse::error(400, "Bad Request");
        assert!(!response.is_success());
        assert_eq!(response.code, 400);
        assert!(response.error.is_some());
    }

    #[test]
    fn test_raw_response_get_error() {
        let response = RawResponse::error(404, "Not Found");
        let error = response.get_error();
        assert!(error.is_some());
        assert_eq!(error.unwrap().code, 404);
    }

    #[test]
    fn test_raw_response_serialization() {
        let response = RawResponse::success_with_data(serde_json::json!({"test": 123}));
        let json = serde_json::to_string(&response).unwrap();
        let parsed: RawResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert!(parsed.is_success());
    }

    #[test]
    fn test_error_info_creation() {
        let error = ErrorInfo {
            code: 500,
            message: "Internal Error".to_string(),
            details: None,
        };
        assert_eq!(error.code, 500);
        assert_eq!(error.message, "Internal Error");
    }

    #[test]
    fn test_response_format() {
        assert_eq!(ResponseFormat::Data, ResponseFormat::Data);
        assert_ne!(ResponseFormat::Data, ResponseFormat::Flatten);
    }

    #[test]
    fn test_response_format_binary() {
        assert_eq!(<Vec<u8>>::data_format(), ResponseFormat::Binary);
    }

    #[test]
    fn test_response_format_default() {
        assert_eq!(<()>::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_response_deserialize_requires_raw_response() {
        let payload = r#"{"code":400,"msg":"Bad Request"}"#;
        let parsed = serde_json::from_str::<Response<serde_json::Value>>(payload);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_response_deserialize_with_raw_response_error_keeps_code_and_msg() {
        let payload = r#"{"raw_response":{"code":400,"msg":"Bad Request","request_id":null,"data":null,"error":null},"data":null}"#;
        let parsed = serde_json::from_str::<Response<serde_json::Value>>(payload)
            .expect("JSON 反序列化失败");
        assert_eq!(parsed.raw_response.code, 400);
        assert_eq!(parsed.raw_response.msg, "Bad Request");
        assert!(!parsed.is_success());
    }
}
