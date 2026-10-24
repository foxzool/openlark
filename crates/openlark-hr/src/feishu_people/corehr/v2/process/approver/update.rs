//! 通过/拒绝审批任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process.approver/update>

use crate::feishu_people::corehr::v2::process_start::create::ProcessFormVariableV2;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 通过/拒绝审批任务请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateApproverBody {
    /// 将审批任务修改为同意/拒绝，可选值：2 拒绝 / 3 通过
    pub status: i32,
    /// 用户 id，按 user_id_type 类型传递；如果 system_approval 为 false，则必填审批任务对应原审批人的 user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 是否为系统身份审批，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_approval: Option<bool>,
    /// 通过/拒绝原因，当拒绝时原因必填（最大长度 1000 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values_v2: Option<Vec<ProcessFormVariableV2>>,
}

/// 通过/拒绝审批任务请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    process_id: Option<String>,
    approver_id: Option<String>,
    /// 请求体
    body: UpdateApproverBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 部门 ID 类型
    department_id_type: Option<String>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
            approver_id: None,
            body: UpdateApproverBody::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = Some(process_id.into());
        self
    }

    /// 设置 `approver_id`。
    pub fn approver_id(mut self, approver_id: impl Into<String>) -> Self {
        self.approver_id = Some(approver_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: UpdateApproverBody) -> Self {
        self.body = body;
        self
    }

    /// 设置审批任务操作状态（2 拒绝 / 3 通过）。
    pub fn status(mut self, status: i32) -> Self {
        self.body.status = status;
        self
    }

    /// 设置用户 id。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = Some(user_id.into());
        self
    }

    /// 设置是否为系统身份审批。
    pub fn system_approval(mut self, system_approval: bool) -> Self {
        self.body.system_approval = Some(system_approval);
        self
    }

    /// 设置通过/拒绝原因（拒绝时必填）。
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.body.reason = Some(reason.into());
        self
    }

    /// 设置表单数据。
    pub fn field_values_v2(mut self, field_values_v2: Vec<ProcessFormVariableV2>) -> Self {
        self.body.field_values_v2 = Some(field_values_v2);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        let process_id = self.process_id.unwrap_or_default();
        let approver_id = self.approver_id.unwrap_or_default();
        validate_required!(process_id.trim(), "process_id 不能为空");
        validate_required!(approver_id.trim(), "approver_id 不能为空");

        let request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/corehr/v2/processes/{process_id}/approvers/{approver_id}"
        ))
        .query_opt("user_id_type", self.user_id_type)
        .query_opt("department_id_type", self.department_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 通过/拒绝审批任务响应（官方 data 为 {code, msg}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 错误码，非 0 表示失败
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 错误描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/corehr/v2/processes/test001/approvers/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/corehr/v2/processes/test001/approvers/test001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {"code": 0, "msg": "success"}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UpdateRequest::new(config)
            .process_id("test001".to_string())
            .approver_id("test001".to_string())
            .status(3)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.code, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
