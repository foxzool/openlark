//! 撤销异动
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_change/revoke>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 撤销异动请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeBody {
    /// 操作人 id（必填，关联了流程的异动需要操作人有流程管理员和审批单管理员权限）
    pub operator_id: String,
}

/// RevokeRequest
#[derive(Debug, Clone)]
pub struct RevokeRequest {
    /// 配置信息
    config: Config,
    /// 异动 ID
    job_change_id: Option<String>,
    /// 请求体
    body: RevokeBody,
    /// 用户 ID 类型（open_id/union_id/user_id/people_admin_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl RevokeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_change_id: None,
            body: RevokeBody::default(),
            user_id_type: None,
        }
    }

    /// 设置异动 ID
    pub fn job_change_id(mut self, job_change_id: impl Into<String>) -> Self {
        self.job_change_id = Some(job_change_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: RevokeBody) -> Self {
        self.body = body;
        self
    }

    /// 设置操作人 id（必填）。
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.body.operator_id = operator_id.into();
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_admin_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RevokeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RevokeResponse> {
        let job_change_id = self.job_change_id.unwrap_or_default();
        validate_required!(job_change_id.trim(), "job_change_id 不能为空");
        validate_required!(self.body.operator_id, "operator_id 不能为空");

        let request = ApiRequest::<RevokeResponse>::post(format!(
            "/open-apis/corehr/v2/job_changes/{job_change_id}/revoke"
        ))
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// RevokeResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevokeResponse {
    /// 撤销结果
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    /// 撤销信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponseTrait for RevokeResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/job_changes/test001/revoke
    #[tokio::test]
    async fn test_revoke_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_changes/test001/revoke"))
            .and(body_json(json!({
                "operator_id": "ou_a294793e8fa21529f2a60e3e9de45520"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "result": true, "message": "成功" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RevokeRequest::new(config)
            .job_change_id("test001".to_string())
            .operator_id("ou_a294793e8fa21529f2a60e3e9de45520")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.result, Some(true));
        assert_eq!(resp.message, Some("成功".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
