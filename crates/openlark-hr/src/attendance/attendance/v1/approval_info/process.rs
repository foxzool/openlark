//! 通知审批状态更新
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/approval_info/process>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 通知审批状态更新请求
#[derive(Debug, Clone)]
pub struct ProcessRequest {
    /// 审批实例 ID（必填）
    approval_id: String,
    /// 审批类型（必填）：`leave`(请假) / `out`(外出) / `overtime`(加班) / `trip`(出差) / `remedy`(补卡)
    approval_type: String,
    /// 审批状态（必填）：`1`(不通过) / `2`(通过) / `4`(撤销)
    status: i32,
    /// 配置信息
    config: Config,
}

impl ProcessRequest {
    /// 创建请求
    ///
    /// - `approval_id`: 审批实例 ID
    /// - `approval_type`: 审批类型（`leave` / `out` / `overtime` / `trip` / `remedy`）
    /// - `status`: 审批状态（`1`=不通过, `2`=通过, `4`=撤销）
    pub fn new(config: Config, approval_id: String, approval_type: String, status: i32) -> Self {
        Self {
            approval_id,
            approval_type,
            status,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ProcessResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProcessResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.approval_id.trim(), "approval_id");
        validate_required!(self.approval_type.trim(), "approval_type");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ApprovalInfoProcess;
        let request = ApiRequest::<ProcessResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = ProcessRequestBody {
            approval_id: self.approval_id,
            approval_type: self.approval_type,
            status: self.status,
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
            "通知审批状态更新响应数据为空",
        )
        .await
    }
}

/// 通知审批状态更新请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequestBody {
    /// 审批实例 ID
    pub approval_id: String,
    /// 审批类型（`leave` / `out` / `overtime` / `trip` / `remedy`）
    pub approval_type: String,
    /// 审批状态（`1`=不通过, `2`=通过, `4`=撤销）
    pub status: i32,
}

/// 通知审批状态更新响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessResponse {
    /// 审批信息
    pub approval_info: ApprovalInfo,
}

/// 审批信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApprovalInfo {
    /// 审批实例 ID
    pub approval_id: String,
    /// 审批类型
    pub approval_type: String,
    /// 审批状态（`0`=待审批, `1`=未通过, `2`=已通过, `3`=已取消, `4`=已撤回）
    pub status: i32,
}

impl ApiResponseTrait for ProcessResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_process_request_builder_new() {
        let request = ProcessRequest::new(
            TestConfigBuilder::new().build(),
            "6737202939523236113".to_string(),
            "remedy".to_string(),
            4,
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_approval_info_process_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 飞书官网真实 response schema：data.approval_info.{approval_id, approval_type, status}
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/approval_infos/process"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "approval_info": {
                        "approval_id": "6737202939523236113",
                        "approval_type": "remedy",
                        "status": 0
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ProcessRequest::new(
            config,
            "6737202939523236113".to_string(),
            "remedy".to_string(),
            4,
        )
        .execute()
        .await
        .expect("attendance_v1_approval_info_process 应成功");

        // 响应解析对齐官网 schema（嵌套 approval_info）
        assert_eq!(data.approval_info.approval_id, "6737202939523236113");
        assert_eq!(data.approval_info.approval_type, "remedy");
        assert_eq!(data.approval_info.status, 0);

        // 请求体对齐官网必填字段，且不含旧错误字段
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/approval_infos/process"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"approval_id\""),
            "请求体缺 approval_id: {body}"
        );
        assert!(
            body.contains("\"approval_type\""),
            "请求体缺 approval_type: {body}"
        );
        assert!(body.contains("\"status\""), "请求体缺 status: {body}");
        assert!(
            !body.contains("approval_instance_id"),
            "请求体不应含旧字段 approval_instance_id: {body}"
        );
        assert!(
            !body.contains("\"result\""),
            "请求体不应含旧字段 result: {body}"
        );
    }
}
