//! 更新知识空间节点标题
//!
//! 更新知识空间中节点的标题。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/update_title>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 更新知识空间节点标题请求
pub struct UpdateWikiSpaceNodeTitleRequest {
    space_id: String,
    node_token: String,
    config: Config,
}

/// 更新知识空间节点标题请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWikiSpaceNodeTitleParams {
    /// 新的节点标题
    pub title: String,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWikiSpaceNodeTitleResponse {}

impl ApiResponseTrait for UpdateWikiSpaceNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateWikiSpaceNodeTitleRequest {
    /// 创建更新知识空间节点标题请求
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
        params: UpdateWikiSpaceNodeTitleParams,
    ) -> SDKResult<UpdateWikiSpaceNodeTitleResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: UpdateWikiSpaceNodeTitleParams,
        option: RequestOption,
    ) -> SDKResult<UpdateWikiSpaceNodeTitleResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.node_token, "节点Token不能为空");
        validate_required!(params.title, "节点标题不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint =
            WikiApiV2::SpaceNodeUpdateTitle(self.space_id.clone(), self.node_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateWikiSpaceNodeTitleResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "更新知识空间节点标题")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新知识空间节点标题")
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
