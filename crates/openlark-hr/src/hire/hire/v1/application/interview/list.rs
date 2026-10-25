//! 获取面试记录列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application.interview/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 获取面试记录列表请求
///
/// 官方文档将 `page_size` / `page_token` / `user_id_type` 列为查询参数，
/// 其中 `page_size` 必填且不能超过 50。
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    application_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 类型（`open_id` / `union_id` / `user_id`，默认 `open_id`）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");
        // 官方文档：page_size 必填，不能超过 50
        let page_size = self.page_size.ok_or_else(|| {
            openlark_core::error::validation_error("page_size", "page_size 不能为空")
        })?;
        if !(1..=50).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-50 之间",
            ));
        }

        let api_endpoint = HireApiV1::ApplicationInterviewList(self.application_id.clone());
        // GET 请求：page_size / page_token / user_id_type 均为查询参数，不走请求体
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url())
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试记录列表响应数据为空",
        )
        .await
    }
}

/// 获取面试记录列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default, alias = "items")]
    /// `records` 字段（官方 `items`，元素为 `interview[]`）。
    pub records: Vec<Interview>,
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

/// 面试（官方 `interview` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Interview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试开始时间（毫秒时间戳）。
    pub begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结束时间（毫秒时间戳）。
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次（从 0 开始计数）。
    pub round: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试关联的投递阶段。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官记录列表。
    pub interview_record_list: Option<Vec<InterviewRecord>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试官记录（官方 `interview_record` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试记录 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官用户 ID。
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 系统预设「记录」题目内容。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议定级下限的职级 ID。
    pub min_job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议定级上限的职级 ID。
    pub max_job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 提交状态（1：已提交 2：未提交）。
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结论（1：通过 2：未通过 3：未开始 4：未提交 5：未到场）。
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评分。
    pub interview_score: Option<InterviewScore>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评分（官方 `interview_score` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewScore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评分 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 分数级别。
    pub level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 中文描述。
    pub zh_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 英文描述。
    pub en_description: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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

    /// 端到端：GET /open-apis/hire/v1/applications/test001/interviews
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/applications/test001/interviews"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListRequest::new(config)
            .application_id("test001".to_string())
            .page_size(10)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 官方契约：page_size 等为查询参数，GET 不携带请求体
        let req = &received[0];
        assert!(req.url.query().unwrap_or_default().contains("page_size=10"));
        assert!(req.body.is_empty());
    }
}
