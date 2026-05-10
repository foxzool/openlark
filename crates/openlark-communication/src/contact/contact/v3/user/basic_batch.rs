//! 获取用户姓名
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/basic_batch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS_BASIC_BATCH,
};

/// 国际化用户名。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct I18nName {
    /// 中文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 日文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    /// 英文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 用户基础信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BasicUser {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 国际化姓名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<I18nName>,
}

/// 获取用户姓名请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BasicBatchUsersBody {
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
}

/// 获取用户姓名响应 data。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BasicBatchUsersResponse {
    /// 用户基础信息列表。
    #[serde(default)]
    pub users: Vec<BasicUser>,
}

impl ApiResponseTrait for BasicBatchUsersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户姓名请求。
pub struct BasicBatchUsersRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID 列表。
    user_ids: Vec<String>,
    /// 用户 ID 类型。
    user_id_type: Option<UserIdType>,
}

impl BasicBatchUsersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            user_id_type: None,
        }
    }

    /// 追加单个用户 ID。
    pub fn push_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 批量设置用户 ID。
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 使用默认请求选项执行请求。
    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<BasicBatchUsersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BasicBatchUsersResponse> {
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空");
        if self.user_ids.len() > 10 {
            return Err(error::validation_error(
                "user_ids 超出上限".to_string(),
                "user_ids 长度不能超过 10".to_string(),
            ));
        }

        let body = BasicBatchUsersBody {
            user_ids: self.user_ids,
        };
        let mut req: ApiRequest<BasicBatchUsersResponse> =
            ApiRequest::post(CONTACT_V3_USERS_BASIC_BATCH)
                .body(serialize_params(&body, "获取用户姓名")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取用户姓名")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::req_option::RequestOption;

    #[test]
    fn test_basic_batch_users_request_defaults() {
        let request = BasicBatchUsersRequest::new(Config::default());
        assert!(request.user_ids.is_empty());
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_basic_batch_users_request_collects_user_ids() {
        let request = BasicBatchUsersRequest::new(Config::default())
            .push_user_id("ou_1")
            .user_ids(["ou_2", "ou_3"]);
        assert_eq!(request.user_ids, vec!["ou_1", "ou_2", "ou_3"]);
    }

    #[test]
    fn test_basic_batch_users_request_accepts_user_id_type() {
        let request = BasicBatchUsersRequest::new(Config::default())
            .push_user_id("ou_1")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_basic_batch_users_response_deserialization() {
        let json = r#"{
            "users": [{
                "user_id": "ou_123",
                "name": "张三",
                "i18n_name": {
                    "zh_cn": "张三",
                    "ja_jp": "佐藤はるか",
                    "en_us": "Jason Zhang"
                }
            }]
        }"#;
        let response: BasicBatchUsersResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.users.len(), 1);
        assert_eq!(response.users[0].name.as_deref(), Some("张三"));
        assert_eq!(
            response.users[0]
                .i18n_name
                .as_ref()
                .and_then(|name| name.en_us.as_deref()),
            Some("Jason Zhang")
        );
    }

    #[tokio::test]
    async fn test_basic_batch_users_request_rejects_empty_user_ids() {
        let result = BasicBatchUsersRequest::new(Config::default())
            .execute_with_options(RequestOption::default())
            .await;
        assert!(result.is_err());
        let error = result.expect_err("缺少 user_ids 应该返回错误").to_string();
        assert!(error.contains("user_ids"));
    }

    #[tokio::test]
    async fn test_basic_batch_users_request_rejects_more_than_ten_user_ids() {
        let user_ids = (0..11).map(|idx| format!("ou_{idx}"));
        let result = BasicBatchUsersRequest::new(Config::default())
            .user_ids(user_ids)
            .execute_with_options(RequestOption::default())
            .await;
        assert!(result.is_err());
        let error = result
            .expect_err("user_ids 超过 10 个应该返回错误")
            .to_string();
        assert!(error.contains("10"));
    }
}
