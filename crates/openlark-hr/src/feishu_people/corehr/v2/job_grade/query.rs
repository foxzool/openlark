//! 查询职等
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_grade/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询职等请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryBody {
    /// 职等 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// 职等编码列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<String>>,
    /// 启停用状态：true 为启用，false 为停用，不传则默认都返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// QueryRequest
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: QueryBody,
    /// 分页大小（必填，取值范围 1～100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: QueryBody::default(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: QueryBody) -> Self {
        self.body = body;
        self
    }

    /// 设置职等 ID 列表。
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.body.ids = Some(ids);
        self
    }

    /// 设置职等编码列表。
    pub fn codes(mut self, codes: Vec<String>) -> Self {
        self.body.codes = Some(codes);
        self
    }

    /// 设置启停用状态筛选。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = Some(active);
        self
    }

    /// 设置分页大小（必填，取值范围 1～100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
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
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }

        let request = ApiRequest::<QueryResponse>::post("/open-apis/corehr/v2/job_grades/query")
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .body(serde_json::to_value(&self.body)?);

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
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/job_grades/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_grades/query"))
            .and(query_param("page_size", "100"))
            .and(body_json(json!({
                "ids": ["4692446793125560154"],
                "codes": ["A01234"],
                "active": true
            })))
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
            .ids(vec!["4692446793125560154".to_string()])
            .codes(vec!["A01234".to_string()])
            .active(true)
            .page_size(100)
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
