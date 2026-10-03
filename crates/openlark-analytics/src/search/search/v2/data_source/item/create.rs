//! 为指定数据项创建索引
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 为指定数据项创建索引请求。
#[derive(Debug, Clone)]
pub struct CreateDataSourceItemRequest {
    config: Arc<Config>,
    data_source_id: String,
}

/// 为指定数据项创建索引响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataSourceItemResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateDataSourceItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDataSourceItemRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
        }
    }

    /// 执行为指定数据项创建索引请求。
    pub async fn execute(self) -> SDKResult<CreateDataSourceItemResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行为指定数据项创建索引请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateDataSourceItemResponse> {
        validate_required!(self.data_source_id, "data_source_id 不能为空");
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items",
            self.data_source_id
        );
        let req: ApiRequest<CreateDataSourceItemResponse> = ApiRequest::post(&path);

        Transport::request_typed(req, &self.config, Some(option), "为指定数据项创建索引").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../data_sources/{data_source_id}/items（修复后路径插值）→ 响应解析。
    #[tokio::test]
    async fn test_create_data_source_item_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/data_sources/ds_001/items"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = CreateDataSourceItemRequest::new(config, "ds_001")
            .execute()
            .await
            .expect("为指定数据项创建索引应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001/items"
        );
    }
}
