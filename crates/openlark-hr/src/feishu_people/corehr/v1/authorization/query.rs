//! 批量查询用户授权
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询用户授权请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::AuthorizationQuery;
        let request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询用户授权响应数据为空",
        )
        .await
    }
}

/// 批量查询用户授权响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 查询的用户授权信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RoleAuthorization>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 用户授权信息（官方 role_authorization 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleAuthorization {
    /// 员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 授权列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_detail_list: Option<Vec<PermissionDetail>>,
}

/// 授权详情（官方 permission_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PermissionDetail {
    /// 角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<SecurityGroupRole>,
    /// 指定管理对象列表；如果该值为 null，则使用设置数据权限（grantor_rule_list）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_organization_list: Option<Vec<Vec<AssignedOrganization>>>,
    /// 设置数据权限；如果该值为 null，则使用指定管理对象列表（assigned_organization_list）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grantor_rule_list: Option<Vec<PermissionSecurityGroup>>,
    /// 更新时间（时间戳，单位：s）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 角色（官方 security_group 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SecurityGroupRole {
    /// 角色 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 角色 code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 角色名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NameI18n>,
    /// 状态，1 表示角色启用，2 表示角色停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i64>,
    /// 角色描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<NameI18n>,
    /// 角色类型：3 = 组织类角色，7 = 非组织类角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i64>,
    /// 创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 更新时间，单位为秒（秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 组织管理维度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_truncation: Option<Vec<OrgTruncation>>,
}

/// 多语名称（官方 name 结构：{zh_cn, en_us}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NameI18n {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 组织管理维度（官方 org_truncation 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrgTruncation {
    /// 组织管理维度名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_key: Option<String>,
    /// 下钻类型：0 = 对当前管理维度及下级管理维度均有权限，1 = 只对当前管理维度有权限
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 下钻深度（单位：层）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
}

/// 指定管理对象（官方 assigned_organization 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AssignedOrganization {
    /// 组织 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_key: Option<String>,
    /// 组织名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_name: Option<NameI18n>,
    /// 组织 id，包含被删除/停用的组织 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id_list: Option<Vec<String>>,
}

/// 设置数据权限（官方 permission_security_group 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PermissionSecurityGroup {
    /// 管理维度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_dimension: Option<RuleDimension>,
    /// 管理类型：0 = 无数据权限，1 = 全部数据权限，2 = 被授权的用户自己，3 = 按规则指定范围
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<i64>,
    /// 规则
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<FilterExpression>,
    /// 更新时间（时间戳，单位：s）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 管理维度（官方 rule_dimension 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RuleDimension {
    /// 维度的 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_key: Option<String>,
    /// 维度名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<NameI18n>,
}

/// 规则表达式（官方 filter_expression 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilterExpression {
    /// 筛选条件
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FilterCondition>>,
    /// 表达式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logic_expression: Option<String>,
}

/// 筛选条件（官方 filter_condition 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilterCondition {
    /// 左值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<FilterRuleValue>,
    /// 右值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<FilterRuleValue>,
    /// 操作符
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<i64>,
    /// 右值类型：1 = 指定值，2 = 引用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_value_type: Option<i64>,
}

/// 规则值（官方 filter_rule_value 结构；value 按规则值类型可为字符串或数组，透传 Value）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilterRuleValue {
    /// 规则值类型（value 的类型）：0 = 字符串，1 = 数字，2 = 字符串数组，3 = 数字数组
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 规则值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// 下钻值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_value: Option<Value>,
    /// 下钻类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_type: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/authorizations/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/authorizations/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "6967639606963471902",
                            "permission_detail_list": [
                                {
                                    "role": {
                                        "id": "7034393015968122400",
                                        "code": "department_manager",
                                        "name": { "zh_cn": "部门经理", "en_us": "department manager" },
                                        "active_status": 1,
                                        "group_type": 1,
                                        "org_truncation": [
                                            { "org_key": "department", "type": 0, "depth": 0 }
                                        ]
                                    },
                                    "assigned_organization_list": [
                                        [
                                            {
                                                "org_key": "department",
                                                "org_name": { "zh_cn": "部门", "en_us": "department" },
                                                "org_id_list": ["6967639606963471902"]
                                            }
                                        ]
                                    ],
                                    "grantor_rule_list": [
                                        {
                                            "rule_dimension": {
                                                "entity_key": "user",
                                                "entity_name": { "zh_cn": "员工", "en_us": "User" }
                                            },
                                            "rule_type": 1,
                                            "expression": {
                                                "conditions": [
                                                    {
                                                        "left": { "type": 1, "value": "a", "lookup_value": "1", "lookup_type": "user" },
                                                        "right": { "type": 1, "value": "a", "lookup_value": "1", "lookup_type": "user" },
                                                        "operator": 1,
                                                        "right_value_type": 1
                                                    }
                                                ],
                                                "logic_expression": "1 and 2"
                                            }
                                        }
                                    ],
                                    "update_time": "1720584039"
                                }
                            ]
                        }
                    ],
                    "has_more": true,
                    "page_token": "1234452132"
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

        let resp = QueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].employment_id.as_deref(),
            Some("6967639606963471902")
        );
        let details = items[0]
            .permission_detail_list
            .as_ref()
            .expect("permission_detail_list 应解码");
        assert_eq!(details.len(), 1);
        let role = details[0].role.as_ref().expect("role 应解码");
        assert_eq!(role.code.as_deref(), Some("department_manager"));
        assert_eq!(
            role.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("部门经理")
        );
        let truncation = role.org_truncation.as_ref().expect("org_truncation 应解码");
        assert_eq!(truncation[0].org_key.as_deref(), Some("department"));
        assert_eq!(truncation[0].r#type, Some(0));
        assert_eq!(
            details[0]
                .assigned_organization_list
                .as_ref()
                .and_then(|list| list.first())
                .and_then(|inner| inner.first())
                .and_then(|org| org.org_key.as_deref()),
            Some("department")
        );
        assert_eq!(
            details[0]
                .grantor_rule_list
                .as_ref()
                .and_then(|list| list.first())
                .and_then(|rule| rule.rule_type),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
