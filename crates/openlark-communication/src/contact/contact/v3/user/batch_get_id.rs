//! 通过手机号或邮箱获取用户 ID
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/batch_get_id>

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS_BATCH_GET_ID,
};

/// 通过手机号或邮箱获取用户 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetIdBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
}

/// 用户状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus {
    #[serde(default)]
    pub is_frozen: bool,
    #[serde(default)]
    pub is_resigned: bool,
    #[serde(default)]
    pub is_activated: bool,
    #[serde(default)]
    pub is_exited: bool,
    #[serde(default)]
    pub is_unjoin: bool,
}

/// 用户 ID 查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 通过手机号或邮箱获取用户 ID 响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdResponse {
    #[serde(default)]
    pub user_list: Vec<BatchGetIdItem>,
}

impl ApiResponseTrait for BatchGetIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通过手机号或邮箱获取用户 ID 请求
///
/// 用于通过手机号或邮箱批量查询对应的用户 ID。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 请求体字段
///
/// - `emails`: 邮箱列表（可选）
/// - `mobiles`: 手机号列表（可选）
/// - `include_resigned`: 是否包含离职用户（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = BatchGetIdBody {
///     emails: Some(vec!["test@example.com".to_string()]),
///     mobiles: None,
///     include_resigned: Some(false),
/// };
/// let request = BatchGetIdRequest::new(config)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct BatchGetIdRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
}

impl BatchGetIdRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/batch_get_id>
    pub async fn execute(self, body: BatchGetIdBody) -> SDKResult<BatchGetIdResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchGetIdBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetIdResponse> {
        // === 必填字段验证 ===
        let has_emails = body.emails.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
        let has_mobiles = body
            .mobiles
            .as_ref()
            .map(|v| !v.is_empty())
            .unwrap_or(false);
        if !has_emails && !has_mobiles {
            return Err(error::validation_error(
                "emails/mobiles 不能为空".to_string(),
                "emails 与 mobiles 至少需要提供其中一个".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/users/batch_get_id
        let mut req: ApiRequest<BatchGetIdResponse> =
            ApiRequest::post(CONTACT_V3_USERS_BATCH_GET_ID)
                .body(serialize_params(&body, "通过手机号或邮箱获取用户 ID")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "通过手机号或邮箱获取用户 ID")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_id_request_builder() {
        let config = Config::default();
        let request = BatchGetIdRequest::new(config);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_batch_get_id_request_with_user_id_type() {
        let config = Config::default();
        let request = BatchGetIdRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_batch_get_id_body_with_emails() {
        let body = BatchGetIdBody {
            emails: Some(vec!["test@example.com".to_string()]),
            mobiles: None,
            include_resigned: Some(false),
        };
        assert_eq!(body.emails, Some(vec!["test@example.com".to_string()]));
    }

    #[test]
    fn test_batch_get_id_body_with_mobiles() {
        let body = BatchGetIdBody {
            emails: None,
            mobiles: Some(vec!["13800138000".to_string()]),
            include_resigned: None,
        };
        assert_eq!(body.mobiles, Some(vec!["13800138000".to_string()]));
    }
}
