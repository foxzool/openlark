//! 获取公共邮箱别名列表
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/list

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::alias::models::PublicMailboxAliasListResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use std::sync::Arc;

/// 公共邮箱别名列表请求
#[derive(Debug, Clone)]
pub struct PublicMailboxAliasListRequest {
    config: Arc<Config>,
    mailbox_id: String,
    /// 分页标记
    page_token: Option<String>,
    /// 每页数量
    page_size: Option<i32>,
}

impl PublicMailboxAliasListRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
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
    pub async fn execute(self) -> SDKResult<PublicMailboxAliasListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PublicMailboxAliasListResponse> {
        let api_endpoint = MailApiV1::PublicMailboxAliasList(self.mailbox_id.clone());
        let mut request = ApiRequest::<PublicMailboxAliasListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取公共邮箱别名列表")
    }
}

impl ApiResponseTrait for PublicMailboxAliasListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_public_mailbox_alias_list_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = PublicMailboxAliasListRequest::new(config, "mailbox_123".to_string())
            .page_token("token_123".to_string())
            .page_size(20);

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.page_token, Some("token_123".to_string()));
        assert_eq!(request.page_size, Some(20));
    }
}
