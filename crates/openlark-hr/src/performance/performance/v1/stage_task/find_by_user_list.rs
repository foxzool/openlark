//! 获取周期任务（指定用户）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v1/stage_task/find_by_user_list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use openlark_core::{validate_required, validate_required_list};
use serde::{Deserialize, Serialize};

/// 获取周期任务（指定用户）请求
#[derive(Debug, Clone)]
pub struct FindByUserListRequest {
    /// 周期 ID（必填，可通过获取周期接口获取）
    semester_id: String,
    /// 用户 ID 列表（必填，与入参 user_id_type 类型一致，最多 50 个）
    user_id_lists: Vec<String>,
    /// 任务分类（可选，1 待完成/2 已完成/3 已逾期，最多 3 个）
    task_option_lists: Option<Vec<i32>>,
    /// 任务截止时间最小值（可选，毫秒时间戳）
    after_time: Option<String>,
    /// 任务截止时间最大值（可选，毫秒时间戳）
    before_time: Option<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl FindByUserListRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String) -> Self {
        Self {
            semester_id,
            user_id_lists: Vec::new(),
            task_option_lists: None,
            after_time: None,
            before_time: None,
            user_id_type: None,
            config,
        }
    }

    /// 追加一个用户 ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_id_lists.push(user_id);
        self
    }

    /// 设置任务分类（可选，1 待完成/2 已完成/3 已逾期）
    pub fn task_option_lists(mut self, task_option_lists: Vec<i32>) -> Self {
        self.task_option_lists = Some(task_option_lists);
        self
    }

    /// 设置任务截止时间最小值（可选，毫秒时间戳）
    pub fn after_time(mut self, after_time: String) -> Self {
        self.after_time = Some(after_time);
        self
    }

    /// 设置任务截止时间最大值（可选，毫秒时间戳）
    pub fn before_time(mut self, before_time: String) -> Self {
        self.before_time = Some(before_time);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FindByUserListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FindByUserListResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.semester_id.trim(), "semester_id");
        validate_required_list!(
            self.user_id_lists,
            50,
            "user_id_lists 不能为空且不能超过 50 个"
        );

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::StageTaskFindByUserList;
        let request = ApiRequest::<FindByUserListResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 2. 构建请求体
        let request_body = FindByUserListRequestBody {
            semester_id: self.semester_id,
            user_id_lists: self.user_id_lists,
            task_option_lists: self.task_option_lists,
            after_time: self.after_time,
            before_time: self.before_time,
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
            "获取周期任务响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindByUserListRequestBody {
    /// 周期 ID
    pub semester_id: String,
    /// 用户 ID 列表
    pub user_id_lists: Vec<String>,
    /// 任务分类（1 待完成/2 已完成/3 已逾期）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_option_lists: Option<Vec<i32>>,
    /// 任务截止时间最小值（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    /// 任务截止时间最大值（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
}

/// 获取周期任务（指定用户）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FindByUserListResponse {
    /// 周期任务列表
    pub items: Vec<StageTask>,
}

/// 周期任务
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageTask {
    /// 用户 ID
    pub user_id: String,
    /// 阶段 ID
    pub stage_id: String,
    /// 任务状态
    pub status: i32,
}

impl ApiResponseTrait for FindByUserListResponse {
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
    async fn test_performance_v1_stage_task_find_by_user_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"items": []}"#).unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v1/stage_tasks/find_by_user_list",
            ))
            .and(body_json(json!({
                "semester_id": "7235911950407352321",
                "user_id_lists": ["bega29ca"],
                "task_option_lists": [1, 2, 3],
                "after_time": "1717142243000",
                "before_time": "1717142245000"
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

        let data = FindByUserListRequest::new(config, "7235911950407352321".to_string())
            .add_user_id("bega29ca".to_string())
            .task_option_lists(vec![1, 2, 3])
            .after_time("1717142243000".to_string())
            .before_time("1717142245000".to_string())
            .execute()
            .await
            .expect("performance_v1_stage_task_find_by_user_list 应成功");

        assert!(data.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v1/stage_tasks/find_by_user_list"
        );
    }
}
