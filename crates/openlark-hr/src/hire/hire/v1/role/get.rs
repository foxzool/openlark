//! 获取角色详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/role/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::{CodeNameObject, I18nText, IdNameObject};
use crate::hire::hire::v1::role::list::RoleItem;

/// 权限集合（官方 `permission_collection`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PermissionCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 功能权限列表。
    pub feature_permissions: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理权限列表。
    pub management_permissions: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 数据权限列表。
    pub data_permissions: Option<Vec<DataPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 业务管理范围列表。
    pub business_management_scopes: Option<Vec<RoleBusinessManagementScope>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 数据权限（官方 `data_permission`：id_name_object + select_status）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DataPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 权限点 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 权限点名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 数据权限状态：0 不可见 / 1 可见 / 2 可编辑。
    pub select_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 角色业务管理范围（官方 `business_management_scope`，entity + permission_groups）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleBusinessManagementScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实体信息。
    pub entity: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 权限分组列表。
    pub permission_groups: Option<Vec<RolePermissionGroup>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 权限分组（官方 `permission_group_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RolePermissionGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 权限点 ID 列表。
    pub permission_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理范围规则。
    pub scope_rule: Option<RoleScopeRule>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 管理范围规则（官方 `permission_scope_rule`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleScopeRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 规则类型：0 无数据权限 / 1 全部数据权限 / 2 按规则指定范围。
    pub rule_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取角色详情请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    role_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
        }
    }

    /// 设置 `role_id`。
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        validate_required!(self.role_id.trim(), "role_id 不能为空");

        let request =
            ApiRequest::<GetResponse>::get(format!("/open-apis/hire/v1/roles/{}", self.role_id));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取角色详情响应数据为空",
        )
        .await
    }
}

/// 获取角色详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `role` 字段。
    pub role: Option<RoleItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
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

    /// 端到端：GET /open-apis/hire/v1/roles/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/roles/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "role": {
                        "id": "6930815272790114324",
                        "name": { "zh_cn": "招聘 HRBP", "en_us": "Recruitment HRBP" },
                        "description": { "zh_cn": "赋予HRBP的权限", "en_us": "Authority given to HRBP" },
                        "modify_time": "1716535727510",
                        "role_status": 1,
                        "role_type": 1,
                        "scope_of_application": 1,
                        "has_business_management_scope": true,
                        "socail_permission_collection": {
                            "feature_permissions": [
                                { "id": "10101000", "name": { "zh_cn": "查看人才", "en_us": "View talent" } }
                            ],
                            "management_permissions": [
                                { "id": "20101001", "name": { "zh_cn": "门店管理", "en_us": "Store management" } }
                            ],
                            "data_permissions": [
                                { "id": "30203005", "name": { "zh_cn": "私密备注", "en_us": "Private notes" }, "select_status": 0 }
                            ],
                            "business_management_scopes": [
                                {
                                    "entity": { "code": "application", "name": { "zh_cn": "投递", "en_us": "Application" } },
                                    "permission_groups": [
                                        { "permission_ids": ["6930815272790114324"], "scope_rule": { "rule_type": 1 } }
                                    ]
                                }
                            ]
                        }
                    }
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

        let resp = GetRequest::new(config)
            .role_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        // 官方 role 详情含 has_business_management_scope 与 socail/campus 权限集合
        let role = resp.role.as_ref().expect("role 应反序列化");
        assert_eq!(role.has_business_management_scope, Some(true));
        let socail = role
            .socail_permission_collection
            .as_ref()
            .expect("socail_permission_collection 应反序列化");
        assert_eq!(
            socail
                .feature_permissions
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.id.as_deref()),
            Some("10101000")
        );
        assert_eq!(
            socail
                .data_permissions
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.select_status),
            Some(0)
        );
        assert_eq!(
            socail
                .business_management_scopes
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.entity.as_ref())
                .and_then(|v| v.code.as_deref()),
            Some("application")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
