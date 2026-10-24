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
    /// 发薪活动 ID（必填，可通过「查询发薪活动列表」获取）
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

        validate_required!(self.activity_id, "activity_id 不能为空");

        // 1. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityArchive(self.activity_id.clone());
        let request = ApiRequest::<ArchiveResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体（官方必填 body：activity_id）
        let request_body = ArchiveRequestBody {
            activity_id: self.activity_id,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "封存发薪活动响应数据为空",
        )
        .await
    }
}

/// 封存发薪活动请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveRequestBody {
    /// 发薪活动 ID，可通过「查询发薪活动列表」获取
    pub activity_id: String,
}

/// 封存发薪活动响应（官方 data 为空对象，成功与否以 code/msg 判定）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveResponse {}

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
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例：data 为空对象
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_activitys/archive"))
            .and(body_json(json!({"activity_id": "7433255159377444875"})))
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

        ArchiveRequest::new(config, "7433255159377444875".to_string())
            .execute()
            .await
            .expect("封存发薪活动应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/payment_activitys/archive"
        );
    }
}
