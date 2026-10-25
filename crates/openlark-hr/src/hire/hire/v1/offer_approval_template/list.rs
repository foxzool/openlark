//! 获取 Offer 审批流列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer_approval_template/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Offer 审批流适用部门（官方 `department`：`{id, name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferApprovalTemplateDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门 ID，与入参 `department_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门中文名。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门英文名。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 审批流（官方 `offer_approval_template`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferApprovalTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 审批流 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 审批流名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 备注。
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 适用部门列表。
    pub department_list: Option<Vec<OfferApprovalTemplateDepartment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取 Offer 审批流列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
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
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::OfferApprovalTemplateList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 Offer 审批流列表响应数据为空",
        )
        .await
    }
}

/// 获取 Offer 审批流列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<OfferApprovalTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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

    /// 端到端：GET /open-apis/hire/v1/offer_approval_templates，items 元素按官方形状解析。
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/offer_approval_templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": true,
                    "page_token": "1234452132",
                    "items": [
                        {
                            "id": "1213213123123",
                            "name": {
                                "zh_cn": "Offer 审批流1",
                                "en_us": "Offer approval flow config 1"
                            },
                            "create_time": "1628512038000",
                            "remark": "test",
                            "department_list": [
                                {
                                    "id": "1213213123123",
                                    "name": "部门名A",
                                    "en_name": "DepartmentA"
                                }
                            ]
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let template = resp.items.first().expect("应解析出 1 个审批流");
        assert_eq!(template.id.as_deref(), Some("1213213123123"));
        assert_eq!(
            template.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("Offer 审批流1")
        );
        assert_eq!(template.create_time.as_deref(), Some("1628512038000"));
        assert_eq!(template.remark.as_deref(), Some("test"));
        let department = template
            .department_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应解析出 department_list");
        assert_eq!(department.id.as_deref(), Some("1213213123123"));
        assert_eq!(department.name.as_deref(), Some("部门名A"));
        assert_eq!(department.en_name.as_deref(), Some("DepartmentA"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
