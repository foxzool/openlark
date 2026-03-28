//! 移除用户组成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group-member/remove>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_GROUP,
};

/// 移除用户组成员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveGroupMemberBody {
    pub member_type: String,
    pub member_id: String,
    pub member_id_type: String,
}

/// 移除用户组成员请求
///
/// 用于从指定用户组移除单个成员。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `group_id`: 用户组 ID，必填
///
/// # 请求参数
///
/// - `member_id_type`: 成员 ID 类型
/// - `member_id`: 成员 ID
///
/// # 示例
///
/// ```rust,ignore
/// let request = RemoveGroupMemberRequest::new(config)
///     .group_id("group_xxx")
///     .execute(UserIdType::OpenId, "user_xxx").await?;
/// ```
pub struct RemoveGroupMemberRequest {
    config: Config,
    group_id: String,
}

impl RemoveGroupMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口目前仅支持 `member_type=user`。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group-member/remove>
    pub async fn execute(
        self,
        member_id_type: UserIdType,
        member_id: impl Into<String>,
    ) -> SDKResult<EmptyData> {
        self.execute_with_options(
            member_id_type,
            member_id,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    pub async fn execute_with_options(
        self,
        member_id_type: UserIdType,
        member_id: impl Into<String>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.group_id, "group_id 不能为空");
        let member_id = member_id.into();
        validate_required!(member_id, "member_id 不能为空");

        // url: POST:/open-apis/contact/v3/group/:group_id/member/remove
        let body = RemoveGroupMemberBody {
            member_type: "user".to_string(),
            member_id,
            member_id_type: member_id_type.as_str().to_string(),
        };
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/member/remove",
            CONTACT_V3_GROUP, self.group_id
        ))
        .body(serialize_params(&body, "移除用户组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "移除用户组成员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_group_member_request_builder() {
        let config = Config::default();
        let request = RemoveGroupMemberRequest::new(config).group_id("group_xxx");
        assert_eq!(request.group_id, "group_xxx");
    }

    #[test]
    fn test_remove_group_member_request_default_values() {
        let config = Config::default();
        let request = RemoveGroupMemberRequest::new(config);
        assert_eq!(request.group_id, "");
    }

    #[test]
    fn test_remove_group_member_body_structure() {
        let body = RemoveGroupMemberBody {
            member_type: "user".to_string(),
            member_id: "user_123".to_string(),
            member_id_type: "open_id".to_string(),
        };
        assert_eq!(body.member_type, "user");
        assert_eq!(body.member_id, "user_123");
    }

    #[test]
    fn test_remove_group_member_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = RemoveGroupMemberRequest::new(config.clone()).group_id("group_111");
        let request2 = RemoveGroupMemberRequest::new(config).group_id("group_222");
        assert_eq!(request1.group_id, "group_111");
        assert_eq!(request2.group_id, "group_222");
    }
}
