//! 获取组织类角色授权列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/assigned_user/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取组织类角色授权列表请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: SearchBody,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: SearchBody::default(),
        }
    }

    /// 设置角色 ID `role_id`（必填）。
    pub fn role_id(mut self, role_id: String) -> Self {
        self.body.role_id = role_id;
        self
    }

    /// 设置管理范围列表 `management_scope_list`（必填）。
    pub fn management_scope_list(mut self, list: Vec<ManagementScopeItem>) -> Self {
        self.body.management_scope_list = list;
        self
    }

    /// 追加单个管理范围。
    pub fn management_scope(mut self, item: ManagementScopeItem) -> Self {
        self.body.management_scope_list.push(item);
        self
    }

    /// 设置查找方式 `search_method`（必填：1 只查找指定范围 / 2 无授权时向上查找）。
    pub fn search_method(mut self, search_method: String) -> Self {
        self.body.search_method = search_method;
        self
    }

    /// 设置分页大小 `page_size`（必填，示例值 100）。
    pub fn page_size(mut self, page_size: String) -> Self {
        self.body.page_size = page_size;
        self
    }

    /// 设置分页标记 `page_token`（可选）。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.body.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.body.role_id, "role_id 不能为空");
        validate_required!(
            self.body.management_scope_list,
            "management_scope_list 不能为空"
        );
        for scope in &self.body.management_scope_list {
            validate_required!(
                scope.management_dimension,
                "management_scope_list.management_dimension 不能为空"
            );
            validate_required!(scope.obj_id, "management_scope_list.obj_id 不能为空");
        }
        validate_required!(self.body.search_method, "search_method 不能为空");
        validate_required!(self.body.page_size, "page_size 不能为空");

        let api_endpoint = FeishuPeopleApiV1::AssignedUserSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取组织类角色授权列表响应数据为空",
        )
        .await
    }
}

/// 获取组织类角色授权列表请求体（`role_id`/`management_scope_list`/`search_method`/`page_size` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBody {
    /// 角色 ID
    pub role_id: String,
    /// 管理范围列表
    pub management_scope_list: Vec<ManagementScopeItem>,
    /// 查找方式：1 只查找指定 部门/工作地点/公司/社保城市；2 无授权信息时向上查找第一个授权记录
    pub search_method: String,
    /// 分页大小
    pub page_size: String,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 请求体管理范围条目（`management_dimension`/`obj_id` 必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManagementScopeItem {
    /// 管理维度（department 部门 / work_location 工作地点 / company 公司 / social_security_location 社保城市）
    pub management_dimension: String,
    /// 被授权部门/工作地点/公司/社保城市 ID
    pub obj_id: String,
}

/// 获取组织类角色授权列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AssignedUserItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 组织类角色授权条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AssignedUserItem {
    /// 员工 ID，可通过【搜索员工信息】接口获取详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 实际授权管理范围
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_scope_list: Option<Vec<ManagementScope>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 实际授权管理范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ManagementScope {
    /// 管理维度（department 部门 / work_location 工作地点 / company 公司 / social_security_location 社保城市）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_dimension: Option<String>,
    /// 被授权部门/工作地点/公司/社保城市 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_id: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/assigned_users/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/assigned_users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "6967639606963471902",
                            "management_scope_list": [
                                {
                                    "management_dimension": "department",
                                    "obj_id": "7023318326176777742"
                                }
                            ]
                        }
                    ],
                    "has_more": true,
                    "page_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
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

        let resp = SearchRequest::new(config)
            .role_id("100".to_string())
            .management_scope(ManagementScopeItem {
                management_dimension: "department".to_string(),
                obj_id: "7023318326176777742".to_string(),
            })
            .search_method("1".to_string())
            .page_size("100".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解析出授权列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].employment_id.as_deref(),
            Some("6967639606963471902")
        );
        let scopes = items[0]
            .management_scope_list
            .as_ref()
            .expect("management_scope_list 应解析");
        assert_eq!(scopes.len(), 1);
        assert_eq!(
            scopes[0].management_dimension.as_deref(),
            Some("department")
        );
        assert_eq!(scopes[0].obj_id.as_deref(), Some("7023318326176777742"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致（官方请求体示例形状）
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "role_id": "100",
                "management_scope_list": [
                    { "management_dimension": "department", "obj_id": "7023318326176777742" }
                ],
                "search_method": "1",
                "page_size": "100"
            })
        );
    }

    /// 必填校验：缺必填字段快速失败
    #[tokio::test]
    async fn test_search_missing_required_fields_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = SearchRequest::new(config)
            .execute()
            .await
            .expect_err("缺必填字段应被拒绝");
        assert!(err.to_string().contains("role_id"), "实际: {err}");
    }
}
