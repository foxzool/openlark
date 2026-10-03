//! 批量关闭系统状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/batch_close>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::models::SystemStatusUserCloseResult;

/// 批量关闭系统状态的请求。
#[derive(Debug, Clone)]
pub struct BatchCloseSystemStatusRequest {
    config: Arc<Config>,
    /// 路径参数 `system_status_id`。
    system_status_id: String,
    /// 查询参数 `user_id_type`。
    user_id_type: Option<String>,
    body: BatchCloseSystemStatusBody,
}

/// 批量关闭系统状态请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCloseSystemStatusBody {
    /// 用户 ID 列表（官方字段名 `user_list`，长度 1～50）。
    pub user_list: Vec<String>,
}

/// 批量关闭系统状态响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCloseSystemStatusResponse {
    /// 关闭结果列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<SystemStatusUserCloseResult>>,
}

impl ApiResponseTrait for BatchCloseSystemStatusResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl BatchCloseSystemStatusRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, system_status_id: impl Into<String>) -> Self {
        Self {
            config,
            system_status_id: system_status_id.into(),
            user_id_type: None,
            body: BatchCloseSystemStatusBody::default(),
        }
    }

    /// 设置用户 ID 类型（查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置用户 ID 列表（body.`user_list`）。
    pub fn user_list(mut self, ids: Vec<String>) -> Self {
        self.body.user_list = ids;
        self
    }

    /// 执行批量关闭系统状态请求。
    pub async fn execute(self) -> SDKResult<BatchCloseSystemStatusResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCloseSystemStatusResponse> {
        validate_required!(self.system_status_id.trim(), "system_status_id 不能为空");
        validate_required_list!(
            self.body.user_list,
            50,
            "user_list 不能为空且不能超过 50 个"
        );

        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_close",
            self.system_status_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<BatchCloseSystemStatusResponse> = ApiRequest::post(&path)
            .query_opt("user_id_type", self.user_id_type.as_ref())
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request_typed(req, &self.config, Some(option), "批量关闭系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../batch_close + body{user_list} → 响应解析。
    #[tokio::test]
    async fn test_batch_close_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_close",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "Success",
                "data": {
                    "result_list": [{
                        "user_id": "ou_1",
                        "result": "Success"
                    }]
                }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = BatchCloseSystemStatusRequest::new(config, "ss_001")
            .user_id_type("open_id")
            .user_list(vec!["ou_1".into(), "ou_2".into()])
            .execute()
            .await
            .expect("批量关闭系统状态应成功");
        assert_eq!(resp.result_list.as_ref().unwrap().len(), 1);
        assert_eq!(
            resp.result_list.as_ref().unwrap()[0].user_id.as_deref(),
            Some("ou_1")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["user_list"].as_array().unwrap().len(), 2);
        assert!(sent.get("user_ids").is_none());
    }
}
