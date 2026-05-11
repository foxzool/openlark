//! API处理模块 - 独立版本
//!
//! 现代化、类型安全的API请求和响应处理系统。
//! 完全独立，不依赖已弃用的api_req/api_resp模块。

// 核心类型定义

pub use responses::RawResponse;
use std::{collections::HashMap, time::Duration};

/// HTTP方法枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// HTTP GET 方法
    Get,
    /// HTTP POST 方法
    Post,
    /// HTTP PUT 方法
    Put,
    /// HTTP DELETE 方法
    Delete,
    /// HTTP PATCH 方法
    Patch,
    /// HTTP HEAD 方法
    Head,
    /// HTTP OPTIONS 方法
    Options,
}

impl HttpMethod {
    /// 获取 HTTP 方法的字符串表示
    pub fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 请求数据枚举
#[derive(Debug, Clone)]
pub enum RequestData {
    /// JSON 格式数据
    Json(serde_json::Value),
    /// 纯文本数据
    Text(String),
    /// 二进制数据
    Binary(Vec<u8>),
    /// 表单数据
    Form(std::collections::HashMap<String, String>),
}

// 处理字符串类型 - 优先使用Text处理
impl From<String> for RequestData {
    fn from(value: String) -> Self {
        RequestData::Text(value)
    }
}

impl From<&str> for RequestData {
    fn from(value: &str) -> Self {
        RequestData::Text(value.to_string())
    }
}

// 为JSON值类型提供直接转换
impl From<serde_json::Value> for RequestData {
    fn from(value: serde_json::Value) -> Self {
        RequestData::Json(value)
    }
}

// 为Vec<u8>提供二进制数据支持
impl From<Vec<u8>> for RequestData {
    fn from(value: Vec<u8>) -> Self {
        RequestData::Binary(value)
    }
}

// 为HashMap<String, String>提供表单数据支持
impl From<std::collections::HashMap<String, String>> for RequestData {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        RequestData::Form(value)
    }
}

// 重新导出响应类型
pub use responses::{ApiResponseTrait, BaseResponse, ErrorInfo, Response, ResponseFormat};

/// 简化的API请求结构
#[derive(Debug, Clone)]
pub struct ApiRequest<R> {
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) query: HashMap<String, String>,
    pub(crate) body: Option<RequestData>,
    pub(crate) file: Option<Vec<u8>>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) _phantom: std::marker::PhantomData<R>,
}

