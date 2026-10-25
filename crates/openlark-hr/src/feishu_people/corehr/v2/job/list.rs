//! 批量查询职务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::CorehrV2Job;

/// ListRequest
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
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
        let mut request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/jobs");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// ListResponse
///
/// 响应 `data` 子树形状：`{items: job[], has_more, page_token}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 查询的职务信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CorehrV2Job>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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

    /// 端到端：GET /open-apis/corehr/v2/jobs
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/jobs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4698040628992333549",
                            "code": "JP422119",
                            "name": [{ "lang": "zh-CN", "value": "软件工程师" }],
                            "description": [{ "lang": "zh-CN", "value": "负责后端开发" }],
                            "active": true,
                            "job_title": [{ "lang": "zh-CN", "value": "高级" }],
                            "pathway_id": "4719519211875096301",
                            "job_family_id_list": ["4719519211875096301"],
                            "job_level_id_list": ["4719519212005299950"],
                            "working_hours_type_id": "6890452208593372679",
                            "effective_time": "2020-01-01 00:00:00",
                            "expiration_time": "2021-01-01 00:00:00",
                            "custom_fields": [
                                { "field_name": "name", "value": "Sandy" }
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
            .execute()
            .await
            .expect("请求应成功");

        let job = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含职务");
        assert_eq!(job.id.as_deref(), Some("4698040628992333549"));
        assert_eq!(
            job.name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("软件工程师")
        );
        assert_eq!(job.active, Some(true));
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(resp.page_token.as_deref(), Some("1234452132"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
