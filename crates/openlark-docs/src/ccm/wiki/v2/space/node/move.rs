//! 移动知识空间节点
//!
//! 移动知识空间中的节点。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/move>
//!
//! 注意：该 API 的 meta.name 为 move（Rust 关键字），模块通过 `r#move` 暴露。

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceNode;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 移动知识空间节点请求
pub struct MoveWikiSpaceNodeRequest {
    space_id: String,
    node_token: String,
    config: Config,
}

/// 移动知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveWikiSpaceNodeParams {
    /// 目标父节点 token
    pub target_parent_token: String,
    /// 目标知识空间 ID
    pub target_space_id: String,
}

/// 移动知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveWikiSpaceNodeResponse {
    /// 移动后的节点信息
    pub node: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for MoveWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MoveWikiSpaceNodeRequest {
    /// 创建移动知识空间节点请求
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
        params: MoveWikiSpaceNodeParams,
    ) -> SDKResult<MoveWikiSpaceNodeResponse> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: MoveWikiSpaceNodeParams,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MoveWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.node_token, "节点Token不能为空");
        validate_required!(params.target_parent_token, "目标父节点token不能为空");
        validate_required!(params.target_space_id, "目标知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeMove(self.space_id.clone(), self.node_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<MoveWikiSpaceNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "移动知识空间节点")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "移动知识空间节点")
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
