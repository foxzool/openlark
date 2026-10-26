//! 更新应用审核状态
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用审核状态的请求体（`status` 可选：0-未知 / 1-审核通过 / 2-审核拒绝 / 3-审核中 / 4-未提交审核）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchApplicationAppVersionBody {
    /// 版本状态（示例值：1）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新应用审核状态的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationAppVersionRequest {
    config: Arc<Config>,
    app_id: String,
    resource_id: String,
    /// 查询参数 `user_id_type`（必填，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
    /// 查询参数 `operator_id`（必填，操作者的 open_id）。
    operator_id: Option<String>,
    /// 查询参数 `reject_reason`（可选，修改版本状态为被驳回时必填）。
    reject_reason: Option<String>,
    body: PatchApplicationAppVersionBody,
}

/// 更新应用审核状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationAppVersionResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationAppVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationAppVersionRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        app_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            resource_id: resource_id.into(),
            user_id_type: None,
            operator_id: None,
            reject_reason: None,
            body: PatchApplicationAppVersionBody::default(),
        }
    }

    /// 设置用户 ID 类型（必填，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置操作者的 open_id（必填）。
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.operator_id = Some(operator_id.into());
        self
    }

    /// 设置驳回理由（可选，修改版本状态为被驳回时必填）。
    pub fn reject_reason(mut self, reject_reason: impl Into<String>) -> Self {
        self.reject_reason = Some(reject_reason.into());
        self
    }

    /// 设置版本状态（可选：0-未知 / 1-审核通过 / 2-审核拒绝 / 3-审核中 / 4-未提交审核）。
    pub fn status(mut self, status: i32) -> Self {
        self.body.status = Some(status);
        self
    }

    /// 执行更新应用审核状态请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationAppVersionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationAppVersionResponse> {
        // user_id_type / operator_id 官方必填，空白字符串视为未填写。
        let user_id_type = self
            .user_id_type
            .ok_or_else(|| CoreError::validation_msg("user_id_type 不能为空"))?;
        validate_required!(user_id_type, "user_id_type 不能为空");
        let operator_id = self
            .operator_id
            .ok_or_else(|| CoreError::validation_msg("operator_id 不能为空"))?;
        validate_required!(operator_id, "operator_id 不能为空");

        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            self.app_id, self.resource_id
        );
        let req: ApiRequest<PatchApplicationAppVersionResponse> = ApiRequest::patch(&path)
            .query("user_id_type", user_id_type)
            .query("operator_id", operator_id)
            .query_opt("reject_reason", self.reject_reason)
            .body(body);

        Transport::request_typed(req, &self.config, Some(option), "更新应用审核状态").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../app_versions/{version_id}?user_id_type=open_id&operator_id=ou_op → 强类型 PatchApplicationAppVersionResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_application_app_version_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_versions/ver_1",
            ))
            .and(query_param("user_id_type", "open_id"))
            .and(query_param("operator_id", "ou_operator"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app", "version_id": "ver_1" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = PatchApplicationAppVersionRequest::new(config, "cli_test_app", "ver_1")
            .user_id_type("open_id")
            .operator_id("ou_operator")
            .status(1)
            .execute()
            .await
            .expect("更新应用审核状态应成功");
        let data = resp.data.unwrap();
        assert_eq!(data["app_id"], "cli_test_app");
        assert_eq!(data["version_id"], "ver_1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_versions/ver_1"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["status"], 1);
    }
}
