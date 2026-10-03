use std::collections::HashMap;

/// 请求选项
///
/// 用于配置 API 请求的各种选项，如认证令牌、请求头等
#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    pub(crate) tenant_key: Option<String>,
    pub(crate) user_access_token: Option<String>,
    pub(crate) app_access_token: Option<String>,
    pub(crate) tenant_access_token: Option<String>,
    pub(crate) need_helpdesk_auth: bool,
    pub(crate) request_id: Option<String>,
    pub(crate) app_ticket: Option<String>,
    pub(crate) file_upload: bool,
    pub(crate) file_download: bool,
    pub(crate) header: HashMap<String, String>,
}

impl RequestOption {
    /// 创建请求选项构建器
    pub fn builder() -> RequestOptionBuilder {
        RequestOptionBuilder::default()
    }
}

/// 请求选项构建器
///
/// 使用 Builder 模式方便配置请求选项
#[derive(Default)]
pub struct RequestOptionBuilder {
    option: RequestOption,
}

impl RequestOptionBuilder {
    /// 设置租户键
    pub fn tenant_key(mut self, tenant_key: impl ToString) -> Self {
        self.option.tenant_key = Some(tenant_key.to_string());
        self
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl ToString) -> Self {
        self.option.user_access_token = Some(user_access_token.to_string());
        self
    }

    /// 设置应用访问令牌
    pub fn app_access_token(mut self, app_access_token: impl ToString) -> Self {
        self.option.app_access_token = Some(app_access_token.to_string());
        self
    }

    /// 设置租户访问令牌
    pub fn tenant_access_token(mut self, tenant_access_token: impl ToString) -> Self {
        self.option.tenant_access_token = Some(tenant_access_token.to_string());
        self
    }

    /// 设置是否需要帮助台认证
    pub fn need_helpdesk_auth(mut self, need_helpdesk_auth: bool) -> Self {
        self.option.need_helpdesk_auth = need_helpdesk_auth;
        self
    }

    /// 设置请求 ID
    pub fn request_id(mut self, request_id: impl ToString) -> Self {
        self.option.request_id = Some(request_id.to_string());
        self
    }

    /// 设置应用票据
    pub fn app_ticket(mut self, app_ticket: impl ToString) -> Self {
        self.option.app_ticket = Some(app_ticket.to_string());
        self
    }

    /// 设置是否上传文件
    pub fn file_upload(mut self, file_upload: bool) -> Self {
        self.option.file_upload = file_upload;
        self
    }

    /// 设置是否下载文件
    pub fn file_download(mut self, file_download: bool) -> Self {
        self.option.file_download = file_download;
        self
    }

