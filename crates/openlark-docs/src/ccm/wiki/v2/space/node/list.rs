//! 获取知识空间节点列表
//!
//! 获取知识空间的节点列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceNode;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间节点列表请求
pub struct ListWikiSpaceNodesRequest {
    space_id: String,
    config: Config,
}

/// 获取知识空间节点列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceNodesParams {
    /// 父节点Token (可选，获取指定节点的子节点)
    pub parent_node_token: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取知识空间节点列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceNodesResponse {
    /// 节点列表
    #[serde(default)]
    pub items: Vec<WikiSpaceNode>,
    /// 下一页 token
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListWikiSpaceNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWikiSpaceNodesRequest {
    /// 创建获取知识空间节点列表请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: Option<ListWikiSpaceNodesParams>,
    ) -> SDKResult<ListWikiSpaceNodesResponse> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: Option<ListWikiSpaceNodesParams>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListWikiSpaceNodesResponse> {
        // ===== 参数校验 =====
        validate_required!(self.space_id, "知识空间ID不能为空");

        // ===== 构建请求 =====
        let api_endpoint = WikiApiV2::SpaceNodeList(self.space_id.clone());

        let mut api_request: ApiRequest<ListWikiSpaceNodesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(params) = params {
            if let Some(parent_node_token) = params.parent_node_token {
                api_request = api_request.query("parent_node_token", &parent_node_token);
            }
            if let Some(page_size) = params.page_size {
                api_request = api_request.query("page_size", &page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                api_request = api_request.query("page_token", &page_token);
            }
        }

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间节点列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_list_wiki_space_nodes_builder() {
        let config = Config::default();
        let request = ListWikiSpaceNodesRequest::new(config).space_id("wiki_space_123");

        assert_eq!(request.space_id, "wiki_space_123");
    }

    /// 测试带参数的请求
    #[test]
    fn test_list_with_params() {
        let config = Config::default();
        ListWikiSpaceNodesRequest::new(config).space_id("wiki_space_123");

        let params = ListWikiSpaceNodesParams {
            parent_node_token: Some("parent_node".to_string()),
            page_size: Some(20),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(params.parent_node_token, Some("parent_node".to_string()));
        assert_eq!(params.page_size, Some(20));
        assert_eq!(params.page_token, Some("token123".to_string()));
    }

    /// 测试响应数据结构
    #[test]
    fn test_list_wiki_space_nodes_response() {
        let response = ListWikiSpaceNodesResponse {
            items: vec![],
            page_token: Some("next_token".to_string()),
            has_more: Some(true),
        };

        assert!(response.items.is_empty());
        assert_eq!(response.has_more, Some(true));
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListWikiSpaceNodesResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试空参数场景
    #[test]
    fn test_list_without_params() {
        let config = Config::default();
        let _request = ListWikiSpaceNodesRequest::new(config).space_id("wiki_space_123");

        // 不传入参数，获取所有节点
        let _params: Option<ListWikiSpaceNodesParams> = None;
    }

    /// 测试只指定父节点
    #[test]
    fn test_list_with_parent_only() {
        let params = ListWikiSpaceNodesParams {
            parent_node_token: Some("parent_node".to_string()),
            page_size: None,
            page_token: None,
        };

        assert_eq!(params.parent_node_token, Some("parent_node".to_string()));
        assert!(params.page_size.is_none());
        assert!(params.page_token.is_none());
    }

    /// 测试分页参数
    #[test]
    fn test_pagination_params() {
        let config = Config::default();
        let _request = ListWikiSpaceNodesRequest::new(config).space_id("wiki_space_123");

        let params = ListWikiSpaceNodesParams {
            parent_node_token: None,
            page_size: Some(50),
            page_token: Some("page_token_abc".to_string()),
        };

        assert_eq!(params.page_size, Some(50));
        assert_eq!(params.page_token, Some("page_token_abc".to_string()));
    }

    /// 测试无更多数据场景
    #[test]
    fn test_no_more_data() {
        let response = ListWikiSpaceNodesResponse {
            items: vec![],
            page_token: None,
            has_more: Some(false),
        };

        assert!(!response.has_more.unwrap());
    }
}
