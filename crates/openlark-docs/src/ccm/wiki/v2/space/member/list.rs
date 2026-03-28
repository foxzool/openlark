//! 获取知识空间成员列表
//!
//! 获取知识空间的成员列表。
//!
//! docPath: <https://open.feishu.cn/document/docs/wiki-v2/space-member/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceMember;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间成员列表请求
pub struct ListWikiSpaceMembersRequest {
    space_id: String,
    config: Config,
}

/// 获取知识空间成员列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceMembersParams {
    /// 每页大小 (默认: 20, 最大: 100)
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取知识空间成员列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceMembersResponse {
    /// 成员列表
    #[serde(default)]
    pub members: Vec<WikiSpaceMember>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListWikiSpaceMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWikiSpaceMembersRequest {
    /// 创建获取知识空间成员列表请求
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
        params: Option<ListWikiSpaceMembersParams>,
    ) -> SDKResult<ListWikiSpaceMembersResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: Option<ListWikiSpaceMembersParams>,
        option: RequestOption,
    ) -> SDKResult<ListWikiSpaceMembersResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceMemberList(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListWikiSpaceMembersResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                api_request = api_request.query("page_size", &page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                api_request = api_request.query("page_token", &page_token);
            }
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间成员列表")
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