    /// 设置请求头
    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.option.header = header;
        self
    }

    /// 添加请求头
    pub fn add_header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.option
            .header
            .insert(key.to_string(), value.to_string());
        self
    }

    /// 构建 RequestOption
    pub fn build(self) -> RequestOption {
        self.option
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_request_option_default() {
        let option = RequestOption::default();
        assert_eq!(option.tenant_key, None);
        assert_eq!(option.user_access_token, None);
        assert_eq!(option.app_access_token, None);
        assert_eq!(option.tenant_access_token, None);
        assert!(!option.need_helpdesk_auth);
        assert_eq!(option.request_id, None);
        assert_eq!(option.app_ticket, None);
        assert!(!option.file_upload);
        assert!(!option.file_download);
        assert!(option.header.is_empty());
    }

    #[test]
    fn test_request_option_builder_creation() {
        let builder = RequestOption::builder();
        let option = builder.build();

        // Should be same as default
        assert_eq!(option.tenant_key, None);
        assert_eq!(option.user_access_token, None);
        assert_eq!(option.app_access_token, None);
        assert_eq!(option.tenant_access_token, None);
        assert!(!option.need_helpdesk_auth);
        assert_eq!(option.request_id, None);
        assert_eq!(option.app_ticket, None);
        assert!(!option.file_upload);
        assert!(!option.file_download);
        assert!(option.header.is_empty());
    }

    #[test]
    fn test_request_option_builder_tenant_key() {
        let option = RequestOption::builder().tenant_key("test_tenant").build();

        assert_eq!(option.tenant_key, Some("test_tenant".to_string()));

        // Test with String
        let option = RequestOption::builder()
            .tenant_key("another_tenant".to_string())
            .build();

        assert_eq!(option.tenant_key, Some("another_tenant".to_string()));
    }

    #[test]
    fn test_request_option_builder_user_access_token() {
        let option = RequestOption::builder()
            .user_access_token("user_token_123")
            .build();

        assert_eq!(option.user_access_token, Some("user_token_123".to_string()));

        // Test with String
        let option = RequestOption::builder()
            .user_access_token("user_token_456".to_string())
            .build();

        assert_eq!(option.user_access_token, Some("user_token_456".to_string()));
    }

    #[test]
    fn test_request_option_builder_app_access_token() {
        let option = RequestOption::builder()
            .app_access_token("app_token_789")
            .build();

        assert_eq!(option.app_access_token, Some("app_token_789".to_string()));

        // Test with String
        let option = RequestOption::builder()
            .app_access_token("app_token_012".to_string())
            .build();

        assert_eq!(option.app_access_token, Some("app_token_012".to_string()));
    }

    #[test]
    fn test_request_option_builder_tenant_access_token() {
        let option = RequestOption::builder()
            .tenant_access_token("tenant_token_345")
            .build();

        assert_eq!(
            option.tenant_access_token,
            Some("tenant_token_345".to_string())
        );

        // Test with String
        let option = RequestOption::builder()
            .tenant_access_token("tenant_token_678".to_string())
            .build();

        assert_eq!(
            option.tenant_access_token,
            Some("tenant_token_678".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_need_helpdesk_auth() {
        let option = RequestOption::builder().need_helpdesk_auth(true).build();

        assert!(option.need_helpdesk_auth);

        let option = RequestOption::builder().need_helpdesk_auth(false).build();

        assert!(!option.need_helpdesk_auth);
    }

    #[test]
    fn test_request_option_builder_request_id() {
        let option = RequestOption::builder().request_id("req_12345").build();

        assert_eq!(option.request_id, Some("req_12345".to_string()));

        // Test with String
        let option = RequestOption::builder()
            .request_id("req_67890".to_string())
            .build();

        assert_eq!(option.request_id, Some("req_67890".to_string()));
    }

    #[test]
    fn test_request_option_builder_app_ticket() {
        let option = RequestOption::builder().app_ticket("ticket_abc").build();

        assert_eq!(option.app_ticket, Some("ticket_abc".to_string()));

        // Test with String
        let option = RequestOption::builder()
            .app_ticket("ticket_def".to_string())
            .build();

        assert_eq!(option.app_ticket, Some("ticket_def".to_string()));
    }

    #[test]
    fn test_request_option_builder_file_upload() {
        let option = RequestOption::builder().file_upload(true).build();

        assert!(option.file_upload);

        let option = RequestOption::builder().file_upload(false).build();

        assert!(!option.file_upload);
    }

    #[test]
    fn test_request_option_builder_file_download() {
        let option = RequestOption::builder().file_download(true).build();

        assert!(option.file_download);

        let option = RequestOption::builder().file_download(false).build();

        assert!(!option.file_download);
    }

    #[test]
    fn test_request_option_builder_header() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let option = RequestOption::builder().header(headers.clone()).build();

        assert_eq!(option.header.len(), 2);
        assert_eq!(
            option.header.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            option.header.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_add_header() {
        let option = RequestOption::builder()
            .add_header("X-Custom-Header", "custom_value")
            .add_header("X-Another-Header", "another_value")
            .build();

        assert_eq!(option.header.len(), 2);
        assert_eq!(
            option.header.get("X-Custom-Header"),
            Some(&"custom_value".to_string())
        );
        assert_eq!(
            option.header.get("X-Another-Header"),
            Some(&"another_value".to_string())
        );

        // Test with String types
        let option = RequestOption::builder()
            .add_header("X-String-Header".to_string(), "string_value".to_string())
            .build();

        assert_eq!(option.header.len(), 1);
        assert_eq!(
            option.header.get("X-String-Header"),
            Some(&"string_value".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_add_header_multiple() {
        let option = RequestOption::builder()
            .add_header("Header1", "value1")
            .add_header("Header2", "value2")
            .add_header("Header3", "value3")
            .build();

        assert_eq!(option.header.len(), 3);
        assert_eq!(option.header.get("Header1"), Some(&"value1".to_string()));
        assert_eq!(option.header.get("Header2"), Some(&"value2".to_string()));
        assert_eq!(option.header.get("Header3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_request_option_builder_header_and_add_header() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("Initial-Header".to_string(), "initial_value".to_string());

        let option = RequestOption::builder()
            .header(initial_headers)
            .add_header("Added-Header", "added_value")
            .build();

        assert_eq!(option.header.len(), 2);
        assert_eq!(
            option.header.get("Initial-Header"),
            Some(&"initial_value".to_string())
        );
        assert_eq!(
            option.header.get("Added-Header"),
            Some(&"added_value".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_chaining() {
        let option = RequestOption::builder()
            .tenant_key("test_tenant")
            .user_access_token("user_token")
            .app_access_token("app_token")
            .tenant_access_token("tenant_token")
            .need_helpdesk_auth(true)
            .request_id("req_123")
            .app_ticket("ticket_456")
            .file_upload(true)
            .file_download(false)
            .add_header("X-Test", "test_value")
            .build();

        assert_eq!(option.tenant_key, Some("test_tenant".to_string()));
        assert_eq!(option.user_access_token, Some("user_token".to_string()));
        assert_eq!(option.app_access_token, Some("app_token".to_string()));
        assert_eq!(option.tenant_access_token, Some("tenant_token".to_string()));
        assert!(option.need_helpdesk_auth);
        assert_eq!(option.request_id, Some("req_123".to_string()));
        assert_eq!(option.app_ticket, Some("ticket_456".to_string()));
        assert!(option.file_upload);
        assert!(!option.file_download);
        assert_eq!(option.header.len(), 1);
        assert_eq!(option.header.get("X-Test"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_request_option_builder_empty_strings() {
        let option = RequestOption::builder()
            .tenant_key("")
            .user_access_token("")
            .app_access_token("")
            .tenant_access_token("")
            .request_id("")
            .app_ticket("")
            .build();

        assert_eq!(option.tenant_key, Some("".to_string()));
        assert_eq!(option.user_access_token, Some("".to_string()));
        assert_eq!(option.app_access_token, Some("".to_string()));
        assert_eq!(option.tenant_access_token, Some("".to_string()));
        assert_eq!(option.request_id, Some("".to_string()));
        assert_eq!(option.app_ticket, Some("".to_string()));
    }

    #[test]
    fn test_request_option_builder_special_characters() {
        let option = RequestOption::builder()
            .tenant_key("tenant@#$%^&*()")
            .user_access_token("token_with_symbols!@#")
            .request_id("req_with_unicode_测试")
            .add_header("X-Special-Chars", "value@#$%")
            .build();

        assert_eq!(option.tenant_key, Some("tenant@#$%^&*()".to_string()));
        assert_eq!(
            option.user_access_token,
            Some("token_with_symbols!@#".to_string())
        );
        assert_eq!(option.request_id, Some("req_with_unicode_测试".to_string()));
        assert_eq!(
            option.header.get("X-Special-Chars"),
            Some(&"value@#$%".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_overwrite_header() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("Test-Header".to_string(), "initial_value".to_string());

        let option = RequestOption::builder()
            .header(initial_headers)
            .add_header("Test-Header", "overwritten_value")
            .build();

        assert_eq!(option.header.len(), 1);
        assert_eq!(
            option.header.get("Test-Header"),
            Some(&"overwritten_value".to_string())
        );
    }

    #[test]
    fn test_request_option_builder_empty_header_map() {
        let empty_headers = HashMap::new();

        let option = RequestOption::builder().header(empty_headers).build();

        assert!(option.header.is_empty());
    }

    #[test]
    fn test_request_option_debug_clone() {
        let option = RequestOption::builder()
            .tenant_key("test")
            .need_helpdesk_auth(true)
            .build();

        // Test Debug trait
        let debug_str = format!("{option:?}");
        assert!(debug_str.contains("RequestOption"));

        // Test Clone trait
        let cloned_option = option.clone();
        assert_eq!(option.tenant_key, cloned_option.tenant_key);
        assert_eq!(option.need_helpdesk_auth, cloned_option.need_helpdesk_auth);
    }

    #[test]
    fn test_request_option_builder_default() {
        let builder = RequestOptionBuilder::default();
        let option = builder.build();

        // Should be equivalent to RequestOption::default()
        let default_option = RequestOption::default();

        assert_eq!(option.tenant_key, default_option.tenant_key);
        assert_eq!(option.user_access_token, default_option.user_access_token);
        assert_eq!(option.app_access_token, default_option.app_access_token);
        assert_eq!(
            option.tenant_access_token,
            default_option.tenant_access_token
        );
        assert_eq!(option.need_helpdesk_auth, default_option.need_helpdesk_auth);
        assert_eq!(option.request_id, default_option.request_id);
        assert_eq!(option.app_ticket, default_option.app_ticket);
        assert_eq!(option.file_upload, default_option.file_upload);
        assert_eq!(option.file_download, default_option.file_download);
        assert_eq!(option.header.len(), default_option.header.len());
    }
}
