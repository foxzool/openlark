//! 获取公共邮箱列表
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/list

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::models::PublicMailboxListResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use std::sync::Arc;

/// 公共邮箱列表请求
#[derive(Debug, Clone)]
pub struct PublicMailboxListRequest {
    config: Arc<Config>,
    /// 分页标记
    page_token: Option<String>,
    /// 每页数量
    page_size: Option<i32>,
}

impl PublicMailboxListRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PublicMailboxListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PublicMailboxListResponse> {
        let api_endpoint = MailApiV1::PublicMailboxList;
        let mut request = ApiRequest::<PublicMailboxListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取公共邮箱列表")
    }
}

impl ApiResponseTrait for PublicMailboxListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_public_mailbox_list_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = PublicMailboxListRequest::new(config.clone())
            .page_token("token_123".to_string())
            .page_size(20);

        assert_eq!(request.page_token, Some("token_123".to_string()));
        assert_eq!(request.page_size, Some(20));
    }
}
