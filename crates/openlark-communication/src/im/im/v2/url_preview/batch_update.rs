//! 更新 URL 预览
//!
//! docPath: <https://open.feishu.cn/document/im-v1/url_preview/batch_update>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V2_URL_PREVIEWS_BATCH_UPDATE,
    im::v2::url_preview::models::BatchUpdateUrlPreviewBody,
};

/// 更新 URL 预览请求
pub struct BatchUpdateUrlPreviewRequest {
    config: Config,
}

impl BatchUpdateUrlPreviewRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/im-v1/url_preview/batch_update>
    pub async fn execute(self, body: BatchUpdateUrlPreviewBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BatchUpdateUrlPreviewBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        if body.preview_tokens.is_empty() {
            return Err(openlark_core::error::validation_error(
                "preview_tokens 不能为空".to_string(),
                "preview_tokens 至少需要 1 个".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v2/url_previews/batch_update
        let req: ApiRequest<EmptyData> = ApiRequest::post(IM_V2_URL_PREVIEWS_BATCH_UPDATE)
            .body(serialize_params(&body, "更新 URL 预览")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新 URL 预览")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/im/v2/url_previews/batch_update
    #[tokio::test]
    async fn test_batch_update_url_preview_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v2/url_previews/batch_update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: BatchUpdateUrlPreviewBody =
            serde_json::from_value(json!({ "preview_tokens": ["test001"] })).expect("body 构造");
        BatchUpdateUrlPreviewRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
