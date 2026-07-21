//! 批量关闭系统状态
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/batch_close>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量关闭系统状态的请求。
#[derive(Debug, Clone)]
pub struct BatchCloseSystemStatusRequest {
    config: Arc<Config>,
    status_id: String,
    body: BatchCloseSystemStatusBody,
}

/// 批量关闭系统状态请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCloseSystemStatusBody {
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
}

/// 批量关闭系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCloseSystemStatusResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCloseSystemStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 成功但无 `data` 字段时的空成功（batch_close 类 API，与旧 `unwrap_or` 默认值一致）。
    fn empty_success() -> Option<Self> {
        Some(Self { data: None })
    }
}

impl BatchCloseSystemStatusRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, status_id: impl Into<String>) -> Self {
        Self {
            config,
            status_id: status_id.into(),
            body: BatchCloseSystemStatusBody::default(),
        }
    }

    /// 设置用户 ID 列表。
    pub fn user_ids(mut self, ids: Vec<String>) -> Self {
        self.body.user_ids = ids;
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
        validate_required!(self.status_id.trim(), "status_id 不能为空");
        validate_required_list!(
            self.body.user_ids,
            1000,
            "user_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_close",
            self.status_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<BatchCloseSystemStatusResponse> = ApiRequest::post(&path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "批量关闭系统状态").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../system_statuses/{status_id}/batch_close + body{user_ids} → 响应解析。
    #[tokio::test]
    async fn test_batch_close_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_close",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "system_status_id": "ss_001" } }
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
            .user_ids(vec!["u_001".into(), "u_002".into()])
            .execute()
            .await
            .expect("批量关闭系统状态应成功");
        assert_eq!(resp.data.unwrap()["system_status_id"], "ss_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_close"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["user_ids"].as_array().unwrap().len(), 2);
    }
}
