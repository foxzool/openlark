//! 批量查询职务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::CorehrV2Job;

/// ListRequest
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 每页获取记录数量（必填，string 类型，最大 100）
    page_size: String,
    /// 分页标记，第一次请求不填
    page_token: Option<String>,
    /// 职务名称
    name: Option<String>,
    /// 语言信息，中文用 zh-CN，英文用 en-US
    query_language: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: String::new(),
            page_token: None,
            name: None,
            query_language: None,
        }
    }

    /// 设置每页获取记录数量（必填，最大 100）
    pub fn page_size(mut self, page_size: impl Into<String>) -> Self {
        self.page_size = page_size.into();
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置职务名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置语言信息（zh-CN / en-US）
    pub fn query_language(mut self, query_language: impl Into<String>) -> Self {
        self.query_language = Some(query_language.into());
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
        validate_required!(self.page_size, "page_size 不能为空");

        let request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/jobs")
            .query("page_size", self.page_size)
            .query_opt("page_token", self.page_token)
            .query_opt("name", self.name)
            .query_opt("query_language", self.query_language);

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
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/jobs（必填查询参数 page_size 按官方文档发送）
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/jobs"))
            .and(query_param("page_size", "100"))
            .and(query_param("query_language", "zh-CN"))
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
            .page_size("100")
            .query_language("zh-CN")
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
