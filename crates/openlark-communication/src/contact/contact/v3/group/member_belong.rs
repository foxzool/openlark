//! 查询用户所属用户组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/member_belong>

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::group::models::MemberBelongGroupsResponse,
    contact::contact::v3::user::models::UserIdType, endpoints::CONTACT_V3_GROUP_MEMBER_BELONG,
};

/// 查询用户所属用户组请求
///
/// 用于查询指定用户所属的所有用户组。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `member_id`: 成员 ID，必填
/// - `member_id_type`: 成员 ID 类型（可选，默认 open_id）
/// - `group_type`: 用户组类型（可选）
/// - `page_size`: 分页大小（可选）
/// - `page_token`: 分页标记（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = MemberBelongGroupsRequest::new(config)
///     .member_id("user_xxx")
///     .group_type(1)
///     .page_size(20);
/// ```
pub struct MemberBelongGroupsRequest {
    config: Config,
    member_id: Option<String>,
    member_id_type: Option<UserIdType>,
    group_type: Option<i32>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl MemberBelongGroupsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            member_id: None,
            member_id_type: None,
            group_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 成员 ID（查询参数，必填）
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_id = Some(member_id.into());
        self
    }

    /// 成员 ID 类型（查询参数，可选，默认 open_id）
    pub fn member_id_type(mut self, member_id_type: UserIdType) -> Self {
        self.member_id_type = Some(member_id_type);
        self
    }

    /// 用户组类型（查询参数，可选）
    pub fn group_type(mut self, group_type: i32) -> Self {
        self.group_type = Some(group_type);
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/member_belong>
    pub async fn execute(self) -> SDKResult<MemberBelongGroupsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MemberBelongGroupsResponse> {
        // === 必填字段验证 ===
        let member_id = self.member_id.ok_or_else(|| {
            error::validation_error(
                "member_id 不能为空".to_string(),
                "member_id 为必填查询参数".to_string(),
            )
        })?;

        // url: GET:/open-apis/contact/v3/group/member_belong
        let mut req: ApiRequest<MemberBelongGroupsResponse> =
            ApiRequest::get(CONTACT_V3_GROUP_MEMBER_BELONG).query("member_id", member_id);

        if let Some(member_id_type) = self.member_id_type {
            req = req.query("member_id_type", member_id_type.as_str());
        }
        if let Some(group_type) = self.group_type {
            req = req.query("group_type", group_type.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询用户所属用户组")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_member_belong_groups_request_builder() {
        let config = Config::default();
        let request = MemberBelongGroupsRequest::new(config).member_id("user_xxx");
        assert_eq!(request.member_id, Some("user_xxx".to_string()));
    }

    #[test]
    fn test_member_belong_groups_request_with_member_id_type() {
        let config = Config::default();
        let request = MemberBelongGroupsRequest::new(config)
            .member_id("user_xxx")
            .member_id_type(UserIdType::OpenId);
        assert_eq!(request.member_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_member_belong_groups_request_with_group_type() {
        let config = Config::default();
        let request = MemberBelongGroupsRequest::new(config)
            .member_id("user_xxx")
            .group_type(1);
        assert_eq!(request.group_type, Some(1));
    }

    #[test]
    fn test_member_belong_groups_request_default_values() {
        let config = Config::default();
        let request = MemberBelongGroupsRequest::new(config);
        assert_eq!(request.member_id, None);
        assert_eq!(request.member_id_type, None);
        assert_eq!(request.page_size, None);
    }

    #[test]
    fn test_member_belong_groups_request_with_all_options() {
        let config = Config::default();
        let request = MemberBelongGroupsRequest::new(config)
            .member_id("user_123")
            .member_id_type(UserIdType::UnionId)
            .group_type(2)
            .page_size(50)
            .page_token("token789");
        assert_eq!(request.member_id, Some("user_123".to_string()));
        assert_eq!(request.member_id_type, Some(UserIdType::UnionId));
        assert_eq!(request.group_type, Some(2));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token789".to_string()));
    }
}
