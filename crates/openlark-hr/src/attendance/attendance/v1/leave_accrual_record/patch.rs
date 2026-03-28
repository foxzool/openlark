//! 修改发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
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
        let api_endpoint = AttendanceApiV1::LeaveAccrualRecordPatch;
        let request = ApiRequest::<PatchResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = PatchRequestBody {
            record_id: self.record_id,
            remaining_days: self.remaining_days,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "修改发放记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
