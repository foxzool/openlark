//! 查询指定时间范围公司版本
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/organization-management/company/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询指定时间范围公司版本请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineBody {
    /// 公司 ID 列表（必填，长度范围 1～10）
    pub company_ids: Vec<String>,
    /// 查询开始时间（包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 查询结束时间（不包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 返回数据的字段列表，可选 company_name 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 查询指定时间范围公司版本的响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryMultiTimelineResponse {
    /// 公司信息列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CompanyTimelineItem>>,
}

/// 公司时间轴信息（官方 company_timeline 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanyTimelineItem {
    /// 公司版本信息。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_version_data: Option<Vec<CompanyVersionData>>,
}

/// 公司版本信息（官方 company_version_data 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanyVersionData {
    /// 公司 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// 公司版本 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_version_id: Option<String>,
    /// 公司名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_names: Option<Vec<I18nText>>,
    /// 上级公司 ID（若查询的是一级公司，则该字段不展示）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_company_id: Option<String>,
    /// 当前版本生效日期（YYYY-MM-DD）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 当前版本失效日期（YYYY-MM-DD）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 是否启用。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 编码。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct I18nText {
    /// 语言（中文用 zh-CN，英文用 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApiResponseTrait for QueryMultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 查询指定时间范围公司版本的请求。
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    config: Arc<Config>,
    body: QueryMultiTimelineBody,
}
/// 查询指定时间范围公司版本请求构建器实现。
impl QueryMultiTimelineRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryMultiTimelineBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: QueryMultiTimelineBody) -> Self {
        self.body = body;
        self
    }
    /// 设置公司 ID 列表（必填，长度范围 1～10）。
    pub fn company_ids(mut self, company_ids: Vec<String>) -> Self {
        self.body.company_ids = company_ids;
        self
    }
    /// 设置查询开始时间（包含），格式 "YYYY-MM-DD"。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.body.start_date = Some(start_date.into());
        self
    }
    /// 设置查询结束时间（不包含），格式 "YYYY-MM-DD"。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = Some(end_date.into());
        self
    }
    /// 设置返回数据的字段列表。
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.body.fields = Some(fields);
        self
    }
    /// 执行查询指定时间范围公司版本请求。
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        validate_required!(self.body.company_ids, "company_ids 不能为空且为 1～10 个");
        if self.body.company_ids.len() > 10 {
            return Err(CoreError::validation_msg("company_ids 长度范围为 1～10"));
        }

        let request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/companies/query_multi_timeline",
        )
        .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询指定时间范围公司版本",
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

    /// 端到端：POST /open-apis/corehr/v2/companies/query_multi_timeline
    #[tokio::test]
    async fn test_query_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/companies/query_multi_timeline"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "company_version_data": [
                                {
                                    "company_id": "4719456877659520852",
                                    "company_version_id": "7238516215202170412",
                                    "company_names": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "parent_company_id": "4719456877659520852",
                                    "effective_date": "2020-05-01",
                                    "expiration_date": "2020-05-02",
                                    "active": true,
                                    "descriptions": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "code": "FJK387"
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

        let resp = QueryMultiTimelineRequest::new(Arc::new(config))
            .company_ids(vec!["4719456877659520852".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        let version_data = items[0]
            .company_version_data
            .clone()
            .expect("company_version_data 应存在");
        assert_eq!(version_data.len(), 1);
        assert_eq!(
            version_data[0].company_version_id,
            Some("7238516215202170412".to_string())
        );
        assert_eq!(
            version_data[0].parent_company_id,
            Some("4719456877659520852".to_string())
        );
        assert_eq!(
            version_data[0].company_names,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
