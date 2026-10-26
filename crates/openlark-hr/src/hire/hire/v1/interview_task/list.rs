//! 获取面试任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_task/list>

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

use crate::hire::hire::common_models::InterviewTaskSummary;

/// 获取面试任务列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 面试官 ID（必填，需与 user_id_type 类型保持一致）
    user_id: String,
    /// 分页大小（默认 10，不能超过 20）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 任务状态：1 未开始 / 2 未评价 / 3 已评价 / 5 已终止
    activity_status: Option<i32>,
    /// 用户 ID 类型（默认 open_id）
    user_id_type: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            page_size: None,
            page_token: None,
            activity_status: None,
            user_id_type: None,
        }
    }

    /// 设置面试官 ID（必填，需与 user_id_type 类型保持一致）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置分页大小（默认 10，不能超过 20）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置任务状态：1 未开始 / 2 未评价 / 3 已评价 / 5 已终止。
    pub fn activity_status(mut self, activity_status: i32) -> Self {
        self.activity_status = Some(activity_status);
        self
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id / people_admin_id，默认 open_id）。
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

        validate_required!(self.user_id.trim(), "user_id 不能为空");
        if let Some(size) = self.page_size
            && !(1..=20).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-20 之间",
            ));
        }

        let api_endpoint = HireApiV1::InterviewTaskList;
        let request =
            ApiRequest::<ListResponse>::get(api_endpoint.to_url()).query("user_id", self.user_id);
        let request = if let Some(page_size) = self.page_size {
            request.query("page_size", page_size.to_string())
        } else {
            request
        };
        let request = request
            .query_opt("page_token", self.page_token.as_deref())
            .query_opt(
                "activity_status",
                self.activity_status.map(|v| v.to_string()),
            )
            .query_opt("user_id_type", self.user_id_type.as_deref());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试任务列表响应数据为空",
        )
        .await
    }
}

/// 获取面试任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<InterviewTaskSummary>,
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

    /// 端到端：GET /open-apis/hire/v1/interview_tasks
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interview_tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEwLCJ0aW1lc3RhbXAiOjE2Mjc1NTUyMjM2NzIsImlkIjpudWxsfQ==",
                    "items": [
                        {
                            "id": "6989202908470446380",
                            "job_id": "6897921873523247373",
                            "talent_id": "7046263097923537196",
                            "application_id": "7046315599510833452",
                            "activity_status": 1
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

        let resp = ListRequest::new(config)
            .user_id("ou_e6139117c300506837def50545420c6a".to_string())
            .page_size(10)
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id.as_deref(), Some("6989202908470446380"));
        assert_eq!(resp.items[0].job_id.as_deref(), Some("6897921873523247373"));
        assert_eq!(
            resp.items[0].talent_id.as_deref(),
            Some("7046263097923537196")
        );
        assert_eq!(
            resp.items[0].application_id.as_deref(),
            Some("7046315599510833452")
        );
        assert_eq!(resp.items[0].activity_status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 官方为 GET 查询参数：user_id 必填，page_size 随查询串传递
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("user_id=ou_e6139117c300506837def50545420c6a"),
            "查询串缺 user_id: {query}"
        );
        assert!(
            query.contains("page_size=10"),
            "查询串缺 page_size: {query}"
        );
        assert!(received[0].body.is_empty(), "GET 请求不应携带请求体");
    }

    /// 缺 user_id 时快速失败，不发起请求。
    #[tokio::test]
    async fn test_list_rejects_missing_user_id() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = ListRequest::new(config)
            .execute()
            .await
            .expect_err("缺 user_id 应被拒绝");
        assert!(err.to_string().contains("user_id"), "实际: {err}");
    }
}
