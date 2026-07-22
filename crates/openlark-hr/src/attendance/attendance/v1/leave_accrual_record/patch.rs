//! 修改发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改发放记录请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 发放记录 ID（必填）
    record_id: String,
    /// 剩余天数（必填）
    remaining_days: f64,
    /// 配置信息
    config: Config,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config, record_id: String, remaining_days: f64) -> Self {
        Self {
            record_id,
            remaining_days,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.record_id.trim(), "record_id");

        // 2. 构建端点
        let api_endpoint =
            AttendanceApiV1::LeaveAccrualRecordPatch(self.record_id.clone()).to_url();
        let request = ApiRequest::<PatchResponse>::patch(&api_endpoint);

        // 3. 构建请求体
        let request_body = PatchRequestBody {
            record_id: self.record_id,
            remaining_days: self.remaining_days,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "修改发放记录响应数据为空",
        )
        .await
    }
}

/// 修改发放记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 发放记录 ID
    pub record_id: String,
    /// 剩余天数
    pub remaining_days: f64,
}

/// 修改发放记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 是否成功
    pub success: bool,
    /// 发放记录 ID
    pub record_id: String,
    /// 修改后的剩余天数
    pub remaining_days: f64,
}

impl ApiResponseTrait for PatchResponse {
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
    async fn test_attendance_v1_leave_accrual_record_patch_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"success": false, "record_id": "test", "remaining_days": 0.0}"#,
        )
        .unwrap();
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/attendance/v1/leave_accrual_record/record_001",
            ))
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

        let data = PatchRequest::new(config, "record_001".to_string(), 1.0)
            .execute()
            .await
            .expect("attendance_v1_leave_accrual_record_patch 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/leave_accrual_record/record_001"
        );
    }
}
