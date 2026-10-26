//! 查询指定时间范围职务版本
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 查询指定时间范围职务版本请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMultiTimelineBody {
    /// 职务 ID 列表（必填，1～10 个）
    pub job_ids: Vec<String>,
    /// 查询开始时间（包含，格式："2024-01-01"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 查询结束时间（不包含，格式："2024-12-31"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 返回数据的字段列表（job_name/code/active/description/effective_date/expiration_date/job_title/job_family/job_level/pathway/working_hours_type）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// QueryMultiTimelineRequest
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: QueryMultiTimelineBody,
}

impl QueryMultiTimelineRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: QueryMultiTimelineBody::default(),
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: QueryMultiTimelineBody) -> Self {
        self.body = body;
        self
    }

    /// 设置职务 ID 列表（必填，1～10 个）。
    pub fn job_ids(mut self, job_ids: Vec<String>) -> Self {
        self.body.job_ids = job_ids;
        self
    }

    /// 设置查询开始时间（包含，格式："2024-01-01"）。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.body.start_date = Some(start_date.into());
        self
    }

    /// 设置查询结束时间（不包含，格式："2024-12-31"）。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = Some(end_date.into());
        self
    }

    /// 设置返回数据的字段列表。
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.body.fields = Some(fields);
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
        validate_required_list!(self.body.job_ids, 10, "job_ids 不能为空且单次最多 10 个");

        let request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/jobs/query_multi_timeline",
        )
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// QueryMultiTimelineResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMultiTimelineResponse {
    /// 职务信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobTimelineItem>>,
}

/// 职务时间轴信息（官方 job_timeline 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobTimelineItem {
    /// 职务版本信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_version_data: Option<Vec<JobVersionData>>,
}

/// 职务版本信息（官方 job_version_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobVersionData {
    /// 职务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职务版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_version_id: Option<String>,
    /// 职务名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_names: Option<Vec<I18nText>>,
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
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务头衔
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_titles: Option<Vec<I18nText>>,
    /// 序列 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_ids: Option<Vec<String>>,
    /// 职级 ID 列表（字段权限：corehr:job.job_level:read）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_ids: Option<Vec<String>>,
    /// 通道 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/jobs/query_multi_timeline
    #[tokio::test]
    async fn test_query_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/jobs/query_multi_timeline"))
            .and(body_json(json!({
                "job_ids": ["7094136522860922111"],
                "start_date": "2024-01-01",
                "end_date": "2024-12-31",
                "fields": ["job_name"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "job_version_data": [
                                {
                                    "job_id": "4719456877659520852",
                                    "job_version_id": "7238516215202170412",
                                    "job_names": [
                                        { "lang": "zh-CN", "value": "软件工程师" }
                                    ],
                                    "effective_date": "2020-05-01",
                                    "expiration_date": "2020-05-02",
                                    "active": true,
                                    "descriptions": [
                                        { "lang": "zh-CN", "value": "负责后端开发" }
                                    ],
                                    "code": "BD38591",
                                    "job_titles": [
                                        { "lang": "zh-CN", "value": "高级" }
                                    ],
                                    "job_family_ids": ["4719519211875096301"],
                                    "job_level_ids": ["4719519212005299950"],
                                    "pathway_id": "4719519211875096301",
                                    "working_hours_type_id": "6890452208593372679"
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
            .job_ids(vec!["7094136522860922111".to_string()])
            .start_date("2024-01-01")
            .end_date("2024-12-31")
            .fields(vec!["job_name".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        let version_data = items[0]
            .job_version_data
            .clone()
            .expect("job_version_data 应存在");
        assert_eq!(version_data.len(), 1);
        assert_eq!(
            version_data[0].job_version_id,
            Some("7238516215202170412".to_string())
        );
        assert_eq!(
            version_data[0].job_names,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("软件工程师".to_string()),
            }])
        );
        assert_eq!(
            version_data[0].descriptions,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("负责后端开发".to_string()),
            }])
        );
        assert_eq!(
            version_data[0].job_level_ids,
            Some(vec!["4719519212005299950".to_string()])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
