//! 查询自定义组织信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/custom_org/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询自定义组织信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
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
        let mut request =
            ApiRequest::<QueryResponse>::post("/open-apis/corehr/v2/custom_orgs/query");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询自定义组织信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 自定义组织信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CustomOrgItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 自定义组织信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomOrgItem {
    /// 组织类型编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    /// 组织名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级自定义组织 ID（查询一级自定义组织时不展示）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 负责人 ID 列表（employment_id 类型）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_ids: Option<Vec<String>>,
    /// 自定义组织描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 当前版本生效日期（YYYY-MM-DD 00:00:00）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 组织角色列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_roles: Option<Vec<OrgRole>>,
    /// 自动匹配的规则组（组间并集、组内交集）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_rule_groups: Option<Vec<MatchRuleGroup>>,
    /// 自定义组织是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 自定义组织 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 组织角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgRole {
    /// 角色 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 角色 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 直接授权的员工 ID 列表（people_corehr_id 类型）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_ids: Option<Vec<String>>,
    /// 因上级组织权限自动获取下级组织权限的员工 ID 列表（people_corehr_id 类型）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherit_employment_ids: Option<Vec<String>>,
}

/// 自动匹配规则组
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchRuleGroup {
    /// 匹配规则列表（组内是交集关系）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_rules: Option<Vec<MatchRule>>,
}

/// 自动匹配规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchRule {
    /// 左值：department / department_hierarchy / work_location / cost_center / job / job_level / job_family / employee_type 等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_value: Option<String>,
    /// 操作符：contains（包含）/ notContains（不包含）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 右值，左值对应的 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_values: Option<Vec<String>>,
}

/// 自定义字段数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 API Name，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：{zh_cn, en_us}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（中文 zh-CN，英文 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

    /// 端到端：POST /open-apis/corehr/v2/custom_orgs/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/custom_orgs/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "object_api_name": "custom_org_01",
                        "names": [{"lang": "zh-CN", "value": "中文示例"}],
                        "code": "MDPD00000023",
                        "parent_id": "6862995757234914824",
                        "manager_ids": ["6862995757234914824"],
                        "description": [{"lang": "zh-CN", "value": "中文示例"}],
                        "effective_time": "2020-01-01",
                        "org_roles": [{
                            "api_name": "hcm_corehr_xxx",
                            "security_group_id": "7034393015968122400",
                            "employment_ids": ["123456789"],
                            "inherit_employment_ids": ["123456789"]
                        }],
                        "match_rule_groups": [{
                            "match_rules": [{
                                "left_value": "department",
                                "operator": "contains",
                                "right_values": ["123456789"]
                            }]
                        }],
                        "active": true,
                        "org_id": "6862995757234914824",
                        "custom_fields": [{
                            "custom_api_name": "name",
                            "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                            "type": 1,
                            "value": "\"231\""
                        }]
                    }],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more": true
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

        let items = resp.items.as_ref().expect("items 应被解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].object_api_name.as_deref(), Some("custom_org_01"));
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );
        assert_eq!(
            items[0]
                .manager_ids
                .as_ref()
                .and_then(|m| m.first())
                .map(String::as_str),
            Some("6862995757234914824")
        );
        assert_eq!(
            items[0]
                .org_roles
                .as_ref()
                .and_then(|r| r.first())
                .and_then(|r| r.security_group_id.as_deref()),
            Some("7034393015968122400")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
