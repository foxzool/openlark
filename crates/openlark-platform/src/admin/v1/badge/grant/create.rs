//! 创建勋章授予名单 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/create>
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create-2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建勋章授予名单请求
pub struct CreateBadgeGrantRequestBuilder {
    badge_id: String,
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

impl CreateBadgeGrantRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
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
    pub async fn execute(self) -> SDKResult<CreateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeGrantResponse> {
        validate_required!(self.badge_id, "勋章 ID 不能为空");
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

        let request_body = CreateBadgeGrantBody {
            name: self.name,
            grant_type: self.grant_type,
            time_zone: self.time_zone,
            rule_detail: self.rule_detail,
            is_grant_all: self.is_grant_all,
            user_ids: self.user_ids,
            department_ids: self.department_ids,
            group_ids: self.group_ids,
        };

        let api_request: ApiRequest<CreateBadgeGrantResponse> = ApiRequest::post(format!(
            "/open-apis/admin/v1/badges/{}/grants",
            self.badge_id
        ))
        .body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "创建勋章授予名单").await
    }
}

/// 创建授予名单请求体
#[derive(Debug, Serialize)]
pub struct CreateBadgeGrantBody {
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

/// 授予名单规则详情（rule_detail）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrantRuleDetail {
    /// 开始生效的时间戳（手动设置有效期类型勋章需要配置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 结束生效的时间戳（永久有效传 "0"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 入职周年日（1～60，根据入职时间发放类型勋章需要配置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anniversary: Option<i32>,
    /// 有效期限（1：一年；2：永久有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_period: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 创建勋章授予名单的响应。
pub struct CreateBadgeGrantResponse {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for CreateBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CreateBadgeGrantRequestBuilder, will be removed in v1.0 (#271)")]
pub type CreateBadgeGrantBuilder = CreateBadgeGrantRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBadgeGrantRequestBuilder::new(config.clone())
            .badge_id("m_DjMzaK".to_string())
            .name("激励勋章的授予名单".to_string())
            .grant_type(0)
            .time_zone("Asia/Shanghai".to_string())
            .rule_detail(BadgeGrantRuleDetail {
                effective_time: Some("1649606400".to_string()),
                expiration_time: Some("1649692799".to_string()),
                anniversary: None,
                effective_period: None,
            })
            .is_grant_all(false)
            .user_ids(vec!["u273y71".to_string()]);
        let _ = request;
    }

    /// 请求体序列化：字段与官方文档一致（name/grant_type/time_zone/rule_detail/is_grant_all 必填）。
    #[test]
    fn test_create_badge_grant_body_serialization() {
        let body = CreateBadgeGrantBody {
            name: "激励勋章的授予名单".to_string(),
            grant_type: 0,
            time_zone: "Asia/Shanghai".to_string(),
            rule_detail: BadgeGrantRuleDetail {
                effective_time: Some("1649606400".to_string()),
                expiration_time: Some("1649692799".to_string()),
                anniversary: Some(1),
                effective_period: Some(1),
            },
            is_grant_all: false,
            user_ids: Some(vec!["u273y71".to_string()]),
            department_ids: None,
            group_ids: None,
        };

        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["name"], "激励勋章的授予名单");
        assert_eq!(value["grant_type"], 0);
        assert_eq!(value["time_zone"], "Asia/Shanghai");
        assert_eq!(value["rule_detail"]["effective_time"], "1649606400");
        assert_eq!(value["rule_detail"]["anniversary"], 1);
        assert_eq!(value["is_grant_all"], false);
        assert_eq!(value["user_ids"][0], "u273y71");
        // 可选字段为 None 时不序列化
        assert!(value.get("department_ids").is_none());
        assert!(value.get("group_ids").is_none());
    }
}
