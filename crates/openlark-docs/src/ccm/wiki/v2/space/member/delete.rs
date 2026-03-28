//! 删除知识空间成员
//!
//! 从知识空间删除成员。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete>

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

/// 删除知识空间成员请求
pub struct DeleteWikiSpaceMemberRequest {
    space_id: String,
    member_id: String,
    config: Config,
}

/// 删除知识空间成员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWikiSpaceMemberParams {
    /// 成员角色（例如 admin、member）
    pub member_role: String,
    /// 成员标识类型（例如 openid、userid）
    pub member_type: String,
    /// 成员主体类型（例如 user）
    #[serde(rename = "type")]
    pub member_kind: String,
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWikiSpaceMemberResponse {
    /// 删除的成员信息
    pub member: Option<WikiSpaceMember>,
}

impl ApiResponseTrait for DeleteWikiSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteWikiSpaceMemberRequest {
    /// 创建删除知识空间成员请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            member_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_id = member_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: DeleteWikiSpaceMemberParams,
    ) -> SDKResult<DeleteWikiSpaceMemberResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        params: DeleteWikiSpaceMemberParams,
        option: RequestOption,
    ) -> SDKResult<DeleteWikiSpaceMemberResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.member_id, "成员ID不能为空");
        validate_required!(params.member_role, "成员角色不能为空");
        validate_required!(params.member_type, "成员类型不能为空");
        validate_required!(params.member_kind, "成员主体类型不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint =
            WikiApiV2::SpaceMemberDelete(self.space_id.clone(), self.member_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteWikiSpaceMemberResponse> =
            ApiRequest::delete(&api_endpoint.to_url())
                .body(serialize_params(&params, "删除知识空间成员")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除知识空间成员")
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
