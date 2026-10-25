//! 获取父部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/parents>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 获取父部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParentsRequest {
    /// 部门 ID
    pub department_id: String,
}

impl ParentsRequest {
    /// 创建请求
    pub fn new(department_id: String) -> Self {
        Self { department_id }
    }
}

/// 获取父部门信息响应
///
/// 响应 `data` 子树形状：`{items: [{department_id, parent_department_list}]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponse {
    /// 父部门查询结果列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentItem>>,
}

/// `DepartmentItem`（官方 `department_parents` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentItem {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 父部门列表，部门按照至底向上的顺序返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_list: Option<Vec<DepartmentParentInfo>>,
}

/// `DepartmentParentInfo`（官方 `department_parent_info` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentParentInfo {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<Vec<I18nText>>,
    /// 上级部门 ID（若查询的是一级部门，则该字段不展示）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 是否根部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言（如 zh-CN / en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for ParentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取父部门信息请求构建器
#[derive(Debug, Clone)]
pub struct ParentsRequestBuilder {
    config: Config,
    department_id: String,
}

impl ParentsRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_id: String) -> Self {
        Self {
            config,
            department_id,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ParentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ParentsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentParents(self.department_id.clone());
        let request = ApiRequest::<ParentsResponse>::post(api_endpoint.to_url()).body(
            serde_json::to_value(ParentsRequest::new(self.department_id)).map_err(|e| {
                openlark_core::error::validation_error(
                    "获取父部门信息请求序列化失败",
                    e.to_string(),
                )
            })?,
        );

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取父部门信息响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/departments/parents
    #[tokio::test]
    async fn test_parents_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/parents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "department_id": "6893014062142064111",
                            "parent_department_list": [
                                {
                                    "department_id": "4719456877659520852",
                                    "department_name": [
                                        { "lang": "zh-CN", "value": "张三" }
                                    ],
                                    "parent_department_id": "4719456877659520111",
                                    "active": true,
                                    "is_root": true
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

        let resp = ParentsRequestBuilder::new(config, "test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含部门父级查询结果");
        assert_eq!(item.department_id.as_deref(), Some("6893014062142064111"));
        let parent = item
            .parent_department_list
            .as_ref()
            .and_then(|list| list.first())
            .expect("parent_department_list 应包含父部门");
        assert_eq!(parent.department_id.as_deref(), Some("4719456877659520852"));
        assert_eq!(
            parent
                .department_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );
        assert_eq!(
            parent.parent_department_id.as_deref(),
            Some("4719456877659520111")
        );
        assert_eq!(parent.active, Some(true));
        assert_eq!(parent.is_root, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
