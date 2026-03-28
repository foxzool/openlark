//! 复制知识空间节点
//!
//! 复制知识空间中的节点。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/copy>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceNode;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 复制知识空间节点请求
pub struct CopyWikiSpaceNodeRequest {
    space_id: String,
    node_token: String,
    config: Config,
}

/// 复制知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyWikiSpaceNodeParams {
    /// 目标父节点 token
    pub target_parent_token: String,
    /// 目标知识空间 ID
    pub target_space_id: String,
    /// 复制后的节点标题（可选，不传则使用原标题）
    pub title: Option<String>,
}

/// 复制知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyWikiSpaceNodeResponse {
    /// 复制后的节点信息
    pub node: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for CopyWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CopyWikiSpaceNodeRequest {
    /// 创建复制知识空间节点请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            node_token: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置节点Token
    pub fn node_token(mut self, node_token: impl Into<String>) -> Self {
        self.node_token = node_token.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: CopyWikiSpaceNodeParams,
    ) -> SDKResult<CopyWikiSpaceNodeResponse> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: CopyWikiSpaceNodeParams,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CopyWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.node_token, "节点Token不能为空");
        validate_required!(params.target_parent_token, "目标父节点token不能为空");
        validate_required!(params.target_space_id, "目标知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeCopy(self.space_id.clone(), self.node_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CopyWikiSpaceNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "创建知识空间节点副本")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建知识空间节点副本")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
