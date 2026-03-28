//! 批量移除用户组成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    endpoints::CONTACT_V3_GROUP,
};

/// 批量移除成员信息（请求体 members[] 元素）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListItem {
    pub member_id: String,
    pub member_id_type: String,
    pub member_type: String,
}

/// 批量移除用户组成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRemoveGroupMembersBody {
    pub members: Vec<MemberListItem>,
}

impl BatchRemoveGroupMembersBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn member(
        mut self,
        member_id: impl Into<String>,
        member_id_type: impl Into<String>,
    ) -> Self {
        self.members.push(MemberListItem {
            member_id: member_id.into(),
            member_id_type: member_id_type.into(),
            member_type: "user".to_string(),
        });
        self
    }
}

/// 批量移除用户组成员请求
///
/// 用于从指定用户组批量移除成员。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `group_id`: 用户组 ID，必填
///
/// # 请求体字段
///
/// - `members`: 成员列表，至少包含 1 个成员
///   - `member_id`: 成员 ID
///   - `member_id_type`: 成员 ID 类型
///   - `member_type`: 成员类型（目前仅支持 user）
///
/// # 示例
///
/// ```rust,ignore
/// let body = BatchRemoveGroupMembersBody::new()
///     .member("user_1", "open_id")
///     .member("user_2", "open_id");
/// let request = BatchRemoveGroupMembersRequest::new(config)
///     .group_id("group_xxx")
///     .execute(body).await?;
/// ```
pub struct BatchRemoveGroupMembersRequest {
    config: Config,
    group_id: String,
}

impl BatchRemoveGroupMembersRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove>
    pub async fn execute(self, body: BatchRemoveGroupMembersBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchRemoveGroupMembersBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.group_id, "group_id 不能为空");
        if body.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members 不能为空".to_string(),
                "members 至少需要包含 1 个成员".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/group/:group_id/member/batch_remove
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/member/batch_remove",
            CONTACT_V3_GROUP, self.group_id
        ))
        .body(serialize_params(&body, "批量移除用户组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量移除用户组成员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_remove_group_members_request_builder() {
        let config = Config::default();
        let request = BatchRemoveGroupMembersRequest::new(config).group_id("group_xxx");
        assert_eq!(request.group_id, "group_xxx");
    }

    #[test]
    fn test_batch_remove_group_members_body_builder() {
        let body = BatchRemoveGroupMembersBody::new()
            .member("user_1", "open_id")
            .member("user_2", "open_id");
        assert_eq!(body.members.len(), 2);
        assert_eq!(body.members[0].member_id, "user_1");
    }

    #[test]
    fn test_batch_remove_group_members_body_default() {
        let body = BatchRemoveGroupMembersBody::new();
        assert_eq!(body.members.len(), 0);
    }

    #[test]
    fn test_batch_remove_group_members_request_default_values() {
        let config = Config::default();
        let request = BatchRemoveGroupMembersRequest::new(config);
        assert_eq!(request.group_id, "");
    }

    #[test]
    fn test_member_list_item_structure() {
        let item = MemberListItem {
            member_id: "user_123".to_string(),
            member_id_type: "open_id".to_string(),
            member_type: "user".to_string(),
        };
        assert_eq!(item.member_id, "user_123");
        assert_eq!(item.member_type, "user");
    }
}
