//! 批量开启系统状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/batch_open>

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

use super::models::{SystemStatusUserOpenParam, SystemStatusUserOpenResult};

/// 批量开启系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusBatchOpenRequest {
    config: Arc<Config>,
    /// 路径参数 `system_status_id`。
    system_status_id: String,
    /// 查询参数 `user_id_type`。
    user_id_type: Option<String>,
    body: SystemStatusBatchOpenBody,
}

/// 批量开启系统状态请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemStatusBatchOpenBody {
    /// 用户列表（官方字段 `user_list`，元素含 `user_id` + `end_time`）。
    pub user_list: Vec<SystemStatusUserOpenParam>,
}

/// 批量开启系统状态响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusBatchOpenResponse {
    /// 开启结果列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<SystemStatusUserOpenResult>>,
}

impl ApiResponseTrait for SystemStatusBatchOpenResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SystemStatusBatchOpenRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, system_status_id: impl Into<String>) -> Self {
        Self {
            config,
            system_status_id: system_status_id.into(),
            user_id_type: None,
            body: SystemStatusBatchOpenBody::default(),
        }
    }

    /// 设置用户 ID 类型（查询参数）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置用户列表（body.`user_list`）。
    pub fn user_list(mut self, user_list: Vec<SystemStatusUserOpenParam>) -> Self {
        self.body.user_list = user_list;
        self
    }

    /// 执行批量开启系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusBatchOpenResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusBatchOpenResponse> {
        validate_required!(self.system_status_id.trim(), "system_status_id 不能为空");
        validate_required_list!(
            self.body.user_list,
            50,
            "user_list 不能为空且不能超过 50 个"
        );
        for item in &self.body.user_list {
            validate_required!(item.user_id.trim(), "user_list[].user_id 不能为空");
            validate_required!(item.end_time.trim(), "user_list[].end_time 不能为空");
        }

        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_open",
            self.system_status_id
        );
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SystemStatusBatchOpenResponse> = ApiRequest::post(&path)
            .query_opt("user_id_type", self.user_id_type.as_ref())
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "批量开启系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../batch_open + body{user_list:[{user_id,end_time}]}。
    #[tokio::test]
    async fn test_batch_open_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_open",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "result_list": [{
                        "user_id": "ou_1",
                        "end_time": "1665990378",
                        "result": "success_show"
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

        let resp = SystemStatusBatchOpenRequest::new(config, "ss_001")
            .user_list(vec![SystemStatusUserOpenParam {
                user_id: "ou_1".into(),
                end_time: "1665990378".into(),
            }])
            .execute()
            .await
            .expect("批量开启系统状态应成功");
        assert_eq!(
            resp.result_list.as_ref().unwrap()[0].result.as_deref(),
            Some("success_show")
        );

        let received = server.received_requests().await.unwrap_or_default();
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["user_list"][0]["user_id"], "ou_1");
        assert_eq!(sent["user_list"][0]["end_time"], "1665990378");
    }
}
