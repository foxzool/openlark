//! 查询实体与标签的绑定关系
//!
//! docPath: <https://open.feishu.cn/document/tenant-tag/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::endpoints::IM_V2_BIZ_ENTITY_TAG_RELATION;

/// 查询实体与标签的绑定关系请求
pub struct GetBizEntityTagRelationRequest {
    /// 配置信息。
    config: Config,
    /// 标签业务类型。
    tag_biz_type: String,
    /// 业务实体 ID。
    biz_entity_id: String,
}

impl GetBizEntityTagRelationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tag_biz_type: String::new(),
            biz_entity_id: String::new(),
        }
    }

    /// 业务类型（查询参数，例如 chat）
    pub fn tag_biz_type(mut self, tag_biz_type: impl Into<String>) -> Self {
        self.tag_biz_type = tag_biz_type.into();
        self
    }

    /// 业务实体 ID（查询参数，例如 chat_id）
    pub fn biz_entity_id(mut self, biz_entity_id: impl Into<String>) -> Self {
        self.biz_entity_id = biz_entity_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/tenant-tag/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.tag_biz_type, "tag_biz_type 不能为空");
        validate_required!(self.biz_entity_id, "biz_entity_id 不能为空");

        // url: GET:/open-apis/im/v2/biz_entity_tag_relation
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(IM_V2_BIZ_ENTITY_TAG_RELATION)
            .query("tag_biz_type", self.tag_biz_type)
            .query("biz_entity_id", self.biz_entity_id);

        Transport::request_typed(req, &self.config, Some(option), "查询实体与标签的绑定关系").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/im/v2/biz_entity_tag_relation
    #[tokio::test]
    async fn test_get_biz_entity_tag_relation_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
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

        GetBizEntityTagRelationRequest::new(config)
            .tag_biz_type("test001".to_string())
            .biz_entity_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
