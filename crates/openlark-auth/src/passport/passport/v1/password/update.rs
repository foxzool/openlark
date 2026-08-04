//! 重置登录密码
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/passport-v1/password/update>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 用户 ID 类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassportUserIdType {
    /// 应用内用户标识。
    #[serde(rename = "open_id")]
    OpenId,
    /// 开发商维度用户标识。
    #[serde(rename = "union_id")]
    UnionId,
    /// 租户内用户标识。
    #[serde(rename = "user_id")]
    UserId,
}

impl PassportUserIdType {
    /// 获取查询参数值。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
        }
    }
}

/// 重置登录密码请求体。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdatePasswordBody {
    /// 待重置密码的用户 ID，类型需与 `user_id_type` 一致。
    pub user_id: String,
    /// 新密码，不少于 8 个字符，且至少包含字母、数字、符号中的两类。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 是否要求用户下次登录时重新设置密码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_reset: Option<bool>,
}

/// 重置登录密码响应数据。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdatePasswordResponse {}

impl ApiResponseTrait for UpdatePasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 重置登录密码请求。
#[derive(Debug, Clone)]
pub struct UpdatePasswordRequest {
    config: Config,
    user_id_type: Option<PassportUserIdType>,
}

impl UpdatePasswordRequest {
    /// 创建重置登录密码请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型；未设置时服务端默认使用 `open_id`。
    pub fn user_id_type(mut self, user_id_type: PassportUserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self, body: UpdatePasswordBody) -> SDKResult<UpdatePasswordResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdatePasswordBody,
        option: RequestOption,
    ) -> SDKResult<UpdatePasswordResponse> {
        validate_required!(body.user_id, "user_id 不能为空");

        let mut req: ApiRequest<UpdatePasswordResponse> =
            ApiRequest::put(PassportApiV1::PasswordUpdate.path())
                .body(serde_json::to_value(&body)?)
                .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        Transport::request_typed(req, &self.config, Some(option), "重置登录密码").await
    }
}
