//! 转移应用所有者

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 转移应用所有者的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAppOwnerBody {
    /// 新的拥有者用户 ID（类型由查询参数 user_id_type 确定）。
    pub owner_id: String,
}

impl UpdateAppOwnerBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.owner_id, "owner_id 不能为空");
        Ok(())
    }
}

/// 转移应用所有者的请求。
#[derive(Debug, Clone)]
pub struct UpdateAppOwnerRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `user_id_type`（可选，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
    body: UpdateAppOwnerBody,
}

/// 转移应用所有者的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppOwnerResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateAppOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateAppOwnerRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            user_id_type: None,
            body: UpdateAppOwnerBody::default(),
        }
    }

    /// 设置用户 ID 类型（可选，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置新的拥有者用户 ID（必填，类型由 user_id_type 确定）。
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.body.owner_id = owner_id.into();
        self
    }

    /// 执行转移应用所有者请求。
    pub async fn execute(self) -> SDKResult<UpdateAppOwnerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateAppOwnerResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/owner",
            self.app_id
        );
        let req: ApiRequest<UpdateAppOwnerResponse> = ApiRequest::put(&path)
            .query_opt("user_id_type", self.user_id_type)
            .body(body);

        Transport::request_typed(req, &self.config, Some(option), "转移应用所有者").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../applications/{app_id}/owner → 强类型
    /// UpdateAppOwnerResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_update_owner_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/owner",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "new_owner": "ou_test_owner" } }
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

        let resp = UpdateAppOwnerRequest::new(config, "cli_test_app")
            .owner_id("ou_test_owner")
            .execute()
            .await
            .expect("转移应用所有者应成功");
        assert_eq!(resp.data.unwrap()["new_owner"], "ou_test_owner");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/owner"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["owner_id"], "ou_test_owner");
    }
}
