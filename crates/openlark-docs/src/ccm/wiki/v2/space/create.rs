//! 创建知识空间
//!
//! 此接口用于创建知识空间。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 创建知识空间请求（流式 Builder 模式）
pub struct CreateWikiSpaceRequest {
    config: Config,
    /// 知识空间名称
    name: String,
    /// 知识空间描述
    description: Option<String>,
}

/// 创建知识空间请求体（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateWikiSpaceRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 创建知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceResponse {
    /// 知识空间信息
    pub space: Option<WikiSpace>,
}

impl ApiResponseTrait for CreateWikiSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceRequest {
    /// 创建创建知识空间请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            description: None,
        }
    }

    /// 设置知识空间名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置知识空间描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateWikiSpaceResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateWikiSpaceResponse> {
        // ===== 参数校验 =====
        validate_required!(self.name, "知识空间名称不能为空");

        // ===== 构建请求 =====
        let api_endpoint = WikiApiV2::SpaceCreate;

        let request_body = CreateWikiSpaceRequestBody {
            name: self.name,
            description: self.description,
        };

        let api_request: ApiRequest<CreateWikiSpaceResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&request_body, "创建知识空间")?);

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建知识空间")
    }
}

/// 创建知识空间请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 CreateWikiSpaceRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceParams {
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式 - 创建知识空间
    #[test]
    fn test_create_wiki_space_builder() {
        let config = Config::default();
        let request = CreateWikiSpaceRequest::new(config)
            .name("测试知识库")
            .description("这是一个测试知识库");

        assert_eq!(request.name, "测试知识库");
        assert_eq!(request.description, Some("这是一个测试知识库".to_string()));
    }

    /// 测试只有名称的创建请求
    #[test]
    fn test_create_wiki_space_name_only() {
        let config = Config::default();
        let request = CreateWikiSpaceRequest::new(config).name("简单知识库");

        assert_eq!(request.name, "简单知识库");
        assert!(request.description.is_none());
    }

    /// 测试响应数据结构
    #[test]
    fn test_create_wiki_space_response() {
        let response = CreateWikiSpaceResponse { space: None };

        assert!(response.space.is_none());
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(CreateWikiSpaceResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试空描述场景
    #[test]
    fn test_empty_description() {
        let config = Config::default();
        let request = CreateWikiSpaceRequest::new(config)
            .name("知识库")
            .description("");

        assert_eq!(request.description, Some("".to_string()));
    }

    /// 测试长描述
    #[test]
    fn test_long_description() {
        let config = Config::default();
        let long_desc = "这是一个很长的描述，".repeat(10);
        let request = CreateWikiSpaceRequest::new(config)
            .name("知识库")
            .description(&long_desc);

        assert_eq!(request.description.unwrap().len(), long_desc.len());
    }

    /// 测试已弃用的参数结构（保留以测试向后兼容性）
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_params() {
        let params = CreateWikiSpaceParams {
            name: "旧API知识库".to_string(),
            description: Some("使用旧API创建".to_string()),
        };

        assert_eq!(params.name, "旧API知识库");
        assert_eq!(params.description, Some("使用旧API创建".to_string()));
    }
}
