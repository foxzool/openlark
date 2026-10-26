//! 查询人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询人工任务 Builder
#[derive(Debug, Clone)]
pub struct UserTaskQueryRequestBuilder {
    config: Config,
    /// 请求体
    body: UserTaskQueryBody,
}

impl UserTaskQueryRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: UserTaskQueryBody {
                kunlun_user_id: String::new(),
                task_type: None,
                source: None,
                limit: None,
                offset: None,
                start_time: None,
                end_time: None,
                api_ids: None,
            },
        }
    }

    /// 设置 kunlunUserID（必填，可通过 aPaaS 用户管理页面获取）
    pub fn kunlun_user_id(mut self, kunlun_user_id: impl Into<String>) -> Self {
        self.body.kunlun_user_id = kunlun_user_id.into();
        self
    }

    /// 设置类型（archived 已完成 / pending 待处理）
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.body.task_type = Some(task_type.into());
        self
    }

    /// 设置来源（fromMe 我发起的 / assignMe 分配给我的 / CCMe 抄送我的）
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.body.source = Some(source.into());
        self
    }

    /// 设置获取条数（1～50）
    pub fn limit(mut self, limit: impl Into<String>) -> Self {
        self.body.limit = Some(limit.into());
        self
    }

    /// 设置起始位置
    pub fn offset(mut self, offset: impl Into<String>) -> Self {
        self.body.offset = Some(offset.into());
        self
    }

    /// 设置开始时间（毫秒）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.body.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间（毫秒）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.body.end_time = Some(end_time.into());
        self
    }

    /// 设置流程 apiid 列表（长度范围 1～100）
    pub fn api_ids(mut self, api_ids: Vec<String>) -> Self {
        self.body.api_ids = Some(api_ids);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserTaskQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserTaskQueryResponse> {
        validate_required!(self.body.kunlun_user_id, "kunlunUserID 不能为空");

        let url = "/open-apis/apaas/v1/user_task/query";

        let req: ApiRequest<UserTaskQueryResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "查询人工任务").await
    }
}

/// 查询请求体（官方字段：kunlun_user_id 必填，其余可选且均为 string）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskQueryBody {
    /// kunlunUserID，可通过 aPaaS 用户管理页面获取
    pub kunlun_user_id: String,
    /// 类型：archived 已完成 / pending 待处理
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 来源：fromMe 我发起的 / assignMe 分配给我的 / CCMe 抄送我的
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 获取条数（1～50）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// 起始位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// 开始时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 流程 apiid 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_ids: Option<Vec<String>>,
}

/// 多语内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskI18n {
    /// 多语 Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// 多语对应的文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// 流程实例
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskWorkflowInstance {
    /// 流程实例 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 流程对应的任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<UserTaskI18n>>,
    /// 流程状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 发起人
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskInitiator {
    /// 用户 KunlunID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 摘要信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskSummary {
    /// 摘要名称（多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<UserTaskI18n>,
    /// 摘要值（多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_value: Option<Vec<UserTaskI18n>>,
}

/// 任务信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTask {
    /// 任务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time: Option<String>,
    /// 流程实例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_instance: Option<UserTaskWorkflowInstance>,
    /// 发起人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<UserTaskInitiator>,
    /// 摘要信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarys: Option<Vec<UserTaskSummary>>,
}

/// 查询响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTaskQueryResponse {
    /// 总任务条数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// 任务信息
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<UserTask>,
}

impl ApiResponseTrait for UserTaskQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UserTaskQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type UserTaskQueryBuilder = UserTaskQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_task/query → 官方请求体与响应形状（count + tasks）。
    #[tokio::test]
    async fn test_query_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/user_task/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "count": "10",
                    "tasks": [
                        {
                            "task_id": "1234",
                            "task_start_time": "1234",
                            "workflow_instance": {
                                "id": "1234",
                                "label": [
                                    { "language_code": "2052", "text": "任务名称" }
                                ],
                                "status": "in_process"
                            },
                            "initiator": {
                                "user_id": "1234",
                                "name": "发起人名称"
                            },
                            "summarys": [
                                {
                                    "file_key": { "language_code": "2052", "text": "摘要名称" },
                                    "file_value": [
                                        { "language_code": "2052", "text": "摘要值" }
                                    ]
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

        let resp = UserTaskQueryRequestBuilder::new(config)
            .kunlun_user_id("1234")
            .task_type("pending")
            .source("assignMe")
            .limit("10")
            .offset("0")
            .api_ids(vec!["package_b40c28__c__action_aadfv6lfu6kai".to_string()])
            .execute()
            .await
            .expect("查询人工任务应成功");
        assert_eq!(resp.count.as_deref(), Some("10"));
        assert_eq!(resp.tasks.len(), 1);
        assert_eq!(resp.tasks[0].task_id.as_deref(), Some("1234"));
        assert_eq!(
            resp.tasks[0]
                .workflow_instance
                .as_ref()
                .and_then(|w| w.id.clone()),
            Some("1234".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_task/query"
        );
        // 请求体须与官方文档一致：kunlun_user_id + 可选筛选字段（type 为官方键名）
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "kunlun_user_id": "1234",
                "type": "pending",
                "source": "assignMe",
                "limit": "10",
                "offset": "0",
                "api_ids": ["package_b40c28__c__action_aadfv6lfu6kai"]
            })
        );
    }
}
