//! 封存发薪活动
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity/archive>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 封存发薪活动请求
#[derive(Debug, Clone)]
pub struct ArchiveRequest {
    /// 发薪活动 ID（必填）
    activity_id: String,
    /// 配置信息
    config: Config,
}

impl ArchiveRequest {
    /// 创建请求
    pub fn new(config: Config, activity_id: String) -> Self {
        Self {
            activity_id,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ArchiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ArchiveResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        validate_required!(self.activity_id.trim(), "activity_id");

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityArchive(self.activity_id.clone());
        let request = ApiRequest::<ArchiveResponse>::post(api_endpoint.to_url());

        // 2. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 3. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "封存发薪活动响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 封存发薪活动响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveResponse {
    /// 是否成功
    pub success: bool,
    /// 发薪活动 ID
    pub activity_id: String,
    /// 封存时间（Unix 时间戳）
    pub archived_at: i64,
}

impl ApiResponseTrait for ArchiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_archive_payment_activity_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"success": true, "activity_id": "act_001", "archived_at": 1700000000}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_activitys/archive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ArchiveRequest::new(config, "act_001".to_string())
            .execute()
            .await
            .expect("封存发薪活动应成功");

        assert!(data.success);
        assert_eq!(data.activity_id, "act_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/payment_activitys/archive"
        );
    }
}
