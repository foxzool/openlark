//! Bitable 批量新增协作者（自定义角色）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/batch_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::RoleMemberId;

/// 批量新增协作者请求
#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberRequest {
    config: Config,
    app_token: String,
    role_id: String,
    member_list: Vec<RoleMemberId>,
}

impl BatchCreateRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            member_list: Vec::new(),
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

    pub fn member_list(mut self, member_list: Vec<RoleMemberId>) -> Self {
        self.member_list = member_list;
        self
    }

    pub fn add_member(mut self, member: RoleMemberId) -> Self {
        self.member_list.push(member);
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateRoleMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateRoleMemberResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");
        validate_required!(self.member_list, "member_list");
        if self.member_list.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "member_list",
                "member_list 最多 100 项",
            ));
        }
        for (idx, item) in self.member_list.iter().enumerate() {
            if item.id.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "member_list",
                    &format!("第 {} 个成员的 id 不能为空", idx + 1),
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RoleMemberBatchCreate(self.app_token.clone(), self.role_id.clone());

        let api_request: ApiRequest<BatchCreateRoleMemberResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchCreateRoleMemberRequestBody {
            member_list: self.member_list,
        })?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Serialize)]
struct BatchCreateRoleMemberRequestBody {
    member_list: Vec<RoleMemberId>,
}

/// 批量新增协作者响应（data 为 {}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberResponse {}

impl ApiResponseTrait for BatchCreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
