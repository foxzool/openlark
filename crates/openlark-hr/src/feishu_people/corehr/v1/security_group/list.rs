//! 批量获取角色列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/security_group/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量获取角色列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（必填）
    page_size: i32,
    /// 分页标记，第一次请求不填表示从头开始遍历
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: 0,
            page_token: None,
        }
    }

    /// 设置分页大小（必填，示例值 100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    /// 设置分页标记（可选，第一次请求不填）。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        if self.page_size < 1 {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 不能为空且须为正整数",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::SecurityGroupList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url())
            .query("page_size", self.page_size.to_string());
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取角色列表响应数据为空",
        )
        .await
    }
}

/// 批量获取角色列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SecurityGroupItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 角色（安全组）条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SecurityGroupItem {
    /// 角色 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 角色 code，通常用于与其他系统进行交互
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 角色名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SecurityGroupName>,
    /// 状态：1 = 停用；2 = 启用；3 = 待启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    /// 角色描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<SecurityGroupName>,
    /// 角色类型：3 = 组织类角色；7 = 非组织类角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    /// 创建人：`sys` 表示系统创建角色；用户 ID 表示自定义角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 组织管理维度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_truncation: Option<Vec<OrgTruncation>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 角色名称/描述：`{zh_cn, en_us}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SecurityGroupName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 组织管理维度：`{org_key, type, depth}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OrgTruncation {
    /// 组织管理维度名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_key: Option<String>,
    /// 下钻类型：0 = 对当前及下级管理维度均有权限；1 = 只对当前管理维度有权限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 下钻深度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
}

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/security_groups
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/security_groups"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "7034393015968122400",
                            "code": "department_manager",
                            "name": { "zh_cn": "cn", "en_us": "en" },
                            "active_status": 1,
                            "description": { "zh_cn": "cn", "en_us": "en" },
                            "group_type": 1,
                            "created_by": "6967639606963471902",
                            "update_time": "1",
                            "org_truncation": [
                                { "org_key": "department", "type": 0, "depth": 0 }
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

        let resp = ListRequest::new(config)
            .page_size(100)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 不能为空");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, Some("7034393015968122400".to_string()));
        assert_eq!(items[0].code, Some("department_manager".to_string()));
        assert_eq!(items[0].created_by, Some("6967639606963471902".to_string()));
        assert_eq!(items[0].active_status, Some(1));
        assert_eq!(items[0].group_type, Some(1));
        let name = items[0].name.as_ref().expect("name 不能为空");
        assert_eq!(name.zh_cn, Some("cn".to_string()));
        assert_eq!(name.en_us, Some("en".to_string()));
        let org_truncation = items[0]
            .org_truncation
            .as_deref()
            .expect("org_truncation 不能为空");
        assert_eq!(org_truncation.len(), 1);
        assert_eq!(org_truncation[0].org_key, Some("department".to_string()));
        assert_eq!(org_truncation[0].r#type, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // page_size 为官方必填查询参数
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=100"),
            "查询串应含 page_size=100，实际 {query}"
        );
    }
}
