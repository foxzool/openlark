//! 获取知识空间信息
//!
//! 此接口用于根据知识空间ID来查询知识空间的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间信息请求
pub struct GetWikiSpaceRequest {
    space_id: String,
    config: Config,
}

/// 获取知识空间信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceResponse {
    /// 知识空间信息
    pub space: Option<WikiSpace>,
}

impl ApiResponseTrait for GetWikiSpaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiSpaceRequest {
    /// 创建获取知识空间信息请求
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
    pub async fn execute(self) -> SDKResult<GetWikiSpaceResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetWikiSpaceResponse> {
        // ===== 参数校验 =====
        validate_required!(self.space_id, "知识空间ID不能为空");

        // ===== 构建请求 =====
        let api_endpoint = WikiApiV2::SpaceGet(self.space_id.clone());

        let api_request: ApiRequest<GetWikiSpaceResponse> = ApiRequest::get(&api_endpoint.to_url());

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_wiki_space_builder() {
        let config = Config::default();
        let request = GetWikiSpaceRequest::new(config).space_id("wiki_space_123");

        assert_eq!(request.space_id, "wiki_space_123");
    }

    /// 测试响应数据结构
    #[test]
    fn test_get_wiki_space_response() {
        let response = GetWikiSpaceResponse { space: None };

        assert!(response.space.is_none());
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(GetWikiSpaceResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试空space_id
    #[test]
    fn test_empty_space_id() {
        let config = Config::default();
        let request = GetWikiSpaceRequest::new(config);

        assert_eq!(request.space_id, "");
    }

    /// 测试特殊字符space_id
    #[test]
    fn test_special_characters_space_id() {
        let config = Config::default();
        let request = GetWikiSpaceRequest::new(config).space_id("wiki_123-456_abc");

        assert_eq!(request.space_id, "wiki_123-456_abc");
    }
}
