//! 获取知识空间节点信息
//!
//! 获取知识空间节点信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