impl<R> ApiRequest<R> {
    /// 创建 GET 请求
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 创建 POST 请求
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 创建 PUT 请求
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 创建 PATCH 请求
    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Patch,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 创建 DELETE 请求
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 添加单个请求头
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// 添加单个查询参数
    pub fn query<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 添加可选查询参数，如果值为None则跳过
    pub fn query_opt<K, V>(mut self, key: K, value: Option<V>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        if let Some(v) = value {
            self.query.insert(key.into(), v.into());
        }
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: impl Into<RequestData>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 设置文件内容 (用于 multipart 上传)
    pub fn file_content(mut self, file: Vec<u8>) -> Self {
        self.file = Some(file);
        self
    }

    /// 为任何可序列化的类型设置请求体
    pub fn json_body<T>(mut self, body: &T) -> Self
    where
        T: serde::Serialize,
    {
        match serde_json::to_value(body) {
            Ok(json_value) => self.body = Some(RequestData::Json(json_value)),
            Err(e) => {
                tracing::warn!(error = %e, "json_body 序列化失败");
                self.body = Some(RequestData::Json(serde_json::Value::Null));
            }
        }
        self
    }

    /// 设置请求超时时间
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// 构建完整 URL（包含查询参数）
    pub fn build_url(&self) -> String {
        if self.query.is_empty() {
            self.url.clone()
        } else {
            let query_string = self
                .query
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", self.url, query_string)
        }
    }

    // 兼容性字段和方法，用于与现有http.rs代码兼容
    /// 获取 HTTP 方法
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// 获取 API 路径（从 URL 中提取）
    pub fn api_path(&self) -> &str {
        // 从URL中提取路径部分
        if let Some(start) = self.url.find(crate::constants::API_PATH_PREFIX) {
            &self.url[start..]
        } else {
            &self.url
        }
    }

    /// 获取支持的访问令牌类型
    pub fn supported_access_token_types(&self) -> Vec<crate::constants::AccessTokenType> {
        // 默认返回用户和租户令牌类型
        vec![
            crate::constants::AccessTokenType::User,
            crate::constants::AccessTokenType::Tenant,
        ]
    }

    /// 将请求体转换为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.body {
            Some(RequestData::Json(data)) => serde_json::to_vec(data).unwrap_or_default(),
            Some(RequestData::Binary(data)) => data.clone(),
            Some(RequestData::Form(data)) => data
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&")
                .into_bytes(),
            Some(RequestData::Text(text)) => text.clone().into_bytes(),
            None => vec![],
        }
    }

    /// 获取 headers 的可变引用，用于直接插入多个 header
    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    /// 获取 query 的可变引用，用于直接插入多个查询参数
    pub fn query_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.query
    }

    /// 获取文件内容
    pub fn file(&self) -> Vec<u8> {
        self.file.clone().unwrap_or_default()
    }

    /// 应用请求选项（兼容方法）
    pub fn request_option(mut self, option: crate::req_option::RequestOption) -> Self {
        // 将 RequestOption 的头部信息添加到请求中
        for (key, value) in option.header {
            self = self.header(key, value);
        }
        self
    }

    /// 设置查询参数（兼容方法）
    pub fn query_param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 设置多个查询参数（兼容方法）
    pub fn query_params<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in params {
            self.query.insert(key.into(), value.into());
        }
        self
    }
}

impl<R> Default for ApiRequest<R> {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            url: String::default(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            file: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

// 类型别名，保持兼容性
/// API 响应类型别名
pub type ApiResponse<R> = Response<R>;

// 子模块

pub mod prelude;
pub mod responses;
pub mod traits;

// 重新导出

pub use traits::{AsyncApiClient, SyncApiClient};

// 测试

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_method() {
        // 测试patch方法是否正确创建ApiRequest
        let request: ApiRequest<()> = ApiRequest::patch("https://example.com/api/resource");

        // 验证HTTP方法
        assert_eq!(request.method, HttpMethod::Patch);

        // 验证URL
        assert_eq!(request.url, "https://example.com/api/resource");

        // 验证HTTP方法字符串
        assert_eq!(request.method.as_str(), "PATCH");

        println!("✅ Patch method test passed!");
    }

    #[test]
    fn test_all_http_methods() {
        // 测试所有HTTP方法
        let get_req: ApiRequest<()> = ApiRequest::get("https://example.com/api");
        let post_req: ApiRequest<()> = ApiRequest::post("https://example.com/api");
        let put_req: ApiRequest<()> = ApiRequest::put("https://example.com/api");
        let patch_req: ApiRequest<()> = ApiRequest::patch("https://example.com/api");
        let delete_req: ApiRequest<()> = ApiRequest::delete("https://example.com/api");

        // 验证HTTP方法
        assert_eq!(get_req.method, HttpMethod::Get);
        assert_eq!(post_req.method, HttpMethod::Post);
        assert_eq!(put_req.method, HttpMethod::Put);
        assert_eq!(patch_req.method, HttpMethod::Patch);
        assert_eq!(delete_req.method, HttpMethod::Delete);

        // 验证HTTP方法字符串
        assert_eq!(get_req.method.as_str(), "GET");
        assert_eq!(post_req.method.as_str(), "POST");
        assert_eq!(put_req.method.as_str(), "PUT");
        assert_eq!(patch_req.method.as_str(), "PATCH");
        assert_eq!(delete_req.method.as_str(), "DELETE");

        println!("✅ All HTTP methods test passed!");
    }
}
