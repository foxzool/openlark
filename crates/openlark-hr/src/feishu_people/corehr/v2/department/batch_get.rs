//! 批量查询部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

use super::search::DepartmentItem;

/// 批量查询部门请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetRequest {
    /// 部门 ID 列表
    pub department_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(department_ids: Vec<String>) -> Self {
        Self { department_ids }
    }

    /// 添加部门 ID
    pub fn add_department_id(mut self, department_id: String) -> Self {
        self.department_ids.push(department_id);
        self
    }
}

/// 批量查询部门响应
///
/// 响应 `data` 子树形状：`{items: department[]}`（由 decode 层解包）。
/// 条目模型与 `department/search.rs` 共享（同一官方 `department` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 部门列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentItem>>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门请求构建器
#[derive(Debug, Clone)]
pub struct BatchGetRequestBuilder {
    config: Config,
    request: BatchGetRequest,
}

impl BatchGetRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_ids: Vec<String>) -> Self {
        Self {
            config,
            request: BatchGetRequest::new(department_ids),
        }
    }

    /// 添加部门 ID
    pub fn add_department_id(mut self, department_id: String) -> Self {
        self.request = self.request.add_department_id(department_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询部门响应数据为空",
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

    /// 端到端：POST /open-apis/corehr/v2/departments/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4719456877659520852",
                            "version_id": "6890452208593372611",
                            "department_name": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "sub_type": {
                                "enum_name": "phone_type",
                                "display": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ]
                            },
                            "parent_department_id": "4719456877659520111",
                            "manager": "6893013238632416777",
                            "tree_order": "001000",
                            "list_order": "001000-001000",
                            "code": "D00000456",
                            "is_root": false,
                            "is_confidential": false,
                            "effective_date": "2020-05-01",
                            "expiration_date": "2020-05-02",
                            "active": true,
                            "description": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "cost_center_id": "7142384817131652652",
                            "created_time": "2020-05-01 00:00:00",
                            "updated_time": "2020-05-02 00:00:00"
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

        let resp = BatchGetRequestBuilder::new(config, vec!["test001".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含部门");
        assert_eq!(item.id, "4719456877659520852");
        assert_eq!(item.version_id.as_deref(), Some("6890452208593372611"));
        assert_eq!(
            item.department_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );
        assert_eq!(
            item.sub_type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(
            item.parent_department_id.as_deref(),
            Some("4719456877659520111")
        );
        assert_eq!(item.manager.as_deref(), Some("6893013238632416777"));
        assert_eq!(item.active, Some(true));
        assert_eq!(item.created_time.as_deref(), Some("2020-05-01 00:00:00"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
