//! 获取知识空间节点信息
//!
//! 获取知识空间节点信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceNode;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间节点信息请求
pub struct GetWikiSpaceNodeRequest {
    config: Config,
}

/// 获取知识空间节点信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceNodeParams {
    /// wiki token 或文档 token
    pub token: String,
    /// 对象类型（当 token 为文档 token 时需要传）
    pub obj_type: Option<String>,
}

/// 获取知识空间节点信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceNodeResponse {
    /// 节点信息
    pub node: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for GetWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiSpaceNodeRequest {
    /// 创建获取知识空间节点信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: GetWikiSpaceNodeParams,
    ) -> SDKResult<GetWikiSpaceNodeResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: GetWikiSpaceNodeParams,
        option: RequestOption,
    ) -> SDKResult<GetWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(params.token, "token不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceGetNode;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetWikiSpaceNodeResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        api_request = api_request.query("token", &params.token);
        if let Some(obj_type) = params.obj_type {
            api_request = api_request.query("obj_type", &obj_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间节点信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/wiki/v2/spaces/get_node → GetWikiSpaceNodeResponse。
    #[tokio::test]
    async fn test_get_wiki_space_node_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/wiki/v2/spaces/get_node"))
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
        let resp = GetWikiSpaceNodeRequest::new(config)
            .execute(GetWikiSpaceNodeParams {
                token: "nt001".into(),
                obj_type: None,
            })
            .await
            .expect("获取知识空间节点应成功");
        assert!(resp.node.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/wiki/v2/spaces/get_node");
    }
}
