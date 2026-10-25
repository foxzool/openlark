//! 更新职位相关人员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job.manager/batch_update>

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

use crate::hire::hire::common_models::JobManagerOperationResult;

/// 更新职位相关人员请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRequest {
    job_id: Option<String>,
    request_body: BatchUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl BatchUpdateRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: BatchUpdateRequestBody) -> Self {
        Self {
            job_id: None,
            request_body,
            config,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: BatchUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchUpdateResponse> {
        self.request_body.validate()?;
        let job_id = self.job_id.unwrap_or_default();
        validate_required!(job_id.trim(), "job_id 不能为空");

        let request = ApiRequest::<BatchUpdateResponse>::post(format!(
            "/open-apis/hire/v1/jobs/{job_id}/managers/batch_update"
        ));
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新职位相关人员响应数据为空",
        )
        .await
    }
}

/// `BatchUpdateRequestBody`。
///
/// 各人员字段在 `update_option_list` 包含对应人员类型时必填。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRequestBody {
    /// 招聘负责人 ID，需与 `user_id_type` 查询参数类型一致；
    /// 当 `update_option_list` 包含「招聘负责人」时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruiter_id: Option<String>,
    /// 招聘协助人 ID 列表，需与 `user_id_type` 查询参数类型一致；
    /// 当 `update_option_list` 包含「招聘协助人」时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id_list: Option<Vec<String>>,
    /// 用人经理 ID 列表，需与 `user_id_type` 查询参数类型一致；
    /// 当 `update_option_list` 包含「用人经理」时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiring_manager_id_list: Option<Vec<String>>,
    /// 更新的人员类型（必填）：1 招聘负责人 / 2 招聘协助人 / 3 用人经理。
    pub update_option_list: Vec<i32>,
    /// 操作者 ID，需与 `user_id_type` 查询参数类型一致，不填默认系统操作。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

impl BatchUpdateRequestBody {
    /// 创建新的请求实例（`update_option_list` 为官方必填字段）。
    pub fn new(update_option_list: Vec<i32>) -> Self {
        Self {
            recruiter_id: None,
            assistant_id_list: None,
            hiring_manager_id_list: None,
            update_option_list,
            creator_id: None,
        }
    }

    /// 设置招聘负责人 ID。
    pub fn recruiter_id(mut self, value: impl Into<String>) -> Self {
        self.recruiter_id = Some(value.into());
        self
    }

    /// 设置招聘协助人 ID 列表。
    pub fn assistant_id_list(mut self, value: Vec<String>) -> Self {
        self.assistant_id_list = Some(value);
        self
    }

    /// 设置用人经理 ID 列表。
    pub fn hiring_manager_id_list(mut self, value: Vec<String>) -> Self {
        self.hiring_manager_id_list = Some(value);
        self
    }

    /// 设置操作者 ID。
    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(
            self.update_option_list,
            "update_option_list 不能为空且需传入要更新的人员类型"
        );

        Ok(())
    }
}

/// 职位负责人（官方 `job.manager`，响应侧形态）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘负责人 ID，与 `user_id_type` 查询参数类型一致。
    pub recruiter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用人经理 ID 列表，与 `user_id_type` 查询参数类型一致。
    pub hiring_manager_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘协助人 ID 列表，与 `user_id_type` 查询参数类型一致。
    pub assistant_id_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 更新职位相关人员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: JobManagerOperationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位负责人。
    pub job_manager: Option<JobManagerView>,
}

impl ApiResponseTrait for BatchUpdateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/jobs/test001/managers/batch_update
    #[tokio::test]
    async fn test_batch_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/jobs/test001/managers/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "job_manager": {
                        "id": "1618209327096",
                        "recruiter_id": "1612209227096",
                        "hiring_manager_id_list": ["on_94a1ee5551019f18cd73f9f111898cf2"]
                    }
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

        let body = BatchUpdateRequestBody::new(vec![1, 3])
            .recruiter_id("ou_e6139117c300506837def50545420c6a")
            .hiring_manager_id_list(vec!["ou_808dad5b8fb21416b4ff242a5c616016".to_string()])
            .creator_id("ou_efk39117c300506837def50545420c6a");

        let resp = BatchUpdateRequest::new(config, body)
            .job_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        let job_manager = resp
            .job_manager
            .as_ref()
            .expect("job_manager 应被类型化解析");
        assert_eq!(job_manager.id.as_deref(), Some("1618209327096"));
        assert_eq!(job_manager.recruiter_id.as_deref(), Some("1612209227096"));
        assert_eq!(
            job_manager.hiring_manager_id_list.as_deref(),
            Some(&["on_94a1ee5551019f18cd73f9f111898cf2".to_string()][..])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({
                "recruiter_id": "ou_e6139117c300506837def50545420c6a",
                "hiring_manager_id_list": ["ou_808dad5b8fb21416b4ff242a5c616016"],
                "update_option_list": [1, 3],
                "creator_id": "ou_efk39117c300506837def50545420c6a"
            })
        );
    }

    /// `update_option_list` 为空时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_batch_update_rejects_empty_update_options() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = BatchUpdateRequest::new(config, BatchUpdateRequestBody::new(Vec::new()))
            .job_id("test001".to_string())
            .execute()
            .await;
        assert!(result.is_err());
    }
}
