//! 审批设备申报
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_apply_record-approve>
//!
//! 文档核对：`PUT /open-apis/security_and_compliance/v2/device_apply_records/{device_apply_record_id}`，
//! body 仅一个必填字段 `{is_approved: boolean}`。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};
use serde::{Deserialize, Serialize};

/// 审批设备申报请求 body（官方请求体仅一个必填字段 `is_approved`）。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ApproveDeviceApplyRecordBody {
    /// 是否审批通过。
    pub is_approved: bool,
}

/// 审批设备申报请求
#[derive(Debug)]
pub struct ApproveDeviceApplyRecordRequest {
    /// 配置信息。
    config: Config,
    /// 申报记录 ID（路径参数，必填）。
    device_apply_record_id: String,
    /// 请求 body。
    body: ApproveDeviceApplyRecordBody,
}

impl ApproveDeviceApplyRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_apply_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_apply_record_id: device_apply_record_id.into(),
            body: ApproveDeviceApplyRecordBody { is_approved: false },
        }
    }

    /// 设置是否审批通过。
    pub fn is_approved(mut self, is_approved: bool) -> Self {
        self.body.is_approved = is_approved;
        self
    }

    /// 快速批准。
    pub fn approve(self) -> Self {
        self.is_approved(true)
    }

    /// 快速拒绝。
    pub fn reject(self) -> Self {
        self.is_approved(false)
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(
            self.device_apply_record_id,
            "device_apply_record_id 不能为空"
        );

        let path = format!(
            "/open-apis/security_and_compliance/v2/device_apply_records/{}",
            self.device_apply_record_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(&path)
            .body(
                serde_json::to_value(self.body)
                    .map_err(|e| validation_error("审批设备申报", format!("序列化失败: {e}")))?,
            )
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "审批设备申报").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_approve_rejects_empty_id() {
        let req = ApproveDeviceApplyRecordRequest::new(test_config(), "").approve();
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("device_apply_record_id")
        );
    }

    /// 端到端：PUT .../device_apply_records/{id} + body {is_approved} 序列化。
    #[tokio::test]
    async fn test_approve_device_apply_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/security_and_compliance/v2/device_apply_records/rec_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "device_apply_record_id": "rec_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ApproveDeviceApplyRecordRequest::new(config, "rec_001")
            .approve()
            .execute()
            .await
            .expect("审批设备申报应成功");
        assert_eq!(data["device_apply_record_id"], "rec_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/security_and_compliance/v2/device_apply_records/rec_001"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        // 官方请求体仅一个必填字段 is_approved
        assert_eq!(sent, json!({ "is_approved": true }));
    }
}
