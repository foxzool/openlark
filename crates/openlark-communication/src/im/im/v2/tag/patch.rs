//! 修改标签
//!
//! docPath: <https://open.feishu.cn/document/tenant-tag/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V2_TAGS,
};

/// 修改标签请求
pub struct PatchTagRequest {
    /// 配置信息。
    config: Config,
    /// 标签 ID。
    tag_id: String,
}

impl PatchTagRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tag_id: String::new(),
        }
    }

    /// 标签 ID（路径参数）
    pub fn tag_id(mut self, tag_id: impl Into<String>) -> Self {
        self.tag_id = tag_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/tenant-tag/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.tag_id, "tag_id 不能为空");

        // url: PATCH:/open-apis/im/v2/tags/:tag_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(format!("{}/{}", IM_V2_TAGS, self.tag_id))
                .body(serialize_params(&body, "修改标签")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "修改标签")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/im/v2/tags/test001
    #[tokio::test]
    async fn test_patch_tag_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/im/v2/tags/test001"))
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

        let body = json!({});
        PatchTagRequest::new(config)
            .tag_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
