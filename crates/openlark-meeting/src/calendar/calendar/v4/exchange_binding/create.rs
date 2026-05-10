//! 将 Exchange 账户绑定到飞书账户
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/create

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
    };
use super::models::{CreateExchangeBindingRequestBody, CreateExchangeBindingResponse};

/// 将 Exchange 账户绑定到飞书账户请求
pub struct CreateExchangeBindingRequest {
    config: Config,
}

impl CreateExchangeBindingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/create
    pub async fn execute(self, body: CreateExchangeBindingRequestBody) -> SDKResult<CreateExchangeBindingResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateExchangeBindingRequestBody,
        option: RequestOption,
    ) -> SDKResult<CreateExchangeBindingResponse> {
        let api_endpoint = CalendarApiV4::ExchangeBindingCreate;
        let req: ApiRequest<CreateExchangeBindingResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "将 Exchange 账户绑定到飞书账户")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "将 Exchange 账户绑定到飞书账户")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateExchangeBindingRequest::new(config.clone());
        let _ = request;
    }
}
