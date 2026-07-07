//! 查询指定数据项
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询指定数据项请求。
#[derive(Debug, Clone)]
pub struct GetDataSourceItemRequest {
    config: Arc<Config>,
    data_source_id: String,
    item_id: String,
}

/// 查询指定数据项响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataSourceItemResponse {
    /// 响应数据。
    pub data: Option<DataSourceItemData>,
}

impl ApiResponseTrait for GetDataSourceItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据项详情数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceItemData {
    /// 数据项 ID。
    pub item_id: String,
    /// 数据项内容。
    pub data: serde_json::Value,
}

impl GetDataSourceItemRequest {
    /// 创建新的请求构建器。
    pub fn new(
        config: Arc<Config>,
        data_source_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
            item_id: item_id.into(),
        }
    }

    /// 执行查询指定数据项请求。
    pub async fn execute(self) -> SDKResult<GetDataSourceItemResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行查询指定数据项请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDataSourceItemResponse> {
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items/{}",
            self.data_source_id, self.item_id
        );
        let req: ApiRequest<GetDataSourceItemResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("查询指定数据项", "响应数据为空"))
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

    /// 端到端：GET /open-apis/search/v2/data_sources/{data_source_id}/items/{item_id} → 响应解析。
    #[tokio::test]
    async fn test_get_data_source_item_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/search/v2/data_sources/ds_001/items/it_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "item_id": "it_001",
                        "data": {}
                    }
                }
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

        let resp = GetDataSourceItemRequest::new(config, "ds_001", "it_001")
            .execute()
            .await
            .expect("查询指定数据项应成功");
        assert_eq!(resp.data.unwrap().item_id, "it_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001/items/it_001"
        );
    }
}
