//! 绑定标签到群
//!
//! docPath: <https://open.feishu.cn/document/tenant-tag/create-2>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V2_BIZ_ENTITY_TAG_RELATION,
    im::v2::biz_entity_tag_relation::models::BizEntityTagRelationBody,
};

/// 绑定标签到群请求
pub struct CreateBizEntityTagRelationRequest {
    config: Config,
}

impl CreateBizEntityTagRelationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/tenant-tag/create-2>
    pub async fn execute(self, body: BizEntityTagRelationBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BizEntityTagRelationBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(body.tag_biz_type, "tag_biz_type 不能为空");
        validate_required!(body.biz_entity_id, "biz_entity_id 不能为空");

        // url: POST:/open-apis/im/v2/biz_entity_tag_relation
        let req: ApiRequest<EmptyData> = ApiRequest::post(IM_V2_BIZ_ENTITY_TAG_RELATION)
            .body(serialize_params(&body, "绑定标签到群")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "绑定标签到群")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/im/v2/biz_entity_tag_relation
    #[tokio::test]
    async fn test_create_biz_entity_tag_relation_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v2/biz_entity_tag_relation"))
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

        let body: BizEntityTagRelationBody = serde_json::from_value(
            json!({ "tag_biz_type": "test001", "biz_entity_id": "test001" }),
        )
        .expect("body 构造");
        CreateBizEntityTagRelationRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
