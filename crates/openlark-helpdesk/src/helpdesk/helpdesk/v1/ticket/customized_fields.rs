//! 获取服务台自定义字段
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/customized_fields>

use crate::common::api_endpoints::HelpdeskApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取自定义字段请求。
#[derive(Debug, Clone)]
pub struct GetCustomizedFieldsRequest {
    config: Arc<Config>,
}

/// 获取自定义字段响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsResponse {
    /// 响应数据。
    pub data: Option<GetCustomizedFieldsData>,
}

impl ApiResponseTrait for GetCustomizedFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 自定义字段响应数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsData {
    /// 自定义字段列表。
    pub fields: Vec<CustomizedField>,
}

/// 自定义字段条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizedField {
    /// 字段 ID。
    pub field_id: String,
    /// 字段名称。
    pub field_name: String,
    /// 字段类型。
    pub field_type: String,
}

impl GetCustomizedFieldsRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetCustomizedFieldsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetCustomizedFieldsResponse> {
        let path = HelpdeskApiV1::TicketCustomizedFields.to_url();
        let req: ApiRequest<GetCustomizedFieldsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取服务台自定义字段", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../customized_fields → 强类型 GetCustomizedFieldsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_customized_fields_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/customized_fields"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "fields": [
                    { "field_id": "f_001", "field_name": "优先级", "field_type": "text" }
                ] } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetCustomizedFieldsRequest::new(config)
            .execute()
            .await
            .expect("获取自定义字段应成功");
        assert!(resp.data.is_some());
        assert_eq!(resp.data.unwrap().fields.len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/customized_fields"
        );
    }
}
