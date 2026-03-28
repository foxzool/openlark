//! Bitable 新增协作者（自定义角色）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::RoleMemberIdType;

/// 新增协作者请求
#[derive(Debug, Clone)]
pub struct CreateRoleMemberRequest {
    config: Config,
    app_token: String,
    role_id: String,
    /// 查询参数：协作者 ID 的类型（默认 open_id）
    member_id_type: Option<RoleMemberIdType>,
    /// 请求体：协作者的 ID
    member_id: String,
}

impl CreateRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            member_id_type: None,
            member_id: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    pub fn member_id_type(mut self, member_id_type: RoleMemberIdType) -> Self {
        self.member_id_type = Some(member_id_type);
        self
    }

    pub fn member_id(mut self, member_id: String) -> Self {
        self.member_id = member_id;
        self
    }

    pub async fn execute(self) -> SDKResult<CreateRoleMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateRoleMemberResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");
        validate_required!(self.member_id.trim(), "member_id");

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RoleMemberCreate(self.app_token.clone(), self.role_id.clone());

        let mut api_request: ApiRequest<CreateRoleMemberResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&CreateRoleMemberRequestBody {
            member_id: self.member_id,
        })?);

        // query 参数需要字符串值（open_id / user_id / ...），这里用枚举做一次映射。
        if let Some(member_id_type) = self.member_id_type {
            let member_id_type = match member_id_type {
                RoleMemberIdType::OpenId => "open_id",
                RoleMemberIdType::UnionId => "union_id",
                RoleMemberIdType::UserId => "user_id",
                RoleMemberIdType::ChatId => "chat_id",
                RoleMemberIdType::DepartmentId => "department_id",
                RoleMemberIdType::OpenDepartmentId => "open_department_id",
            };
            api_request = api_request.query("member_id_type", member_id_type);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Serialize)]
struct CreateRoleMemberRequestBody {
    member_id: String,
}

/// 新增协作者响应（data 为 {}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleMemberResponse {}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = CreateRoleMemberRequest::new(config)
            .app_token("".to_string())
            .role_id("role_id".to_string())
            .member_id("member_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_role_id() {
        let config = Config::default();
        let request = CreateRoleMemberRequest::new(config)
            .app_token("app_token".to_string())
            .role_id("".to_string())
            .member_id("member_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("role_id"));
    }

    #[test]
    fn test_empty_member_id() {
        let config = Config::default();
        let request = CreateRoleMemberRequest::new(config)
            .app_token("app_token".to_string())
            .role_id("role_id".to_string())
            .member_id("".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("member_id"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateRoleMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
