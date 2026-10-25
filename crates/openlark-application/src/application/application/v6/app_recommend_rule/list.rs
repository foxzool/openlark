//! 获取当前设置的推荐规则列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_recommend_rule/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取当前设置的推荐规则列表的请求。
#[derive(Debug, Clone)]
pub struct ListAppRecommendRuleRequest {
    config: Arc<Config>,
    /// 查询参数 `page_size`（必填，取值范围 1-50）。
    page_size: Option<i32>,
    /// 查询参数 `page_token`。
    page_token: Option<String>,
    /// 查询参数 `user_id_type`（默认 open_id，可选 open_id/union_id/user_id）。
    user_id_type: Option<String>,
}

/// 获取当前设置的推荐规则列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAppRecommendRuleResponse {
    /// 推荐规则列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AppRecommendRule>>,
    /// 分页标记，当 has_more 为 true 时返回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 推荐规则。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRule {
    /// 推荐规则 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 推荐规则名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 推荐规则启用状态：open-启用 / closed-停用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 推荐规则可见性信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_info: Option<AppRecommendRuleVisibilityInfo>,
    /// 不可移除推荐应用项列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_item_infos: Option<Vec<AppRecommendRuleItemInfo>>,
    /// 可移除推荐应用项列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributed_recommend_item_infos: Option<Vec<AppRecommendRuleItemInfo>>,
}

/// 推荐规则可见性信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRuleVisibilityInfo {
    /// 是否全员可见。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    /// 可见部门 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 可见用户 ID 列表（形态由 `user_id_type` 决定）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 可见用户组 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

/// 推荐应用项。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRuleItemInfo {
    /// 推荐应用项 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 推荐应用项类型：application-应用 / link-链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// 推荐应用项名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 推荐应用项描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 链接类型应用项的跳转链接（应用类型该字段为空）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    /// 应用类型应用项的 app id（链接类型该字段为空）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// 应用项图标链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 链接类型应用项的默认展示语种（应用类型该字段为空）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    /// 应用项的多语种名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<AppRecommendRuleItemI18nName>,
}

/// 应用项的多语种名称。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRecommendRuleItemI18nName {
    /// 简体中文名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 繁体中文（中国香港）名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<String>,
    /// 繁体中文（中国台湾）名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<String>,
    /// 英文名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

impl ApiResponseTrait for ListAppRecommendRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListAppRecommendRuleRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小（必填，取值范围 1-50）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（默认 open_id，可选 open_id/union_id/user_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行获取当前设置的推荐规则列表请求。
    pub async fn execute(self) -> SDKResult<ListAppRecommendRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAppRecommendRuleResponse> {
        // page_size 官方必填，取值范围 [1, 50]（超范围服务端报 210502）。
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=50).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1-50"));
        }

        let req: ApiRequest<ListAppRecommendRuleResponse> =
            ApiRequest::get("/open-apis/application/v6/app_recommend_rules")
                .query("page_size", page_size.to_string())
                .query_opt("page_token", self.page_token)
                .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "获取推荐规则列表").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../app_recommend_rules?page_size=... → 类型化 ListAppRecommendRuleResponse 解析。
    #[tokio::test]
    async fn test_list_app_recommend_rule_returns_typed_rules_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v6/app_recommend_rules"))
            .and(query_param("page_size", "10"))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "rules": [
                        {
                            "id": "7137896480337641492",
                            "name": "管理员小白的推荐规则",
                            "status": "open",
                            "visibility_info": {
                                "is_all": true,
                                "department_ids": ["od-fef5204d44fa03fad66278cb8e8036e4"],
                                "user_ids": ["ou_35101d538d5b3ad10a5f3480f4c27972"],
                                "group_ids": ["7137608198773686291"]
                            },
                            "recommend_item_infos": [
                                {
                                    "item_id": "7137896480337625108",
                                    "item_type": "application",
                                    "name": "审批",
                                    "description": "审批应用的描述",
                                    "link_url": "",
                                    "client_id": "cli_a274440757b8901c",
                                    "icon_url": "https://example.com/icon.webp",
                                    "default_locale": "",
                                    "i18n_name": {
                                        "zh_cn": "审批",
                                        "zh_hk": "審批",
                                        "zh_tw": "審批",
                                        "en_us": "Approval",
                                        "ja_jp": "承認"
                                    }
                                }
                            ],
                            "distributed_recommend_item_infos": []
                        }
                    ],
                    "page_token": "new-e11ee058b4a8ed2881da11ac7e37c4fc",
                    "has_more": true
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = ListAppRecommendRuleRequest::new(config)
            .page_size(10)
            .user_id_type("open_id")
            .execute()
            .await
            .expect("获取推荐规则列表应成功");

        let rules = resp.rules.expect("rules 应存在");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id.as_deref(), Some("7137896480337641492"));
        assert_eq!(rules[0].name.as_deref(), Some("管理员小白的推荐规则"));
        assert_eq!(rules[0].status.as_deref(), Some("open"));
        let visibility = rules[0]
            .visibility_info
            .as_ref()
            .expect("visibility_info 应存在");
        assert_eq!(visibility.is_all, Some(true));
        assert_eq!(visibility.department_ids.as_ref().map(|v| v.len()), Some(1));
        let items = rules[0]
            .recommend_item_infos
            .as_ref()
            .expect("recommend_item_infos 应存在");
        assert_eq!(items[0].item_id.as_deref(), Some("7137896480337625108"));
        assert_eq!(items[0].item_type.as_deref(), Some("application"));
        assert_eq!(
            items[0].i18n_name.as_ref().unwrap().zh_cn.as_deref(),
            Some("审批")
        );
        assert_eq!(
            resp.page_token.as_deref(),
            Some("new-e11ee058b4a8ed2881da11ac7e37c4fc")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/app_recommend_rules"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(query.contains("page_size=10"));
        assert!(query.contains("user_id_type=open_id"));
    }

    /// page_size 官方必填：缺省报校验错误，不发起请求。
    #[tokio::test]
    async fn test_list_app_recommend_rule_requires_page_size() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .enable_token_cache(false)
                .build(),
        );

        let err = ListAppRecommendRuleRequest::new(config)
            .execute()
            .await
            .expect_err("缺少 page_size 应失败");
        assert!(err.to_string().contains("page_size 不能为空"));
    }

    /// page_size 超出官方范围 [1, 50] 报校验错误。
    #[tokio::test]
    async fn test_list_app_recommend_rule_rejects_out_of_range_page_size() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .enable_token_cache(false)
                .build(),
        );

        let err = ListAppRecommendRuleRequest::new(config)
            .page_size(51)
            .execute()
            .await
            .expect_err("page_size=51 应失败");
        assert!(err.to_string().contains("1-50"));
    }
}
