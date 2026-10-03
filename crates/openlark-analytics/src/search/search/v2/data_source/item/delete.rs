//! 删除数据项
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/delete>

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

/// 删除数据项请求。
#[derive(Debug, Clone)]
pub struct DeleteDataSourceItemRequest {
    config: Arc<Config>,
    data_source_id: String,
    item_id: String,
}

/// 删除数据项响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataSourceItemResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteDataSourceItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteDataSourceItemRequest {
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

    /// 执行删除数据项请求。
    pub async fn execute(self) -> SDKResult<DeleteDataSourceItemResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行删除数据项请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteDataSourceItemResponse> {
        validate_required!(self.data_source_id, "data_source_id 不能为空");
        validate_required!(self.item_id, "item_id 不能为空");
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items/{}",
            self.data_source_id, self.item_id
        );
        let req: ApiRequest<DeleteDataSourceItemResponse> = ApiRequest::delete(&path);

        Transport::request_typed(req, &self.config, Some(option), "删除数据项").await
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

    /// 端到端：DELETE .../data_sources/{data_source_id}/items/{item_id}（修复后路径插值）→ 响应解析。
    #[tokio::test]
    async fn test_delete_data_source_item_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/search/v2/data_sources/ds_001/items/it_001",
            ))
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

        let resp = DeleteDataSourceItemRequest::new(config, "ds_001", "it_001")
            .execute()
            .await
            .expect("删除数据项应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001/items/it_001"
        );
    }
}
