//! 获取关联组织双方共享成员范围
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collboration_share_entity/list>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list-3>
//!
//! 必填查询参数 target_tenant_key；可选 target_department_id / target_group_id /
//! is_select_subject / page_token / page_size。

use std::collections::HashMap;

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织双方共享成员范围 Builder
#[derive(Debug, Clone)]
pub struct CollaborationShareEntityListRequestBuilder {
    config: Config,
    /// 对方组织的 tenant key（必填查询参数）
    target_tenant_key: String,
    /// 指定部门下钻查询（不填查询整个组织的分享范围）
    target_department_id: Option<String>,
    /// 获取用户组下的成员（填写后忽略 target_department_id）
    target_group_id: Option<String>,
    /// 是否主体组织分享范围（默认是客体组织的分享范围）
    is_select_subject: Option<bool>,
    /// 分页标记（第一次请求不填）
    page_token: Option<String>,
    /// 分页大小（0~100，默认 100）
    page_size: Option<u32>,
}

impl CollaborationShareEntityListRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `target_tenant_key` 为对方组织的 tenant key，可通过
    /// 「管理员获取所有关联组织列表」接口获取。
    pub fn new(config: Config, target_tenant_key: impl Into<String>) -> Self {
        Self {
            config,
            target_tenant_key: target_tenant_key.into(),
            target_department_id: None,
            target_group_id: None,
            is_select_subject: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置指定部门下钻查询（不填查询整个组织的分享范围）
    pub fn target_department_id(mut self, target_department_id: impl Into<String>) -> Self {
        self.target_department_id = Some(target_department_id.into());
        self
    }

    /// 设置用户组 ID（获取用户组下的成员，填写后忽略 target_department_id）
    pub fn target_group_id(mut self, target_group_id: impl Into<String>) -> Self {
        self.target_group_id = Some(target_group_id.into());
        self
    }

    /// 设置是否主体组织分享范围（默认是客体组织的分享范围）
    pub fn is_select_subject(mut self, is_select_subject: bool) -> Self {
        self.is_select_subject = Some(is_select_subject);
        self
    }

    /// 设置分页标记（第一次请求不填，后续传上一次返回的 page_token）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（0~100，默认 100）
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationShareEntityListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationShareEntityListResponse> {
        validate_required!(self.target_tenant_key, "target_tenant_key 不能为空");

        let url = "/open-apis/directory/v1/share_entities";

        let req: ApiRequest<CollaborationShareEntityListResponse> = ApiRequest::get(url)
            .query("target_tenant_key", self.target_tenant_key)
            .query_opt("target_department_id", self.target_department_id)
            .query_opt("target_group_id", self.target_group_id)
            .query_opt(
                "is_select_subject",
                self.is_select_subject.map(|v| v.to_string()),
            )
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));
        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "获取关联组织双方共享成员范围",
        )
        .await
    }
}

/// i18n 文本（官方 i18n_text）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct I18nText {
    /// 默认值
    pub default_value: String,
    /// 国际化值（key 为 zh_cn/ja_jp/en_us）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<HashMap<String, String>>,
}

/// 分享的部门信息（官方 share_department）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareDepartment {
    /// 部门 open ID
    pub open_department_id: String,
    /// 部门名称
    pub name: I18nText,
}

/// 分享的用户组信息（官方 share_group）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareGroup {
    /// 用户组 open ID
    pub open_group_id: String,
    /// 用户组名称
    pub name: I18nText,
}

/// 用户头像（官方 image_link）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareUserAvatar {
    /// 72*72 像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 240*240 像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 640*640 像素头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 原始头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 分享的用户信息（官方 share_user）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareUser {
    /// 用户 open ID
    pub open_user_id: String,
    /// 用户名称
    pub name: I18nText,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ShareUserAvatar>,
}

/// 获取关联组织双方共享成员范围响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationShareEntityListResponse {
    /// 分页标记（has_more 为 true 时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分享的部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_departments: Option<Vec<ShareDepartment>>,
    /// 分享的用户组信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_groups: Option<Vec<ShareGroup>>,
    /// 分享的用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_users: Option<Vec<ShareUser>>,
}

impl ApiResponseTrait for CollaborationShareEntityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationShareEntityListRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationShareEntityListBuilder = CollaborationShareEntityListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET 携带必填 target_tenant_key 与可选参数 → 官方 share_* 响应结构。
    #[tokio::test]
    async fn test_list_share_entities_sends_required_query() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/directory/v1/share_entities"))
            .and(query_param("target_tenant_key", "test_key"))
            .and(query_param("is_select_subject", "true"))
            .and(query_param("page_size", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more": true,
                    "share_departments": [
                        {
                            "open_department_id": "ou-12121xxxx",
                            "name": {
                                "default_value": "张三",
                                "i18n_value": { "en_us": "test" }
                            }
                        }
                    ],
                    "share_groups": [],
                    "share_users": [
                        {
                            "open_user_id": "ou-12121212",
                            "name": { "default_value": "张三" },
                            "avatar": {
                                "avatar_72": "http://qwed.com",
                                "avatar_240": "http://wssd.com",
                                "avatar_640": "http://wssd.com",
                                "avatar_origin": "https://internal-api/image"
                            }
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CollaborationShareEntityListRequestBuilder::new(config, "test_key")
            .is_select_subject(true)
            .page_size(10)
            .execute()
            .await
            .expect("获取关联组织双方共享成员范围应成功");
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(
            resp.share_departments.as_ref().unwrap()[0].open_department_id,
            "ou-12121xxxx"
        );
        assert_eq!(
            resp.share_users.as_ref().unwrap()[0]
                .avatar
                .as_ref()
                .unwrap()
                .avatar_72
                .as_deref(),
            Some("http://qwed.com")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "GET");
    }

    /// 必填校验：target_tenant_key 为空时不发请求。
    #[tokio::test]
    async fn test_list_share_entities_validates_required_query() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CollaborationShareEntityListRequestBuilder::new(config, " ")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
