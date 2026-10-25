//! 获取周期任务（全部用户）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/stage_task/find_by_page>

use crate::performance::performance::v1::stage_task::models::{SemesterBaseInfo, StageTask};
use openlark_core::validate_required;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取周期任务（全部用户）请求
#[derive(Debug, Clone)]
pub struct FindByPageRequest {
    /// 周期 ID（必填，可通过获取周期接口获取）
    semester_id: String,
    /// 任务分类（可选，1 待完成/2 已完成/3 已逾期，最多 3 个）
    task_option_lists: Option<Vec<i32>>,
    /// 任务截止时间最小值（可选，毫秒时间戳）
    after_time: Option<String>,
    /// 任务截止时间最大值（可选，毫秒时间戳）
    before_time: Option<String>,
    /// 分页标记（可选，请求体参数）
    page_token: Option<String>,
    /// 分页大小（可选，默认 20，最大 50，请求体参数）
    page_size: Option<i32>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl FindByPageRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String) -> Self {
        Self {
            semester_id,
            task_option_lists: None,
            after_time: None,
            before_time: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置任务分类（可选，1 待完成/2 已完成/3 已逾期，最多 3 个）
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

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选，默认 20，最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FindByPageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FindByPageResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.semester_id.trim(), "semester_id");

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::StageTaskFindByPage;
        let request = ApiRequest::<FindByPageResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 2. 构建请求体
        let request_body = FindByPageRequestBody {
            semester_id: self.semester_id,
            task_option_lists: self.task_option_lists,
            after_time: self.after_time,
            before_time: self.before_time,
            page_token: self.page_token,
            page_size: self.page_size,
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
pub struct FindByPageRequestBody {
    /// 周期 ID
    pub semester_id: String,
    /// 任务分类（1 待完成/2 已完成/3 已逾期）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_option_lists: Option<Vec<i32>>,
    /// 任务截止时间最小值（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    /// 任务截止时间最大值（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小（默认 20，最大 50）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 获取周期任务（全部用户）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FindByPageResponse {
    /// 周期基本信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<SemesterBaseInfo>,
    /// 周期任务列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<StageTask>>,
    /// 是否还有更多项（官方文档字段表列于 data 下，示例置于信封顶层，此处兼容保留）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for FindByPageResponse {
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
    async fn test_performance_v1_stage_task_find_by_page_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（base + items + has_more/page_token，
        // 注意本接口示例中任务分类键名为 task_option_id）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "base": {
                    "end_time": "1748707140000",
                    "semester_id": "7235911950407352321",
                    "semester_name": { "en-US": "Demo Semester", "zh-CN": "示例周期" },
                    "start_time": "1684684800000"
                },
                "items": [
                    {
                        "stage_num_lists": [
                            { "task_option_id": 1, "stage_num": 1 },
                            { "task_option_id": 2, "stage_num": 1 },
                            { "task_option_id": 3, "stage_num": 0 }
                        ],
                        "stage_task_info_lists": [
                            {
                                "deadline": "1717142244000",
                                "jump_url": "https://performance.feishu-pre.cn/perf/review/7235911950407352321/7235906557915987996/members",
                                "name": { "en-US": "confirm_invitation", "zh-CN": "360° 确认" },
                                "need_todo_count": 10,
                                "stage_id": "7270346432875692033",
                                "stage_task_status": "need_todo",
                                "task_option_id": 1
                            }
                        ],
                        "user_id": "bega29ca"
                    }
                ],
                "has_more": true,
                "page_token": "84I9nKzkGJjwv1FrJ1sIveEE2ij7xnes6nhcmxNOXLaLLe52sFVu8S0NPxRFkMhLFCMutL9a6Eod7kzOqWQeu47R7NWiZEmBKJ93Asg5VZv1oJkrG318reCJlrljfaeR"
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v1/stage_tasks/find_by_page"))
            .and(body_json(json!({
                "semester_id": "7235911950407352321",
                "task_option_lists": [1, 2, 3],
                "after_time": "1717142243000",
                "before_time": "1717142245000",
                "page_size": 30
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

        let data = FindByPageRequest::new(config, "7235911950407352321".to_string())
            .task_option_lists(vec![1, 2, 3])
            .after_time("1717142243000".to_string())
            .before_time("1717142245000".to_string())
            .page_size(30)
            .execute()
            .await
            .expect("performance_v1_stage_task_find_by_page 应成功");

        let base = data.base.as_ref().expect("base 应存在");
        assert_eq!(base.semester_id.as_deref(), Some("7235911950407352321"));
        assert_eq!(
            base.semester_name.as_ref().and_then(|n| n.en_us.as_deref()),
            Some("Demo Semester")
        );
        let items = data.items.as_ref().expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].user_id.as_deref(), Some("bega29ca"));
        let nums = items[0]
            .stage_num_lists
            .as_ref()
            .expect("stage_num_lists 应存在");
        assert_eq!(nums[2].task_option_id, Some(3));
        assert_eq!(nums[2].stage_num, Some(0));
        let infos = items[0]
            .stage_task_info_lists
            .as_ref()
            .expect("stage_task_info_lists 应存在");
        assert_eq!(infos[0].stage_task_status.as_deref(), Some("need_todo"));
        assert_eq!(infos[0].task_option_id, Some(1));
        assert_eq!(data.has_more, Some(true));
        assert!(data.page_token.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v1/stage_tasks/find_by_page"
        );
    }
}
