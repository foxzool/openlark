//! 重启职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/open>

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

/// 重启职位请求体（官方必填 `is_never_expired`；为 `false` 时 `expiry_time` 必填且大于当前时间）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenBody {
    /// 是否长期有效（必填）：`true` 长期有效 / `false` 指定到期日期
    pub is_never_expired: bool,
    /// 到期日期，毫秒时间戳；`is_never_expired` 为 `false` 时必填且大于当前时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<i64>,
}

/// 重启职位请求
#[derive(Debug, Clone)]
pub struct OpenRequest {
    job_id: String,
    /// 配置信息
    config: Config,
    body: OpenBody,
}

impl OpenRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            job_id: String::new(),
            config,
            body: OpenBody {
                is_never_expired: true,
                expiry_time: None,
            },
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }

    /// 设置是否长期有效（必填，官方 `is_never_expired`，默认 `true`）。
    pub fn is_never_expired(mut self, is_never_expired: bool) -> Self {
        self.body.is_never_expired = is_never_expired;
        self
    }

    /// 设置到期日期（毫秒时间戳，官方 `expiry_time`；`is_never_expired` 为 `false` 时必填）。
    pub fn expiry_time(mut self, expiry_time: i64) -> Self {
        self.body.expiry_time = Some(expiry_time);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OpenResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");
        if !self.body.is_never_expired && self.body.expiry_time.is_none() {
            return Err(openlark_core::error::validation_error(
                "expiry_time 不能为空",
                "is_never_expired 为 false 时必须指定到期日期 expiry_time",
            ));
        }

        let api_endpoint = HireApiV1::JobOpen(self.job_id);
        let request = ApiRequest::<OpenResponse>::post(api_endpoint.to_url()).body(
            serde_json::to_value(&self.body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?,
        );
        Transport::request_typed(request, &self.config, Some(option), "重启职位响应数据为空").await
    }
}

/// 重启职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for OpenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn test_open_request_builder_new() {
        let request = OpenRequest::new(TestConfigBuilder::new().build()).job_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_open_request_validation_fails_on_default_request() {
        let request = OpenRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    /// 端到端：POST /open-apis/hire/v1/jobs/test001/open，请求体按官方形状（is_never_expired 必填）
    #[tokio::test]
    async fn test_open_sends_official_body_shape() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/jobs/test001/open"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        OpenRequest::new(config)
            .job_id("test001".to_string())
            .is_never_expired(false)
            .expiry_time(1830259120000)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({ "is_never_expired": false, "expiry_time": 1830259120000i64 })
        );
    }

    /// is_never_expired=false 且未填 expiry_time 时快速失败
    #[tokio::test]
    async fn test_open_rejects_missing_expiry_time_when_not_never_expired() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = OpenRequest::new(config)
            .job_id("test001".to_string())
            .is_never_expired(false)
            .execute()
            .await
            .expect_err("缺 expiry_time 应被拒绝");
        assert!(err.to_string().contains("expiry_time"), "实际: {err}");
    }
}
