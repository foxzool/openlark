//! 获取笔试列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/test/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// 获取笔试列表请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        let mut request = ApiRequest::<SearchResponse>::post("/open-apis/hire/v1/tests/search");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取笔试列表响应数据为空",
        )
        .await
    }
}

/// 笔试时间（官方 `test_schedule`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始时间（毫秒时间戳）。
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 结束时间（毫秒时间戳）。
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 笔试科目详细成绩（官方 `test_result_detail`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestResultDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试科目。
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试科目结论。
    pub result: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 笔试信息（官方 `test`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试 ID。
    pub test_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试卷 ID。
    pub test_paper_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试卷名称。
    pub test_paper_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试卷来源 ID。
    pub test_paper_source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试卷来源名称。
    pub test_paper_source_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试答复状态：1 已接受 / 2 已拒绝 / 3 未反馈。
    pub reply_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试状态：1 未安排 / 2 未作答 / 3 已作答 / 4 待阅卷 / 5 通过 / 6 不通过 / 7 安排失败。
    pub test_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试时间。
    pub test_schedule: Option<TestSchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试作答完成时间（毫秒时间戳）。
    pub test_complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试报告链接列表。
    pub report_url_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试详细成绩列表。
    pub result_detail_list: Option<Vec<TestResultDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试结果上传时间（毫秒时间戳，仅手动录入的笔试成绩有）。
    pub result_upload_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阅卷得分。
    pub score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阅卷结果最新提交时间（毫秒时间戳）。
    pub score_submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阅卷人用户 ID，与 `user_id_type` 查询参数类型一致。
    pub reviewer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阅卷安排时间（毫秒时间戳）。
    pub review_created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 笔试创建时间（毫秒时间戳）。
    pub created_at: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取笔试列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<TestItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for SearchResponse {
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

    /// 端到端：POST /open-apis/hire/v1/tests/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/tests/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "test_id": "6963784391454918956",
                            "application_id": "6920544531948800269",
                            "talent_id": "6830437484509675522",
                            "job_id": "6920544531948800269",
                            "test_paper_id": "6963549885526346028",
                            "test_paper_name": "数据结构测验",
                            "test_paper_source_id": "5",
                            "test_paper_source_name": {
                                "zh_cn": "高考全国卷（一）",
                                "en_us": "National Exam 1"
                            },
                            "reply_status": 1,
                            "test_status": 1,
                            "test_schedule": {
                                "start_time": "1621390695000",
                                "end_time": "1632839400000"
                            },
                            "test_complete_time": "1608725989000",
                            "report_url_list": ["https://open.feishu.cn/xxx/xxx/result.pdf"],
                            "result_detail_list": [
                                { "subject": "定制化报告测验", "result": "合格" }
                            ],
                            "result_upload_time": "1608725989000",
                            "score": "60",
                            "score_submit_time": "1608725989000",
                            "reviewer": "ou_266ed047ed37f24554e5a0afdbb15556",
                            "review_created_at": "1608725989000",
                            "created_at": "1608725989000"
                        }
                    ],
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEwLCJ0aW1lc3RhbXAiOjE2Mjc1NTUyMjM2NzIsImlkIjpudWxsfQ=="
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.test_id.as_deref(), Some("6963784391454918956"));
        assert_eq!(item.test_paper_id.as_deref(), Some("6963549885526346028"));
        assert_eq!(item.test_paper_name.as_deref(), Some("数据结构测验"));
        assert_eq!(item.reply_status, Some(1));
        assert_eq!(item.test_status, Some(1));
        assert_eq!(
            item.test_schedule
                .as_ref()
                .and_then(|s| s.start_time.as_deref()),
            Some("1621390695000")
        );
        assert_eq!(item.result_detail_list.as_ref().map(|l| l.len()), Some(1));
        assert_eq!(
            item.result_detail_list
                .as_ref()
                .and_then(|l| l[0].subject.as_deref()),
            Some("定制化报告测验")
        );
        assert_eq!(item.score.as_deref(), Some("60"));
        assert_eq!(
            resp.page_token.as_deref(),
            Some("eyJvZmZzZXQiOjEwLCJ0aW1lc3RhbXAiOjE2Mjc1NTUyMjM2NzIsImlkIjpudWxsfQ==")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
