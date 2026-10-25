//! 查询职等
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_grade/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// QueryRequest
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl QueryRequest {
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
            ApiRequest::<QueryResponse>::post("/open-apis/corehr/v2/job_grades/query");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// QueryResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 职等信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobGrade>>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 职等信息（官方 job_grade 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobGrade {
    /// 职等 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    /// 职等数值，数字越大代表职等越高
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_order: Option<i64>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言，中文用 zh-CN，英文用 en-US
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 内容
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

    /// 端到端：POST /open-apis/corehr/v2/job_grades/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_grades/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "job_grade_id": "4692446793125560154",
                            "grade_order": 9999,
                            "code": "A01234",
                            "names": [
                                { "lang": "zh-CN", "value": "张三" }
                            ],
                            "descriptions": [
                                { "lang": "zh-CN", "value": "张三" }
                            ],
                            "active": true
                        }
                    ],
                    "page_token": "4692446793125560154",
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

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].job_grade_id.as_deref(),
            Some("4692446793125560154")
        );
        assert_eq!(items[0].grade_order, Some(9999));
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|names| names.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
