//! 获取标签填写题配置
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/question/query>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取标签填写题配置请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 标签填写题 ID 列表（可选，≤50，不传返回所有）
    tag_based_question_ids: Option<Vec<String>>,
    /// 分页标记（可选 query 参数）
    page_token: Option<String>,
    /// 分页大小（可选 query 参数，默认 20，0～50）
    page_size: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            tag_based_question_ids: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置标签填写题 ID 列表（可选，≤50）
    pub fn tag_based_question_ids(mut self, tag_based_question_ids: Vec<String>) -> Self {
        self.tag_based_question_ids = Some(tag_based_question_ids);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选，默认 20，0～50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::QuestionQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            tag_based_question_ids: self.tag_based_question_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取标签填写题配置响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 标签填写题 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_based_question_ids: Option<Vec<String>>,
}

/// 获取标签填写题配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 标签填写题列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_based_questions: Option<Vec<TagBasedQuestion>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 标签填写题（官方 question 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagBasedQuestion {
    /// 填写题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    /// 填写题名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 标签列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_items: Option<Vec<TagItem>>,
}

/// 标签（官方 tag_item 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagItem {
    /// 标签 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标签名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_question_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（tag_based_questions 容器 + tag_items）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "tag_based_questions": [
                    {
                        "question_id": "7343513161666707459",
                        "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "tag_items": [
                            {
                                "id": "7343513161666707459",
                                "name": { "zh_cn": "体验", "en_us": "Interactive experience" }
                            }
                        ]
                    }
                ],
                "has_more": true,
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/questions/query"))
            .and(body_json(json!({
                "tag_based_question_ids": ["7343513161666707459"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(config)
            .tag_based_question_ids(vec!["7343513161666707459".to_string()])
            .page_size(30)
            .execute()
            .await
            .expect("performance_v2_question_query 应成功");

        let questions = data
            .tag_based_questions
            .as_ref()
            .expect("tag_based_questions 应存在");
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions[0].question_id.as_deref(),
            Some("7343513161666707459")
        );
        assert_eq!(
            questions[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        let tag_items = questions[0].tag_items.as_ref().expect("tag_items 应存在");
        assert_eq!(tag_items[0].id.as_deref(), Some("7343513161666707459"));
        assert_eq!(
            tag_items[0].name.as_ref().and_then(|n| n.en_us.as_deref()),
            Some("Interactive experience")
        );
        assert_eq!(data.has_more, Some(true));
        assert!(data.page_token.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/questions/query"
        );
    }
}
