//! 查询指定时间范围序列版本
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_family/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// QueryMultiTimelineRequest
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl QueryMultiTimelineRequest {
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
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        let mut request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/job_families/query_multi_timeline",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// QueryMultiTimelineResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMultiTimelineResponse {
    /// 序列信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobFamilyTimelineItem>>,
}

/// 序列时间轴信息（官方 job_family_timeline 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobFamilyTimelineItem {
    /// 序列版本信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_version_data: Option<Vec<JobFamilyVersionData>>,
}

/// 序列版本信息（官方 job_family_version_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobFamilyVersionData {
    /// 序列 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 序列版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_version_id: Option<String>,
    /// 序列名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_names: Option<Vec<I18nText>>,
    /// 当前版本生效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 当前版本失效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 是否可被使用，true 为可被使用，false 为不可被使用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectable: Option<bool>,
    /// 上级序列 ID（若查询的是一级序列，则该字段不展示）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    /// 通道 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_ids: Option<Vec<String>>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/job_families/query_multi_timeline
    #[tokio::test]
    async fn test_query_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/job_families/query_multi_timeline",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "job_family_version_data": [
                                {
                                    "job_family_id": "4719456877659520852",
                                    "job_family_version_id": "7238516215202170412",
                                    "job_family_names": [
                                        { "lang": "zh-CN", "value": "技术序列" }
                                    ],
                                    "effective_date": "2020-05-01",
                                    "expiration_date": "2020-05-02",
                                    "active": true,
                                    "descriptions": [
                                        { "lang": "zh-CN", "value": "负责软件产品设计与开发的序列" }
                                    ],
                                    "selectable": true,
                                    "parent_job_family_id": "4698020757495316313",
                                    "pathway_ids": ["4719519211875096301"],
                                    "code": "123456"
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

        let resp = QueryMultiTimelineRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        let version_data = items[0]
            .job_family_version_data
            .clone()
            .expect("job_family_version_data 应存在");
        assert_eq!(version_data.len(), 1);
        assert_eq!(
            version_data[0].job_family_version_id,
            Some("7238516215202170412".to_string())
        );
        assert_eq!(
            version_data[0].job_family_names,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("技术序列".to_string()),
            }])
        );
        assert_eq!(
            version_data[0].pathway_ids,
            Some(vec!["4719519211875096301".to_string()])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
