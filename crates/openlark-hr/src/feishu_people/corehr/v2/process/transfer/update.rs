//! 转交审批任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process.transfer/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 转交审批任务请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBody {
    /// 被转交人（必填，按 user_id_type 类型传递）
    pub to_user_id: String,
    /// 待转交审批任务 id 列表（必填，1～20 个）
    pub approver_ids: Vec<String>,
    /// 操作人（按 user_id_type 类型传递；system_user 为 true 时可不填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 是否以系统身份操作（false 时 operator 必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_user: Option<bool>,
}

/// 转交审批任务请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    process_id: Option<String>,
    /// 请求体
    body: UpdateBody,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
            body: UpdateBody::default(),
            user_id_type: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = Some(process_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: UpdateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置被转交人（必填，按 user_id_type 类型传递）。
    pub fn to_user_id(mut self, to_user_id: impl Into<String>) -> Self {
        self.body.to_user_id = to_user_id.into();
        self
    }

    /// 设置待转交审批任务 id 列表（必填，1～20 个）。
    pub fn approver_ids(mut self, approver_ids: Vec<String>) -> Self {
        self.body.approver_ids = approver_ids;
        self
    }

    /// 设置操作人（system_user 为 false 时必填）。
    pub fn operator(mut self, operator: impl Into<String>) -> Self {
        self.body.operator = Some(operator.into());
        self
    }

    /// 设置备注。
    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.body.remark = Some(remark.into());
        self
    }

    /// 设置是否以系统身份操作。
    pub fn system_user(mut self, system_user: bool) -> Self {
        self.body.system_user = Some(system_user);
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
        validate_required!(process_id.trim(), "process_id 不能为空");
        validate_required!(self.body.to_user_id, "to_user_id 不能为空");
        validate_required_list!(
            self.body.approver_ids,
            20,
            "approver_ids 不能为空且最多 20 个"
        );

        let request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/corehr/v2/processes/{process_id}/transfer"
        ))
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 转交审批任务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 响应数据
    /// process信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessInfo>,
}

/// 流程信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessInfo {
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT /open-apis/corehr/v2/processes/test001/transfer
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/corehr/v2/processes/test001/transfer"))
            .and(body_json(json!({
                "operator": "7184703091806602796",
                "to_user_id": "7184703091806602796",
                "approver_ids": ["7424452160928106028"],
                "remark": "备注",
                "system_user": true
            })))
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

        UpdateRequest::new(config)
            .process_id("test001".to_string())
            .to_user_id("7184703091806602796")
            .approver_ids(vec!["7424452160928106028".to_string()])
            .operator("7184703091806602796")
            .remark("备注")
            .system_user(true)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    /// to_user_id 缺失时快速失败
    #[tokio::test]
    async fn test_update_rejects_empty_to_user_id() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("http://127.0.0.1:1")
            .enable_token_cache(false)
            .build();

        let err = UpdateRequest::new(config)
            .process_id("test001")
            .approver_ids(vec!["7424452160928106028".to_string()])
            .execute()
            .await;
        assert!(err.is_err());
    }
}
