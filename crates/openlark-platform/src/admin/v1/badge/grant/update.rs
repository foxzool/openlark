//! 修改勋章授予名单 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update>

use super::create::BadgeGrantRuleDetail;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改勋章授予名单的请求构建器。
pub struct UpdateBadgeGrantRequestBuilder {
    badge_id: String,
    grant_id: String,
    name: String,
    grant_type: i32,
    time_zone: String,
    rule_detail: BadgeGrantRuleDetail,
    is_grant_all: bool,
    user_ids: Option<Vec<String>>,
    department_ids: Option<Vec<String>>,
    group_ids: Option<Vec<String>>,
    config: Config,
}

impl UpdateBadgeGrantRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            name: String::new(),
            grant_type: 0,
            time_zone: String::new(),
            rule_detail: BadgeGrantRuleDetail::default(),
            is_grant_all: false,
            user_ids: None,
            department_ids: None,
            group_ids: None,
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置授予记录 ID。
    pub fn grant_id(mut self, grant_id: impl Into<String>) -> Self {
        self.grant_id = grant_id.into();
        self
    }

    /// 设置授予事项名称（勋章下唯一，最多 100 个字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置授予名单类型（0：手动选择有效期；1：匹配系统入职时间）。
    pub fn grant_type(mut self, grant_type: i32) -> Self {
        self.grant_type = grant_type;
        self
    }

    /// 设置生效时区（TZ database name，如 Asia/Shanghai）。
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = time_zone.into();
        self
    }

    /// 设置规则详情。
    pub fn rule_detail(mut self, rule_detail: BadgeGrantRuleDetail) -> Self {
        self.rule_detail = rule_detail;
        self
    }

    /// 设置是否授予给全员。
    pub fn is_grant_all(mut self, is_grant_all: bool) -> Self {
        self.is_grant_all = is_grant_all;
        self
    }

    /// 设置授予的用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// 设置授予的部门 ID 列表。
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置授予的用户组 ID 列表。
    pub fn group_ids(mut self, group_ids: Vec<String>) -> Self {
        self.group_ids = Some(group_ids);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeGrantResponse> {
        validate_required!(self.badge_id, "勋章 ID 不能为空");
        validate_required!(self.grant_id, "授予名单 ID 不能为空");
        validate_required!(self.name, "授予事项名称不能为空");
        validate_required!(self.time_zone, "时区不能为空");

        let user_count = self.user_ids.as_ref().map_or(0, Vec::len)
            + self.department_ids.as_ref().map_or(0, Vec::len)
            + self.group_ids.as_ref().map_or(0, Vec::len);
        if !self.is_grant_all && user_count == 0 {
            return Err(openlark_core::error::validation_error(
                "user_ids",
                "非全员授予类型名单需关联 1~500 个用户群体（用户/部门/用户组）",
            ));
        }

        let url = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            self.badge_id, self.grant_id
        );
        let request_body = UpdateBadgeGrantBody {
            name: self.name,
            grant_type: self.grant_type,
            time_zone: self.time_zone,
            rule_detail: self.rule_detail,
            is_grant_all: self.is_grant_all,
            user_ids: self.user_ids,
            department_ids: self.department_ids,
            group_ids: self.group_ids,
        };
        let api_request: ApiRequest<UpdateBadgeGrantResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "修改勋章授予名单").await
    }
}

/// 修改授予名单请求体
#[derive(Debug, Serialize)]
pub struct UpdateBadgeGrantBody {
    /// 勋章下唯一的授予事项，最多 100 个字符
    pub name: String,
    /// 授予名单类型（0：手动选择有效期；1：匹配系统入职时间）
    pub grant_type: i32,
    /// 授予名单的生效时间对应的时区（TZ database name）
    pub time_zone: String,
    /// 规则详情
    pub rule_detail: BadgeGrantRuleDetail,
    /// 是否授予给全员
    pub is_grant_all: bool,
    /// 授予的用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 授予的部门 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 授予的用户组 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章授予名单的响应。
pub struct UpdateBadgeGrantResponse {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UpdateBadgeGrantRequestBuilder, will be removed in v1.0 (#271)")]
pub type UpdateBadgeGrantBuilder = UpdateBadgeGrantRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PUT .../badges/{badge_id}/grants/{grant_id} → 强类型 UpdateBadgeGrantResponse。
    #[tokio::test]
    async fn test_update_badge_grant_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/admin/v1/badges/badge_001/grants/grant_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "grant_id": "grant_001",
                    "badge_id": "badge_001",
                    "user_ids": ["u_001", "u_002"],
                    "update_time": "1717000000"
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

        let resp = UpdateBadgeGrantRequestBuilder::new(config)
            .badge_id("badge_001")
            .grant_id("grant_001")
            .name("激励勋章的授予名单")
            .grant_type(0)
            .time_zone("Asia/Shanghai")
            .rule_detail(BadgeGrantRuleDetail {
                effective_time: Some("1649606400".to_string()),
                expiration_time: Some("1649692799".to_string()),
                anniversary: None,
                effective_period: None,
            })
            .is_grant_all(false)
            .user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .execute()
            .await
            .expect("修改勋章授予名单应成功");
        assert_eq!(resp.grant_id, "grant_001");
        assert_eq!(resp.badge_id, "badge_001");
        assert_eq!(
            resp.user_ids,
            vec!["u_001".to_string(), "u_002".to_string()]
        );
        assert_eq!(resp.update_time, "1717000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "PUT");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/admin/v1/badges/badge_001/grants/grant_001"
        );
        // 请求体须与官方文档一致：name/grant_type/time_zone/rule_detail/is_grant_all 必填
        let body: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(body["name"], "激励勋章的授予名单");
        assert_eq!(body["grant_type"], 0);
        assert_eq!(body["time_zone"], "Asia/Shanghai");
        assert_eq!(body["rule_detail"]["effective_time"], "1649606400");
        assert_eq!(body["is_grant_all"], false);
        assert_eq!(body["user_ids"][0], "u_001");
    }
}
